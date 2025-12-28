use crate::shared_config::PerlinNoiseConfig;
use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;
use noise::{Fbm, Perlin, MultiFractal};

/// Holds the initialized noise generator and configuration.
#[allow(dead_code)]
pub struct NoiseGenerator {
    fbm: Fbm<Perlin>,
    config: PerlinNoiseConfig,
}

impl NoiseGenerator {
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

    // Valid atlas indices for a 16x16 atlas (256 tiles)
    const TILE_BLACK: u16 = 222;
    const TILE_BLUE:  u16 = 223;

    let s = chunk.size;
    let total = (s * s) as usize;

    // Ensure tile buffer is sized
    chunk.tiles.resize(total, TileData::default());

    for ly in 0..s {
        for lx in 0..s {
            let world_x = chunk.position.0 * s as i32 + lx as i32;
            let world_y = chunk.position.1 * s as i32 + ly as i32;

            let is_even = ((world_x + world_y) % 2) == 0;
            let atlas_index = if is_even { TILE_BLACK } else { TILE_BLUE };

            // IMPORTANT:
            // - tile_id must be non-zero so MeshBuilder doesn't skip it
            // - atlas_coords is what the renderer actually uses for UVs
            let tile_data = TileData {
                tile_id: 1,
                atlas_coords: atlas_index,
                rotation_flags: 0,
                custom_data: 0,
            };

            let index = (ly * s + lx) as usize;
            chunk.tiles[index] = tile_data;
        }
    }

    Ok(chunk)
}
