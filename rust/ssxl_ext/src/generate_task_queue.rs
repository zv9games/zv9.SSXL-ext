use crate::shared_config::GenerationConfig;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GenerationTask {
    pub chunk_pos: (i32, i32),
    pub chunk_size: u32,
    pub seed: u64,
    pub config: Arc<GenerationConfig>,
}

impl GenerationTask {
    pub fn new(
        chunk_pos: (i32, i32),
        chunk_size: u32,
        seed: u64,
        config: Arc<GenerationConfig>,
    ) -> Self {
        Self {
            chunk_pos,
            chunk_size,
            seed,
            config,
        }
    }
}
