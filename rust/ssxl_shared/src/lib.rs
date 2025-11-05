// ssxl_shared/src/lib.rs

//! The Foundation Layer crate. Defines all core data structures, common utilities,
//! and the canonical error type for the entire Aetherion Engine workspace.

use serde::{Serialize, Deserialize};
use tracing; // Import tracing for logging initialization

// --- MODULE DEFINITIONS ---
// P1 Data Structures
pub mod chunk_data;
pub mod tile_data;
pub mod grid_bounds;
pub mod tile_type;
pub mod errors;
pub mod messages;
pub mod config; // Add this line
pub mod generation_message; // Add this line

// P1 Math/Utility (To be implemented next)
pub mod math_primitives;

// --- CRITICAL TYPE EXPORTS ---
// Export the primary data structures
pub use chunk_data::ChunkData;
pub use chunk_data::CHUNK_SIZE;
pub use tile_data::TileData;
pub use grid_bounds::GridBounds;
pub use tile_type::TileType;

// Export the canonical error type and the engine-wide Result alias
pub use errors::{SSXLError, SSXLResult};

// ------------------------------------------------------------------
// CORE ENGINE FUNCTIONALITY
// ------------------------------------------------------------------

/// Initializes global data primitives and configuration for the engine.
/// This function is typically called once on engine startup by the FFI bridge.
// FIX: Added the missing function (E0425) expected by aetherion_engine_ffi.
pub fn initialize_shared_data() {
    tracing::info!("SSXL Shared Data Primitives initialized (Priority 1 complete).");
    // Future work: Add validation of global constants (e.g., CHUNK_SIZE) here.
}

// ------------------------------------------------------------------
// COMPATIBILITY EXPORTS (Temporary/Legacy/Simple Structures)
// ------------------------------------------------------------------

// Re-export the anyhow crate for macro usage (e.g., anyhow!("...")).
pub use anyhow;

/// A simple, generic data structure used by placeholder logic (e.g., aetherion_cache).
/// This structure provides the fields expected by the current implementation of calculate_data_hash.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SSXLData {
    pub id: u64,
    pub timestamp: u64,
    pub value: String,
}

// --- COMMON EXPORTS (PRELUDE) ---
/// A common prelude to be imported by other crates for quick access
/// to fundamental types and the core error handling alias.
pub mod prelude {
    // Shared Data
    pub use super::chunk_data::ChunkData;
    pub use super::tile_data::TileData;
    pub use super::grid_bounds::GridBounds;
    pub use super::tile_type::TileType;
    
    // Shared Error Handling
    pub use super::errors::{SSXLError, SSXLResult};
}