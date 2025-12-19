// rust/SSXL-ext/src/generate_perlin.rs

use crate::shared_config::PerlinNoiseConfig;
use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;
use noise::{Fbm, Perlin, NoiseFn, MultiFractal};

/// Holds the initialized noise generator and configuration.
pub struct NoiseGenerator {
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

/// Generates the base noise map for a specific chunk.
/// TANK MODE: Seamless across all chunk boundaries.
pub fn generate_noise_map(
    mut chunk: Chunk,
    generator: &NoiseGenerator
) -> Result<Chunk, String> {

    let chunk_x = chunk.position.0; // i32
    let chunk_y = chunk.position.1; // i32
    let s = chunk.size;             // u32

    // ✅ Tank-mode frequency: decoupled from chunk size
    let frequency = 0.01;

    let threshold = generator.config.threshold;

    // ✅ Allocate tile buffer using u32 math (no signed overflow)
    let total = (s * s) as usize;
    chunk.tiles.resize(total, TileData::default());

    // ✅ Iterate using u32, convert to i32 only for world coords
    for ly in 0..s {
        for lx in 0..s {

            // ✅ 1. Compute world coordinates using correct types
            let world_x = chunk_x * s as i32 + lx as i32;
            let world_y = chunk_y * s as i32 + ly as i32;

            // ✅ 2. Sample noise using frequency (NOT scale)
            let noise_value = generator.fbm.get([
                world_x as f64 * frequency,
                world_y as f64 * frequency,
            ]);

            // ✅ 3. Convert noise to tile data
            let tile_data = if noise_value > threshold {
                TileData {
                    tile_id: 1,
                    atlas_coords: 0,
                    rotation_flags: 0,
                    custom_data: (noise_value * 255.0).abs().round() as u8,
                }
            } else {
                TileData::default()
            };

            // ✅ 4. Write tile using u32 indexing (no signed math)
            let index = (ly * s + lx) as usize;
            chunk.tiles[index] = tile_data;
        }
    }

    Ok(chunk)
}
