// ssxl_generate/src/conductor/sync.rs

// FIX: Removed unused import `self` from tokio::sync::mpsc
use tokio::sync::mpsc::{Receiver, UnboundedSender};
use crate::task::task_queue::GenerationMessage;
use crate::task::task_queue::GenerationTask;

pub type ConductorRequestSender = UnboundedSender<GenerationTask>;

pub struct ConductorProgressReceiver {
    pub rx: Receiver<GenerationMessage>,
}

impl ConductorProgressReceiver {
    pub fn new(rx: Receiver<GenerationMessage>) -> Self {
        ConductorProgressReceiver { rx }
    }
}