//! # SSXL Shared Data Crate (`ssxl_shared`)
//!
//! This crate contains fundamental data structures, configuration constants, and
//! messaging definitions utilized across all modules of the SSXL-ext procedural
//! generation engine. Its purpose is to ensure data integrity and consistency
//! between the high-speed worker threads and the main Godot runtime.

use serde::{Deserialize, Serialize};
use tracing;

// --- Module Declarations ---

pub mod chunk_data;
pub mod tile_data;
pub mod grid_bounds;
pub mod tile_type;
pub mod errors;
pub mod messages;
pub mod config;
pub mod generation_message;
pub mod math_primitives;

// --- Public Re-exports (The Main Crate API) ---

// 1. Primitive Spatial IDs (Re-exported from ssxl_math)
pub use ssxl_math::primitives::{ChunkId, TileCoord}; 

// 2. Core Data Structures & Constants
pub use chunk_data::{ChunkData, CHUNK_SIZE}; 
pub use tile_data::{AnimationUpdate, TileData}; // Canonical source for AnimationUpdate
pub use grid_bounds::GridBounds;
pub use tile_type::TileType;

// 3. Cycle-Breaking Communication Types (From the messages module)
pub use messages::{
    AnimationCommand,
    AnimationType,
    AnimationPayload,
    UpdateSender,
    AnimationConductorHandle,
    AnimationState,
    CommandResult,
};

// 4. Error Handling
pub use errors::{SSXLError, SSXLResult};
pub use anyhow;

// --- Generic Data Structure ---

/// A generic struct used to represent serializable, time-stamped data payloads.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SSXLData {
    pub id: u64,
    pub timestamp: u64,
    pub value: String,
}

// --- Initialization Function ---

/// Initializes the shared data module.
pub fn initialize_shared_data() {
    tracing::info!("SSXL Shared Data Primitives initialized (Priority 1 complete).");
}

// --- Prelude for Ergonomics ---

/// A convenience module that re-exports all essential types for ergonomic use.
/// Other SSXL-ext crates are encouraged to use `use ssxl_shared::prelude::*`.
pub mod prelude {
    // Re-export core structs
    pub use super::{ChunkData, TileData, GridBounds, TileType};
    // Re-export Primitives
    pub use super::{ChunkId, TileCoord};
    // Re-export Errors
    pub use super::{SSXLError, SSXLResult};
    // Re-export Communication Types (essential for high-level module interaction)
    pub use super::{AnimationCommand, AnimationUpdate, AnimationType};
}