//!
//! Defines the coordinate system types used throughout the engine and provides
//! conversion utilities between them.
//!
//! The Aetherion engine uses a chunked, tile-based world system.
//! - WorldPos: Global coordinate, precise to the tile (i64).
//! - ChunkKey: Identifier for the chunk the tile belongs to (i64).
//! - TileOffset: Local position of the tile within its chunk (i64, though 0-31).

use serde::{Serialize, Deserialize};
// 📐 BULLDOZER FIX: Imported CHUNK_SIZE_I64 and switched from IVec3 to I64Vec3.
use crate::primitives::CHUNK_SIZE_I64;
use glam::I64Vec3;


// --- 1. Coordinate Types ---

/// A global coordinate precise to the tile level.
/// Used for physics, networking, and high-level queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPos(pub I64Vec3);

/// The key/identifier for a specific chunk in the world grid.
/// Used for data fetching, loading/unloading, and persistent storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkKey(pub I64Vec3);

/// The local coordinate of a tile *within* its chunk.
/// Used for array indexing and rendering offsets.
// Switched to I64Vec3 for consistency with I64 world coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileOffset(pub I64Vec3);


// --- 2. Coordinate Conversion Functions ---

impl WorldPos {
    /// Converts a global WorldPos into its corresponding ChunkKey and local TileOffset.
    ///
    /// The conversion uses the relationship:
    /// WorldPos = (ChunkKey * CHUNK_SIZE) + TileOffset
    ///
    /// This requires using the Euclidean remainder (`rem_euclid`) for the offset,
    /// and then deriving the ChunkKey via floor division to correctly handle negative coordinates.
    pub fn to_chunk_coords(&self) -> (ChunkKey, TileOffset) {
        
        let chunk_size: i64 = CHUNK_SIZE_I64;

        // 1. Tile Offset calculation (uses Euclidean remainder, which is correct)
        // Self.0 is I64Vec3, so all coordinates are i64.
        let tile_x = self.0.x.rem_euclid(chunk_size);
        let tile_y = self.0.y.rem_euclid(chunk_size);
        let tile_z = self.0.z.rem_euclid(chunk_size);

        // 2. Chunk Key calculation (Use Euclidean quotient derivation)
        // ChunkKey = (WorldPos - TileOffset) / CHUNK_SIZE_I64
        let chunk_x = (self.0.x - tile_x) / chunk_size;
        let chunk_y = (self.0.y - tile_y) / chunk_size;
        let chunk_z = (self.0.z - tile_z) / chunk_size;

        let chunk_key = ChunkKey(I64Vec3::new(chunk_x, chunk_y, chunk_z));
        let tile_offset = TileOffset(I64Vec3::new(tile_x, tile_y, tile_z));

        (chunk_key, tile_offset)
    }
}

impl ChunkKey {
    /// Converts a ChunkKey and a local TileOffset back into a global WorldPos.
    pub fn to_world_pos(&self, offset: TileOffset) -> WorldPos {
        let chunk_size: i64 = CHUNK_SIZE_I64;

        let world_x = self.0.x * chunk_size + offset.0.x;
        let world_y = self.0.y * chunk_size + offset.0.y;
        let world_z = self.0.z * chunk_size + offset.0.z;

        WorldPos(I64Vec3::new(world_x, world_y, world_z))
    }
}

// ---------------------------
// IMPL: Unit Tests (Updated to use I64Vec3 and test high coordinates)
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::I64Vec3;
    // NOTE: CHUNK_SIZE_I64 is 32 for test context.

    #[test]
    fn test_world_to_chunk_positive() {
        // Test high coordinate (i64 test)
        let huge_coord = 5_000_000_000i64; // Well over i32::MAX
        let expected_chunk_key = huge_coord / CHUNK_SIZE_I64; // 156,250,000
        let expected_offset = huge_coord % CHUNK_SIZE_I64; // 0
        
        // Position (huge_coord, 10, 64)
        let pos = WorldPos(I64Vec3::new(huge_coord, 10, 64));
        let (key, offset) = pos.to_chunk_coords();
        
        assert_eq!(key.0, I64Vec3::new(expected_chunk_key, 0, 2), "ChunkKey failed for i64 coord");
        assert_eq!(offset.0, I64Vec3::new(expected_offset, 10, 0), "TileOffset failed for i64 coord");
    }

    #[test]
    fn test_world_to_chunk_negative_crucial() {
        // Test 1: Position (-1, -1, -1) - The critical check for Euclidean remainder.
        let pos = WorldPos(I64Vec3::new(-1, -1, -1));
        let (key, offset) = pos.to_chunk_coords();

        // Must correctly resolve to chunk key (-1, -1, -1)
        assert_eq!(key.0, I64Vec3::new(-1, -1, -1), "ChunkKey for -1 failed to be -1");
        // Must correctly resolve to max offset (31, 31, 31)
        assert_eq!(offset.0, I64Vec3::new(31, 31, 31), "TileOffset for -1 failed to be 31");

        // Test 2: Deep inside the negative space
        let pos_deep = WorldPos(I64Vec3::new(-33, -64, -100));
        let (key_deep, offset_deep) = pos_deep.to_chunk_coords();
        
        // Expected Key: -2, -2, -4. Expected Offset: 31, 0, 28.
        assert_eq!(key_deep.0, I64Vec3::new(-2, -2, -4), "Deep negative ChunkKey failed");
        assert_eq!(offset_deep.0, I64Vec3::new(31, 0, 28), "Deep negative TileOffset failed");
    }

    #[test]
    fn test_round_trip() {
        // Test round trip with large negative coordinate to ensure no i64 overflow errors
        let original_pos = WorldPos(I64Vec3::new(-9_876_543_210, 456, -789));
        
        // 1. Convert
        let (key, offset) = original_pos.to_chunk_coords();
        
        // 2. Convert back
        let final_pos = key.to_world_pos(offset);

        // 3. Verify
        assert_eq!(original_pos, final_pos, "WorldPos to ChunkKey and back must be idempotent with i64");
    }
}