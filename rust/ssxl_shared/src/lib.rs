use serde::{Deserialize, Serialize};
use tracing;

pub mod chunk;
pub mod tile;
pub mod error;
pub mod config;
pub mod message;
pub mod math;
pub mod job;

pub use config::config::{get_config_from_path, SSXLConfig}; // ADDED: Re-exporting Config access
pub use ssxl_math::primitives::{ChunkId, TileCoord};

pub use chunk::chunk_data::{ChunkData, CHUNK_SIZE};
pub use tile::tile_data::TileData;
pub use chunk::grid_bounds::GridBounds;
pub use tile::tile_type::TileType;

pub use message::messages::{
    AnimationCommand,
    AnimationType,
    AnimationPayload,
    UpdateSender,
    AnimationConductorHandle,
    AnimationState,
    CommandResult,
    AnimationUpdate,
};

pub use error::errors::{SSXLError, SSXLResult};
pub use anyhow;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SSXLData {
    pub id: u64,
    pub timestamp: u64,
    pub value: String,
}

pub fn initialize_shared_data() {
    tracing::info!("SSXL Shared Data Primitives initialized (Priority 1 complete).");
}

pub mod prelude {
    pub use super::chunk::chunk_data::{ChunkData, CHUNK_SIZE};
    pub use super::tile::tile_data::TileData;
    pub use super::chunk::grid_bounds::GridBounds;
    pub use super::tile::tile_type::TileType;
    
    pub use super::{ChunkId, TileCoord};
    
    pub use super::error::errors::{SSXLError, SSXLResult};
    
    // UPDATED: Added AnimationState to the prelude
    pub use super::message::messages::{AnimationCommand, AnimationType, AnimationState};
    pub use super::message::messages::AnimationUpdate;

    // ADDED: Adding SSXLConfig to the prelude for easy access
    pub use super::config::config::SSXLConfig;
}

use std::sync::atomic::AtomicUsize;

pub static CHUNKS_COMPLETED_COUNT: AtomicUsize = AtomicUsize::new(0);