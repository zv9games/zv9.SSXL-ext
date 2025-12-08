// ============================================================================
// 🎼 Perlin Module Declaration (`crate::perlin`)
// ----------------------------------------------------------------------------
// This `mod.rs` file defines the structure and public API surface of the Perlin
// noise subsystem in the SSXL engine. It organizes submodules related to Perlin
// noise generation and re-exports their contents for easier access.
//
// Purpose:
//   • Serve as the "entry point" for all Perlin-related functionality.
//   • Expose the `perlin_generator` submodule, which implements the Perlin noise algorithm.
//   • Re-export items so external code can import directly from `crate::perlin`
//     without drilling down into internal paths.
//
// Submodules:
//   • perlin_generator
//       - Contains the actual implementation of the Perlin noise generator.
//       - Conforms to the `Generator` trait, ensuring compatibility with the
//         Conductor and GeneratorManager systems.
//       - Provides deterministic chunk generation based on Perlin noise fields.
//
// Re-exports:
//   • pub use perlin_generator::*
//       - Exposes all items from the `perlin_generator` submodule.
//       - Simplifies external imports by flattening the module hierarchy.
//       - Example:
//           • Without re-export → `use crate::perlin::perlin_generator::PerlinGenerator;`
//           • With re-export    → `use crate::perlin::PerlinGenerator;`
//
// Workflow:
//   1. External code imports from `crate::perlin`.
//   2. The `perlin_generator` submodule provides Perlin noise generation logic.
//   3. Re-export ensures ergonomic access to `PerlinGenerator` and related items.
//   4. GeneratorManager registers Perlin as one of the available algorithms.
//
// Educational Note:
//   • This file demonstrates how Rust’s module system can be used to organize
//     specialized subsystems (like Perlin noise) while keeping the public API clean.
//   • By re-exporting, developers gain ergonomic access without sacrificing
//     modularity or maintainability.
// ============================================================================


pub mod perlin_generator;

pub use perlin_generator::*;
