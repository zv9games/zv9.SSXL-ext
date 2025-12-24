use crate::shared_config::PerlinNoiseConfig;
use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;
use noise::{Fbm, Perlin, MultiFractal};

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

/// Checkerboard generator (replaces Perlin for debugging).
pub fn generate_noise_map(
    mut chunk: Chunk,
    _generator: &NoiseGenerator,
) -> Result<Chunk, String> {

    // Real atlas tile IDs (u16!)
    const TILE_BLACK: u16 = 430; // (14, 13)
    const TILE_BLUE:  u16 = 431; // (15, 13)

    let s = chunk.size;
    let total = (s * s) as usize;

    // Ensure tile buffer is sized
    chunk.tiles.resize(total, TileData::default());

    let mut debug_first = true;

    for ly in 0..s {
        for lx in 0..s {

            // Global checkerboard (continues across chunks)
            let world_x = chunk.position.0 * s as i32 + lx as i32;
            let world_y = chunk.position.1 * s as i32 + ly as i32;

            let is_even = ((world_x + world_y) % 2) == 0;

            let tile_data = TileData {
                tile_id: if is_even { TILE_BLACK } else { TILE_BLUE },
                atlas_coords: 0,
                rotation_flags: 0,
                custom_data: 0,
            };

            if debug_first {
                eprintln!(
                    "[Checkerboard] DEBUG first tile: world=({}, {}) local=({}, {}) tile_id={}",
                    world_x, world_y, lx, ly, tile_data.tile_id
                );
                debug_first = false;
            }

            let index = (ly * s + lx) as usize;
            chunk.tiles[index] = tile_data;
        }
    }

    Ok(chunk)
}
