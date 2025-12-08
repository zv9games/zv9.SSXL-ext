// ============================================================================
// 🌍 Coordinate System (`crate::coordinates`)
// ----------------------------------------------------------------------------
// This module defines the fundamental coordinate types used by the SSXL engine.
// It provides a robust system for mapping between global world positions,
// chunk indices, and local tile offsets, ensuring consistency across infinite
// 3D space.
//
// Purpose:
//   • Represent positions in an infinite 3D world using 64-bit integer vectors.
//   • Split global coordinates into chunk-local coordinates for efficient storage.
//   • Reconstruct global positions from chunk indices and tile offsets.
//   • Handle both positive and negative coordinates correctly using Euclidean division.
//
// Key Types:
//   • WorldPos
//       - Wraps an `I64Vec3` representing a specific point in the world.
//       - Supports extremely large coordinates beyond i32 limits.
//       - Provides `to_chunk_coords()` for splitting into chunk + tile offset.
//
//   • ChunkKey
//       - Wraps an `I64Vec3` representing the index of a chunk in world space.
//       - Each chunk is a cubic volume of size `CHUNK_SIZE_I64`.
//       - Provides `to_world_pos()` for reconstructing global positions.
//
//   • TileOffset
//       - Wraps an `I64Vec3` representing the local position of a tile inside
//         its parent chunk.
//       - Always non-negative, ranging from 0 to `CHUNK_SIZE_I64 - 1`.
//
// Conversion Logic:
//   • WorldPos → (ChunkKey, TileOffset)
//       - Uses `rem_euclid` to ensure non-negative offsets even for negative coordinates.
//       - ChunkKey is computed by subtracting the offset and dividing by chunk size.
//   • ChunkKey + TileOffset → WorldPos
//       - Formula: World = (Chunk Index * Chunk Size) + Tile Offset.
//       - Ensures round-trip consistency with `WorldPos::to_chunk_coords`.
//
// Tests:
//   • Validates correctness for large positive coordinates (well beyond i32::MAX).
//   • Validates correctness for negative coordinates, ensuring offsets remain non-negative.
//   • Round-trip tests confirm idempotency: converting WorldPos → ChunkKey/TileOffset → WorldPos
//     yields the original position.
//
// Design Choices:
//   • 64-bit vectors (`I64Vec3`) allow for virtually infinite coordinate ranges.
//   • Euclidean division ensures offsets are always valid within chunk boundaries.
//   • Separation of WorldPos, ChunkKey, and TileOffset provides semantic clarity
//     and prevents misuse of raw vectors.
//
// Educational Note:
//   • This module demonstrates how to design a coordinate system for infinite
//     voxel or tile-based worlds. By splitting global positions into chunk-local
//     coordinates, the engine can efficiently manage memory, rendering, and
//     procedural generation across vast spaces.
// ============================================================================


use serde::{Serialize, Deserialize};
use crate::primitives::CHUNK_SIZE_I64;
use glam::I64Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPos(pub I64Vec3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkKey(pub I64Vec3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileOffset(pub I64Vec3);

impl WorldPos {
    pub fn to_chunk_coords(&self) -> (ChunkKey, TileOffset) {
        let chunk_size = CHUNK_SIZE_I64;

        let tile_x = self.0.x.rem_euclid(chunk_size);
        let tile_y = self.0.y.rem_euclid(chunk_size);
        let tile_z = self.0.z.rem_euclid(chunk_size);

        let chunk_x = (self.0.x - tile_x) / chunk_size;
        let chunk_y = (self.0.y - tile_y) / chunk_size;
        let chunk_z = (self.0.z - tile_z) / chunk_size;

        let chunk_key = ChunkKey(I64Vec3::new(chunk_x, chunk_y, chunk_z));
        let tile_offset = TileOffset(I64Vec3::new(tile_x, tile_y, tile_z));

        (chunk_key, tile_offset)
    }
}

impl ChunkKey {
    pub fn to_world_pos(&self, offset: TileOffset) -> WorldPos {
        let chunk_size = CHUNK_SIZE_I64;

        let world_x = self.0.x * chunk_size + offset.0.x;
        let world_y = self.0.y * chunk_size + offset.0.y;
        let world_z = self.0.z * chunk_size + offset.0.z;

        WorldPos(I64Vec3::new(world_x, world_y, world_z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::I64Vec3;

    #[test]
    fn test_world_to_chunk_positive() {
        let huge_coord = 5_000_000_000i64;
        let expected_chunk_key = huge_coord / CHUNK_SIZE_I64;
        let expected_offset = huge_coord % CHUNK_SIZE_I64;

        let pos = WorldPos(I64Vec3::new(huge_coord, 10, 64));
        let (key, offset) = pos.to_chunk_coords();

        assert_eq!(key.0, I64Vec3::new(expected_chunk_key, 0, 2), "ChunkKey failed for i64 coord");
        assert_eq!(offset.0, I64Vec3::new(expected_offset, 10, 0), "TileOffset failed for i64 coord");
    }

    #[test]
    fn test_world_to_chunk_negative_crucial() {
        let pos = WorldPos(I64Vec3::new(-1, -1, -1));
        let (key, offset) = pos.to_chunk_coords();

        assert_eq!(key.0, I64Vec3::new(-1, -1, -1), "ChunkKey for -1 failed to be -1");
        assert_eq!(offset.0, I64Vec3::new(31, 31, 31), "TileOffset for -1 failed to be 31");

        let pos_deep = WorldPos(I64Vec3::new(-33, -64, -100));
        let (key_deep, offset_deep) = pos_deep.to_chunk_coords();

        assert_eq!(key_deep.0, I64Vec3::new(-2, -2, -4), "Deep negative ChunkKey failed");
        assert_eq!(offset_deep.0, I64Vec3::new(31, 0, 28), "Deep negative TileOffset failed");
    }

    #[test]
    fn test_round_trip() {
        let original_pos = WorldPos(I64Vec3::new(-9_876_543_210, 456, -789));

        let (key, offset) = original_pos.to_chunk_coords();
        let final_pos = key.to_world_pos(offset);

        assert_eq!(original_pos, final_pos, "WorldPos to ChunkKey and back must be idempotent with i64");
    }
}
