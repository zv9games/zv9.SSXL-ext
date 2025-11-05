//! The central manager for the procedural generation pipeline.
//! This struct is now an orchestrator that delegates tasks to dedicated modules.

use tokio::sync::mpsc::{self, Sender};
use tracing::{info, error};
use std::sync::{Arc, Mutex};
use std::io;

// --- CORE DEPENDENCIES (External Crates) ---
use ssxl_math::Vec2i;
use ssxl_cache::ChunkCache;
use ssxl_shared::chunk_data::ChunkData;
use ssxl_tools::get_config_from_path;

// --- DELEGATE MODULES ---
use crate::runtime_manager::RuntimeManager;
use crate::config_validator::{ConfigValidator, GeneratorConfig};
use crate::task_queue::{
    handle_chunk_unit, start_request_loop,
    // Note: The imports below are made public inside task_queue.rs
    GenerationTask as ChunkRequest, GenerationMessage, 
};
pub use crate::conductor_state::{ConductorState, ConductorStatus}; // Added pub use for ConductorStatus
// FIX (E0603): Remove Generator from this import path since it's defined at crate root
use crate::generator_manager::GeneratorManager; 
// FIX (E0603): Import Generator trait directly from crate root, as suggested by the note.
use crate::Generator; 
use crate::batch_processor::spawn_batch_generation_task; // NEW IMPORT

// --- MPSC Channel Configuration (Bounded for progress, Unbounded for requests) ---
const PROGRESS_CHANNEL_BOUND: usize = 1024;
// NOTE: Request channel is now UNBOUNDED to match start_request_loop signature
const REQUEST_CHANNEL_BOUND: usize = 128; // This constant is now unused for request_sender, but kept for context


// -----------------------------------------------------------------------------
// CONDUCTOR MANAGER
// -----------------------------------------------------------------------------

/// The central manager for the procedural generation pipeline.
/// This struct is the lightweight orchestrator for the entire generation process.
pub struct Conductor {
    // Delegates
    runtime_manager: RuntimeManager,
    generator_manager: GeneratorManager,
    // State
    internal_state: ConductorState,
    chunk_cache: Arc<Mutex<ChunkCache>>,
    // Communication
    progress_sender: Sender<GenerationMessage>,
    // FIX: Changed request_sender type to mpsc::UnboundedSender in the struct to match the channel creation below
    request_sender: mpsc::UnboundedSender<ChunkRequest>,
}

