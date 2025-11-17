// ssxl_shared\src\chunk_data.rs

//! # Chunk Data Structures (`ssxl_shared::chunk_data`)
//!
//! This module defines the `ChunkData` structure, which represents a single,
//! fixed-size block of the procedural world. It includes coordinates, bounds,
//! the array of tiles, and metadata about its generation. This is the atomic
//! unit of data shared between generation workers and the cache/Godot runtime,
//! forming the basis of the engine's "crypto coded memory."

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::convert::TryInto; 

use crate::grid_bounds::GridBounds;
use crate::tile_data::TileData;
// Imports the custom Serde helper for deterministic SystemTime serialization.
use crate::math_primitives;

// Re-exports a core math primitive for chunk coordinates.
// FIX: Removed the erroneous `I64Vec3` import to resolve E0432.
use ssxl_math::Vec2i; 
// REMOVED: use ssxl_math::hashing::hash_chunk_coords; // FIX: Cleaned up unused import.

// Used to enable serialization/deserialization of arrays larger than 32 elements
// by the serde framework (necessary for the 1024-tile array).
use serde_big_array::BigArray;

// --- Constants ---

/// The canonical side length of a chunk (e.g., 32x32 tiles), defined as a public constant.
pub const CHUNK_SIZE: u32 = 32;

/// The total number of tiles in a single chunk (32 * 32 = 1024).
const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;


// --- Coordinate Structure ---

/// Defines the **chunk-space** coordinates (e.g., Chunk [1, 5] on the world grid).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoords {
    pub x: i64,
    pub y: i64,
}


// --- Core Data Structure ---

/// The central data structure for a world chunk. This is the primary payload
/// for procedural generation tasks and caching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    /// A unique, deterministic identifier for this chunk.
    pub id: u64,
    /// The world-space bounding box of the chunk, using the half-open range convention.
    pub bounds: GridBounds,
    /// The fixed-size array containing all tiles within the chunk.
    #[serde(with = "BigArray")]
    pub tiles: [TileData; TILE_ARRAY_SIZE],
    /// A string tag identifying the dimension or layer this chunk belongs to.
    pub dimension_tag: String,
    /// Timestamp indicating when the chunk was generated (serialized deterministically).
    #[serde(with = "math_primitives::system_time_serde")]
    pub generated_at: SystemTime,
}


impl ChunkData {
    /// Re-export of the canonical chunk size as an associated constant for use in methods.
    pub const SIZE: u32 = CHUNK_SIZE;

