// ssxl_generate/src/perlin_generator.rs

use crate::Generator;
// FIX 1: Import ChunkData and CHUNK_SIZE directly from chunk_data.
use ssxl_shared::chunk_data::{ChunkData, CHUNK_SIZE};
// FIX 2: Import GridBounds directly from its sub-module (aetherion_shared/src/grid_bounds.rs).
use ssxl_shared::grid_bounds::GridBounds; 
use ssxl_math::Vec2i;
use ssxl_shared::tile_data::TileData;
use ssxl_shared::tile_type::TileType;

// --- External Dependency ---
// FIX 3 (Warning): Removed unused 'Seedable' import.
use noise::{NoiseFn, Perlin}; 
use tracing::info;

/// Implements the Minimal Viable Generator (MVG) using Perlin Noise.
pub struct PerlinGenerator {
    /// Noise function instance for generation.
    perlin: Perlin,
    /// The scale factor to control the 'zoom' of the noise.
    scale: f64,
}

impl PerlinGenerator {
    /// Creates a new PerlinGenerator with a fixed seed and configurable scale.
    pub fn new(scale: f64) -> Self {
        const DEFAULT_SEED: u32 = 42; // A simple, fixed seed for consistency
        PerlinGenerator { 
            // FIX: Supply the required u32 seed argument to Perlin::new()
            perlin: Perlin::new(DEFAULT_SEED), // <--- CHANGE THIS LINE
            scale,
        }
    }
}

impl Generator for PerlinGenerator {
    fn id(&self) -> &str {
        "perlin_basic_2d"
    }

    /// Generates the content for a single chunk using 2D Perlin noise.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let chunk_tile_size = CHUNK_SIZE as i32; 

        // 1. Calculate the World-Space Start Position of the chunk
        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;

        // ChunkData::new arguments: id, bounds, dimension_name
        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);
        
        // FIX 4: GridBounds::new requires min_x, min_y, max_x, max_y (all i64)
        let bounds = GridBounds::new(
            world_start_x as i64, 
            world_start_y as i64, 
            (world_start_x + chunk_tile_size) as i64, 
            (world_start_y + chunk_tile_size) as i64
        ); 
        
        let dimension_name = "2D_Noise".to_string(); 
        
        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);
        
        let mut tiles: Vec<TileData> = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        // 2. Loop through every tile in the chunk
        for y in 0..chunk_tile_size {
            for x in 0..chunk_tile_size {
                let world_x = (world_start_x + x) as f64;
                let world_y = (world_start_y + y) as f64;

                // 3. Use the noise function to get a height value
                let noise_value = self.perlin.get([world_x / self.scale, world_y / self.scale]);

                // Map the noise range [-1.0, 1.0] to a positive range [0.0, 1.0]
                let normalized_value = (noise_value + 1.0) / 2.0;

                // 4. Map the height value to a TileType and populate the ChunkData
                let tile_type = if normalized_value < 0.35 {
                    TileType::Water
                } else if normalized_value < 0.60 {
                    TileType::Grass
                } else {
                    TileType::Mountain
                };
                
                // TileData::new requires 2 arguments: tile_type, noise_value: f32
                let tile = TileData::new(tile_type, normalized_value as f32);
                tiles.push(tile);
            }
        }
        
        // set_tiles renamed to insert_tiles
        chunk_data.insert_tiles(tiles);

        info!(
            "MVG generated Chunk {:?} using Perlin ({} tiles processed)", 
            chunk_coords, 
            (CHUNK_SIZE * CHUNK_SIZE) 
        );
        
        chunk_data // Return the generated and populated chunk data
    }
}