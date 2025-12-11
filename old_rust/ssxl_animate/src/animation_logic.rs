// ============================================================================
// 🎨 Chunk Animation Executor (`execute_for_chunk`)
// ----------------------------------------------------------------------------
// This function defines how animations are applied across all tiles in a single
// chunk of the SSXL engine. It is a core utility in the animation subsystem,
// transforming raw chunk data into a stream of `AnimationUpdate` messages.
//
// Key Concepts:
//   • ChunkData: Represents a fixed-size grid of tiles (here assumed 16x16).
//   • ChunkId: Identifies the chunk’s position in world-space (x,y offset).
//   • TileCoord: World-space coordinates of an individual tile.
//   • AnimationType: Enum describing which animation logic to apply.
//   • AnimationPayload: Encapsulates the actual animation update (e.g., new frame).
//   • AnimationUpdate: Struct combining tile coordinates with payload for FFI.
//
// Workflow:
//   1. Iterate over all tiles in the chunk using `enumerate()`.
//      - Index provides position within the chunk.
//      - Tile reference is unused here but could be extended for tile-specific logic.
//   2. Compute world coordinates for each tile.
//      - x = chunk_id.x * 16 + column index
//      - y = chunk_id.y * 16 + row index
//   3. Match on `AnimationType` to decide what update (if any) to produce.
//      - TileFlip: Computes a new frame index based on tile coordinates.
//      - PulseFade: Placeholder, currently returns (0,0).
//      - TweenMove / CustomScripted: Skips update entirely (returns None).
//   4. Wrap the computed frame index into an `AnimationPayload::FrameUpdate`.
//   5. Yield an `AnimationUpdate` containing the tile’s coordinates and payload.
//      - Uses `filter_map` so skipped tiles (None) are excluded from the iterator.
//
// Design Choices:
//   • Lazy iterator: No heap allocation; updates are produced on demand.
//   • `filter_map`: Elegant way to skip tiles that don’t require updates.
//   • Wrapping arithmetic: Ensures safe frame index computation without overflow.
//   • Extensible match arms: New animation types can be added easily.
//
// Educational Note:
//   • This function demonstrates how functional iteration patterns (`map`,
//     `filter_map`) can replace imperative loops for clarity and efficiency.
//   • By returning an iterator, the system avoids building intermediate vectors,
//     keeping memory usage low and throughput high.
//   • This is a common pattern in game engines: compute updates lazily, then
//     stream them into the rendering or FFI layer.
// ============================================================================


use ssxl_shared::{
    AnimationType,
    AnimationUpdate,
    ChunkData,
    ChunkId,
    TileCoord,
    AnimationPayload,
};

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

        Some(AnimationUpdate { coord, payload })
    })
}
