use godot::prelude::*;
#[allow(unused_imports)]
use aetherion_core::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};

/// 🧩 Extension trait for converting MapDataChunk to Godot Dictionary.
pub trait MapDataChunkExt {
    fn to_dictionary(&self) -> Dictionary;
}

impl MapDataChunkExt for MapDataChunk {
    fn to_dictionary(&self) -> Dictionary {
        let mut dict = Dictionary::new();


        for (pos, tile) in &self.tiles {
            let key = Vector2i::new(pos.x, pos.y).to_variant();

            let mut tile_dict = Dictionary::new();
            let _ = tile_dict.insert("source_id", tile.source_id);
            let _ = tile_dict.insert("atlas_coords", Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y));
            let _ = tile_dict.insert("alternate_id", tile.alternate_id);
            let _ = tile_dict.insert("rotation", tile.rotation);
            let _ = tile_dict.insert("layer", tile.layer);
            let _ = tile_dict.insert("flags", tile.flags);

            if let Some(variant_id) = tile.variant_id {
                let _ = tile_dict.insert("variant_id", variant_id);
            }
            if let Some(frame_count) = tile.frame_count {
                let _ = tile_dict.insert("frame_count", frame_count);
            }
            if let Some(animation_speed) = tile.animation_speed {
                let _ = tile_dict.insert("animation_speed", animation_speed);
            }

            let _ = dict.insert(key, tile_dict);
        }

        dict
    }
}
