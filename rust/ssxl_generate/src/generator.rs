// ssxl_generate/src/generator.rs
//! Defines the core Generator trait, the contract for all procedural generation algorithms.
//!
//! Any module (e.g., CellularAutomataGenerator, PerlinGenerator) intended to be managed
//! by the GeneratorManager and executed by the Conductor must implement this trait.

use ssxl_shared::chunk_data::ChunkData;

use ssxl_math::Vec2i;

/// The fundamental trait for all world generation algorithms in the SSXL Engine.
///
/// This contract ensures that all generators can be treated uniformly by the
/// Conductor runtime, maintaining the engine's modularity and high **tempo**.
#[allow(dead_code)] // The trait itself isn't used directly, but its implementors are.
pub trait Generator {
    /// Returns a unique, static string identifier for this specific generator instance.
    ///
    /// This ID is used by the Conductor to select the active generation strategy
    /// and should be unique across all registered generators.
    fn id(&self) -> &str;

    /// Executes the generation algorithm for a single chunk at the specified coordinates.
    ///
    /// The implementation must be **deterministic** and **self-contained**, relying only on
    /// the chunk coordinates and internal seed/ruleset to produce the `ChunkData`.
    ///
    /// # Arguments
    /// * `chunk_coords`: The global coordinate (X, Y) of the chunk to generate.
    ///
    /// # Returns
    /// The fully generated `ChunkData` structure.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}