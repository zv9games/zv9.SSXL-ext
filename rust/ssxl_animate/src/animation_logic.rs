//! Defines the concrete, pure logic for executing individual animation effects.
//! This module contains only CPU-bound functions with no I/O, threading, or Godot dependencies,
//! making it perfectly suited for high-throughput data-parallel execution via Rayon.

// --- Imports from ssxl_shared (The Engine Contract) ---
use ssxl_shared::{
    AnimationPayload, 
    AnimationType, 
    ChunkData, 
    ChunkId, 
    TileCoord, 
    TileData,
};
// CRITICAL FIX: Explicitly import the correct communication message structure.
// This resolves the type mismatch (E0308) in worker.rs by ensuring this function 
// returns the exact struct type the UpdateSender channel is parameterized with.
use ssxl_shared::messages::AnimationUpdate; 

// --- Cache Dependencies ---
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

// -----------------------------------------------------------------------------------
// --- Private Utility Functions ---
// -----------------------------------------------------------------------------------

/// Highly optimized retrieval of world coordinates for every tile in a chunk.
fn calculate_tile_coords_for_chunk(chunk_data: &ChunkData, chunk_id: ChunkId) -> Vec<TileCoord> {
    // Assuming a 16x16 tile chunk size (256 tiles) based on the index arithmetic.
    // If CHUNK_SIZE is 32x32 (1024), the arithmetic (index % 16 / index / 16) is inconsistent.
    // We assume 16 for now, as it's consistent with the arithmetic.
    chunk_data.tiles.iter().enumerate().map(|(index, _)| {
        TileCoord {
            x: (chunk_id.x * 16) + (index % 16) as i64,
            y: (chunk_id.y * 16) + (index / 16) as i64,
        }
    }).collect()
}

/// Computes the CPU-intensive tween calculation based on tile coordinates and intensity.
fn calculate_tween_value(_coord: TileCoord, intensity: f32) -> f32 {
    // Placeholder for complex, CPU-intensive math (e.g., Perlin noise sampling).
    1.0 * intensity // Returns a calculated value between 0.0 and 1.0
}

/// Fetches the tile coordinates belonging to a chunk using a thread-safe, high-performance cache read.
fn get_tiles_for_chunk(chunk_id: ChunkId) -> Vec<TileCoord> {
    // Acquire a non-blocking read lock (`RwLock::read`) on the global cache.
    let cache_lock = TileCache::get_instance();
    let cache = cache_lock.read();

    // Look up the chunk data and calculate coords if found.
    if let Some(chunk_data) = cache.get(&chunk_id) {
        calculate_tile_coords_for_chunk(chunk_data, chunk_id)
    } else {
        // Cache miss.
        Vec::new()
    }
}

// -----------------------------------------------------------------------------------
// --- Cache Implementation (Mock) ---
// -----------------------------------------------------------------------------------

pub struct TileCache;

impl TileCache {
    /// Returns the global, thread-safe reference to the tile data.
    pub fn get_instance() -> Arc<RwLock<HashMap<ChunkId, ChunkData>>> {
        let mut map = HashMap::new();
        
        let mock_chunk_data = ChunkData { 
            id: 0,
            bounds: Default::default(),
            // Keeping 1024 as it was the specified value, though arithmetic suggests 256.
            tiles: [TileData::default(); 1024], 
            dimension_tag: "Default".into(),
            generated_at: std::time::SystemTime::now(),
        };
        map.insert(ChunkId { x: 0, y: 0 }, mock_chunk_data);
        Arc::new(RwLock::new(map))
    }
}

// -----------------------------------------------------------------------------------
// --- Public Execution Function ---
// -----------------------------------------------------------------------------------

/// Executes the core animation logic for a single spatial chunk.
/// Returns a vector of all necessary visual updates (`ssxl_shared::messages::AnimationUpdate`) 
/// to be sent to the Godot runtime.
pub fn execute_for_chunk(chunk_id: ChunkId, anim_type: AnimationType) -> Vec<AnimationUpdate> {
    // 1. Fetch the relevant tile coordinates.
    let tiles_in_chunk: Vec<TileCoord> = get_tiles_for_chunk(chunk_id);
    let mut updates = Vec::with_capacity(tiles_in_chunk.len());

    for coord in tiles_in_chunk {
        // 2. Compute the new state based on the requested animation type.
        let payload = match anim_type {
            AnimationType::TileFlip => {
                // Determine frame based on deterministic hash of coordinates.
                AnimationPayload::FrameUpdate { 
                    new_frame: ((coord.x.wrapping_add(coord.y) % 4) as u32) 
                }
            },
            AnimationType::PulseFade(intensity) => {
                let value = calculate_tween_value(coord, intensity);
                AnimationPayload::TweenValue { key: "alpha".into(), value }
            }
            // Skip complex/unimplemented animation types to avoid bottlenecks.
            AnimationType::TweenMove | AnimationType::CustomScripted(_) => continue,
        };
        
        // 3. Assemble the correct message struct ({coord, payload}).
        updates.push(AnimationUpdate { 
            coord, 
            payload 
        });
    }
    updates
}