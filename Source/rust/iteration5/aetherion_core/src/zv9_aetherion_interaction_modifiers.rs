#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use crate::pipeline::data::SerializableVector2i;
use crate::pipeline::data::TileInfo;

#[cfg(feature = "rand_id")]
use rand::Rng; // Guarded import for rand_id feature.

/// 🧠 Trait for applying mutations to a tile chunk.
pub trait TileModifier {
    fn apply(&self, chunk: &mut MapDataChunk);
}

/// 🎨 Paints a single tile at a given position.
pub struct PaintTile {
    pub pos: SerializableVector2i,
    pub tile: TileInfo,
}

impl TileModifier for PaintTile {
    fn apply(&self, chunk: &mut MapDataChunk) {
        chunk.insert(self.pos, self.tile.clone());
    }
}

/// 🔁 Toggles a tile on/off (e.g. structure presence).
pub struct ToggleTile {
    pub pos: SerializableVector2i,
}

impl TileModifier for ToggleTile {
    fn apply(&self, chunk: &mut MapDataChunk) {
        if chunk.tiles.contains_key(&self.pos) {
            chunk.tiles.remove(&self.pos);
        } else {
            let tile = TileInfo {
                source_id: 0,
                atlas_coords: SerializableVector2i { x: 0, y: 0 },
                alternate_id: 0,
                rotation: 0,
                layer: 0,
                flags: 0,
                variant_id: None,
                frame_count: None,
                animation_speed: None,
            };
            chunk.insert(self.pos, tile);
        }
    }
}

/// 🌫️ Applies randomized noise to the chunk for testing or visual variation.
#[cfg(feature = "rand_id")]
pub fn apply_debug_noise(chunk: &mut MapDataChunk) {
    let mut rng = rand::thread_rng();
    for (_pos, tile) in chunk.tiles.iter_mut() {
        if rng.gen_bool(0.01) {
            tile.source_id = 999; // Arbitrary marker for noise
        }
    }
}