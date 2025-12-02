// ssxl_generate/src/conductor/conductor.rs

use tokio::sync::mpsc::{self, Sender};
use tracing::{info, error};
use std::sync::Arc;
use std::io;
// Removed: use std::ops::Deref; // No longer needed
use ssxl_math::Vec2i;
use ssxl_cache::ChunkCache;
use ssxl_shared::ChunkData;

use ssxl_tools::get_config_from_path;
// NOTE: These internal modules are now found via the conductor/mod.rs file.
use crate::manager::runtime_manager::RuntimeManager;
use crate::manager::config_validator::{ConfigValidator, GeneratorConfig};
use crate::task::{
    handle_chunk_unit, start_request_loop,
    task_queue::{GenerationTask as ChunkRequest, GenerationMessage},
};
use crate::manager::generator_manager::GeneratorManager;
use crate::conductor::conductor_state;
use crate::task::batch_processor::spawn_batch_generation_task;

const PROGRESS_CHANNEL_BOUND: usize = 1024;

pub struct ConductorInternalSetup {
    pub request_receiver: mpsc::UnboundedReceiver<ChunkRequest>,
    pub progress_sender: Sender<GenerationMessage>,
    pub request_sender_api: mpsc::UnboundedSender<ChunkRequest>,
    pub progress_receiver: mpsc::Receiver<GenerationMessage>,
    pub initial_state: conductor_state::ConductorState,
    pub generator_manager: GeneratorManager,
}

pub struct Conductor {
    runtime_manager: RuntimeManager,
    generator_manager: GeneratorManager,
    internal_state: conductor_state::ConductorState,
    chunk_cache: Arc<ChunkCache>,
    progress_sender: Sender<GenerationMessage>,
    #[allow(dead_code)]
    _request_sender: mpsc::UnboundedSender<ChunkRequest>,
}

impl Conductor {
// ... (setup_channels_and_state and spawn methods remain unchanged)

