// rust/SSXL-ext/src/generate_ca.rs

use crate::shared_config::CellularAutomataConfig; // Loaded from TOML/Godot

/// Defines the rule set for the Cellular Automata simulation.
/// Uses specific parameters for Cave/Wall generation.
#[derive(Debug, Clone, Copy)]
pub struct CA_Rules {
    pub death_limit: u8,   // Cells with fewer neighbors than this die.
    pub birth_limit: u8,   // Cells with more neighbors than this are born.
    pub steps: u8,         // Number of simulation iterations to run.
}

impl From<CellularAutomataConfig> for CA_Rules {
    fn from(config: CellularAutomataConfig) -> Self {
        CA_Rules {
            death_limit: config.death_limit,
            birth_limit: config.birth_limit,
            steps: config.steps,
        }
    }
}

// rust/SSXL-ext/src/generate_ca.rs

use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData; // Used for initial state/final write

/// Runs the Cellular Automata simulation on a given chunk.
/// This is the CPU-intensive operation executed by the ThreadPool.
pub fn simulate_ca(mut chunk: Chunk, rules: CA_Rules) -> Result<Chunk, String> {
    // We need a separate buffer (back_buffer) for the next iteration's state
    // to ensure all cells are calculated based on the *previous* state (front_buffer).
    let mut back_buffer = chunk.tiles.clone();
    let size = chunk.size as usize; // e.g., 16 for a 16x16 chunk

    for step in 0..rules.steps {
        // Swap buffers: back_buffer becomes the read source (front), 
        // and chunk.tiles (the original buffer) becomes the write destination (back).
        std::mem::swap(&mut chunk.tiles, &mut back_buffer);
        let front_buffer = &back_buffer;

        // Iterate over every cell in the chunk
        for y in 0..size {
            for x in 0..size {
                let current_index = y * size + x;
                
                // 1. Calculate Neighbor Count
                let neighbor_count = count_live_neighbors(front_buffer, size, x as i32, y as i32);
                
                // 2. Apply CA Rule
                let current_tile = front_buffer[current_index];
                
                // Determine the new state based on the current state and neighbor count
                let new_tile = if is_live(current_tile) {
                    // Cell is currently "alive" (e.g., a wall)
                    if neighbor_count < rules.death_limit {
                        make_dead_tile() // Cell dies
                    } else {
                        current_tile    // Cell lives
                    }
                } else {
                    // Cell is currently "dead" (e.g., open space)
                    if neighbor_count > rules.birth_limit {
                        make_live_tile() // Cell is born
                    } else {
                        current_tile     // Cell remains dead
                    }
                };

                // Write the new state to the back buffer (chunk.tiles)
                chunk.tiles[current_index] = new_tile;
            }
        }
    }

    Ok(chunk) // The chunk with the final, iterated CA state
}

// rust/SSXL-ext/src/generate_ca.rs

/// Checks the state (e.g., TileID) to determine if a tile is "live" (a wall/solid).
fn is_live(tile: TileData) -> bool {
    // Assumption: Tile ID > 0 is a solid wall tile
    tile.tile_id > 0 
}

fn make_live_tile() -> TileData {
    // Placeholder for setting a 'wall' tile type
    TileData { tile_id: 1, atlas_coords: 0, rotation_flags: 0, custom_data: 0 } 
}

fn make_dead_tile() -> TileData {
    // Placeholder for setting an 'air' or 'floor' tile type
    TileData { tile_id: 0, atlas_coords: 0, rotation_flags: 0, custom_data: 0 }
}


/// Calculates the number of live neighbors in the 3x3 area around (cx, cy).
/// Uses simple bounds checking (no wrapping/toroidal logic for simplicity).
fn count_live_neighbors(tiles: &[TileData], size: usize, cx: i32, cy: i32) -> u8 {
    let mut count = 0;
    
    // Iterate over the 3x3 neighborhood grid
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue; // Skip the center cell
            }
            
            let nx = cx + x;
            let ny = cy + y;
            
            // Bounds check
            if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                let neighbor_index = (ny as usize) * size + (nx as usize);
                if is_live(tiles[neighbor_index]) {
                    count += 1;
                }
            }
        }
    }
    count
}