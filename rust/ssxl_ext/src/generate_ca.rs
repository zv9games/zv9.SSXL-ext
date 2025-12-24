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
/// CA is now disabled — the chunk is returned unchanged.
/// This prevents CA from wiping out Perlin or pattern‑generated tiles.
pub fn simulate_ca(chunk: Chunk, _rules: CaRules) -> Result<Chunk, String> {
    Ok(chunk)
}

/// Checks the state (e.g., TileID) to determine if a tile is "live" (a wall/solid).
fn is_live(tile: TileData) -> bool {
    tile.tile_id > 0
}

fn make_live_tile() -> TileData {
    TileData {
        tile_id: 1,
        atlas_coords: 0,
        rotation_flags: 0,
        custom_data: 0,
    }
}

fn make_dead_tile() -> TileData {
    TileData {
        tile_id: 0,
        atlas_coords: 0,
        rotation_flags: 0,
        custom_data: 0,
    }
}

/// Chunk‑local neighbor count (unused now, but kept for future CA re‑enable).
fn count_live_neighbors_chunk_local(
    tiles: &[TileData],
    size: usize,
    cx: i32,
    cy: i32,
) -> u8 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = cx + dx;
            let ny = cy + dy;

            if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                let idx = (ny as usize) * size + (nx as usize);
                if is_live(tiles[idx]) {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Neighbor‑aware CA stub (unchanged).
pub fn simulate_ca_with_neighbors(
    chunk: Chunk,
    _rules: CaRules,
    _get_neighbor_tile: impl Fn(i32, i32) -> TileData,
) -> Result<Chunk, String> {
    Ok(chunk)
}
