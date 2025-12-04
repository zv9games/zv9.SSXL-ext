// ssxl_animate/src/animation_logic.rs (O(1) Zero-Entropy)

use ssxl_shared::{
    AnimationType,
    AnimationUpdate,
    ChunkData,
    ChunkId,
    TileCoord,
    AnimationPayload, // Added: Ensure payload is in scope
};

/// Executes animation logic for all tiles within a single chunk.
/// FIX: Returns a zero-allocation Iterator to enforce Zero-Entropy / O(1) mechanism.
pub fn execute_for_chunk(
    chunk_id: ChunkId,
    chunk_data: &ChunkData,
    anim_type: AnimationType,
) -> impl Iterator<Item = AnimationUpdate> + '_ {
    chunk_data.tiles.iter().enumerate().filter_map(move |(index, _tile)| {
        let coord = TileCoord {
            x: (chunk_id.x * 16) + (index % 16) as i64,
            y: (chunk_id.y * 16) + (index / 16) as i64,
        };

        let (source_id_u32, _new_atlas_coords_i32) = match anim_type {
            AnimationType::TileFlip => {
                let new_frame = (coord.x.wrapping_add(coord.y) % 4) as u32;
                (new_frame, 0)
            }
            AnimationType::PulseFade(_intensity) => (0, 0),
            AnimationType::TweenMove | AnimationType::CustomScripted(_) => return None,
        };

        let payload = AnimationPayload::FrameUpdate {
            new_frame: source_id_u32,
        };

        // O(1) yield: No heap allocation
        Some(AnimationUpdate { coord, payload })
    })
}