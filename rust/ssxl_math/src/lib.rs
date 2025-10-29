// ssxl_math/src/lib.rs

//! The Foundation Layer crate for all spatial types, vector math,
//! coordinate systems, and deterministic hashing algorithms.

// --- MODULE DEFINITIONS ---
pub mod coordinate_system;
pub mod generation_utils;
pub mod hashing;
// --- FIX: Renamed to primitives to match usage below ---
pub mod primitives; 

// -------------------------------------------------------------------------
// EXPORTS FOR DOWNSTREAM CRATES
// -------------------------------------------------------------------------
pub use coordinate_system::{ChunkKey, WorldPos, TileOffset};
pub use crate::generation_utils::process_data;
// Exports correctly point to the 'primitives' module
pub use crate::primitives::Vec2i; 
pub use crate::primitives::SSXLResult;
pub use crate::primitives::SSXLData;

/// Math-specific prelude for convenient imports in downstream crates.
pub mod prelude {
    pub use super::coordinate_system::*;
    pub use super::generation_utils::*;
    pub use super::hashing::*;
    // Exports correctly point to the 'primitives' module
    pub use super::primitives::*; 
}

pub fn initialize_math_system() {
    tracing::info!("SSXL Math system initialized and ready.");
}