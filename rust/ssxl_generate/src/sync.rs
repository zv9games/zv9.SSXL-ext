// ssxl_generate/src/sync.rs

use tokio::sync::mpsc;
// ✅ FIX: Changed the import path as suggested by the compiler.
use crate::task_queue::GenerationMessage; 
use crate::task_queue::GenerationTask;

// --- Public Type Aliases for Communication Channels ---

/// The channel sender used by Godot/main thread to request new chunks from the Conductor worker.
pub type ConductorRequestSender = mpsc::UnboundedSender<GenerationTask>;

/// The channel receiver used by Godot/main thread to read chunk generation results/progress.
// ✅ This type alias is now correct and matches the return type of Conductor::new().
pub type ConductorProgressReceiver = mpsc::Receiver<GenerationMessage>;