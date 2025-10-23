use godot::prelude::*;
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
//use aetherion_core::log_component;

/// 🌱 AetherionGenerator — Godot-facing node for procedural tile creation.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionGenerator;

#[godot_api]
impl AetherionGenerator {
    

    /// Generates a tile using noise at the given coordinates and seed.
    #[func]
    fn generate_noise(&self, x: f32, y: f32, seed: i64) -> Dictionary {
        let tile = generate_noise_tile(x, y, seed);
        godot_print!("🧪 Noise tile generated at ({}, {}) with seed {}", x, y, seed);
        Self::tile_to_dict(tile)
    }

    /// Generates a tile using a named pattern.
    #[func]
    fn generate_pattern(&self, pattern_name: String, x: i32, y: i32) -> Dictionary {
        let tile = generate_pattern_tile(&pattern_name, x, y);
        godot_print!("🎨 Pattern '{}' tile generated at ({}, {})", pattern_name, x, y);
        Self::tile_to_dict(tile)
    }

    /// Converts a TileInfo into a Godot Dictionary.
    fn tile_to_dict(tile: TileInfo) -> Dictionary {
        let mut dict = Dictionary::new();
        let _ = dict.insert("source_id", tile.source_id);
        let _ = dict.insert("atlas_coords", Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y));
        let _ = dict.insert("alternate_id", tile.alternate_id);
        let _ = dict.insert("rotation", tile.rotation);
        let _ = dict.insert("layer", tile.layer);
        dict
    }
}

pub fn generate_noise_tile(x: f32, y: f32, seed: i64) -> TileInfo {
    let hash = ((x * 73856093.0) as i64 ^ (y * 19349663.0) as i64 ^ seed) & 0xFFFF;
    let alt = (hash % 4) as i32;

    TileInfo {
        source_id: 0,
        atlas_coords: SerializableVector2i { x: x as i32, y: y as i32 },
        alternate_id: alt,
        rotation: 0,
        layer: 0,
        flags: 0,
        variant_id: None,
        frame_count: None,
        animation_speed: None,
    }
}

pub fn generate_pattern_tile(pattern_name: &str, x: i32, y: i32) -> TileInfo {
    let alt = match pattern_name {
        "floor" => 1,
        "wall" => 2,
        "path" => 3,
        _ => 0,
    };

    TileInfo {
        source_id: 1,
        atlas_coords: SerializableVector2i { x, y },
        alternate_id: alt,
        rotation: 0,
        layer: 1,
        flags: 0,
        variant_id: None,
        frame_count: None,
        animation_speed: None,
    }
}


