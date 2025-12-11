// ============================================================================
// 🎼 Conductor Module (`crate::conductor`)
// ----------------------------------------------------------------------------
// The Conductor is the central orchestrator of the SSXL engine’s procedural
// generation system. It manages runtime execution, generator selection,
// communication channels, and caching of generated chunks.
//
// Purpose:
//   • Provide a unified interface for starting, controlling, and shutting down
//     the generation system.
//   • Manage async communication between generation tasks and the engine.
//   • Track conductor state (active generator, lifecycle status).
//   • Expose constructors for both internal use and FFI integration.
//
// Key Components:
//   • Conductor Struct
//       - Holds references to:
//           • RuntimeManager: manages the async runtime.
//           • GeneratorManager: manages available generator instances.
//           • ConductorState: tracks active generator and lifecycle status.
//           • ChunkCache: shared cache for generated chunks.
//           • progress_sender: channel for sending progress updates.
//           • _request_sender: channel for sending chunk generation requests.
//
//   • Constructors
//       - `new`
//           • Initializes conductor with channels and state.
//           • Returns tuple: (Conductor, ConductorState, request sender, progress receiver).
//       - `new_for_ffi`
//           • Specialized for external bindings (FFI).
//           • Returns tuple: (Conductor, ConductorState, command sender, response receiver, progress receiver).
//
//   • Control Methods
//       - `get_active_generator_id`: returns ID of currently active generator.
//       - `set_generator`: switches active generator and logs the change.
//       - `stop_generation`: signals conductor to stop generation tasks.
//       - `graceful_teardown`: consumes conductor, shuts down runtime gracefully.
//       - `signal_shutdown_graceful`: signals shutdown without consuming conductor.
//       - `request_shutdown`: signals shutdown and stops runtime.
//
// Workflow:
//   1. Conductor is created via `new` or `new_for_ffi`.
//   2. Async request loop is spawned, handling chunk generation tasks.
//   3. Progress updates flow through bounded channels; requests flow through unbounded channels.
//   4. Conductor methods allow switching generators, stopping tasks, or shutting down gracefully.
//
// Design Choices:
//   • Separation of constructors for internal vs. FFI use improves flexibility.
//   • Arc-based cache ensures safe concurrent access to generated chunks.
//   • Logging provides visibility into generator changes and lifecycle events.
//   • Explicit shutdown methods prevent resource leaks and ensure graceful teardown.
//
// Educational Note:
//   • The Conductor demonstrates how to structure a central orchestrator in Rust,
//     combining async runtime management, modular generators, and safe concurrency.
//   • By exposing clear constructors and lifecycle controls, it provides a robust
//     foundation for procedural generation workflows in game engines or simulations.
// ============================================================================


use crate::conductor::builder::{setup_channels_and_state, spawn};

use ssxl_cache::ChunkCache;
use ssxl_shared::message::{GenerationCommand, GenerationResponse};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, Receiver};

use tracing::info;
use std::error::Error;
use std::io;
use std::sync::Arc;

use crate::manager::runtime_manager::RuntimeManager;
use crate::manager::generator_manager::GeneratorManager;
use crate::task::task_queue::{GenerationTask as ChunkRequest, GenerationMessage};

use crate::conductor::conductor_state::{self, ConductorState};

pub struct Conductor {
    pub(crate) runtime_manager: RuntimeManager,
    pub(crate) generator_manager: GeneratorManager,
    pub(crate) internal_state: conductor_state::ConductorState,
    pub(crate) chunk_cache: Arc<ChunkCache>,
    pub progress_sender: tokio::sync::mpsc::Sender<GenerationMessage>,
    #[allow(dead_code)]
    pub(crate) _request_sender: UnboundedSender<ChunkRequest>,
}

impl Conductor {
    pub fn new(
        config_path: Option<&str>,
    ) -> Result<
        (
            Self,
            ConductorState,
            UnboundedSender<ChunkRequest>,
            Receiver<GenerationMessage>,
        ),
        io::Error,
    > {
        let setup = setup_channels_and_state(config_path)?;
        spawn(setup)
    }

    pub fn new_for_ffi(
        config_path: Option<&str>,
    ) -> Result<
        (
            Self,
            ConductorState,
            UnboundedSender<GenerationCommand>,
            UnboundedReceiver<GenerationResponse>,
            Receiver<GenerationMessage>,
        ),
        io::Error,
    > {
        let setup = setup_channels_and_state(config_path)?;
        let (conductor, state, _req_tx, progress_rx) = spawn(setup)?;
        let (cmd_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (_, resp_rx) = tokio::sync::mpsc::unbounded_channel();
        Ok((conductor, state, cmd_tx, resp_rx, progress_rx))
    }

    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }

    pub fn set_generator(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        self.internal_state.set_active_generator_id(id);
        info!("Active generator switched to: {}", id);
        Ok(())
    }

    pub fn stop_generation(&self) -> Result<(), Box<dyn Error>> {
        self.internal_state.set_status(conductor_state::ConductorStatus::Stopping);
        info!("Global stop requested.");
        Ok(())
    }

    pub fn graceful_teardown(self) {
        self.internal_state.set_status(conductor_state::ConductorStatus::ShuttingDown);
        self.runtime_manager.shutdown_graceful();
        info!("Conductor gracefully shut down.");
    }

    pub fn signal_shutdown_graceful(&self) {
        self.internal_state.set_status(conductor_state::ConductorStatus::ShuttingDown);
    }

    pub fn request_shutdown(&self) {
        self.signal_shutdown_graceful();
        self.runtime_manager.shutdown_graceful();
    }
}
