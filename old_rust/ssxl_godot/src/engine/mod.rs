// ============================================================================
// 🎼 Engine Module Root (`crate::engine::mod.rs`)
// ----------------------------------------------------------------------------
// This file defines the structure of the `engine` module in the SSXL Godot
// integration. It acts as the central hub, organizing all submodules and
// re-exporting key items so external code can access them easily.
//
// Purpose:
//   • Serve as the "table of contents" for the engine layer.
//   • Organize submodules into clear responsibilities (init, API, tick, commands, etc.).
//   • Re-export important functions and types for ergonomic external use.
//   • Provide a single entry point for accessing the `SSXLEngine` Godot class.
//
// Key Components:
//   • #[macro_use] state
//       - Imports macros defined in the `state` module (e.g., `state!`).
//       - Ensures macros are available throughout the engine without explicit imports.
//   • Submodules
//       - init: Runtime initialization logic (channels, conductor setup).
//       - api: Public-facing API for Godot to interact with the engine.
//       - tick: Game loop integration, processes engine state each frame.
//       - commands: Lightweight runtime commands (stop generation, toggle animation, switch generator).
//       - query: Read-only queries for engine state (tile count, status, active generator).
//       - cleanup: Graceful shutdown and resource release logic.
//       - render_batch: Rendering-related batch operations (integration with Godot visuals).
//       - query_data: Data access layer for chunk/tile queries.
//       - api_initializers: Helper functions for setting up channels and conductor state.
//   • Re-exports
//       - Flatten the API surface so external code can import directly from `engine`
//         without drilling down into submodules.
//       - Example:
//           Without re-export → `use crate::engine::commands::stop_generation_logic;`
//           With re-export    → `use crate::engine::stop_generation_logic;`
//   • SSXLEngine
//       - The Godot-exposed class representing the engine state.
//       - Re-exported here for clean external access, so consumers only need
//         `use crate::engine::SSXLEngine;`.
//
// Design Choices:
//   • Centralized organization improves discoverability and maintainability.
//   • Re-exports flatten hierarchy for ergonomic usage in external crates.
//   • Macro import ensures consistent access to state management utilities.
//   • Modular separation of concerns keeps each file focused and testable.
//
// Educational Note:
//   • This module demonstrates how Rust projects can use `mod.rs` files as
//     organizational hubs. By combining submodule declarations, macro imports,
//     and re-exports, SSXL provides a clean, ergonomic API surface while
//     maintaining internal modularity.
// ============================================================================


#[macro_use] 
pub mod state;

pub mod init;
pub mod api;
pub mod tick;
pub mod commands;
pub mod query;
pub mod cleanup;
pub mod render_batch;
pub mod query_data;
pub mod api_initializers; 

pub use state::*;
pub use init::*;
pub use api::*;
pub use tick::*;
pub use commands::*;
pub use query::*;
pub use cleanup::*;

pub use state::SSXLEngine;