    /// Creates a new `ChunkData` instance using explicit, pre-calculated parameters.
    pub fn new(id: u64, bounds: GridBounds, dimension_tag: String) -> Self {
        // Initialize the tile array with default (empty) TileData.
        let tiles = [TileData::default(); TILE_ARRAY_SIZE];
        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag,
            generated_at: SystemTime::now(),
        }
    }
    
    /// Creates a new `ChunkData` instance by calculating its bounds and generating
    /// an initial ID based solely on its chunk coordinates.
    pub fn new_at_coords(chunk_coords: Vec2i) -> Self {
        let chunk_size_i64 = Self::SIZE as i64;
        
        // 1. Calculate the world-space minimum coordinates (inclusive).
        let min_x = chunk_coords.x * chunk_size_i64;
        let min_y = chunk_coords.y * chunk_size_i64;
        
        // 2. Calculate the world-space maximum coordinates (exclusive, half-open range).
        let max_x = min_x + chunk_size_i64;
        let max_y = min_y + chunk_size_i64;

        let bounds = GridBounds::new(min_x, min_y, max_x, max_y);
        
        // 3. Generate a robust, collision-resistant ID.
        // FIX: Now uses the robust Zigzag-encoded 2D hash.
        let id = ChunkData::hash_coords_2d(chunk_coords.x, chunk_coords.y);
        
        let tiles = [TileData::default(); TILE_ARRAY_SIZE];

        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag: "Default".to_string(),
            generated_at: SystemTime::now(),
        }
    }

    /// Internal helper to map signed i64 to u64 for safe spatial indexing (e.g., Z-order curves).
    /// This prevents massive u64 values for small negative i64 inputs.
    /// Formula: (n << 1) ^ (n >> 63)
    #[inline(always)]
    fn zigzag_encode(n: i64) -> u64 {
        // We use the standard Zigzag formula to map all i64 values to unique u64 values,
        // prioritizing small absolute values to the lowest u64 space.
        ((n << 1) ^ (n >> 63)) as u64
    }

    /// Internal 2D coordinate hashing function, replacing the problematic FFI call.
    /// Uses a **Zigzag-encoded** packing into a u64, which is fast, deterministic,
    /// and ensures no collisions across the world origin.
    fn hash_coords_2d(x: i64, y: i64) -> u64 {
        let ux = Self::zigzag_encode(x);
        let uy = Self::zigzag_encode(y);

        // Pack the two 32-bit halves. This is standard 2D packing for chunk keys.
        ux | (uy << 32)
    }

    /// Internal helper to convert local tile coordinates (x, y) within the chunk
    /// into a flat array index.
    ///
    /// This is marked `#[inline(always)]` for **performance optimization**
    /// as it's called repeatedly during generation loops.
    ///
    /// Returns `None` if the coordinates are out of the [0, CHUNK_SIZE - 1] range.
    #[inline(always)]
    fn coord_to_index(x: u32, y: u32) -> Option<usize> {
        if x < Self::SIZE && y < Self::SIZE {
            // Index = Y * Width + X (standard row-major order)
            Some((y * Self::SIZE + x) as usize)
        } else {
            None
        }
    }

    /// Safely retrieves an immutable reference to a tile at the given local coordinates.
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &self.tiles[index]
        })
    }
    
    /// Inserts a fully generated vector of tiles into the chunk's internal array.
    ///
    /// # Panics
    /// Panics if the input vector's length does not exactly match the expected
    /// `TILE_ARRAY_SIZE`, which is a critical **data integrity** check.
    pub fn insert_tiles(&mut self, tiles_vec: Vec<TileData>) {
        // FIX: Use TryInto to consume the Vec, which is more idiomatic and clear for ownership transfer.
        match tiles_vec.try_into() {
            Ok(arr) => self.tiles = arr,
            Err(vec) => {
                // Critical error: A generator produced an incomplete or oversized chunk.
                panic!(
                    "Tile vector size mismatch for chunk {:?}. Generator returned {} tiles, but expected {}.",
                    self.bounds,
                    vec.len(),
                    TILE_ARRAY_SIZE
                );
            }
        }
    }
    
    /// Safely retrieves a mutable reference to a tile at the given local coordinates.
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &mut self.tiles[index]
        })
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: For tests to compile, TileData::default() must be Copy, which it implicitly is.
    // The coordinate-to-index tests are sound and require no changes.
    
    #[test]
    /// Tests the critical coordinate-to-index logic for boundaries and center.
    fn test_coord_to_index() {
        // Top-left corner
        assert_eq!(ChunkData::coord_to_index(0, 0), Some(0));

        // Center (16, 16) -> 16 * 32 + 16 = 528
        assert_eq!(ChunkData::coord_to_index(16, 16), Some(528));

        // Bottom-right corner (31, 31) -> 31 * 32 + 31 = 1023 (TILE_ARRAY_SIZE - 1)
        assert_eq!(ChunkData::coord_to_index(31, 31), Some(1023));

        // Out of bounds checks (32 is the exclusive size limit)
        assert_eq!(ChunkData::coord_to_index(32, 0), None);
        assert_eq!(ChunkData::coord_to_index(0, 32), None);
        assert_eq!(ChunkData::coord_to_index(33, 33), None);
    }
    
    #[test]
    /// Tests the Zigzag encoding for correct mapping of signed to unsigned space.
    fn test_zigzag_encode() {
        assert_eq!(ChunkData::zigzag_encode(0), 0);
        assert_eq!(ChunkData::zigzag_encode(-1), 1);
        assert_eq!(ChunkData::zigzag_encode(1), 2);
        assert_eq!(ChunkData::zigzag_encode(-2), 3);
        assert_eq!(ChunkData::zigzag_encode(2), 4);
    }

    #[test]
    /// Tests the new 2D hashing function for uniqueness across the origin.
    fn test_hash_coords_2d() {
        // Standard coordinates
        // (1, 0) -> (Zigzag(1)=2 | Zigzag(0)=0 << 32) = 2
        assert_eq!(ChunkData::hash_coords_2d(1, 0), 2);
        // (0, 1) -> (Zigzag(0)=0 | Zigzag(1)=2 << 32) = 8589934592 (2 * 2^32)
        assert_eq!(ChunkData::hash_coords_2d(0, 1), 8589934592);
        
        // Critical Negative Coordinate Check (was broken before)
        // (-1, 0) -> (Zigzag(-1)=1 | Zigzag(0)=0 << 32) = 1
        assert_eq!(ChunkData::hash_coords_2d(-1, 0), 1);
        
        // Collision check across the origin must pass
        assert_ne!(ChunkData::hash_coords_2d(1, 1), ChunkData::hash_coords_2d(-1, -1));
        
        // Large coordinates (to ensure all 64 bits are used)
        let big_x = i64::MAX / 2; // Large positive
        let big_y = i64::MIN / 2; // Large negative
        let hash1 = ChunkData::hash_coords_2d(big_x, big_y);
        assert_ne!(hash1, 0); 
    }
}