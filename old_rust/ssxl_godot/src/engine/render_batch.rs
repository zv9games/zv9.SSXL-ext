// ============================================================================
// 🎨 Render Batch Dictionary (`crate::engine::render_batch`)
// ----------------------------------------------------------------------------
// This module defines the `create_render_batch_dictionary` function, which
// translates raw `ChunkData` from the SSXL engine into a Godot-compatible
// `Dictionary`. The dictionary is structured specifically for Godot’s TileMap
// API, enabling efficient rendering of terrain chunks.
//
// Purpose:
//   • Convert engine-generated tile data into Godot-native structures.
//   • Provide arrays of positions, source IDs, atlas coordinates, and alt tiles.
//   • Act as the bridge between Rust’s generation system and Godot’s rendering layer.
//
// Key Components:
//   • CHUNK_SIZE
//       - Fixed dimension of a chunk (32x32 tiles).
//   • DEFAULT_SOURCE_ID
//       - Default identifier for the tile source in Godot’s TileMap.
//   • Dictionary
//       - Godot’s built-in key-value container used to return structured data.
//   • PackedInt32Array
//       - Efficient array type for storing integer sequences (positions, IDs, coords).
//   • TileType
//       - Enum describing tile types (grass, water, empty, etc.).
//       - Provides helpers like `is_empty()` and atlas coordinate lookup.
//
// Function: create_render_batch_dictionary
//   • Arguments:
//       - chunk_data: reference to `ChunkData` containing tile information.
//       - chunk_x, chunk_y: coordinates of the chunk in world space.
//   • Workflow:
//       1. Initialize output dictionary and arrays for rendering data.
//       2. Iterate over every tile in the chunk grid (32x32).
//       3. Skip empty tiles (no rendering needed).
//       4. Compute world-space coordinates for each tile.
//       5. Push positions, source IDs, atlas coordinates, and alt tile indices.
//       6. Store arrays into dictionary (converted to Godot Variants).
//       7. Log debug message if any tiles were prepared.
//       8. Return the fully constructed dictionary.
//   • Returns:
//       - Dictionary formatted for Godot’s TileMap API, e.g.:
//         {
//            "layer": 0,
//            "positions": PackedInt32Array [x1, y1, x2, y2, ...],
//            "source_ids": PackedInt32Array [id, id, ...],
//            "atlas_coords": PackedInt32Array [ax1, ay1, ax2, ay2, ...],
//            "alt_tiles": PackedInt32Array [0, 0, ...]
//         }
//
// Design Choices:
//   • Non-empty tiles only → avoids unnecessary rendering overhead.
//   • Default source ID simplifies integration with Godot TileSets.
//   • Atlas coordinates provide flexible mapping to visual assets.
//   • Logging ensures visibility into rendering pipeline performance.
//
// Educational Note:
//   • This function demonstrates how Rust can act as a translation layer
//     between procedural generation systems and external rendering engines.
//     By structuring data into Godot-native containers, it ensures seamless
//     integration while maintaining efficiency and clarity.
// ============================================================================


use godot::builtin::*;
use ssxl_shared::{ChunkData, TileType};
use godot::prelude::ToGodot; 

pub const CHUNK_SIZE: i32 = 32;
pub const DEFAULT_SOURCE_ID: i32 = 1;

pub fn create_render_batch_dictionary(chunk_data: &ChunkData, chunk_x: i32, chunk_y: i32) -> Dictionary {
    let mut dict = Dictionary::new();

    let mut positions = PackedInt32Array::new();
    let mut source_ids = PackedInt32Array::new();
    let mut atlas_coords = PackedInt32Array::new();
    let mut alt_tiles = PackedInt32Array::new();

    dict.set("layer", 0i64);

    let mut tile_count = 0usize;

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let idx = (y * CHUNK_SIZE + x) as usize;

            if let Some(tile) = chunk_data.tiles.get(idx) {
                if TileType::is_empty(tile.tile_type) {
                    continue;
                }

                let world_x = chunk_x * CHUNK_SIZE + x;
                let world_y = chunk_y * CHUNK_SIZE + y;

                positions.push(world_x);
                positions.push(world_y);

                source_ids.push(DEFAULT_SOURCE_ID);

                let (ax, ay) = TileType::get_default_atlas_coords(tile.tile_type);
                atlas_coords.push(ax as i32);
                atlas_coords.push(ay as i32);

                alt_tiles.push(0);

                tile_count += 1;
            }
        }
    }

    dict.set("positions", positions.to_variant());
    dict.set("source_ids", source_ids.to_variant());
    dict.set("atlas_coords", atlas_coords.to_variant());
    dict.set("alt_tiles", alt_tiles.to_variant());

    if tile_count > 0 {
        tracing::debug!("render_batch: {tile_count} tiles prepared for chunk ({chunk_x}, {chunk_y})");
    }

    dict
}
