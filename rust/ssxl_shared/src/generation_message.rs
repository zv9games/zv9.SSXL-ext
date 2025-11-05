// ssxl_shared/src/generation_message.rs

use crate::chunk_data::ChunkData;
use ssxl_math::Vec2i;
use std::sync::Arc;

// --- Task Sent TO the Generator Worker ---

/// Represents a single request for a chunk generation task.
#[derive(Debug, Clone)]
pub struct GenerationTask {
    /// The coordinates of the chunk to be generated (world-space tile origin).
    pub chunk_coords: Vec2i,
    /// The ID of the generator to use for this task.
    pub generator_id: String,
}


// --- Message Sent FROM the Generator Worker ---

/// Represents a progress update or completion signal from the generation worker.
#[derive(Debug)]
pub enum GenerationMessage {
    /// Sent when a chunk has been successfully loaded or generated.
    ChunkGenerated(Vec2i, Arc<ChunkData>),
    /// Signals that the worker pool has finished processing all tasks.
    // This variant matches the usage in ssxl_generate/src/batch_processor.rs and ssxl_generate/src/task_queue.rs.
    GenerationComplete, 
}