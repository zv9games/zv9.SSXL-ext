// rust/SSXL-ext/src/shared_job.rs

use crate::generate_task_queue::GenerationTask;

/// Defines the sequential steps required to fully process a single chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobStep {
    /// Initial state: The task has been queued.
    Queued,
    /// Step 1: Raw Perlin noise generation is running.
    NoiseGeneration,
    /// Step 2: Cellular Automata refinement is running.
    CARefinement,
    /// Step 3: Post-processing and refinement (e.g., placing entities, blending).
    PostProcessing,
    /// Final state: The chunk data is ready to be sent to the Conductor's finisher queue.
    Finished,
    /// Error state: A worker encountered an unrecoverable error.
    Failed,
}

// rust/SSXL-ext/src/shared_job.rs

use crate::shared_chunk::Chunk;
use crate::shared_math::ChunkCoords;

/// Represents a complete, multi-stage task for generating a single world chunk.
#[derive(Debug, Clone)]
pub struct GenerationJob {
    /// The unique identifier for this job (e.g., the chunk's coordinates).
    pub id: ChunkCoords,
    /// The core data and parameters for the job.
    pub task: GenerationTask,
    /// The current stage of processing for this job.
    pub current_step: JobStep,
    /// The partially or fully completed chunk data.
    pub chunk_data: Chunk,
}

impl GenerationJob {
    /// Creates a new job, initializing its state.
    pub fn new(task: GenerationTask) -> Self {
        let chunk_data = Chunk::new(task.chunk_pos, task.chunk_size);
        
        GenerationJob {
            id: task.chunk_pos,
            task,
            current_step: JobStep::Queued,
            chunk_data,
        }
    }

    /// Advances the job to the next sequential step.
    pub fn advance_step(&mut self) {
        self.current_step = match self.current_step {
            JobStep::Queued => JobStep::NoiseGeneration,
            JobStep::NoiseGeneration => JobStep::CARefinement,
            JobStep::CARefinement => JobStep::PostProcessing,
            JobStep::PostProcessing => JobStep::Finished,
            // Finished and Failed are terminal states
            _ => self.current_step,
        };
    }
}