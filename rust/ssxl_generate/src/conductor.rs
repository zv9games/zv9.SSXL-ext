// ssxl_generate/src/conductor.rs

//! The Conductor: Central command and control for the SSXL procedural generation runtime.
//!
//! The Conductor initializes all core components (runtime, cache, generators) and manages
//! the asynchronous request and progress channels, ensuring thread-safe, high-speed
//! world generation. This is the **Conductor Genesis** system.

use tokio::sync::mpsc::{self, Sender};
use tracing::{info, error};
use std::sync::{Arc, Mutex};
use std::io;

use ssxl_math::Vec2i;
use ssxl_cache::ChunkCache;
use ssxl_shared::chunk_data::ChunkData;
use ssxl_tools::get_config_from_path;

use crate::runtime_manager::RuntimeManager;
use crate::config_validator::{ConfigValidator, GeneratorConfig};
use crate::task_queue::{
    handle_chunk_unit, start_request_loop,
    GenerationTask as ChunkRequest, GenerationMessage,
};
pub use crate::conductor_state::{ConductorState, ConductorStatus};
use crate::generator_manager::GeneratorManager;
use crate::batch_processor::spawn_batch_generation_task;

// --- 1. Channel Constants ---

/// Bound for the progress channel (Conductor -> External World). Generous buffer for status updates.
const PROGRESS_CHANNEL_BOUND: usize = 1024;

// --- 2. Conductor Structure ---

/// The core runtime manager and orchestrator for the SSXL Engine.
pub struct Conductor {
    /// Manages the Tokio asynchronous runtime.
    runtime_manager: RuntimeManager,
    /// Manages all registered procedural generation algorithms.
    generator_manager: GeneratorManager,
    /// Thread-safe state tracking (Status, Active Generator ID).
    internal_state: ConductorState,
    /// Thread-safe, in-memory cache for generated chunk data (The **crypto coded memory**).
    chunk_cache: Arc<Mutex<ChunkCache>>,
    /// Sender for progress updates (e.g., ChunkGenerated, GenerationComplete).
    progress_sender: Sender<GenerationMessage>,
    /// Sender for new chunk requests. Stored internally for potential control signals (e.g., clearing the queue).
    request_sender: mpsc::UnboundedSender<ChunkRequest>, // 💡 ADDED: Internal handle for the request channel.
}

