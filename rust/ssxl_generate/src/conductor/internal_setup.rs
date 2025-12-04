// src/conductor/internal_setup.rs

use crate::conductor::conductor_state::ConductorState;
use crate::manager::generator_manager::GeneratorManager;
use crate::task::task_queue::{GenerationTask as ChunkRequest, GenerationMessage};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, Receiver, Sender};

/// Internal setup struct — shared between conductor modules
pub(crate) struct ConductorInternalSetup {
    pub request_receiver: UnboundedReceiver<ChunkRequest>,
    pub progress_sender: Sender<GenerationMessage>,
    pub request_sender_api: UnboundedSender<ChunkRequest>,
    pub progress_receiver: Receiver<GenerationMessage>,
    pub initial_state: ConductorState,
    pub generator_manager: GeneratorManager,
}