// ============================================================================
// 🎼 Conductor Setup and Spawn (`crate::conductor::setup`)
// ----------------------------------------------------------------------------
// This module provides the initialization and spawning logic for the SSXL
// Conductor, the central orchestrator of procedural generation tasks. It
// prepares channels, managers, and state, then launches the asynchronous
// request loop that drives chunk generation.
//
// Purpose:
//   • Bundle together configuration, managers, channels, and initial state.
//   • Provide a clean entry point for starting the Conductor runtime.
//   • Ensure safe, modular setup of async communication and caching systems.
//
// Key Components:
//   • PROGRESS_CHANNEL_BOUND
//       - Defines bounded capacity for the progress channel.
//       - Prevents unbounded queuing of progress messages, applying backpressure.
//
//   • setup_channels_and_state
//       - Loads configuration from a file path (or defaults).
//       - Initializes the GeneratorManager, which tracks available generators.
//       - Creates bounded progress channel for updates and unbounded request
//         channel for chunk generation tasks.
//       - Determines initial generator ID from config.
//       - Creates initial ConductorState with that ID.
//       - Returns a `ConductorInternalSetup` bundle containing all components.
//
//   • spawn
//       - Consumes `ConductorInternalSetup` and spawns the async request loop.
//       - Initializes a RuntimeManager (Tokio runtime wrapper).
//       - Clones generator map and creates a chunk cache (LRU, capacity 4096).
//       - Marks conductor state as running.
//       - Starts the async request loop with runtime handle, channels, generators,
//         cache, and state.
//       - Constructs a `Conductor` instance with runtime, managers, state, cache,
//         and channels.
//       - Returns tuple: (Conductor, ConductorState, request sender, progress receiver).
//
// Workflow:
//   1. `setup_channels_and_state` prepares configuration, managers, channels, and state.
//   2. `spawn` consumes setup bundle and starts async request loop.
//   3. Conductor instance is returned, ready to manage generation tasks.
//   4. Progress updates flow through bounded channel; requests flow through unbounded channel.
//   5. Chunk cache ensures efficient reuse of generated chunks.
//
// Design Choices:
//   • Separation of setup and spawn improves modularity and testability.
//   • Bounded progress channel prevents overload; unbounded request channel ensures flexibility.
//   • Arc-based sharing allows safe concurrent access to generators, cache, and state.
//   • Logging provides visibility into initialization and runtime events.
//
// Educational Note:
//   • This module demonstrates how to structure async orchestration in Rust,
//     combining configuration, state management, channels, and runtime spawning.
//   • By encapsulating setup and spawn logic, the Conductor remains extensible
//     and maintainable, supporting complex procedural generation workflows.
// ============================================================================


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
