//! # SSXL-ext GDExtension Core Library
//!
//! This file is the **root manifest** for the `ssxl_godot` crate. It defines the module
//! structure, imports necessary Godot framework components, and sets up the required
//! boilerplate for the Rust code to be loaded as a GDExtension dynamic library.

// -----------------------------------------------------------------------------
// Public Modules (Exposed to the GDExtension Interface)
// -----------------------------------------------------------------------------

/// The core class that wraps the GDExtension Node and orchestrates the SSXL engine.
pub mod ssxl_engine;
/// Defines custom signals for communication between the Rust core and Godot scripts.
pub mod ssxl_signals;
/// A potential future module for querying engine state or procedural data (the "Oracle").
pub mod ssxl_oracle;

// -----------------------------------------------------------------------------
// Private Modules (Internal Implementation Details)
// -----------------------------------------------------------------------------

/// Handles non-blocking, asynchronous polling of message channels from worker threads.
mod async_poll;
/// Delegates the final rendering of generated chunk data onto the Godot TileMap.
mod chunk_presenter;
/// Processes messages drained by the `AsyncPoller` and delegates to the `ChunkPresenter`.
mod channel_handler;
/// Contains utility structs and functions for initial configuration of the APIs.
mod api_initializers;
/// The exposed API for starting and configuring map generation from Godot.
mod generation_api;
/// The exposed API for controlling real-time tile animation and simulation.
mod animation_api;

// --- Modules Required by ssxl_engine.rs ---
/// Contains static methods for reporting the internal state (used by SSXLEngine).
mod status_reporter;
/// Defines a trait for API delegation in SSXLEngine.
mod engine_api_extension;

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
/// This acts as the **primary entry point** for the Godot runtime.
///
/// NOTE: The `gde_api_defs.rs` file handles the class registration (like `SSXLEngine`),
/// while this block handles the low-level library initialization hooks.
#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    /// Called by Godot when a new initialization level is reached.
    /// This hook is used for potential setup that needs to occur before/after
    /// the game engine, editor, or scene is initialized.
    fn on_level_init(_level: InitLevel) {
        // Currently, no actions are required at this low-level init stage,
        // as setup is handled in `SSXLEngine::_init` and `_ready`.
    }

    // `on_level_deinit` is implicitly handled or omitted if no cleanup is needed.
}