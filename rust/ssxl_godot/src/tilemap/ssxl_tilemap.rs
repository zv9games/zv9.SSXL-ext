// ssxl_godot/src/tilemap/ssxl_tilemap.rs
//
// The final render sink.
// Receives render batches from the async core.
// Pure, fast, panic-safe, zero bloat.

use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};
use godot::obj::Base;
// FIX: Removed unused import: `Vector2`
use godot::builtin::{Vector2i, PackedVector2Array, PackedInt32Array};

#[derive(GodotClass)]
#[class(base = TileMap)]
pub struct SSXLTilemap {
    base: Base<TileMap>,

    #[export]
    tile_source_id: i32,
}

#[godot_api]
impl ITileMap for SSXLTilemap {
    fn init(base: Base<TileMap>) -> Self {
        Self {
            base,
            tile_source_id: 1,
        }
    }
}

#[godot_api]
impl SSXLTilemap {
    /// Primary render entrypoint — called from Rust via signal
    /// Expects the exact format from render_batch.rs
    #[func]
    pub fn batch_set_tiles(&mut self, batch: Dictionary) {
        // FIX E0308: Convert Result<i64, ConvertError> to Option<i64> using `.ok()`
        let Some(layer) = batch.get("layer").and_then(|v| v.try_to::<i64>().ok()) else {
            godot_warn!("SSXLTilemap: missing or invalid 'layer'");
            return;
        };
        let layer = layer as i32;

        // FIX E0308: Convert inner Result to Option using `.ok()`
        let positions = match batch.get("positions").and_then(|v| v.try_to::<PackedVector2Array>().ok()) {
            Some(p) => p,
            None => {
                godot_warn!("SSXLTilemap: missing 'positions'");
                return;
            }
        };

        // FIX E0308: Convert inner Result to Option using `.ok()`
        let atlas_coords = match batch.get("atlas_coords").and_then(|v| v.try_to::<PackedVector2Array>().ok()) {
            Some(a) => a,
            None => {
                godot_warn!("SSXLTilemap: missing 'atlas_coords'");
                return;
            }
        };

        // FIX E0308: Convert inner Result to Option using `.ok()`
        let alt_tiles = batch
            .get("alt_tiles")
            .and_then(|v| v.try_to::<PackedInt32Array>().ok())
            .unwrap_or_default();

        let source_id = self.tile_source_id;
        let mut tilemap = self.base_mut();

        tilemap.set_layer_enabled(layer, true);

        let len = positions.len();
        if len == 0 {
            return;
        }

        for i in 0..len {
            // FIX E0609: Unwrap the Option<Vector2> before accessing .x and .y fields.
            let pos = positions.get(i).unwrap();
            let atlas = atlas_coords.get(i).unwrap();
            let alt = alt_tiles.get(i).unwrap_or(0);

            // Note: pos and atlas coordinates come as Vector2 (f32 fields), but they represent integer tile coordinates.
            // Casting to i32 is appropriate here.
            let cell = Vector2i::new(pos.x as i32, pos.y as i32);
            let atlas = Vector2i::new(atlas.x as i32, atlas.y as i32);

            tilemap
                .set_cell_ex(layer, cell)
                .source_id(source_id)
                .atlas_coords(atlas)
                .alternative_tile(alt)
                .done();
        }

        godot_print!("SSXLTilemap: Rendered {len} tiles on layer {layer}");
    }
}