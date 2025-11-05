// ssxl_godot/src/gde_api_defs.rs

use godot::prelude::*;
use godot::classes::Node; // Explicitly import Node for the Base<Node> usage
use crate::ssxl_engine::SSXLEngine;

// --- Godot Initialization and Registration ---

/// Registers all Godot classes exposed by the SSXL-ext GDExtension library.
/// This is the entry point for Godot to discover our Rust-defined classes.
#[gdextension]
unsafe fn ssxl_godot_init(builder: &mut InitHandle) {
    builder.add_class::<SSXLEngine>();
}

// --- Trait Implementations for Godot Lifecycle ---

/// Required trait implementation to expose Rust methods to GDScript.
impl ExtensionLibrary for SSXLEngine {
    /// Initialization logic is delegated to the main SSXLEngine constructor
    /// to keep this file clean.
    fn _init(base: Base<Node>) -> Self {
        SSXLEngine::init(base)
    }

    /// Godot's callback for the main game loop update.
    /// This delegates to the internal `tick()` for MPSC polling.
    fn _process(&mut self, delta: f64) {
        // Pass the delta time, cast to u64. This is a placeholder for a true tick count
        // and is ignored by the current SSXLEngine::tick logic.
        self.tick(delta as u64); 
    }
    
    /// Godot's callback when the node enters the scene tree.
    fn _ready(&mut self) {
        // Initialization of presenters, poller, and channel configuration occurs here.
        self.on_ready(); 
    }
}