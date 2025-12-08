// ============================================================================
// 🧩 SSXL Cellular Automata Generator (`ssxl_generate::ca_generator`)
// ----------------------------------------------------------------------------
// This module implements a procedural generation algorithm based on Cellular
// Automata (CA). It is responsible for producing chunk data (tile grids) that
// simulate cave-like, maze-like, or static patterns depending on the ruleset.
//
// Purpose:
//   • Provide a flexible generator that can evolve tile states using CA rules.
//   • Support both iterative simulations and static pattern generation.
//   • Ensure deterministic results by seeding a PRNG with chunk coordinates.
//
// Key Components:
//   • Constants
//       - CA_ITERATIONS: number of iterations to stabilize the CA pattern.
//       - INITIAL_FILL_PERCENT: percentage of tiles initially seeded as Rock.
//
//   • CellularAutomataGenerator
//       - Struct holding a numeric ruleset ID.
//       - Provides `new` constructor for initialization.
//       - Implements the `Generator` trait for integration with the engine.
//
//   • generate_static_pattern
//       - Produces non-iterative static patterns (Solid fill, Checkerboard).
//       - Bypasses CA simulation for efficiency.
//       - Builds chunk data directly from ruleset logic.
//
//   • run_ca_simulation
//       - Executes iterative CA simulation using double-buffering.
//       - For each tile, counts live neighbors and applies ruleset logic.
//       - Swaps buffers after each iteration to evolve the state.
//       - Logs progress after each iteration.
//
//   • Generator Trait Implementation
//       - `id`: returns a string identifier for the generator based on ruleset.
//       - `generate_chunk`: orchestrates chunk generation workflow:
//           1. Logs start of generation.
//           2. Handles static rulesets directly.
//           3. Seeds PRNG deterministically from chunk coordinates.
//           4. Builds initial randomized tile state.
//           5. Runs CA simulation to evolve state.
//           6. Returns final chunk data.
//
// Workflow:
//   1. A chunk coordinate is passed to `generate_chunk`.
//   2. If ruleset is static, generate a direct pattern.
//   3. Otherwise, seed PRNG and build randomized initial state.
//   4. Run CA iterations to evolve tile states.
//   5. Return final `ChunkData` for rendering or further processing.
//
// Design Choices:
//   • Deterministic seeding ensures reproducible worlds across runs.
//   • Double-buffering avoids in-place mutation errors during CA iteration.
//   • Static patterns allow efficient bypass for simple rulesets.
//   • Logging (`tracing`) provides visibility into generation steps.
//
// Educational Note:
//   • Cellular Automata are widely used in procedural generation for caves,
//     dungeons, and organic structures.
//   • This implementation demonstrates how Rust can combine deterministic PRNG,
//     efficient memory handling, and modular design to produce complex,
//     reproducible terrain patterns.
// ============================================================================


use crate::Generator;
use ssxl_math::prelude::Vec2i;
use fastrand;
use ssxl_shared::{
    ChunkData,
    CHUNK_SIZE,
    GridBounds,
    TileData,
    TileType,
};
use tracing::{info, warn};

use crate::ca::rule_set::{RULE_SOLID, RULE_CHECKERBOARD, get_next_tile_type};
use crate::ca::neighbor_check::count_live_neighbors;

const CA_ITERATIONS: u8 = 4;
const INITIAL_FILL_PERCENT: u8 = 45;

pub struct CellularAutomataGenerator {
    ruleset: u8,
}

impl CellularAutomataGenerator {
    pub fn new(ruleset: u8) -> Self {
        CellularAutomataGenerator { ruleset }
    }
}

fn generate_static_pattern(chunk_coords: Vec2i, ruleset: u8) -> ChunkData {
    let chunk_tile_size = CHUNK_SIZE as i64;

    let world_start_x = chunk_coords.x * chunk_tile_size;
    let world_start_y = chunk_coords.y * chunk_tile_size;

    let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

    let bounds = GridBounds::new(
        world_start_x,
        world_start_y,
        world_start_x + chunk_tile_size,
        world_start_y + chunk_tile_size,
    );

    let dimension_name = match ruleset {
        RULE_SOLID => "Solid_Fill".to_string(),
        RULE_CHECKERBOARD => "Checkerboard".to_string(),
        _ => "Static_Pattern_Unknown".to_string(),
    };

    let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);

    let mut tiles_vec = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let tile_type = match ruleset {
                RULE_SOLID => TileType::Rock,
                RULE_CHECKERBOARD => {
                    if (x + y) % 2 == 0 { TileType::Rock } else { TileType::Void }
                }
                _ => TileType::Void,
            };
            tiles_vec.push(TileData::new(tile_type, 0.0));
        }
    }

    chunk_data.insert_tiles(tiles_vec);
    info!("CA Generator: Finished static chunk at {:?}.", chunk_coords);
    chunk_data
}

fn run_ca_simulation(mut chunk_data: ChunkData, ruleset: u8) -> ChunkData {
    let mut target_tiles = chunk_data.tiles.clone();

    for i in 0..CA_ITERATIONS {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let index = (y * CHUNK_SIZE + x) as usize;

                let current_tile = &chunk_data.tiles[index];

                let live_neighbors = count_live_neighbors(&chunk_data, x, y);

                let new_type = get_next_tile_type(
                    current_tile.tile_type,
                    live_neighbors,
                    ruleset
                );

                target_tiles[index] = TileData::new(new_type, current_tile.noise_value);
            }
        }

        std::mem::swap(&mut chunk_data.tiles, &mut target_tiles);
        info!("CA Generator: Iteration {} complete.", i + 1);
    }

    chunk_data
}

impl Generator for CellularAutomataGenerator {
    fn id(&self) -> &str {
        match self.ruleset {
            crate::ca::rule_set::RULE_MAZE => "cellular_automata_maze",
            crate::ca::rule_set::RULE_SOLID => "cellular_automata_solid",
            crate::ca::rule_set::RULE_CHECKERBOARD => "cellular_automata_checkerboard",
            crate::ca::rule_set::RULE_BASIC_CAVE | _ => "cellular_automata_basic",
        }
    }

    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        info!("CA Generator: Starting chunk generation at {:?} with ruleset {}.", chunk_coords, self.ruleset);

        if self.ruleset == RULE_SOLID || self.ruleset == RULE_CHECKERBOARD {
            warn!("CA Generator: Using static pattern ruleset ({}). Bypassing CA steps.", self.ruleset);
            return generate_static_pattern(chunk_coords, self.ruleset);
        }

        let seed_x = chunk_coords.x as u64;
        let seed_y = chunk_coords.y as u64;
        let seed = seed_x.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(seed_y);
        fastrand::seed(seed);
        info!("CA Generator: Seeded PRNG with deterministic value: {}.", seed);

        let chunk_tile_size = CHUNK_SIZE as i64;
        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;
        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

        let bounds = GridBounds::new(
            world_start_x,
            world_start_y,
            world_start_x + chunk_tile_size,
            world_start_y + chunk_tile_size,
        );

        let dimension_name = self.id().to_string();

        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);

        let mut tiles_vec = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
            let random_val: u8 = fastrand::u8(0..100);
            let is_rock = random_val < INITIAL_FILL_PERCENT;
            let tile_type = if is_rock { TileType::Rock } else { TileType::Void };
            tiles_vec.push(TileData::new(tile_type, 0.0));
        }
        chunk_data.insert_tiles(tiles_vec);

        let final_chunk_data = run_ca_simulation(chunk_data, self.ruleset);

        warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
        final_chunk_data
    }
}
