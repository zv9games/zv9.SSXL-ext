use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};
use godot::builtin::{Vector2i, PackedInt32Array, Dictionary, Array};
use godot::meta::error::ConvertError;
use godot::obj::Base;
use tracing::error;

#[derive(GodotClass)]
#[class(base=TileMap)]
pub struct SSXLTileMap {
    base: Base<TileMap>,
}

#[godot_api]
impl ITileMap for SSXLTileMap {
    fn init(base: Base<TileMap>) -> Self {
        SSXLTileMap { base }
    }
}

#[godot_api]
impl SSXLTileMap {
    #[func]
    pub fn batch_set_tiles_v4(&mut self, batch_data: Dictionary) {
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

        let mut base_tilemap = self.base_mut();
        
        let args = [
            layer.to_variant(),
            positions.to_variant(),
            source_ids.to_variant(),
            atlas_coords.to_variant(),
            alt_tiles.to_variant(),
        ];
        
        let _ = base_tilemap.call("set_cells_coord_array", &args);
    }
}