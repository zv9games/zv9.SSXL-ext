// ============================================================================
// 🎼 Perlin Noise Generator (`crate::perlin::perlin_generator`)
// ----------------------------------------------------------------------------
// This module implements a procedural terrain generator based on Perlin noise.
// It conforms to the `Generator` trait, making it interchangeable with other
// generation algorithms in the SSXL engine.
//
// Purpose:
//   • Provide deterministic terrain generation using Perlin noise fields.
//   • Translate continuous noise values into discrete tile types (Water, Grass, Mountain).
//   • Ensure reproducibility by seeding the noise function.
//   • Integrate seamlessly with the Conductor and GeneratorManager systems.
//
// Key Components:
//   • PerlinGenerator (struct)
//       - Fields:
//           • perlin: Perlin noise object seeded for deterministic output.
//           • scale: scaling factor applied to coordinates before sampling noise.
//       - Larger scale → smoother, larger terrain features.
//       - Smaller scale → finer detail and variation.
//
//   • new (constructor)
//       - Creates a new PerlinGenerator with a given scale.
//       - Uses a hardcoded DEFAULT_SEED for reproducibility.
//       - Ensures consistent terrain generation across runs.
//
//   • id (trait method)
//       - Returns a unique identifier string ("perlin_basic_2d").
//       - Used by Conductor and GeneratorManager to select this generator.
//
//   • generate_chunk (trait method)
//       - Generates a single chunk of terrain using Perlin noise.
//       - Steps:
//           1. Compute chunk size in tiles.
//           2. Calculate world coordinates for chunk origin.
//           3. Create unique chunk ID by packing coordinates into u64.
//           4. Define spatial bounds for the chunk.
//           5. Initialize ChunkData container.
//           6. Pre-allocate vector for TileData entries.
//           7. Iterate over each tile in the chunk:
//                a. Compute world coordinates.
//                b. Sample Perlin noise at scaled coordinates.
//                c. Normalize noise value from [-1, 1] → [0, 1].
//                d. Threshold into TileType (Water, Grass, Mountain).
//                e. Create TileData with type + raw noise metadata.
//                f. Push into tile vector.
//           8. Insert generated tiles into ChunkData.
//           9. Log completion message with chunk coordinates and tile count.
//          10. Return fully populated ChunkData.
//
// Workflow:
//   1. Conductor requests chunk generation via GeneratorManager.
//   2. PerlinGenerator samples noise field at scaled coordinates.
//   3. Noise values are normalized and mapped to discrete tile types.
//   4. ChunkData is populated with TileData entries.
//   5. Completed chunk is cached and returned for use in the world grid.
//
// Design Choices:
//   • Deterministic seed ensures reproducibility across runs.
//   • Scaling factor allows tuning of terrain smoothness vs. detail.
//   • Thresholds map continuous noise into meaningful terrain categories.
//   • Logging provides visibility into generation process.
//
// Educational Note:
//   • This module demonstrates how continuous mathematical noise functions
//     can be transformed into discrete, game-ready terrain data.
//   • By combining Perlin noise with clear thresholds, it produces varied yet
//     deterministic landscapes suitable for procedural world generation.
// ============================================================================


use crate::Generator;
use ssxl_math::prelude::Vec2i;
use ssxl_shared::{
    ChunkData,
    CHUNK_SIZE,
    GridBounds,
    TileData,
    TileType,
};
use noise::{NoiseFn, Perlin};
use tracing::info;

pub struct PerlinGenerator {
    perlin: Perlin,
    scale: f64,
}

impl PerlinGenerator {
    pub fn new(scale: f64) -> Self {
        const DEFAULT_SEED: u32 = 42;
        
        PerlinGenerator {
            perlin: Perlin::new(DEFAULT_SEED),
            scale,
        }
    }
}

impl Generator for PerlinGenerator {
    fn id(&self) -> &str {
        "perlin_basic_2d"
    }

    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let chunk_tile_size = CHUNK_SIZE as i64;

        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;

        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

        let bounds = GridBounds::new(
            world_start_x,
            world_start_y,
            world_start_x + chunk_tile_size,
            world_start_y + chunk_tile_size
        );

        let dimension_name = "2D_Noise".to_string();

        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);

        let mut tiles: Vec<TileData> = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);

        for y in 0..chunk_tile_size {
            for x in 0..chunk_tile_size {
                let world_x = (world_start_x + x) as f64;
                let world_y = (world_start_y + y) as f64;

                let noise_value = self.perlin.get([world_x / self.scale, world_y / self.scale]);

                let normalized_value = (noise_value + 1.0) / 2.0;

                let tile_type = if normalized_value < 0.35 {
                    TileType::Water
                } else if normalized_value < 0.60 {
                    TileType::Grass
                } else {
                    TileType::Mountain
                };

                let tile = TileData::new(tile_type, normalized_value as f32);

                tiles.push(tile);
            }
        }

        chunk_data.insert_tiles(tiles);

        info!(
            "MVG generated Chunk {:?} using Perlin ({} tiles processed)",
            chunk_coords,
            (CHUNK_SIZE * CHUNK_SIZE)
        );

        chunk_data
    }
}