impl Conductor {
    /// Initializes the Conductor, starts the runtime, creates the MPSC channels.
    /// 
    /// Returns the Conductor, its internal state, the progress receiver (ProgressReceiver),
    /// and the request sender (RequestSender).
    pub fn new(config_path: Option<&str>) -> Result<(Self, ConductorState, mpsc::Receiver<GenerationMessage>, mpsc::UnboundedSender<ChunkRequest>), io::Error> {
        
        let config = get_config_from_path(config_path)?;

        let runtime_manager = RuntimeManager::new()?;
        let handle = runtime_manager.get_handle();

        let generator_manager = GeneratorManager::new().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Generator setup failed: {}", e))
        })?;
        // NOTE: generator_manager.get_map_clone() returns HashMap<String, Arc<DynGenerator>>
        // The start_request_loop expects Arc<HashMap<String, Arc<DynGenerator>>>.
        // We must Arc-wrap the clone *before* passing it.
        let generators_for_loop = Arc::new(generator_manager.get_map_clone());

        let chunk_cache = Arc::new(Mutex::new(ChunkCache::new().map_err(|e| {
            error!("Failed to initialize ChunkCache: {:?}", e);
            io::Error::new(io::ErrorKind::Other, format!("Cache initialization failed: {:?}", e))
        })?));
        let chunk_cache_for_loop = chunk_cache.clone();

        // 1. Create Bounded channel for progress (MPSC::Sender/Receiver)
        let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);
        
        // 2. FIX: Create UNBOUNDED channel for requests (mpsc::UnboundedSender/UnboundedReceiver)
        // This resolves the expected UnboundedReceiver<GenerationTask> vs found Receiver<_> error (E0308).
        let (request_sender, request_receiver) = mpsc::unbounded_channel(); 
        
        let initial_id = generator_manager.get_initial_id(config.get_default_generator_id());
        
        let state = ConductorState::new(initial_id.clone());
        state.set_status(ConductorStatus::Running);
        let state_for_loop = state.clone();

        info!("Conductor initialized. Active generator: {}", initial_id);

        // Start Request Processing Loop (Delegated)
        // FIX (E0308): Arguments reordered and Arc-wrapped to match start_request_loop signature:
        // (handle, request_rx, progress_tx, generators, chunk_cache, conductor_state)
        start_request_loop(
            handle,                                         // 1. rt_handle: Handle
            request_receiver,                               // 2. request_rx: mpsc::UnboundedReceiver<GenerationTask> (FIXED channel type above)
            progress_sender.clone(),                        // 3. progress_tx: mpsc::Sender<GenerationMessage>
            generators_for_loop,                            // 4. generators: Arc<HashMap<...>> (FIXED Arc::new() above)
            chunk_cache_for_loop,                           // 5. chunk_cache: Arc<Mutex<...>>
            Arc::new(state_for_loop),                       // 6. conductor_state: Arc<ConductorState> (FIXED: Wrapped in Arc::new())
        );

        let conductor = Conductor {
            runtime_manager,
            generator_manager,
            internal_state: state.clone(),
            chunk_cache,
            progress_sender: progress_sender.clone(),
            request_sender: request_sender.clone(), // This is now mpsc::UnboundedSender
        };

        // FIX: Update the returned type of request_sender
        Ok((conductor, state, progress_receiver, request_sender))
    }
    // ... (rest of impl Conductor remains unchanged)
    
    /// Provides public read-access to the active generator ID.
    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }

    // -------------------------------------------------------------------------
    // CORE GENERATION COMMAND ENTRY POINT (Delegated Batch Jobs)
    // -------------------------------------------------------------------------

    /// Handles the main map generation command from an FFI wrapper (like Godot).
    pub fn start_generation(&mut self, config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Validation
        ConfigValidator::validate_map_dimensions(&config)?;

        // 2. Set the active generator
        self.set_active_generator(&config.generator_name)?;
        
        // ⚠️ CRITICAL FIX: Set the status to Generating immediately before spawning the batch job.
        // This provides debug visibility and ensures the EngineMonitor receives the status change
        // even if the batch job panics later.
        self.internal_state.set_status(ConductorStatus::Generating);

        // 3. Delegate the async, blocking generation task
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

    // -------------------------------------------------------------------------
    // SYNCHRONOUS ACCESS PATHWAY (Used by FFI/Godot read functions)
    // -------------------------------------------------------------------------

    /// SYNCHRONOUS ACCESS: Loads or generates a single chunk, blocking the caller.
    pub fn get_chunk_data(&self, chunk_coords: &Vec2i) -> ChunkData {
        let active_generator_id = self.internal_state.get_active_generator_id();

        // Create a temporary channel to capture the result
        let (temp_sender, mut temp_receiver) = mpsc::channel(1);

        // Call the core processing logic helper
        handle_chunk_unit(
            *chunk_coords,
            &active_generator_id,
            self.generator_manager.get_map_ref(),
            &self.chunk_cache,
            &temp_sender,
        );
        
        // Block here to ensure the data is returned synchronously as requested.
        match temp_receiver.blocking_recv() {
            Some(GenerationMessage::ChunkGenerated(_coords, chunk_data_arc)) => {
                // Try to unwrap the Arc for the owned ChunkData, fall back to cloning if other references exist.
                Arc::try_unwrap(chunk_data_arc).unwrap_or_else(|arc| (*arc).clone())
            },
            _ => {
                panic!("get_chunk_data failed to receive ChunkGenerated message.");
            }
        }
    }

    // -------------------------------------------------------------------------
    // GENERATOR MANAGEMENT & SHUTDOWN
    // -------------------------------------------------------------------------

    /// Changes the algorithm used for subsequent generation tasks.
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

    /// SYNCHRONOUS PATHWAY (For CLI/Benchmarking).
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let active_id = self.internal_state.get_active_generator_id();
        self.generator_manager.generate_single_chunk(chunk_coords, &active_id)
    }
    
    /// Signals the Conductor's internal state to begin a graceful shutdown process.
    pub fn signal_shutdown_graceful(&self) {
        info!("SSXL Conductor signaled for shutdown. Setting status to ShuttingDown.");
        self.internal_state.set_status(ConductorStatus::ShuttingDown);
    }

    /// Performs a full, graceful teardown of the Conductor.
    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        self.runtime_manager.shutdown_graceful();
        info!("SSXL Conductor full teardown complete.");
    }
}