// ssxl_godot/src/gde_api_defs.rs

//! # Godot Extension API Definitions (GDExtension Interface)
//!
//! This module defines the essential Rust-side bindings that connect the `SSXLEngine`
//! struct to the Godot engine's runtime environment. It implements the necessary
//! traits to register the class and hook into the standard Node lifecycle methods.

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::classes::Node;

// --- Local Crate Imports ---
// Import the core SSXL engine implementation.
use crate::ssxl_engine::SSXLEngine;


// -----------------------------------------------------------------------------
// GDExtension Initialization Macro
// -----------------------------------------------------------------------------

/// The mandatory **GDExtension entry point**.
///
/// This function is called by Godot when the dynamic library is loaded.
/// The `#[gdextension]` attribute marks it as the entry function.
/// It is declared `unsafe` because it interacts with the C/C++ FFI of Godot.
#[gdextension]
unsafe fn ssxl_godot_init(builder: &mut InitHandle) {
    // 1. **Class Registration:** Registers the core `SSXLEngine` struct with Godot.
    // This makes the Rust struct available in the Godot scripting environment (GDScript/C#).
    builder.add_class::<SSXLEngine>();
}


// -----------------------------------------------------------------------------
// Godot Node Lifecycle Implementation
// -----------------------------------------------------------------------------

/// Implements the `ExtensionLibrary` trait for `SSXLEngine`.
/// This trait binds the Rust struct to a Godot class (in this case, `Node`)
/// and maps Godot's virtual methods (like `_init`, `_process`) to the Rust implementation.
impl ExtensionLibrary for SSXLEngine {
    /// **Godot's Constructor (`_init`)**:
    /// This runs when an instance of `SSXLEngine` is created in Godot.
    /// It delegates initialization to the custom `SSXLEngine::init` method.
    fn _init(base: Base<Node>) -> Self {
        // `base` is the underlying Godot Node that this struct wraps.
        SSXLEngine::init(base)
    }

    /// **Godot's Frame Update (`_process(delta)`)**:
    /// This is called every frame, **adapting** the engine's logic to the game's **tempo**.
    /// The `delta` is the time elapsed since the last frame.
    fn _process(&mut self, delta: f64) {
        // Call the internal game loop tick function, passing the delta time.
        // The delta is cast to `u64` (milliseconds or similar unit) as used internally
        // by the SSXL-ext game loop logic (which may need adjustment depending on the unit).
        self.tick(delta as u64);
    }

    /// **Godot's Initialization Hook (`_ready`)**:
    /// This runs once when the node and all its children have entered the scene tree.
    /// This is the ideal place to start the multi-threaded SSXL engine.
    fn _ready(&mut self) {
        self.on_ready();
    }
}