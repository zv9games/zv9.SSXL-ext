use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineConfig {
    pub tile_size: i32,
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub seed: i64,
    pub enable_voxel_mode: bool,
}