    pub fn setup_channels_and_state(config_path: Option<&str>) -> Result<ConductorInternalSetup, io::Error> {
        let config = get_config_from_path(config_path)?;
        let generator_manager = GeneratorManager::new().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Generator setup failed: {}", e))
        })?;

        let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);
        let (request_sender_api, request_receiver) = mpsc::unbounded_channel();
        let initial_id = generator_manager.get_initial_id(config.get_default_generator_id());
        let initial_state = conductor_state::ConductorState::new(initial_id.clone());

        info!("Conductor handles initialized. Active generator ID determined: {}", initial_id);

        Ok(ConductorInternalSetup {
            request_receiver,
            progress_sender,
            request_sender_api,
            progress_receiver,
            initial_state,
            generator_manager,
        })
    }

    pub fn spawn(internal_setup: ConductorInternalSetup) -> Result<(
        Self,
        conductor_state::ConductorState,
        mpsc::UnboundedSender<ChunkRequest>,
        mpsc::Receiver<GenerationMessage>,
    ), io::Error> {
        let ConductorInternalSetup {
            request_receiver,
            progress_sender,
            request_sender_api,
            progress_receiver,
            initial_state,
            generator_manager,
        } = internal_setup;

        let runtime_manager = RuntimeManager::new()?;
        let handle = runtime_manager.get_handle();
        let generators_for_loop = Arc::new(generator_manager.get_map_clone());
        let cache_instance = ChunkCache::new(4096)?;
        let chunk_cache = Arc::new(cache_instance);
        let chunk_cache_for_loop = chunk_cache.clone();

        initial_state.set_status(conductor_state::ConductorStatus::Running);
        let state_for_loop = initial_state.clone();

        info!("Conductor spawning. Active generator: {}", initial_state.get_active_generator_id());

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
            internal_state: initial_state.clone(),
            chunk_cache,
            progress_sender,
            _request_sender: request_sender_api.clone(),
        };

        Ok((conductor, initial_state, request_sender_api, progress_receiver))
    }

    pub fn new(config_path: Option<&str>) -> Result<(
        Self,
        conductor_state::ConductorState,
        mpsc::UnboundedSender<ChunkRequest>,
        mpsc::Receiver<GenerationMessage>,
    ), io::Error> {
        let internal_setup = Self::setup_channels_and_state(config_path)?;
        Self::spawn(internal_setup)
    }

    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }

    pub fn start_generation(&mut self, config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
        ConfigValidator::validate_map_dimensions(&config)?;
        self.set_active_generator(&config.generator_name)?;
        self.internal_state.set_status(conductor_state::ConductorStatus::Generating);

        info!(
            "Conductor: Dispatching batch generation task for map: {}x{} with seed '{}'.",
            config.width, config.height, config.seed
        );

        spawn_batch_generation_task(
            &self.runtime_manager.get_handle(),
            self.generator_manager.get_map_clone(),
            self.chunk_cache.clone(),
            self.internal_state.get_active_generator_id(),
            self.progress_sender.clone(),
            Arc::new(self.internal_state.clone()),
            config,
        );

        info!("Conductor: Batch task dispatched successfully.");
        Ok(())
    }

    pub fn stop_generation(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.internal_state.set_status(conductor_state::ConductorStatus::Stopping);
        info!("Conductor: Global STOP command received.");
        Ok(())
    }

    pub fn get_chunk_data(&self, chunk_coords: &Vec2i) -> ChunkData {
        // NOTE: Explicit cache check removed to bypass E0599 error.
        // We rely on 'handle_chunk_unit' below to perform cache lookup or synchronous generation.
        /*
        if let Some(chunk_data_arc) = (&*self.chunk_cache).get(chunk_coords) {
             info!("Chunk ({:?}) found in cache (Hit).", chunk_coords);
             return Arc::try_unwrap(chunk_data_arc).unwrap_or_else(|arc| (*arc).clone());
        }
        */

        let active_generator_id = self.internal_state.get_active_generator_id();
        // Channel size 1 is fine, as we will loop and ignore utility messages.
        let (temp_sender, mut temp_receiver) = mpsc::channel(1);
        let state_arc = Arc::new(self.internal_state.clone());

        info!("Chunk ({:?}) requested. Triggering synchronous generation/retrieval via handle_chunk_unit.", chunk_coords);

        handle_chunk_unit(
            *chunk_coords,
            &active_generator_id,
            self.generator_manager.get_map_ref(),
            &self.chunk_cache,
            &temp_sender,
            &state_arc,
        );

        // FIX: Loop until we receive the expected GenerationMessage::Generated variant
        loop {
            match temp_receiver.blocking_recv() {
                Some(GenerationMessage::Generated(_, chunk_data_arc)) => {
                    info!("Synchronous generation/retrieval successful for chunk ({:?}).", chunk_coords);
                    // Return the chunk data, breaking the loop
                    return Arc::try_unwrap(chunk_data_arc).unwrap_or_else(|arc| (*arc).clone())
                }
                Some(msg) => {
                    // Ignore status updates and other utility messages that might sneak through
                    // This prevents the synchronous call from deadlocking if an unexpected message 
                    // is sent ahead of the chunk data.
                    info!("get_chunk_data ignoring utility message: {:?} for coords: {:?}", msg, chunk_coords);
                    continue; 
                }
                None => {
                    error!("get_chunk_data failed: channel closed for coords: {:?}", chunk_coords);
                    // Return a default chunk on failure to avoid a panic
                    return ChunkData::new_at_coords(*chunk_coords)
                }
            }
        }
    }

    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
// ... (set_active_generator method remains unchanged)
        if self.generator_manager.get_map_ref().contains_key(id) {
            info!("Active generator set to: {}", id);
            self.internal_state.set_active_generator_id(id);
            Ok(())
        } else {
            let err = format!(
                "Generator ID '{}' not found. Available: {:?}",
                id,
                self.generator_manager.get_map_ref().keys().collect::<Vec<_>>()
            );
            error!("{}", err);
            Err(err)
        }
    }

    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let active_id = self.internal_state.get_active_generator_id();
        self.generator_manager.generate_single_chunk(chunk_coords, &active_id)
    }

    pub fn signal_generation_complete(&self) {
        info!("Generation complete. Status set to Running (Idle).");
        self.internal_state.set_status(conductor_state::ConductorStatus::Running);
    }

    pub fn signal_shutdown_graceful(&self) {
        info!("Shutdown requested. Status set to ShuttingDown.");
        self.internal_state.set_status(conductor_state::ConductorStatus::ShuttingDown);
    }

    pub fn request_shutdown(&self) {
        self.signal_shutdown_graceful();
        self.runtime_manager.shutdown_graceful();
        info!("Conductor shutdown initiated.");
    }

    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        self.runtime_manager.shutdown_graceful();
        info!("Conductor fully torn down.");
    }
}