// ssxl_generate/src/perlin/perlin_generator.rs

//! Implements the Generator trait using the Perlin noise function.
//!
//! This provides the engine's primary continuous, organic generation layer,
//! mapping noise values to different TileTypes based on a fixed threshold.

use crate::Generator;
use ssxl_math::prelude::Vec2i;

// FIX: Import all components directly from the ssxl_shared crate root.
use ssxl_shared::{
    ChunkData,
    CHUNK_SIZE,
    GridBounds,
    TileData,
    TileType,
};

use noise::{NoiseFn, Perlin};
use tracing::info;

// --- 1. Generator Structure ---

/// A generator that uses the Perlin noise algorithm to create deterministic terrain.
pub struct PerlinGenerator {
    /// The noise object instance, which is thread-safe and deterministic based on its seed.
    perlin: Perlin,
    /// The scaling factor applied to world coordinates before generating noise.
    /// A smaller scale results in larger, smoother features.
    scale: f64,
}

impl PerlinGenerator {
    /// Creates a new PerlinGenerator instance.
    ///
    /// # Arguments
    /// * `scale`: The frequency/scale of the noise (e.g., 64.0).
    pub fn new(scale: f64) -> Self {
        // NOTE: Default seed is currently hardcoded for deterministic, repeatable generation.
        const DEFAULT_SEED: u32 = 42;
        
        PerlinGenerator {
            perlin: Perlin::new(DEFAULT_SEED),
            scale,
        }
    }
}

// --- 2. Generator Trait Implementation ---

impl Generator for PerlinGenerator {
    /// Returns the unique identifier for this generator.
    fn id(&self) -> &str {
        "perlin_basic_2d"
    }

    /// Generates a single chunk based on the Perlin noise field.
    ///
    /// The logic is: 1) Convert chunk coordinates to world tile coordinates,
    /// 2) Sample the Perlin function, 3) Map the noise value to a `TileType`.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let chunk_tile_size = CHUNK_SIZE as i64;

        // Calculate the world coordinate of the chunk's bottom-left corner.
        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;

        // Create a unique Chunk ID by packing the 2D coordinates into a 64-bit integer.
        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

        // Define the world bounds covered by this chunk.
        let bounds = GridBounds::new(
            world_start_x,
            world_start_y,
            world_start_x + chunk_tile_size,
            world_start_y + chunk_tile_size
        );

        let dimension_name = "2D_Noise".to_string();

        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);

        // Pre-allocate vector to hold all tile data for the chunk.
        let mut tiles: Vec<TileData> = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        // Iterate through all tiles within the chunk.
        for y in 0..chunk_tile_size {
            for x in 0..chunk_tile_size {
                let world_x = (world_start_x + x) as f64;
                let world_y = (world_start_y + y) as f64;

                // Sample the Perlin noise function. Coordinates are scaled down.
                let noise_value = self.perlin.get([world_x / self.scale, world_y / self.scale]);

                // Perlin output is typically [-1.0, 1.0]. Normalize to [0.0, 1.0].
                let normalized_value = (noise_value + 1.0) / 2.0;

                // Thresholding: Map the noise value to a concrete TileType (Water, Grass, Mountain).
                let tile_type = if normalized_value < 0.35 {
                    TileType::Water
                } else if normalized_value < 0.60 {
                    TileType::Grass
                } else {
                    TileType::Mountain
                };

                // Create the TileData, storing the raw noise value as metadata (useful for blending/details).
                let tile = TileData::new(tile_type, normalized_value as f32);
                tiles.push(tile);
            }
        }

        // Insert the generated tile array into the chunk data structure.
        chunk_data.insert_tiles(tiles);

        info!(
            "MVG generated Chunk {:?} using Perlin ({} tiles processed)",
            chunk_coords,
            (CHUNK_SIZE * CHUNK_SIZE)
        );

        chunk_data
    }
}