// ssxl_shared/src/config.rs

//! # Global Configuration Constants (`ssxl_shared::config`)
//!
//! This module defines fundamental, immutable constants that govern the structure
//! and scale of the SSXL procedural world. These values must be consistent
//! across all SSXL-ext crates (math, generate, cache, godot) to ensure data
//! integrity and system entropy is controlled.

// --- World Geometry Constants ---

/// The canonical side length of a procedural chunk in tiles.
///
/// **Value:** 32 (meaning chunks are 32x32 tiles).
///
/// This constant defines the resolution and granularity of the generated world
/// and is critical for both memory allocation and generation performance.
pub const CHUNK_SIZE: u32 = 32;

/// The total number of tiles contained within a single `ChunkData` structure.
///
/// **Calculation:** CHUNK_SIZE * CHUNK_SIZE (32 * 32 = 1024).
///
/// This value is used to define the fixed-size array in `ChunkData` and for
/// ensuring generators return the correct payload size.
pub const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;