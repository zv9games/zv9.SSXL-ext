// ssxl_math/src/lib.rs

//! # SSXL Math Crate (`ssxl_math`)
//!
//! This crate contains the core mathematical utilities, data structures, and coordinate
//! system logic required by the SSXL-ext procedural generation engine.
//!
//! Key functionalities include:
//! - **Coordinate System:** Defining world and chunk positions (`ChunkKey`, `WorldPos`).
//! - **Hashing:** Deterministic generation of unique chunk and content identifiers.
//! - **Primitives:** Shared types and result wrappers (`SSXLResult`, `Vec2i`).

// --- Module Declarations ---

/// Defines the global and local coordinate system structures (`ChunkKey`, `WorldPos`).
pub mod coordinate_system;

/// Houses various utility functions for procedural generation algorithms.
pub mod generation_utils;

/// Provides deterministic hashing functions (e.g., `hash_chunk_coords`).
pub mod hashing;

/// Core mathematical primitives, custom types, and error handling results.
pub mod primitives;

// --- Public Re-exports (The Main Crate API) ---

// Re-export key coordinate types for direct use by other SSXL-ext crates.
pub use coordinate_system::{ChunkKey, TileOffset, WorldPos};

// Re-export core functions and utilities.
pub use crate::generation_utils::process_data;

// Re-export essential primitives and type aliases.
pub use crate::primitives::Vec2i;
pub use crate::primitives::SSXLData;
pub use crate::primitives::SSXLResult;
pub use crate::primitives::TileCoord; // ⭐ FIXED: Changed 'crate primitives' to 'crate::primitives'

// --- Prelude for Internal Engine Use ---

/// A convenience module that re-exports all essential types and traits
/// from the `ssxl_math` crate.
///
/// Crates in the SSXL-ext project are encouraged to use `use ssxl_math::prelude::*`
/// to easily import the most commonly needed math components. This adheres to the
/// common Rust practice for making libraries ergonomic.
pub mod prelude {
    pub use super::coordinate_system::*;
    pub use super::generation_utils::*;
    pub use super::hashing::*;
    pub use super::primitives::*;
}

// --- Initialization Function ---

/// Initializes the SSXL Math system.
///
/// Currently, this only logs an informational message. It acts as a potential
/// future hook for any necessary global math configuration or setup checks.
pub fn initialize_math_system() {
    tracing::info!("SSXL Math system initialized and ready.");
}