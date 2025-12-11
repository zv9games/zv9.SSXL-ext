// ============================================================================
// 🚀 SSXL Godot Crate Root (`lib.rs`)
// ----------------------------------------------------------------------------
// This file serves as the entry point for the `ssxl_godot` crate. It defines
// the top-level module structure and provides the GDExtension boilerplate
// required for Godot to load this Rust library.
//
// Purpose:
//   • Declare the public modules that make up the SSXL engine integration.
//   • Provide the mandatory `ExtensionLibrary` implementation for Godot.
//   • Act as the root manifest tying together engine, FFI, and tilemap logic.
//
// Module Structure:
//   • engine
//       - Core engine logic (SSXLEngine, conductor, tick loop, APIs).
//   • ffi
//       - Godot-facing FFI adapter nodes (SSXLOracle, SSXLSignals).
//       - Bridges Rust logic with Godot signals and scripting.
//   • tilemap
//       - TileMap integration (SSXLTilemap, async_poll, status_reporter).
//       - Handles rendering, async updates, and status reporting.
//
// Godot Integration:
//   • Uses `godot::prelude::*` for macros, traits, and types that enable
//     Rust ↔ Godot interoperability (e.g., #[gdextension], Gd<T>, GodotClass).
//   • Imports `ExtensionLibrary` and `InitLevel` to define the extension’s
//     lifecycle behavior.
//
// SSXLExtension (struct):
//   • Placeholder struct representing the extension library itself.
//   • Contains no fields because all logic is delegated to modules.
//
// ExtensionLibrary Implementation:
//   • #[gdextension] marks this as the entrypoint for Godot’s GDExtension system.
//   • unsafe impl ExtensionLibrary for SSXLExtension:
//       - Required by godot-rust to hook into Godot’s lifecycle.
//       - on_level_init(_level): called when Godot reaches a new initialization
//         stage (Core, Scene, Editor, etc.).
//       - Currently does nothing, but can be extended to register classes,
//         initialize resources, or set up global state.
//
// Educational Note:
//   • This file demonstrates how Rust crates integrate with Godot via
//     GDExtension. By separating engine, FFI, and tilemap into modules,
//     the design remains modular and maintainable, while the extension
//     entrypoint ensures Godot can load and interact with the library.
// ============================================================================

#![feature(int_roundings)]
pub mod engine;
pub mod ffi;
pub mod tilemap; 

use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel};

struct SSXLExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    fn on_level_init(_level: InitLevel) {}
}
