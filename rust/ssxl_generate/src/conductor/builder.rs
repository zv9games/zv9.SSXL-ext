// src/conductor/builder.rs
use super::{conductor_state, Conductor};
use super::internal_setup::ConductorInternalSetup; 

use crate::manager::{GeneratorManager, RuntimeManager};
use crate::task::{start_request_loop, GenerationMessage, GenerationTask as ChunkRequest};
use ssxl_cache::ChunkCache;
use ssxl_shared::config::config::get_config_from_path;
use tokio::sync::mpsc::{self, Receiver, UnboundedSender};
use tracing::info;
use std::io;
use std::sync::Arc;

const PROGRESS_CHANNEL_BOUND: usize = 1024;

pub(crate) fn setup_channels_and_state(
    config_path: Option<&str>,
) -> Result<ConductorInternalSetup, io::Error> {
    let config = get_config_from_path(config_path);
    // TODO: (Implied) Handle errors in GeneratorManager::new().
    // The previous code already includes error handling for GeneratorManager::new()
    // by using the `map_err` and `?` operators to convert the custom error into an io::Error.
    let generator_manager = GeneratorManager::new().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Generator setup failed: {}", e))
    })?;

    let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);
    let (request_sender_api, request_receiver) = mpsc::unbounded_channel();

    let initial_id = generator_manager.get_initial_id(&config.default_generator_id());
    let initial_state = conductor_state::ConductorState::new(initial_id.clone());

    info!("Conductor initialized. Active generator: {}", initial_id);

    Ok(ConductorInternalSetup {
        request_receiver,
        progress_sender,
        request_sender_api,
        progress_receiver,
        initial_state,
        generator_manager,
    })
}

pub(crate) fn spawn(
    internal_setup: ConductorInternalSetup,
) -> Result<
    (
        Conductor,
        conductor_state::ConductorState,
        UnboundedSender<ChunkRequest>,
        Receiver<GenerationMessage>,
    ),
    io::Error,
> {
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
    let chunk_cache = Arc::new(ChunkCache::new(4096)?);
    let chunk_cache_for_loop = chunk_cache.clone();

    initial_state.set_status(conductor_state::ConductorStatus::Running);
    let state_for_loop = initial_state.clone();

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