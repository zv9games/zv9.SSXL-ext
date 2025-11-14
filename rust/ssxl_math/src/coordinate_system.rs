// ssxl_math/src/coordinate_system.rs

//! # Coordinate System
//!
//! Defines the core coordinate types for the SSXL-ext engine, using 64-bit integers (`I64Vec3`)
//! to support extremely large, virtually infinite world dimensions.
//!
//! This module implements the crucial logic for converting a global `WorldPos` into its
//! localized `(ChunkKey, TileOffset)` components, ensuring mathematical correctness
//! even for large **negative coordinates** through the use of Euclidean division.

use serde::{Serialize, Deserialize};
use crate::primitives::CHUNK_SIZE_I64;
use glam::I64Vec3;


// -----------------------------------------------------------------------------
// Core Coordinate Types
// -----------------------------------------------------------------------------

/// Represents a specific point in the vast 3D world space.
/// The underlying `I64Vec3` ensures the system can handle coordinates well beyond i32 limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPos(pub I64Vec3);

/// The unique identifier for a 3D chunk.
/// This acts as the *base coordinate* for a 32x32x32 (or `CHUNK_SIZE`) cubic volume of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkKey(pub I64Vec3);

/// The local position of a tile *within* its parent chunk.
/// Coordinates are always non-negative and range from `[0, CHUNK_SIZE - 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileOffset(pub I64Vec3);


// -----------------------------------------------------------------------------
// WorldPos Implementation
// -----------------------------------------------------------------------------

impl WorldPos {
    /// Converts a global `WorldPos` into its discrete `ChunkKey` and local `TileOffset` components.
    ///
    /// This conversion is robust against **negative world coordinates** by using
    /// `rem_euclid`, which guarantees the `TileOffset` is always positive (or zero).
    pub fn to_chunk_coords(&self) -> (ChunkKey, TileOffset) {

        let chunk_size = CHUNK_SIZE_I64;

        // 1. Calculate the Tile Offset (Local Position)
        // `rem_euclid` ensures the remainder is always non-negative, correctly wrapping
        // negative world coordinates (e.g., -1 rem_euclid 32 = 31).
        let tile_x = self.0.x.rem_euclid(chunk_size);
        let tile_y = self.0.y.rem_euclid(chunk_size);
        let tile_z = self.0.z.rem_euclid(chunk_size);

        // 2. Calculate the Chunk Key (Chunk Index)
        // The chunk index is calculated by subtracting the tile offset (remainder)
        // and then performing simple integer division. This is the correct Euclidean
        // division for finding the quotient (chunk index).
        let chunk_x = (self.0.x - tile_x) / chunk_size;
        let chunk_y = (self.0.y - tile_y) / chunk_size;
        let chunk_z = (self.0.z - tile_z) / chunk_size;

        let chunk_key = ChunkKey(I64Vec3::new(chunk_x, chunk_y, chunk_z));
        let tile_offset = TileOffset(I64Vec3::new(tile_x, tile_y, tile_z));

        (chunk_key, tile_offset)
    }
}

// -----------------------------------------------------------------------------
// ChunkKey Implementation
// -----------------------------------------------------------------------------

impl ChunkKey {
    /// Reconstructs the original `WorldPos` from the `ChunkKey` and `TileOffset`.
    ///
    /// This is the inverse operation, verifying the idempotence of the coordinate system.
    pub fn to_world_pos(&self, offset: TileOffset) -> WorldPos {
        let chunk_size = CHUNK_SIZE_I64;

        // Formula: World = (Chunk Index * Chunk Size) + Tile Offset
        let world_x = self.0.x * chunk_size + offset.0.x;
        let world_y = self.0.y * chunk_size + offset.0.y;
        let world_z = self.0.z * chunk_size + offset.0.z;

        WorldPos(I64Vec3::new(world_x, world_y, world_z))
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::I64Vec3;

    // NOTE: CHUNK_SIZE_I64 is assumed to be 32 for these tests.

    #[test]
    /// Tests conversion for a very large positive coordinate, confirming I64 robustness.
    fn test_world_to_chunk_positive() {
        let huge_coord = 5_000_000_000i64; // Well over i32::MAX
        let expected_chunk_key = huge_coord / CHUNK_SIZE_I64; // 156,250,000
        let expected_offset = huge_coord % CHUNK_SIZE_I64; // 0

        let pos = WorldPos(I64Vec3::new(huge_coord, 10, 64));
        let (key, offset) = pos.to_chunk_coords();

        // 64 / 32 = 2 (chunk key), 64 rem_euclid 32 = 0 (offset)
        assert_eq!(key.0, I64Vec3::new(expected_chunk_key, 0, 2), "ChunkKey failed for i64 coord");
        assert_eq!(offset.0, I64Vec3::new(expected_offset, 10, 0), "TileOffset failed for i64 coord");
    }

    #[test]
    /// CRUCIAL test for negative coordinates, verifying the Euclidean division logic.
    fn test_world_to_chunk_negative_crucial() {
        // Test case 1: (-1, -1, -1)
        // Expected: Chunk index should be -1. Offset should be 31 (32 - 1).
        let pos = WorldPos(I64Vec3::new(-1, -1, -1));
        let (key, offset) = pos.to_chunk_coords();

        assert_eq!(key.0, I64Vec3::new(-1, -1, -1), "ChunkKey for -1 failed to be -1");
        assert_eq!(offset.0, I64Vec3::new(31, 31, 31), "TileOffset for -1 failed to be 31");

        // Test case 2: (-33, -64, -100)
        // X: -33 rem 32 = 31, (-33 - 31)/32 = -2.
        // Y: -64 rem 32 = 0, (-64 - 0)/32 = -2.
        // Z: -100 rem 32 = 28, (-100 - 28)/32 = -4.
        let pos_deep = WorldPos(I64Vec3::new(-33, -64, -100));
        let (key_deep, offset_deep) = pos_deep.to_chunk_coords();

        assert_eq!(key_deep.0, I64Vec3::new(-2, -2, -4), "Deep negative ChunkKey failed");
        assert_eq!(offset_deep.0, I64Vec3::new(31, 0, 28), "Deep negative TileOffset failed");
    }

    #[test]
    /// Tests that converting a WorldPos to (ChunkKey, TileOffset) and back results in the original WorldPos.
    fn test_round_trip() {
        let original_pos = WorldPos(I64Vec3::new(-9_876_543_210, 456, -789));

        let (key, offset) = original_pos.to_chunk_coords();

        let final_pos = key.to_world_pos(offset);

        assert_eq!(original_pos, final_pos, "WorldPos to ChunkKey and back must be idempotent with i64");
    }
}