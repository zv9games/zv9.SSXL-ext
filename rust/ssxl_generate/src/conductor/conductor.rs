// src/conductor/conductor.rs

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