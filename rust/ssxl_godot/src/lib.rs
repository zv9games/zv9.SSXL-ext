// ssxl_godot/src/lib.rs (Refactored Zero-Entropy Manifest)

//! # SSXL-ext GDExtension Core Library
//!
//! This file is the **root manifest** for the `ssxl_godot` crate. It has been
//! refactored to use a hierarchical structure, exposing only the top-level,
//! logically segregated modules: `engine`, `ffi`, and `tilemap`.

// -----------------------------------------------------------------------------
// Public Modules (Exposed to the GDExtension Interface)
// -----------------------------------------------------------------------------

/// Contains the SSXLEngine struct and its core logic (API, commands, oracle, tick).
pub mod engine;
/// Contains all Godot-exposed FFI Adapter Nodes (SSXLOracle, SSXLSignals).
pub mod ffi;

// FIX: Changed 'pub mod ssxl_tilemap;' to 'pub mod tilemap;'
// This tells the compiler to look for the 'tilemap' directory/module (src/tilemap/mod.rs or src/tilemap.rs).
// The 'tilemap/mod.rs' file will then declare 'pub mod ssxl_tilemap;'.
/// The module for the SSXLTileMap Godot Node and its related logic.
pub mod tilemap; 

// NOTE: All previous flat modules (e.g., ssxl_engine, ssxl_signals, async_poll,
// generation_api, channel_handler) have been consolidated into the `engine` and
// `ffi` hierarchies for a zero-entropy structure.

// -----------------------------------------------------------------------------
// GDExtension Boilerplate
// -----------------------------------------------------------------------------

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel};


/// Placeholder struct required by the `godot-rust` library to implement
/// the `ExtensionLibrary` trait, which defines the dynamic library's behavior.
struct SSXLExtension;

/// Implements the required trait for the GDExtension to be loaded.
#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    /// Called by Godot when a new initialization level is reached.
    fn on_level_init(_level: InitLevel) {
        // No actions required at this low-level init stage.
    }
}