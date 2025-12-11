// rust/SSXL-ext/src/generate_perlin.rs

use crate::shared_config::PerlinNoiseConfig;
use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;
use noise::{Fbm, Perlin, NoiseFn, Seedable, MultiFractal};

/// Holds the initialized noise generator and configuration.
pub struct NoiseGenerator {
    // Fbm (Fractal Brownian Motion) is typically used for complex terrain.
    fbm: Fbm<Perlin>,
    config: PerlinNoiseConfig,
}

impl NoiseGenerator {
    /// Creates a generator instance from configuration.
    pub fn new(config: PerlinNoiseConfig, seed: u64) -> Self {
        let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(seed as u32)
            .set_octaves(config.octaves)
            .set_lacunarity(config.lacunarity)
            .set_persistence(config.persistence);

        NoiseGenerator { fbm, config }
    }
}

// rust/SSXL-ext/src/generate_perlin.rs

/// Generates the base noise map for a specific chunk.
/// This is one of the initial steps inside the ThreadPool's worker execution.
pub fn generate_noise_map(
    mut chunk: Chunk, 
    generator: &NoiseGenerator
) -> Result<Chunk, String> {
    
    let chunk_x = chunk.position.0;
    let chunk_y = chunk.position.1;
    let size = chunk.size as i32;
    let scale = generator.config.scale;
    let threshold = generator.config.threshold;

    // Use a dense Vec for speed, matching the TileData layout.
    chunk.tiles.resize( (size * size) as usize, TileData::default() );

    for local_y in 0..size {
        for local_x in 0..size {
            // 1. Calculate the World Coordinates (Crucial for continuity)
            // This translates local chunk coordinates into continuous world coordinates.
            let world_x = (chunk_x * size) + local_x;
            let world_y = (chunk_y * size) + local_y;
            
            // 2. Sample the Noise Function
            let noise_value = generator.fbm.get([
                world_x as f64 / scale, 
                world_y as f64 / scale
            ]);

            // 3. Map Value to Tile Data
            let tile_data = if noise_value > threshold {
                // Above threshold: Solid / Wall (the target for CA refinement)
                TileData {
                    tile_id: 1, // Wall tile ID
                    atlas_coords: 0,
                    rotation_flags: 0,
                    custom_data: (noise_value * 255.0).abs().round() as u8, // Store raw density
                }
            } else {
                // Below threshold: Air / Floor
                TileData::default() 
            };
            
            // 4. Write to Chunk Buffer
            let index = (local_y * size + local_x) as usize;
            chunk.tiles[index] = tile_data;
        }
    }

    Ok(chunk)
}