use crate::Generator;
use ssxl_math::Vec2i;
use fastrand;
use ssxl_shared::{
    chunk_data::{ChunkData, CHUNK_SIZE},
    grid_bounds::GridBounds,
    tile_data::TileData,
    tile_type::TileType,
};
use tracing::{info, warn};

// --- MODULE DECLARATIONS ---
// Removed: mod rule_set; and mod neighbor_check;
// These are now declared in ssxl_generate/src/ca.rs

// --- Imports from new modules (using the correct crate::ca path) ---
use crate::ca::rule_set::{RULE_SOLID, RULE_CHECKERBOARD, get_next_tile_type};
use crate::ca::neighbor_check::count_live_neighbors;


// --- CONSTANTS ---
const CA_ITERATIONS: u8 = 4;
const INITIAL_FILL_PERCENT: u8 = 45; // 45% of tiles start as 'Rock'


/// ⚙️ Implements a 2D Cellular Automata (CA) generator for pattern-based terrain.
#[allow(dead_code)]
pub struct CellularAutomataGenerator {
    /// The ID of the CA ruleset to use (e.g., 0 for Cave, 1 for Maze).
    ruleset: u8,
}

impl CellularAutomataGenerator {
    pub fn new(ruleset: u8) -> Self {
        CellularAutomataGenerator { ruleset }
    }
}

// -------------------------------------------------------------------------
// STATIC PATTERN GENERATION
// -------------------------------------------------------------------------

/// Generates a simple, static pattern that does not require CA iterations.
fn generate_static_pattern(chunk_coords: Vec2i, ruleset: u8) -> ChunkData {
    // 1. Chunk Metadata Initialization
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
    
    // Using the imported constants
    let dimension_name = match ruleset {
        RULE_SOLID => "Solid_Fill".to_string(),
        RULE_CHECKERBOARD => "Checkerboard".to_string(),
        _ => "Static_Pattern_Unknown".to_string(),
    };
    
    let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);
    let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

    // 2. Pattern Generation
    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let tile_type = match ruleset {
                RULE_SOLID => TileType::Rock,
                RULE_CHECKERBOARD => {
                    // Tile type alternates based on (x + y) parity
                    if (x + y) % 2 == 0 {
                        TileType::Rock
                    } else {
                        TileType::Void
                    }
                }
                _ => TileType::Void, // Should not happen
            };
            tiles.push(TileData::new(tile_type, 0.0));
        }
    }

    chunk_data.insert_tiles(tiles);
    info!("CA Generator: Finished static chunk at {:?}.", chunk_coords);
    chunk_data
}


/// Applies one step of the Cellular Automata rule to the chunk grid.
fn apply_ca_step(chunk_data: &mut ChunkData, ruleset: u8) {
    let mut new_tiles: Vec<TileData> = chunk_data.tiles.iter().cloned().collect();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let index = (y * CHUNK_SIZE + x) as usize;
            let current_tile = &chunk_data.tiles[index];
            
            // Logic moved to neighbor_check::count_live_neighbors
            let live_neighbors = count_live_neighbors(chunk_data, x as u32, y as u32);

            // Logic moved to rule_set::get_next_tile_type
            let new_type = get_next_tile_type(
                current_tile.tile_type,
                live_neighbors,
                ruleset
            );

            // Retain existing noise value
            new_tiles[index] = TileData::new(new_type, current_tile.noise_value);
        }
    }
    chunk_data.insert_tiles(new_tiles);
}


// --- TRAIT IMPLEMENTATION ---

impl Generator for CellularAutomataGenerator {
    fn id(&self) -> &str {
        // Updated to use the correct crate::ca::rule_set path
        match self.ruleset {
            crate::ca::rule_set::RULE_MAZE => "cellular_automata_maze",
            crate::ca::rule_set::RULE_SOLID => "cellular_automata_solid",
            crate::ca::rule_set::RULE_CHECKERBOARD => "cellular_automata_checkerboard",
            crate::ca::rule_set::RULE_BASIC_CAVE | _ => "cellular_automata_basic",
        }
    }

    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        info!("CA Generator: Starting chunk generation at {:?} with ruleset {}.", chunk_coords, self.ruleset);

        // 1. Check for static patterns and execute non-CA logic
        if self.ruleset == RULE_SOLID || self.ruleset == RULE_CHECKERBOARD {
            warn!("CA Generator: Using static pattern ruleset ({}). Bypassing CA steps.", self.ruleset);
            return generate_static_pattern(chunk_coords, self.ruleset);
        }

        // 2. DETERMINISTIC PRNG SEEDING
        // Use chunk coordinates to create a predictable seed for fastrand's thread-local generator.
        let seed_x = chunk_coords.x as u64;
        let seed_y = chunk_coords.y as u64;
        // Simple scrambling of the coordinates for distribution.
        let seed = seed_x.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(seed_y);
        fastrand::seed(seed);
        info!("CA Generator: Seeded PRNG with deterministic value: {}.", seed);
        
        // 3. CHUNK METADATA INITIALIZATION (for CA algorithms)
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

        let dimension_name = self.id().to_string(); // Use the ID as the dimension name

        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);

        // 4. INITIAL RANDOM FILL (Seed)
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
            // Generate a random number from 0 to 99.
            let random_val: u8 = fastrand::u8(0..100);
            // The tile is rock if the random number is less than INITIAL_FILL_PERCENT.
            let is_rock = random_val < INITIAL_FILL_PERCENT;

            let tile_type = if is_rock {
                TileType::Rock
            } else {
                TileType::Void
            };
            tiles.push(TileData::new(tile_type, 0.0));
        }
        chunk_data.insert_tiles(tiles);

        // 5. APPLY CA ITERATIONS
        for i in 0..CA_ITERATIONS {
            apply_ca_step(&mut chunk_data, self.ruleset);
            info!("CA Generator: Iteration {} complete.", i + 1);
        }

        warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
        chunk_data
    }
}