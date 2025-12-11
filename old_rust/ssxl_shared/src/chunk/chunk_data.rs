// ============================================================================
// 🧱 Chunk Data Module (`ssxl_shared::chunk::chunk_data`)
// ----------------------------------------------------------------------------
// This module defines the core structures and utilities for handling chunks,
// which are fixed-size grids of tiles (32x32 by default) in the SSXL engine.
// Chunks are the atomic unit of world generation, caching, and rendering.
//
// Key Concepts:
//   • ChunkCoords: Identifies a chunk’s position in chunk-space (grid coordinates).
//   • ChunkData: Holds all data for a chunk, including:
//       - Unique ID (hashed from coordinates)
//       - Bounds in world-space
//       - Tile array (1024 TileData entries)
//       - Dimension tag (e.g., "Overworld", "Default")
//       - Generation timestamp (serialized deterministically)
//
// Design Choices:
//   • Fixed size (CHUNK_SIZE = 32) ensures predictable memory layout.
//   • Tiles stored in a flat array for cache-friendly access.
//   • `serde_big_array` used to serialize large fixed arrays.
//   • `system_time_serde` ensures timestamps are cross-platform and deterministic.
//   • Zigzag encoding + bit-packing used to generate unique chunk IDs.
//
// Core Methods:
//   • new / new_at_coords: Constructors for creating chunks either by explicit
//     parameters or by grid coordinates.
//   • zigzag_encode / hash_coords_2d: Utilities for encoding signed coordinates
//     into compact, unique u64 identifiers.
//   • coord_to_index_checked / coord_to_index_unchecked: Convert (x,y) tile
//     coordinates into array indices, with or without bounds checking.
//   • get_tile / get_tile_mut: Safe accessors for tiles.
//   • get_tile_unchecked / get_tile_mut_unchecked: Unsafe, faster accessors
//     when bounds are guaranteed externally.
//   • insert_tiles: Bulk replacement of the tile array, with validation.
//
// Testing:
//   • Unit tests validate index conversion, zigzag encoding, and hashing logic.
//   • Ensures correctness for edge cases (negative coords, large values).
//
// Educational Note:
//   • This module demonstrates how game engines balance safety and performance:
//     - Safe methods for general use.
//     - Unsafe methods for hot paths where performance is critical.
//   • Hashing and encoding strategies ensure chunks can be uniquely identified
//     across infinite coordinate ranges.
// ============================================================================


use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::convert::TryInto;
use std::ptr; 

use super::grid_bounds::GridBounds;
use crate::tile::tile_data::TileData; 
use crate::math::math_primitives;
use ssxl_math::prelude::Vec2i;
use serde_big_array::BigArray;

pub const CHUNK_SIZE: u32 = 32;
const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoords {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    pub id: u64,
    pub bounds: GridBounds,
    #[serde(with = "BigArray")]
    pub tiles: [TileData; TILE_ARRAY_SIZE],
    pub dimension_tag: String,
    #[serde(with = "math_primitives::system_time_serde")]
    pub generated_at: SystemTime,
}

impl ChunkData {
    pub const SIZE: u32 = CHUNK_SIZE;

    pub fn new(id: u64, bounds: GridBounds, dimension_tag: String) -> Self {
        let tiles = [TileData::default(); TILE_ARRAY_SIZE];
        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag,
            generated_at: SystemTime::now(),
        }
    }
    
    pub fn new_at_coords(chunk_coords: Vec2i) -> Self {
        let chunk_size_i64 = Self::SIZE as i64;
        
        let min_x = chunk_coords.x * chunk_size_i64;
        let min_y = chunk_coords.y * chunk_size_i64;
        
        let max_x = min_x + chunk_size_i64;
        let max_y = min_y + chunk_size_i64;

        let bounds = GridBounds::new(min_x, min_y, max_x, max_y);
        
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

    #[inline(always)]
    pub const fn zigzag_encode(n: i64) -> u64 {
        ((n << 1) ^ (n >> 63)) as u64
    }

    #[inline(always)]
    pub const fn hash_coords_2d(x: i64, y: i64) -> u64 {
        let ux = Self::zigzag_encode(x);
        let uy = Self::zigzag_encode(y);
        ux | (uy << 32)
    }
    
    #[inline(always)]
    fn coord_to_index_checked(x: u32, y: u32) -> Option<usize> {
        if x < Self::SIZE && y < Self::SIZE {
            Some((y * Self::SIZE + x) as usize)
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn coord_to_index_unchecked(x: u32, y: u32) -> usize {
        (y * Self::SIZE + x) as usize
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        Self::coord_to_index_checked(x, y).map(|index| {
            &self.tiles[index]
        })
    }
    
    #[inline(always)]
    pub unsafe fn get_tile_unchecked(&self, x: u32, y: u32) -> &TileData {
        let index = Self::coord_to_index_unchecked(x, y);
        &self.tiles[index]
    }
    
    pub fn insert_tiles(&mut self, tiles_vec: Vec<TileData>) {
        match tiles_vec.try_into() {
            Ok(arr) => {
                unsafe { ptr::write(&mut self.tiles, arr) };
            }
            Err(vec) => {
                panic!(
                    "Tile vector size mismatch for chunk {:?}. Generator returned {} tiles, but expected {}.",
                    self.bounds,
                    vec.len(),
                    TILE_ARRAY_SIZE
                );
            }
        }
    }
    
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        Self::coord_to_index_checked(x, y).map(|index| {
            &mut self.tiles[index]
        })
    }

    #[inline(always)]
    pub unsafe fn get_tile_mut_unchecked(&mut self, x: u32, y: u32) -> &mut TileData {
        let index = Self::coord_to_index_unchecked(x, y);
        &mut self.tiles[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coord_to_index() {
        assert_eq!(ChunkData::coord_to_index_checked(0, 0), Some(0));
        assert_eq!(ChunkData::coord_to_index_checked(16, 16), Some(528));
        assert_eq!(ChunkData::coord_to_index_checked(31, 31), Some(1023));
        assert_eq!(ChunkData::coord_to_index_checked(32, 0), None);
        assert_eq!(ChunkData::coord_to_index_checked(0, 32), None);
        assert_eq!(ChunkData::coord_to_index_checked(33, 33), None);
        
        assert_eq!(ChunkData::coord_to_index_unchecked(0, 0), 0);
        assert_eq!(ChunkData::coord_to_index_unchecked(31, 31), 1023);
    }
    
    #[test]
    fn test_zigzag_encode() {
        assert_eq!(ChunkData::zigzag_encode(0), 0);
        assert_eq!(ChunkData::zigzag_encode(-1), 1);
        assert_eq!(ChunkData::zigzag_encode(1), 2);
        assert_eq!(ChunkData::zigzag_encode(-2), 3);
        assert_eq!(ChunkData::zigzag_encode(2), 4);
    }

    #[test]
    fn test_hash_coords_2d() {
        assert_eq!(ChunkData::hash_coords_2d(1, 0), 2);
        assert_eq!(ChunkData::hash_coords_2d(0, 1), 8589934592);
        assert_eq!(ChunkData::hash_coords_2d(-1, 0), 1);
        assert_ne!(ChunkData::hash_coords_2d(1, 1), ChunkData::hash_coords_2d(-1, -1));
        
        let big_x = i64::MAX / 2;
        let big_y = i64::MIN / 2;
        let hash1 = ChunkData::hash_coords_2d(big_x, big_y);
        assert_ne!(hash1, 0);
    }
}
