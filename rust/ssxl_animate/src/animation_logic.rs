// ssxl_animate/src/animation_logic.rs (Final Optimized)

use ssxl_shared::{
    AnimationType,
    AnimationUpdate, 
    ChunkData,
    ChunkId,
    TileCoord,
    // FIX: Removed unused import `TileData`
    // FIX: Removed unused import `AnimationPayload` since it's used with fully qualified path (AnimationPayload::*)
    // FIX: Removed unused import `TileData`
};
// FIX: Removed unused import `ssxl_math::Vec2i`

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

fn calculate_tile_coords_for_chunk(chunk_data: &ChunkData, chunk_id: ChunkId) -> Vec<TileCoord> {
    chunk_data.tiles.iter().enumerate().map(|(index, _)| {
        TileCoord {
            x: (chunk_id.x * 16) + (index % 16) as i64,
            y: (chunk_id.y * 16) + (index / 16) as i64,
        }
    }).collect()
}

// FIX: Removed unused function `calculate_tween_value`
/*
fn calculate_tween_value(_coord: TileCoord, intensity: f32) -> f32 {
    1.0 * intensity
}
*/

fn get_tiles_for_chunk(chunk_id: ChunkId) -> Vec<TileCoord> {
    let cache_lock = TileCache::get_instance();
    let cache = cache_lock.read();

    if let Some(chunk_data) = cache.get(&chunk_id) {
        calculate_tile_coords_for_chunk(chunk_data, chunk_id)
    } else {
        Vec::new()
    }
}

pub struct TileCache;

impl TileCache {
    pub fn get_instance() -> Arc<RwLock<HashMap<ChunkId, ChunkData>>> {
        let mut map = HashMap::new();
        
        let mock_chunk_data = ChunkData { 
            id: 0,
            bounds: Default::default(),
            // The TileData type is still implicitly needed here for the array definition,
            // but the import was removed because it was only referenced as a type inside
            // an array literal, not as a standalone item/function/macro.
            tiles: [Default::default(); 1024], 
            dimension_tag: "Default".into(),
            generated_at: std::time::SystemTime::now(),
        };
        map.insert(ChunkId { x: 0, y: 0 }, mock_chunk_data);
        Arc::new(RwLock::new(map))
    }
}

pub fn execute_for_chunk(chunk_id: ChunkId, anim_type: AnimationType) -> Vec<AnimationUpdate> {
    let tiles_in_chunk: Vec<TileCoord> = get_tiles_for_chunk(chunk_id);
    let mut updates = Vec::with_capacity(tiles_in_chunk.len());

    for coord in tiles_in_chunk {
        let (source_id_u32, _new_atlas_coords_i32) = match anim_type {
            AnimationType::TileFlip => {
                let new_frame = (coord.x.wrapping_add(coord.y) % 4) as u32;
                (new_frame, 0)
            },
            AnimationType::PulseFade(_intensity) => {
                (0, 0)
            }
            // `continue` jumps to the next iteration of the `for` loop, skipping the update push.
            AnimationType::TweenMove | AnimationType::CustomScripted(_) => continue, 
        };
        
        // Use fully qualified path for AnimationPayload to avoid re-importing it.
        let payload = ssxl_shared::AnimationPayload::FrameUpdate {
            new_frame: source_id_u32,
        };

        updates.push(AnimationUpdate { 
            coord,
            payload,
        });
    }
    updates
}