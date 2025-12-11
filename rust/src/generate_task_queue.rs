// rust/SSXL-ext/src/generate_task_queue.rs

use crate::shared_config::GenerationConfig;

/// The strict data structure representing one unit of work (one chunk) 
/// to be processed by a worker thread.
#[derive(Debug, Clone)]
pub struct GenerationTask {
    pub chunk_pos: (i32, i32),
    pub chunk_size: u32,
    pub seed: u64,
    pub config: GenerationConfig, // Includes CA rules, Perlin settings, etc.
}

impl GenerationTask {
    pub fn new(chunk_pos: (i32, i32), chunk_size: u32) -> Self {
        // ... simple constructor ...
        Self { 
            chunk_pos, 
            chunk_size, 
            seed: 12345, // Example
            config: GenerationConfig::default(), // Example
        }
    }
}