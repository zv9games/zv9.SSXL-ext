// ssxl_generate/src/cellular_automata_generator.rs

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
/// (Static pattern generation is unchanged and remains efficient)
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
    // Since chunk_data.tiles is a fixed-size array, we must fill a temporary Vec
    // and let ChunkData::insert_tiles handle the conversion/copy.
    let mut tiles_vec = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

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
            tiles_vec.push(TileData::new(tile_type, 0.0));
        }
    }

    chunk_data.insert_tiles(tiles_vec);
    info!("CA Generator: Finished static chunk at {:?}.", chunk_coords);
    chunk_data
}

/// **OPTIMIZED:** Runs the full Cellular Automata simulation using double-buffering.
///
/// This function allocates the `target_tiles` array only once by cloning the initial state.
/// **FIXED:** Corrected array handling using `[TileData; N]` for zero-cost `std::mem::swap` in the loop.
fn run_ca_simulation(mut chunk_data: ChunkData, ruleset: u8) -> ChunkData {
    // 1. The initial state is in `chunk_data.tiles` (Source buffer). 
    // Create the second buffer (Target) by cloning the array once.
    let mut target_tiles = chunk_data.tiles.clone();
    
    // `chunk_data.tiles` is the Source (Read), `target_tiles` is the Target (Write).

    for i in 0..CA_ITERATIONS {
        // Core loop: Read from `chunk_data.tiles` (Source), write to `target_tiles` (Target).
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let index = (y * CHUNK_SIZE + x) as usize;
                
                // Read current tile properties from the source array
                let current_tile = &chunk_data.tiles[index];
                
                // 1. Check Neighbors: reads from the current state within `&chunk_data`.
                let live_neighbors = count_live_neighbors(&chunk_data, x, y);

                // 2. Apply Rule Set
                let new_type = get_next_tile_type(
                    current_tile.tile_type,
                    live_neighbors,
                    ruleset
                );

                // 3. Update the new tile state in the TARGET buffer.
                // We preserve the noise value from the previous step.
                target_tiles[index] = TileData::new(new_type, current_tile.noise_value);
            }
        }
        
        // EFFICIENT SWAP: Exchange the contents of the two arrays. O(1) pointer swap.
        // `chunk_data.tiles` now holds the new state (Source for next iteration).
        // `target_tiles` now holds the stale state (Target for next iteration).
        std::mem::swap(&mut chunk_data.tiles, &mut target_tiles);
        
        info!("CA Generator: Iteration {} complete.", i + 1);
    }
    
    // The final result is in `chunk_data.tiles`.
    chunk_data
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
        let seed_x = chunk_coords.x as u64;
        let seed_y = chunk_coords.y as u64;
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
        // Use a temporary Vec to build the initial randomized state
        let mut tiles_vec = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        // Initial randomization based on INITIAL_FILL_PERCENT
        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
            let random_val: u8 = fastrand::u8(0..100);
            let is_rock = random_val < INITIAL_FILL_PERCENT;

            let tile_type = if is_rock {
                TileType::Rock
            } else {
                TileType::Void
            };
            // Noise value is typically unused in base CA but kept for data integrity
            tiles_vec.push(TileData::new(tile_type, 0.0));
        }
        // Insert the initial state, which copies the data into the internal fixed-size array.
        chunk_data.insert_tiles(tiles_vec); 

        // --- Simulation Iterations (Refactored to single, optimized call) ---
        let final_chunk_data = run_ca_simulation(chunk_data, self.ruleset);

        warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
        final_chunk_data
    }
}