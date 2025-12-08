// ============================================================================
// 🧩 Cellular Automata Module (`crate::ca`)
// ----------------------------------------------------------------------------
// This module serves as the entry point for the Cellular Automata (CA) system
// within the SSXL engine. It organizes and exposes submodules that implement
// different aspects of CA-based procedural generation.
//
// Purpose:
//   • Provide a modular, extensible framework for CA-driven terrain and structure generation.
//   • Separate concerns into distinct submodules for clarity and maintainability.
//   • Expose a clean public API surface so other parts of the engine can access
//     CA functionality via `crate::ca::<submodule>`.
//
// Submodules:
//   • cellular_automata_generator
//       - Contains the `CellularAutomataGenerator` struct.
//       - Implements the `Generator` trait for CA-based chunk generation.
//       - Responsible for seeding chunks, running CA iterations, and producing
//         final `ChunkData`.
//
//   • rule_set
//       - Defines specific CA rulesets (e.g., Solid fill, Checkerboard, Basic Cave, Maze).
//       - Encapsulates logic for determining the next tile type based on current
//         state and neighbor counts.
//       - Enables experimentation and swapping of rule sets without changing
//         generator logic.
//
//   • neighbor_check
//       - Provides low-level functions for inspecting tile neighbors.
//       - Example: `count_live_neighbors` counts adjacent Rock tiles.
//       - Supplies local state information to the `rule_set` logic.
//       - Keeps neighborhood definitions modular (e.g., Moore vs. Von Neumann).
//
// Workflow:
//   1. The generator seeds a chunk with initial tile states.
//   2. Neighbor checks provide local context for each tile.
//   3. Rule sets apply CA logic to evolve tile states.
//   4. The generator produces final chunk data for rendering or simulation.
//
// Design Choices:
//   • Modular submodules improve readability and allow independent evolution.
//   • Clear separation of generator, rules, and neighbor logic supports reuse
//     and experimentation.
//   • Public module declarations (`pub mod`) ensure external access to CA
//     components while maintaining internal organization.
//
// Educational Note:
//   • Cellular Automata are a powerful tool for procedural generation,
//     producing organic, cave-like, or maze-like structures.
//   • This module demonstrates how Rust’s module system can be leveraged to
//     build a clean, extensible architecture for complex algorithms.
// ============================================================================


pub mod cellular_automata_generator;
pub mod rule_set;
pub mod neighbor_check;
