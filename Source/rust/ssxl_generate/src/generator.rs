// ssxl_generate/src/generator.rs

use ssxl_shared::chunk_data::ChunkData;
// FIX: Imported Vec2i from aetherion_math, where it is now defined and exported.

use ssxl_math::Vec2i;

/// Defines the core contract for all procedural generation algorithms.
/// Every generator (Perlin, CA, DiamondSquare, etc.) must implement this trait.
#[allow(dead_code)]
pub trait Generator {
    /// The unique identifier for this specific algorithm (e.g., "perlin_2d_v1").
    fn id(&self) -> &str;

    /// Generates the content for a single Chunk.
    /// It takes a Vec2i which is the world-space coordinate of the chunk.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}