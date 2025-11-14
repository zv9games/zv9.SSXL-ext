//ssxl_generate/src/cellular_automata_generator.rs
//! Implements a procedural generator based on Cellular Automata (CA) rules.
//!
//! This generator is responsible for creating cave systems, mazes, and other
//! structured patterns by iterating on an initially random chunk state. It
//! delegates complex logic to the `ca::rule_set` and `ca::neighbor_check` modules.

use crate::Generator;
use ssxl_math::Vec2i;
use fastrand; // Lightweight, fast, and thread-safe PRNG
use ssxl_shared::{
    chunk_data::{ChunkData, CHUNK_SIZE},
    grid_bounds::GridBounds,
    tile_data::TileData,
    tile_type::TileType,
};
use tracing::{info, warn};

use crate::ca::rule_set::{RULE_SOLID, RULE_CHECKERBOARD, get_next_tile_type};
use crate::ca::neighbor_check::count_live_neighbors;

// --- 1. Generator Constants ---

/// The fixed number of iterations for the CA simulation to stabilize the pattern.
const CA_ITERATIONS: u8 = 4;
/// The percentage of tiles randomly initialized as `TileType::Rock` (the "live" state).
const INITIAL_FILL_PERCENT: u8 = 45;

// --- 2. Generator Structure and Implementation ---

/// A generator that uses Cellular Automata rules to produce structured patterns.
#[allow(dead_code)] // Allowed since this struct is instantiated via the GeneratorManager
pub struct CellularAutomataGenerator {
    /// The specific B/S ruleset (e.g., RULE_BASIC_CAVE or RULE_MAZE) to apply.
    ruleset: u8,
}

impl CellularAutomataGenerator {
    /// Creates a new CA generator instance with the specified ruleset.
    pub fn new(ruleset: u8) -> Self {
        CellularAutomataGenerator { ruleset }
    }
}

// --- 3. Internal Generation Helper Functions ---

/// Generates static, non-simulated patterns (Solid or Checkerboard).
///
/// This bypasses the iterative CA steps entirely for simple, fixed designs.
fn generate_static_pattern(chunk_coords: Vec2i, ruleset: u8) -> ChunkData {
    let chunk_tile_size = CHUNK_SIZE as i64;
    
    // Calculate world boundaries for the ChunkData metadata
    let world_start_x = chunk_coords.x * chunk_tile_size;
    let world_start_y = chunk_coords.y * chunk_tile_size;
    
    // Create a unique 64-bit Chunk ID
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
    let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let tile_type = match ruleset {
                RULE_SOLID => TileType::Rock,
                RULE_CHECKERBOARD => {
                    // Checkerboard pattern: alternate based on coordinate parity
                    if (x + y) % 2 == 0 {
                        TileType::Rock
                    } else {
                        TileType::Void
                    }
                }
                _ => TileType::Void, // Should not be reached, but defaults to Void
            };
            tiles.push(TileData::new(tile_type, 0.0));
        }
    }

    chunk_data.insert_tiles(tiles);
    info!("CA Generator: Finished static chunk at {:?}.", chunk_coords);
    chunk_data
}

/// Applies one iteration (step) of the Cellular Automata simulation.
///
/// This involves creating a copy of the tile array to prevent changes in the current
/// iteration from affecting neighbor counts for subsequent tiles in the same iteration.
fn apply_ca_step(chunk_data: &mut ChunkData, ruleset: u8) {
    // Clone the current state to calculate the *next* state without self-interference.
    let mut new_tiles: Vec<TileData> = chunk_data.tiles.iter().cloned().collect();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let index = (y * CHUNK_SIZE + x) as usize;
            let current_tile = &chunk_data.tiles[index];
            
            // 1. Check Neighbors (Delegated to specialized module)
            let live_neighbors = count_live_neighbors(chunk_data, x as u32, y as u32);

            // 2. Apply Rule Set (Delegated to specialized module)
            let new_type = get_next_tile_type(
                current_tile.tile_type,
                live_neighbors,
                ruleset
            );

            // 3. Update the new tile state, preserving the original noise value (if any).
            new_tiles[index] = TileData::new(new_type, current_tile.noise_value);
        }
    }
    // Swap the updated tile array back into the ChunkData.
    chunk_data.insert_tiles(new_tiles);
}

// --- 4. Trait Implementation (Generator API) ---

impl Generator for CellularAutomataGenerator {
    /// Returns a unique identifier string for this generator instance.
    fn id(&self) -> &str {
        match self.ruleset {
            crate::ca::rule_set::RULE_MAZE => "cellular_automata_maze",
            crate::ca::rule_set::RULE_SOLID => "cellular_automata_solid",
            crate::ca::rule_set::RULE_CHECKERBOARD => "cellular_automata_checkerboard",
            crate::ca::rule_set::RULE_BASIC_CAVE | _ => "cellular_automata_basic",
        }
    }

    /// The main logic to generate a single chunk of data.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        info!("CA Generator: Starting chunk generation at {:?} with ruleset {}.", chunk_coords, self.ruleset);

        // Handle static patterns quickly without the iterative CA loop.
        if self.ruleset == RULE_SOLID || self.ruleset == RULE_CHECKERBOARD {
            warn!("CA Generator: Using static pattern ruleset ({}). Bypassing CA steps.", self.ruleset);
            return generate_static_pattern(chunk_coords, self.ruleset);
        }

        // --- Seeding for Determinism (Crypto Coded Memory) ---
        // Creates a unique, deterministic seed based on the chunk coordinates.
        let seed_x = chunk_coords.x as u64;
        let seed_y = chunk_coords.y as u64;
        // Use a large prime number for mixing (0x9e3779b97f4a7c15 is the golden ratio approximation)
        let seed = seed_x.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(seed_y);
        fastrand::seed(seed);
        info!("CA Generator: Seeded PRNG with deterministic value: {}.", seed);
        
        // --- Initialization ---
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
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        // Initial randomization based on INITIAL_FILL_PERCENT
        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
            let random_val: u8 = fastrand::u8(0..100);
            let is_rock = random_val < INITIAL_FILL_PERCENT;

            let tile_type = if is_rock {
                TileType::Rock
            } else {
                TileType::Void
            };
            tiles.push(TileData::new(tile_type, 0.0));
        }
        chunk_data.insert_tiles(tiles);

        // --- Simulation Iterations ---
        for i in 0..CA_ITERATIONS {
            apply_ca_step(&mut chunk_data, self.ruleset);
            info!("CA Generator: Iteration {} complete.", i + 1);
        }

        warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
        chunk_data
    }
}