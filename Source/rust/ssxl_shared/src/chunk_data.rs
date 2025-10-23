// aetherion_shared/src/chunk_data.rs
//! Canonical data structure for a single, dimension-agnostic chunk of procedural data.

use serde::{Serialize, Deserialize};
use std::time::SystemTime;

// Import types from other modules in aetherion_shared
use crate::grid_bounds::GridBounds;
use crate::tile_data::TileData; 
use crate::math_primitives; // Used for the SystemTime serde helper

// Import Vec2i from the dedicated aetherion_math crate (External Dependency)
use ssxl_math::Vec2i; 

use serde_big_array::BigArray;

// --- CONSTANTS ---

/// The canonical size for all chunks in the Aetherion Engine (32x32 tiles).
pub const CHUNK_SIZE: u32 = 32;
/// The total number of tiles in a single chunk (32 * 32 = 1024).
const TILE_COUNT: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize; 

// --- STRUCT DEFINITION ---
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ChunkData {
    pub id: u64,
    pub bounds: GridBounds,
    /// The fixed-size array containing all tile data. `BigArray` is used for efficient
    /// serialization of the large array.
    #[serde(with = "BigArray")]
    pub tiles: [TileData; TILE_COUNT],
    pub dimension_tag: String,
    #[serde(with = "math_primitives::system_time_serde")]
    pub generated_at: SystemTime,
}

// --- IMPLEMENTATION ---

impl ChunkData {
    pub const SIZE: u32 = CHUNK_SIZE;

    /// Creates a new, empty ChunkData instance initialized with default data.
    pub fn new(id: u64, bounds: GridBounds, dimension_tag: String) -> Self {
        let tiles = [TileData::default(); TILE_COUNT];
        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag,
            generated_at: SystemTime::now(),
        }
    }
    
    /// Creates a new ChunkData instance using only the chunk coordinates.
    pub fn new_at_coords(chunk_coords: Vec2i) -> Self {
        let chunk_size_i64 = CHUNK_SIZE as i64;
        
        // Convert chunk coordinates to world-space grid coordinates (i64 for GridBounds)
        let min_x = chunk_coords.x as i64 * chunk_size_i64;
        let min_y = chunk_coords.y as i64 * chunk_size_i64;
        let max_x = min_x + chunk_size_i64 - 1;
        let max_y = min_y + chunk_size_i64 - 1;

        let bounds = GridBounds::new(min_x, min_y, max_x, max_y);
        
        // NOTE: In a final system, the ID should be derived via robust hashing.
        let id = chunk_coords.x as u64 ^ chunk_coords.y as u64; 
        let tiles = [TileData::default(); TILE_COUNT];

        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag: "Default".to_string(),
            generated_at: SystemTime::now(),
        }
    }

    /// Converts local (x, y) coordinates to a flattened array index.
    #[inline(always)]
    fn coord_to_index(x: u32, y: u32) -> Option<usize> {
        if x < Self::SIZE && y < Self::SIZE {
            Some((y * Self::SIZE + x) as usize)
        } else {
            None
        }
    }

    /// Returns the data for a tile at the given local (x, y) coordinates.
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &self.tiles[index]
        })
    }
    
    /// **CRITICAL FIX for CA Generator:** Replaces the chunk's fixed-size tile array with a new set of tiles.
    /// Used by generators (like Cellular Automata) that produce a `Vec<TileData>`.
    pub fn insert_tiles(&mut self, tiles_vec: Vec<TileData>) {
        if tiles_vec.len() == TILE_COUNT {
            // Efficiently copy the vector's contents into the fixed-size array
            self.tiles.clone_from_slice(&tiles_vec);
        } else {
            panic!(
                "Tile vector size mismatch for chunk {:?}: Expected {} but got {}", 
                self.bounds, 
                TILE_COUNT, 
                tiles_vec.len()
            );
        }
    }
    
    /// Returns a mutable reference to the tile data at the given local (x, y) coordinates.
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &mut self.tiles[index]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the coordinate flattening function for standard and edge cases.
    #[test]
    fn test_coord_to_index() {
        // Chunk size is 32 (CHUNK_SIZE)
        
        // 1. Basic check (0, 0)
        assert_eq!(ChunkData::coord_to_index(0, 0), Some(0));

        // 2. Middle point check
        // (Y=16 * SIZE=32) + X=16 = 512 + 16 = 528
        assert_eq!(ChunkData::coord_to_index(16, 16), Some(528));

        // 3. Max boundary check (31, 31)
        // (Y=31 * SIZE=32) + X=31 = 992 + 31 = 1023 (TILE_COUNT - 1)
        assert_eq!(ChunkData::coord_to_index(31, 31), Some(1023));

        // 4. Out-of-bounds check (size is 32, so 32 is out)
        assert_eq!(ChunkData::coord_to_index(32, 0), None);
        assert_eq!(ChunkData::coord_to_index(0, 32), None);
        assert_eq!(ChunkData::coord_to_index(33, 33), None);
    } // <-- Missing brace restored
} // <-- Missing brace restored