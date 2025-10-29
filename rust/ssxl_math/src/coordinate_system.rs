// ssxl_math/src/coordinate_system.rs
//!
//! Defines the coordinate system types used throughout the engine and provides
//! conversion utilities between them.
//!
//! The Aetherion engine uses a chunked, tile-based world system.
//! - WorldPos: Global coordinate, precise to the tile (i32).
//! - ChunkKey: Identifier for the chunk the tile belongs to (i32).
//! - TileOffset: Local position of the tile within its chunk (u8).

use serde::{Serialize, Deserialize};
// FIX: Imported CHUNK_SIZE_I32, as the constant was renamed in math_primitives.rs
use crate::primitives::CHUNK_SIZE_I32;
use glam::IVec3;


// --- 1. Coordinate Types ---

/// A global coordinate precise to the tile level.
/// Used for physics, networking, and high-level queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPos(pub IVec3);

/// The key/identifier for a specific chunk in the world grid.
/// Used for data fetching, loading/unloading, and persistent storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkKey(pub IVec3);

/// The local coordinate of a tile *within* its chunk.
/// Used for array indexing and rendering offsets.
// We use IVec3 (i32) here for consistency, though components will be small (0 to CHUNK_SIZE-1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileOffset(pub IVec3);


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
        
        // 1. Tile Offset calculation (uses Euclidean remainder, which is correct)
        let tile_x = self.0.x.rem_euclid(CHUNK_SIZE_I32);
        let tile_y = self.0.y.rem_euclid(CHUNK_SIZE_I32);
        let tile_z = self.0.z.rem_euclid(CHUNK_SIZE_I32);

        // 2. Chunk Key calculation (FIXED: Use Euclidean quotient derivation)
        // We calculate the quotient by solving: ChunkKey = (WorldPos - TileOffset) / CHUNK_SIZE_I32
        let chunk_x = (self.0.x - tile_x) / CHUNK_SIZE_I32;
        let chunk_y = (self.0.y - tile_y) / CHUNK_SIZE_I32;
        let chunk_z = (self.0.z - tile_z) / CHUNK_SIZE_I32;

        let chunk_key = ChunkKey(IVec3::new(chunk_x, chunk_y, chunk_z));
        let tile_offset = TileOffset(IVec3::new(tile_x, tile_y, tile_z));

        (chunk_key, tile_offset)
    }
}

impl ChunkKey {
    /// Converts a ChunkKey and a local TileOffset back into a global WorldPos.
    pub fn to_world_pos(&self, offset: TileOffset) -> WorldPos {
        let world_x = self.0.x * CHUNK_SIZE_I32 + offset.0.x;
        let world_y = self.0.y * CHUNK_SIZE_I32 + offset.0.y;
        let world_z = self.0.z * CHUNK_SIZE_I32 + offset.0.z;

        WorldPos(IVec3::new(world_x, world_y, world_z))
    }
}

// ---------------------------
// IMPL: Unit Tests (Unchanged)
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;
    // NOTE: CHUNK_SIZE_I32 is imported, let's assume it's 32 for test context.

    #[test]
    fn test_world_to_chunk_positive() {
        // Position (33, 10, 64) -> 1 chunk over, 10 offset, 2 chunks over, 0 offset
        let pos = WorldPos(IVec3::new(33, 10, 64));
        let (key, offset) = pos.to_chunk_coords();
        
        assert_eq!(key.0, IVec3::new(1, 0, 2), "ChunkKey should be (1, 0, 2)");
        assert_eq!(offset.0, IVec3::new(1, 10, 0), "TileOffset should be (1, 10, 0)");
    }

    #[test]
    fn test_world_to_chunk_negative_crucial() {
        // Test 1: Position (-1, -1, -1) - The critical check for Euclidean remainder.
        let pos = WorldPos(IVec3::new(-1, -1, -1));
        let (key, offset) = pos.to_chunk_coords();

        // Must correctly resolve to chunk key (-1, -1, -1)
        assert_eq!(key.0, IVec3::new(-1, -1, -1), "ChunkKey for -1 failed to be -1");
        // Must correctly resolve to max offset (31, 31, 31)
        assert_eq!(offset.0, IVec3::new(31, 31, 31), "TileOffset for -1 failed to be 31");

        // Test 2: Deep inside the negative space
        let pos_deep = WorldPos(IVec3::new(-33, -64, -100));
        let (key_deep, offset_deep) = pos_deep.to_chunk_coords();
        
        // Expected Key: -2, -2, -4. Expected Offset: 31, 0, 28.
        assert_eq!(key_deep.0, IVec3::new(-2, -2, -4), "Deep negative ChunkKey failed");
        assert_eq!(offset_deep.0, IVec3::new(31, 0, 28), "Deep negative TileOffset failed");
    }

    #[test]
    fn test_round_trip() {
        let original_pos = WorldPos(IVec3::new(-123, 456, -789));
        
        // 1. Convert
        let (key, offset) = original_pos.to_chunk_coords();
        
        // 2. Convert back
        let final_pos = key.to_world_pos(offset);

        // 3. Verify
        assert_eq!(original_pos, final_pos, "WorldPos to ChunkKey and back must be idempotent");
    }
}