// ============================================================================
// 🎼 Manager Module Declaration (`crate::manager`)
// ----------------------------------------------------------------------------
// This `mod.rs` file defines the structure and public API surface of the
// Manager system. It organizes submodules related to configuration validation,
// generator logic, generator registry, and runtime management. By re-exporting
// key items, it simplifies usage for external code.
//
// Purpose:
//   • Serve as the "table of contents" for all management-related functionality.
//   • Expose submodules that implement configuration checks, generator contracts,
//     generator registry, and runtime orchestration.
//   • Re-export commonly used items to provide a clean, ergonomic public API.
//
// Submodules:
//   • config_validator
//       - Provides validation logic for generator configurations.
//       - Ensures map dimensions and chunk limits are safe before generation.
//   • generator
//       - Defines the `Generator` trait, the contract for all generation algorithms.
//       - Enforces modularity and extensibility across procedural generators.
//   • generator_manager
//       - Acts as a registry for all available generators (Perlin, Cellular Automata, etc.).
//       - Provides lookup, fallback, and chunk generation execution.
//   • runtime_manager
//       - Manages the async runtime environment for executing generation tasks.
//       - Handles graceful shutdown and lifecycle control.
//
// Re-exports:
//   • config_validator::*
//       - Exposes `ConfigValidator` and related validation utilities.
//   • generator::*
//       - Exposes the `Generator` trait and related items.
//   • generator_manager::*
//       - Exposes `GeneratorManager` and type aliases for dynamic generators.
//   • runtime_manager::*
//       - Exposes `RuntimeManager` and runtime utilities.
//
// Workflow:
//   1. External code imports from `crate::manager`.
//   2. Submodules provide specialized functionality (validation, generator logic, registry, runtime).
//   3. Re-exports simplify usage by exposing key items directly.
//   4. Manager orchestrates configuration, generator selection, and runtime execution.
//
// Educational Note:
//   • This file demonstrates how Rust’s module system can be used to organize
//     complex subsystems into clear, modular components.
//   • By re-exporting key items, the public API remains clean and ergonomic,
//     while internal organization stays modular and maintainable.
// ============================================================================


pub mod config_validator;
pub mod generator;
pub mod generator_manager;
pub mod runtime_manager;

pub use config_validator::*;
pub use generator::*;
pub use generator_manager::*;
pub use runtime_manager::*;
