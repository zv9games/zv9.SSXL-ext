use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};
use godot::builtin::{Vector2i, PackedInt32Array, Dictionary, Array};
use godot::meta::error::ConvertError;
use godot::obj::Base;
use tracing::{error, info};

#[derive(GodotClass)]
#[class(base=TileMap)]
pub struct SSXLTileMap {
    base: Base<TileMap>,
}

#[godot_api]
impl ITileMap for SSXLTileMap {
    fn init(base: Base<TileMap>) -> Self {
        info!("SSXLTileMap Initialized: Ready to process SSXL batches.");
        SSXLTileMap { base }
    }
}

#[godot_api]
impl SSXLTileMap {
    /// The high-performance, single-call batch function to set multiple tiles.
    #[func]
    pub fn batch_set_tiles_v4(&mut self, batch_data: Dictionary) {
        // --- 1. Unpack & Validate Data (Your logic is gold—keeping it!) ---
        let layer: i32 = batch_data.get("layer")
            .unwrap_or_default()
            .try_to()
            .unwrap_or(0);

        let Ok(positions) = batch_data.get("positions")
            .map(|v| v.try_to::<Array<Vector2i>>())
            .unwrap_or(Err(ConvertError::default()))
        else { error!("Batch Data Error: Missing or invalid 'positions' array."); return; };

        let Ok(source_ids) = batch_data.get("source_ids")
            .map(|v| v.try_to::<PackedInt32Array>())
            .unwrap_or(Err(ConvertError::default()))
        else { error!("Batch Data Error: Missing or invalid 'source_ids' array."); return; };

        let Ok(atlas_coords) = batch_data.get("atlas_coords")
            .map(|v| v.try_to::<Array<Vector2i>>())
            .unwrap_or(Err(ConvertError::default()))
        else { error!("Batch Data Error: Missing or invalid 'atlas_coords' array."); return; };

        let Ok(alt_tiles) = batch_data.get("alt_tiles")
            .map(|v| v.try_to::<PackedInt32Array>())
            .unwrap_or(Err(ConvertError::default()))
        else { error!("Batch Data Error: Missing or invalid 'alt_tiles' array."); return; };

        let tile_count = positions.len();
        if tile_count == 0 {
            return;
        }

        // --- 2. Ultra-High-Speed Update Loop (Now with set_cell_ex magic!) ---
        let mut base_tilemap = self.base_mut();

        for i in 0..tile_count {
            let i_usize = i as usize;
            let pos: Vector2i = positions.get(i_usize).unwrap();
            let source_id: i32 = source_ids.get(i_usize).unwrap();
            let atlas_coords_val: Vector2i = atlas_coords.get(i_usize).unwrap();
            let alt_tile: i32 = alt_tiles.get(i_usize).unwrap();

            // Boom! Chain it up for that single, efficient FFI dispatch.
            // This is the "ex" way—optimized, no extra traits needed.
            let _ = base_tilemap
                .set_cell_ex(layer, pos)
                .source_id(source_id)
                .atlas_coords(atlas_coords_val)
                .alternative_tile(alt_tile);
        }

        info!("Batch Completed: Processed {} tiles in Layer {}—set_cell_ex FTW!", tile_count, layer);
    }
}