impl Conductor {
    /// Initializes the entire SSXL generation runtime.
    ///
    /// This function sets up the Tokio runtime, the cache, the generators, and starts
    /// the main request processing loop on a background task.
    ///
    /// # Returns
    /// A tuple containing: (The Conductor itself, ConductorState for external monitoring,
    /// progress receiver, and the unbounded request sender).
    pub fn new(config_path: Option<&str>) -> Result<(Self, ConductorState, mpsc::Receiver<GenerationMessage>, mpsc::UnboundedSender<ChunkRequest>), io::Error> {
        
        // Load initial configuration
        let config = get_config_from_path(config_path)?;

        // Initialize the asynchronous runtime manager
        let runtime_manager = RuntimeManager::new()?;
        let handle = runtime_manager.get_handle();

        // Initialize generator manager and prepare map for sharing
        let generator_manager = GeneratorManager::new().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Generator setup failed: {}", e))
        })?;
        let generators_for_loop = Arc::new(generator_manager.get_map_clone());

        // Initialize thread-safe chunk cache
        let chunk_cache = Arc::new(Mutex::new(ChunkCache::new().map_err(|e| {
            error!("Failed to initialize ChunkCache: {:?}", e);
            io::Error::new(io::ErrorKind::Other, format!("Cache initialization failed: {:?}", e))
        })?));
        let chunk_cache_for_loop = chunk_cache.clone();

        // Set up communication channels
        let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);
        // Unbounded for flexibility. This sender is what the external API uses.
        let (request_sender_api, request_receiver) = mpsc::unbounded_channel(); 
        
        // Clone the sender to store inside the Conductor struct for internal control/signals.
        let request_sender_internal = request_sender_api.clone();

        // Initialize state tracker
        let initial_id = generator_manager.get_initial_id(config.get_default_generator_id());
        let state = ConductorState::new(initial_id.clone());
        state.set_status(ConductorStatus::Running);
        let state_for_loop = state.clone();

        info!("Conductor initialized. Active generator: {}", initial_id);

        // Start the request processing loop on the Tokio runtime
        start_request_loop(
            handle,
            request_receiver,
            progress_sender.clone(),
            generators_for_loop,
            chunk_cache_for_loop,
            Arc::new(state_for_loop),
        );

        let conductor = Conductor {
            runtime_manager,
            generator_manager,
            internal_state: state.clone(),
            chunk_cache,
            progress_sender: progress_sender.clone(),
            request_sender: request_sender_internal, // Stored internally
        };

        // Return the conductor and the external interface components (state, receivers, senders)
        // The 'request_sender_api' is returned here for use by the external caller.
        Ok((conductor, state, progress_receiver, request_sender_api))
    }
    
    /// Returns the ID of the generator currently selected for use.
    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }

    /// Starts a large, CPU-intensive batch generation task using `spawn_batch_generation_task`.
    ///
    /// This is the **bulldozer** operation, leveraging Rayon to generate chunks in parallel
    /// on a separate blocking thread pool to maintain the async core's **tempo**.
    pub fn start_generation(&mut self, config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
        ConfigValidator::validate_map_dimensions(&config)?;

        self.set_active_generator(&config.generator_name)?;
        
        self.internal_state.set_status(ConductorStatus::Generating);

        // Spawn the batch task onto the blocking pool via the RuntimeManager's handle.
        spawn_batch_generation_task(
            &self.runtime_manager.get_handle(),
            self.generator_manager.get_map_clone(),
            self.chunk_cache.clone(),
            self.internal_state.get_active_generator_id(),
            self.progress_sender.clone(),
            self.internal_state.clone(),
            config,
        );

        Ok(())
    }

    /// FFI Command: Immediately stops all pending and active generation tasks.
    ///
    /// This method sets the Conductor state to `Stopping`. The request processing loop
    /// (`start_request_loop`) is responsible for observing this state and refusing
    /// to process further `ChunkRequest` tasks, allowing the queue to drain harmlessly.
    pub fn stop_generation(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.internal_state.set_status(ConductorStatus::Stopping);
        info!("Conductor: Global STOP command received. Status set to Stopping.");
        
        // No need to explicitly drop the internal sender here, as relying on the status 
        // check in the worker loop is the most robust way to stop tasks already in the queue.
        Ok(())
    }

    /// Synchronously retrieves a single chunk's data, either from cache or by immediate generation.
    ///
    /// This uses a temporary channel to force a blocking wait for the result, making it
    /// suitable for external FFI calls that require an immediate return value.
    pub fn get_chunk_data(&self, chunk_coords: &Vec2i) -> ChunkData {
        let active_generator_id = self.internal_state.get_active_generator_id();

        // Create a temporary, tightly-bound MPSC channel for a one-shot request/response.
        let (temp_sender, mut temp_receiver) = mpsc::channel(1);

        // Run the chunk generation logic. It checks the cache first.
        handle_chunk_unit(
            *chunk_coords,
            &active_generator_id,
            self.generator_manager.get_map_ref(),
            &self.chunk_cache,
            &temp_sender, // Sender will send result back through this channel.
        );
        
        // Block the current thread and wait for the result from the temporary channel.
        match temp_receiver.blocking_recv() {
            Some(GenerationMessage::ChunkGenerated(_coords, chunk_data_arc)) => {
                // Attempt to unwrap the Arc if possible, otherwise clone the ChunkData.
                Arc::try_unwrap(chunk_data_arc).unwrap_or_else(|arc| (*arc).clone())
            },
            _ => {
                // If the channel closes or sends an unexpected message, panic.
                panic!("get_chunk_data failed to receive ChunkGenerated message.");
            }
        }
    }

    /// Sets the generator to be used for all future requests, if the ID is valid.
    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
        if self.generator_manager.get_map_ref().contains_key(id) {
            info!("Active generator set to: {}", id);
            self.internal_state.set_active_generator_id(id);
            Ok(())
        } else {
            let err = format!("Generator ID '{}' not found. Available IDs: {:?}", id, self.generator_manager.get_map_ref().keys());
            error!("{}", err);
            Err(err)
        }
    }

    /// Runs the generation process for a single chunk immediately, without cache check or channels.
    /// Used mainly for internal structural tests or single-run debug generation.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let active_id = self.internal_state.get_active_generator_id();
        self.generator_manager.generate_single_chunk(chunk_coords, &active_id)
    }

    /// Signals the Conductor's internal state that the batch generation task is complete.
    /// Sets status back to Running (Idle) after a batch generation is done.
    pub fn signal_generation_complete(&self) {
        info!("SSXL Conductor received GenerationComplete. Setting status to Running (Idle).");
        self.internal_state.set_status(ConductorStatus::Running);
    }
    
    /// Signals the Conductor's internal state that shutdown is beginning.
    pub fn signal_shutdown_graceful(&self) {
        info!("SSXL Conductor signaled for shutdown. Setting status to ShuttingDown.");
        self.internal_state.set_status(ConductorStatus::ShuttingDown);
    }

    /// Performs a full, graceful teardown of the entire runtime.
    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        // The RuntimeManager handles shutting down the Tokio threads gracefully.
        self.runtime_manager.shutdown_graceful();
        info!("SSXL Conductor full teardown complete.");
    }
}