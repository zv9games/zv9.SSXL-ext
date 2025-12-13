// rust/SSXL-ext/src/shared_message.rs

/// Control messages sent to the generation workers (sync_pool.rs).
#[derive(Debug, Clone, Copy)]
pub enum GenerationControlMessage {
    /// Instructs the workers to stop processing new tasks but finish current ones.
    Pause,
    /// Instructs the workers to stop immediately and shut down the thread.
    Stop,
    /// Forces a worker to reload its configuration from the global state.
    ReloadConfig,
}

// rust/SSXL-ext/src/shared_message.rs

use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;

/// Data messages sent from the generation workers back to the main thread Conductor.
#[derive(Debug)]
pub enum GenerationDataMessage {
    /// A completed chunk ready for direct writing to the TileMap.
    CompletedChunk(Chunk),
    /// A fatal error occurred during the processing of a specific chunk.
    JobFailure(SSXLCoreError),
    /// A simple acknowledgement that a worker is initialized or has cleared its state.
    Ack,
}

// rust/SSXL-ext/src/shared_message.rs

use crate::shared_config::AnimationConfig;

/// Control messages sent to the animation workers (animate_worker.rs).
#[derive(Debug, Clone, Copy)]
pub enum AnimationControlMessage {
    Pause,
    Stop,
    UpdateConfig(AnimationConfig), 
}

// rust/SSXL-ext/src/shared_message.rs

use crate::animate_events::AnimationEvent;

/// Data messages sent from the animation workers back to the main thread Conductor.
#[derive(Debug, Clone)]
pub enum AnimationDataMessage {
    /// A single, ready-to-render visual event.
    Event(AnimationEvent),
    /// A worker encountered an unrecoverable error during simulation.
    WorkerPanic(SSXLCoreError),
}