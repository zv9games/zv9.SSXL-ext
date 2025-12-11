// ============================================================================
// 🎼 Generator Trait (`crate::generator::generator`)
// ----------------------------------------------------------------------------
// This module defines the `Generator` trait, the core contract for all
// procedural generation algorithms in the SSXL engine. By enforcing a common
// interface, the engine can treat all generators uniformly, swap them
// dynamically, and maintain modularity and extensibility.
//
// Purpose:
//   • Provide a unified interface for chunk generation algorithms.
//   • Ensure deterministic, self-contained generation logic.
//   • Enable the Conductor to manage multiple generator types seamlessly.
//
// Key Components:
//   • id (method)
//       - Returns a unique, static string identifier for the generator.
//       - Used by the Conductor to distinguish between generator types.
//       - Useful for logging, debugging, and configuration.
//       - Examples: "cellular_automata_basic", "perlin_noise".
//
//   • generate_chunk (method)
//       - Executes the generator’s algorithm to produce a single chunk of world data.
//       - Arguments:
//           • chunk_coords: Vec2i representing the global coordinates of the chunk.
//       - Returns:
//           • ChunkData: the fully generated chunk, ready for caching and use.
//       - Requirements:
//           • Must be deterministic: same coordinates and seed → identical output.
//           • Must be self-contained: should not rely on external mutable state.
//       - Examples:
//           • Cave generator: runs a cellular automata simulation.
//           • Noise generator: samples Perlin/Simplex noise fields.
//
// Workflow:
//   1. Conductor requests chunk generation by calling `generate_chunk`.
//   2. Generator implementation produces deterministic tile data for the chunk.
//   3. Resulting `ChunkData` is cached and returned for use in the world grid.
//   4. Generator ID is logged and tracked for debugging and configuration.
//
// Design Choices:
//   • Trait-based design enforces consistency across all generator implementations.
//   • Separation of ID and generation logic improves clarity and modularity.
//   • Deterministic output ensures reproducibility across runs and seeds.
//
// Educational Note:
//   • This trait demonstrates how Rust traits can define contracts for modular,
//     pluggable systems.
//   • By requiring both identification and generation methods, it ensures that
//     all generators are discoverable, traceable, and interchangeable.
// ============================================================================


use ssxl_shared::ChunkData;
use ssxl_math::prelude::Vec2i;

#[allow(dead_code)]
pub trait Generator {
    fn id(&self) -> &str;
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}
