use std::collections::HashMap;

use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};

use crate::shared_tile::TileData;

/// Native TileMap bridge for SSXL.
/// - Owns a per-chunk buffer of `TileData`.
/// - Exposes raw pointers into those buffers for the SSXL core.
/// - Applies those buffers back onto the Godot TileMap cells.
#[derive(GodotClass)]
#[class(base = TileMap)]
pub struct SSXLTileMap {
    /// Base TileMap reference (Godot side).
    #[base]
    base: Base<TileMap>,

    /// Map of (cx, cy) -> flat tile buffer for that chunk.
    chunk_buffers: HashMap<(i32, i32), Vec<TileData>>,

    /// Cached chunk size in cells (must match SSXL config / host_state).
    chunk_size: i32,
}

#[godot_api]
impl ITileMap for SSXLTileMap {
    /// Godot constructor: required for #[derive(GodotClass)].
    fn init(base: Base<TileMap>) -> Self {
        SSXLTileMap {
            base,
            chunk_buffers: HashMap::new(),
            chunk_size: 32, // default; can be overridden from Godot
        }
    }
}

#[godot_api]
impl SSXLTileMap {
    /// Optional: allow Godot to set the chunk size explicitly.
    #[func]
    pub fn set_chunk_size(&mut self, size: i32) {
        if size > 0 {
            self.chunk_size = size;
        }
    }

    #[func]
    pub fn get_chunk_size(&self) -> i32 {
        self.chunk_size
    }

    /// Core bridge method:
    /// Returns a raw pointer into a contiguous TileData buffer for chunk (cx, cy).
    #[func]
    pub fn get_raw_chunk_data_ptr(&mut self, layer: i32, cx: i32, cy: i32) -> *mut u8 {
        let _ = layer; // placeholder for multi-layer support

        let key = (cx, cy);
        let chunk_area = (self.chunk_size * self.chunk_size) as usize;

        // Ensure buffer exists and is large enough
        let buf = self
            .chunk_buffers
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(chunk_area));

        if buf.len() < chunk_area {
            buf.resize(chunk_area, TileData::default());
        }

        // Return raw pointer as *mut u8
        buf.as_mut_ptr() as *mut u8
    }

    /// Called by SSXL after writing into the chunk buffer.
    /// Applies the TileData to the actual Godot TileMap.
    #[func]
    pub fn notify_chunk_data_changed(&mut self, layer: i32, cx: i32, cy: i32) {
        let key = (cx, cy);

        // ✅ Clone buffer to avoid borrow conflicts
        let buf: Vec<TileData> = match self.chunk_buffers.get(&key) {
            Some(b) => b.clone(),
            None => return,
        };

        let chunk_size = self.chunk_size;
        let tiles_per_side = chunk_size;
        let mut base = self.base_mut();

        // Compute the origin (cell-space) of this chunk
        let origin_x = cx * chunk_size;
        let origin_y = cy * chunk_size;

        for local_y in 0..tiles_per_side {
            for local_x in 0..tiles_per_side {
                let idx = (local_y * tiles_per_side + local_x) as usize;
                if idx >= buf.len() {
                    continue;
                }

                let tile = buf[idx];

                // Skip empty tiles
                if tile.tile_id == 0 {
                    continue;
                }

                let global_x = origin_x + local_x;
                let global_y = origin_y + local_y;
                let cell = Vector2i::new(global_x, global_y);

                // ✅ Decode atlas coords (packed u16 → (x,y))
                let atlas_x = (tile.atlas_coords & 0x00FF) as i32;
                let atlas_y = ((tile.atlas_coords >> 8) & 0x00FF) as i32;
                let atlas_coords = Vector2i::new(atlas_x, atlas_y);

                // ✅ TileSet source ID (always 0 for your atlas)
                let source_id = tile.tile_id as i32;

                // ✅ Call full Godot TileMap API via Object::call
                base.call(
                    "set_cell",
                    &[
                        layer.to_variant(),
                        cell.to_variant(),
                        source_id.to_variant(),
                        atlas_coords.to_variant(),
                        0.to_variant(), // alternative tile index
                    ],
                );
            }
        }
    }

    /// Optional helper to clear all SSXL chunk buffers and TileMap cells.
    #[func]
    pub fn clear_ssxl_chunks(&mut self, layer: i32) {
        self.chunk_buffers.clear();
        let mut base = self.base_mut();
        base.clear_layer(layer);
    }
}
