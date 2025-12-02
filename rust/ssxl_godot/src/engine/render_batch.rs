// ssxl_godot/src/engine/render_batch.rs
//
// Pure, zero-cost, panic-free translation from ChunkData → Godot render batch.
// Used by query.rs and tick.rs.
// No allocation beyond what's required.
// No dependencies beyond godot-rust and ssxl_shared.
// Eternal.

use godot::builtin::*;
use ssxl_shared::{ChunkData, TileType};
// FIX: Import the ToGodot trait to bring the to_variant() method into scope for PackedInt32Array.
use godot::prelude::ToGodot; 

pub const CHUNK_SIZE: i32 = 32;
pub const DEFAULT_SOURCE_ID: i32 = 1;

/// Converts a generated chunk into a render-ready Dictionary
/// Expected format:
/// {
///    "layer": 0,
///    "positions": PackedInt32Array [x1, y1, x2, y2, ...],
///    "source_ids": PackedInt32Array [id, id, ...],
///    "atlas_coords": PackedInt32Array [ax1, ay1, ax2, ay2, ...],
///    "alt_tiles": PackedInt32Array [0, 0, ...]
/// }
///
/// This is the **only** place that knows how to speak Godot's TileMap language.
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

    // Only set if we have data — Godot ignores empty arrays gracefully
    dict.set("positions", positions.to_variant());
    dict.set("source_ids", source_ids.to_variant());
    dict.set("atlas_coords", atlas_coords.to_variant());
    dict.set("alt_tiles", alt_tiles.to_variant());

    if tile_count > 0 {
        tracing::debug!("render_batch: {tile_count} tiles prepared for chunk ({chunk_x}, {chunk_y})");
    }

    dict
}