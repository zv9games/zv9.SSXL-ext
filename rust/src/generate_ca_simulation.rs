// rust/SSXL-ext/src/generate_ca_simulation.rs

use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;
use crate::shared_config::CellularAutomataConfig;
use crate::cache::NoiseCache; // Required for boundary lookups
use crate::shared_error::SSXLCoreError;
// --- FIX: Correct module path from 'math' to 'shared_math' ---
use crate::shared_math::{ChunkCoords, CA_NEIGHBOR_COUNT};
// --- FIX: Import logging macro from the crate root ---
use crate::ssxl_info;

/// Runs the Cellular Automata simulation for the specified number of steps on the given chunk.
pub fn run_simulation_steps(
    mut chunk: Chunk,
    config: CellularAutomataConfig,
    // The Noise Cache is passed in so the CA can query neighbor chunks' initial states.
    noise_cache: &NoiseCache, 
    // The coordinates of the current chunk being processed.
    chunk_coords: ChunkCoords,
) -> Result<Chunk, SSXLCoreError> {
    
    // We need a secondary buffer to store the next state before applying it, 
    // ensuring all calculations use the data from the *start* of the step.
    let mut next_state_buffer = chunk.tiles.clone();
    
    ssxl_info!("CA Sim: Starting {} steps for chunk {:?}", config.steps, chunk_coords);

    for _step in 0..config.steps {
        
        for y in 0..chunk.size {
            for x in 0..chunk.size {
                
                // 1. Count Neighbors
                let live_neighbors = count_live_neighbors(
                    x as i32, 
                    y as i32, 
                    &chunk, 
                    chunk_coords, 
                    noise_cache
                );

                // 2. Apply Rules
                let current_tile = chunk.get_tile(x, y).unwrap();
                let next_state_tile = calculate_next_state(
                    current_tile, 
                    live_neighbors, 
                    &config
                );

                // 3. Stage Result in Buffer
                let index = chunk.get_index(x, y);
                next_state_buffer[index] = next_state_tile;
            }
        }
        
        // After iterating all tiles, swap the buffer to the current state for the next step.
        chunk.tiles.copy_from_slice(&next_state_buffer);
    }
    
    Ok(chunk)
}

// rust/SSXL-ext/src/generate_ca_simulation.rs

/// Counts the number of 'live' neighbors for a cell, handling cross-chunk boundary lookups.
fn count_live_neighbors(
    local_x: i32, 
    local_y: i32, 
    current_chunk: &Chunk,
    chunk_coords: ChunkCoords,
    noise_cache: &NoiseCache,
) -> u8 {
    let mut live_count = 0;
    let chunk_size = current_chunk.size as i32;
    
    // Iterates through a 3x3 grid centered on (local_x, local_y)
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; } // Skip the center cell
            
            let neighbor_x = local_x + dx;
            let neighbor_y = local_y + dy;
            
            // --- Determine if the neighbor is INSIDE or OUTSIDE the current chunk ---
            if neighbor_x >= 0 && neighbor_x < chunk_size && neighbor_y >= 0 && neighbor_y < chunk_size {
                // INTERNAL NEIGHBOR (Fast lookup)
                if current_chunk.get_tile(neighbor_x as u32, neighbor_y as u32)
                    // The t in the closure is &TileData. We assume TileData implements Copy
                    // and is_live() takes self (by value) to force the dereference.
                    .map_or(false, |t| (*t).is_live()) 
                {
                    live_count += 1;
                }
            } else {
                // EXTERNAL NEIGHBOR (Requires a cache lookup, slower)
                // 1. Calculate the adjacent chunk coordinates
                let adjacent_chunk_coords = (
                    chunk_coords.0 + if neighbor_x < 0 { -1 } else if neighbor_x >= chunk_size { 1 } else { 0 },
                    chunk_coords.1 + if neighbor_y < 0 { -1 } else if neighbor_y >= chunk_size { 1 } else { 0 },
                );
                
                // 2. Calculate the local coordinates within the adjacent chunk
                let local_adj_x = neighbor_x.rem_euclid(chunk_size);
                let local_adj_y = neighbor_y.rem_euclid(chunk_size);

                // 3. Query the NoiseCache for the adjacent chunk's initial state
                if noise_cache.get_noise_data(adjacent_chunk_coords)
                    // FIX: Changed inner `map_or` to `map`. 
                    // and_then requires the closure to return Option<T>. 
                    // map returns Option<bool>, which satisfies and_then.
                    .and_then(|data| {
                        // Look up the tile's state in the cached noise data
                        let index = (local_adj_y * chunk_size + local_adj_x) as usize;
                        // Use map to convert Option<&NoiseValue> to Option<bool>
                        data.get(index).map(|&noise_val| noise_val > 0.5) 
                    })
                    .unwrap_or(false) 
                {
                    live_count += 1;
                }
            }
        }
    }
    live_count
}

// rust/SSXL-ext/src/generate_ca_simulation.rs

/// Applies the birth and death limits to determine the next state of a cell.
fn calculate_next_state(
    current_tile: &TileData,
    live_neighbors: u8,
    config: &CellularAutomataConfig,
) -> TileData {
    
    // Start with a copy of the current state
    let mut next_state = *current_tile;

    if current_tile.is_live() {
        // --- Live Cell (Survival/Death) ---
        // If live neighbors are LESS than the Death Limit, the cell dies (becomes 'dead').
        if live_neighbors < config.death_limit {
            next_state.set_live(false); // Method call will be found after implementing TileData::set_live
        }
    } else {
        // --- Dead Cell (Birth) ---
        // If live neighbors are GREATER than or equal to the Birth Limit, the cell is born (becomes 'live').
        if live_neighbors >= config.birth_limit {
            next_state.set_live(true); // Method call will be found after implementing TileData::set_live
        }
    }
    
    next_state
}