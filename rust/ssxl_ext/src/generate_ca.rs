use crate::shared_config::CellularAutomataConfig;
use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;

/// Defines the rule set for the Cellular Automata simulation.
/// Uses specific parameters for Cave/Wall generation.
#[derive(Debug, Clone, Copy)]
pub struct CaRules {
    pub death_limit: u8,
    pub birth_limit: u8,
    pub steps: u8,
}

impl From<CellularAutomataConfig> for CaRules {
    fn from(config: CellularAutomataConfig) -> Self {
        CaRules {
            death_limit: config.death_limit,
            birth_limit: config.birth_limit,
            steps: config.steps,
        }
    }
}

/// Public entry point: run CA on a single chunk.
///
/// For now this uses only the chunk's own tiles (no cross‑chunk neighbors),
/// but the internal implementation is factored so we can later plug in
/// neighbor‑aware / halo logic without changing the call‑site API.
pub fn simulate_ca(mut chunk: Chunk, rules: CaRules) -> Result<Chunk, String> {
    let size = chunk.size as usize;

    // Front/back buffers live inside the chunk right now.
    let mut back_buffer = chunk.tiles.clone();

    for _step in 0..rules.steps {
        // Swap buffers: back_buffer becomes the read source (front),
        // and chunk.tiles becomes the write destination (back).
        std::mem::swap(&mut chunk.tiles, &mut back_buffer);
        let front_buffer = &back_buffer;

        // Iterate over every cell in the chunk.
        for y in 0..size {
            for x in 0..size {
                let current_index = y * size + x;

                // 1. Calculate neighbor count (currently chunk‑local only).
                let neighbor_count =
                    count_live_neighbors_chunk_local(front_buffer, size, x as i32, y as i32);

                // 2. Apply CA rule.
                let current_tile = front_buffer[current_index];

                let new_tile = if is_live(current_tile) {
                    // Cell is currently "alive" (e.g., a wall).
                    if neighbor_count < rules.death_limit {
                        make_dead_tile() // Cell dies.
                    } else {
                        current_tile // Cell lives.
                    }
                } else {
                    // Cell is currently "dead" (e.g., open space).
                    if neighbor_count > rules.birth_limit {
                        make_live_tile() // Cell is born.
                    } else {
                        current_tile // Cell remains dead.
                    }
                };

                // 3. Write the new state to the back buffer (chunk.tiles).
                chunk.tiles[current_index] = new_tile;
            }
        }
    }

    Ok(chunk)
}

/// Checks the state (e.g., TileID) to determine if a tile is "live" (a wall/solid).
fn is_live(tile: TileData) -> bool {
    // Assumption: Tile ID > 0 is a solid wall tile.
    tile.tile_id > 0
}

fn make_live_tile() -> TileData {
    // Placeholder for setting a 'wall' tile type.
    TileData {
        tile_id: 1,
        atlas_coords: 0,
        rotation_flags: 0,
        custom_data: 0,
    }
}

fn make_dead_tile() -> TileData {
    // Placeholder for setting an 'air' or 'floor' tile type.
    TileData {
        tile_id: 0,
        atlas_coords: 0,
        rotation_flags: 0,
        custom_data: 0,
    }
}

/// Chunk‑local neighbor count: only sees tiles inside this chunk.
///
/// This preserves your existing behavior, but is split out so we can
/// later replace calls to this with a neighbor‑aware version that can
/// see across chunk boundaries via a callback or halo region.
fn count_live_neighbors_chunk_local(
    tiles: &[TileData],
    size: usize,
    cx: i32,
    cy: i32,
) -> u8 {
    let mut count = 0;

    // Iterate over the 3×3 neighborhood grid.
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the center cell.
            }

            let nx = cx + dx;
            let ny = cy + dy;

            // Bounds check stays chunk‑local for now.
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

pub fn simulate_ca_with_neighbors(
    chunk: Chunk,
    _rules: CaRules,
    _get_neighbor_tile: impl Fn(i32, i32) -> TileData,
) -> Result<Chunk, String> {
    // Temporary stub: just return the chunk unchanged.
    Ok(chunk)
}
