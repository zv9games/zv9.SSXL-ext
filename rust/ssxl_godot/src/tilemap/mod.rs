// ============================================================================
// 🧩 Tilemap Module (`crate::tilemap`)
// ----------------------------------------------------------------------------
// This module serves as the parent namespace for all tilemap-related components
// in the SSXL engine. It organizes submodules that handle tilemap logic,
// asynchronous channel polling, and status reporting, and re-exports them for
// clean access throughout the engine.
//
// Purpose:
//   • Provide a central entry point for tilemap functionality.
//   • Encapsulate core tilemap logic, async communication, and status utilities
//     in dedicated submodules.
//   • Simplify imports by exposing submodules directly under `crate::tilemap`.
//
// Submodules:
//   • ssxl_tilemap
//       - Contains the main `SSXLTilemap` implementation.
//       - Responsible for managing tile placement, rendering, and integration
//         with Godot’s `TileMap` node.
//       - Provides APIs for interacting with chunks and tiles.
//
//   • async_poll
//       - Defines the `AsyncPoller` struct.
//       - Bridges Tokio async channels with Godot’s synchronous frame loop.
//       - Safely polls generation and animation channels each tick.
//       - Ensures non-blocking, panic-free message handling.
//
//   • status_reporter
//       - Provides utilities for reporting tilemap status back to Godot.
//       - Emits human-readable summaries of engine/tilemap state.
//       - Helps external scripts monitor lifecycle and debug tilemap behavior.
//
// Design Choices:
//   • Modular separation keeps responsibilities clear: rendering, async polling,
//     and status reporting are independent but unified under `tilemap`.
//   • Re-exporting submodules improves ergonomics, allowing external code to
//     import directly from `crate::tilemap` without deep paths.
//   • This structure mirrors Godot’s scene graph philosophy: small, focused
//     components that work together to form a cohesive system.
//
// Educational Note:
//   • This module demonstrates how Rust projects can use submodules and
//     re-exports to create clean, maintainable APIs. By centralizing tilemap
//     functionality under `tilemap`, SSXL ensures that engine code and Godot
//     scripts have a simple, intuitive interface for managing tile-based worlds.
// ============================================================================


pub mod ssxl_tilemap;
pub mod async_poll;
pub mod status_reporter;
