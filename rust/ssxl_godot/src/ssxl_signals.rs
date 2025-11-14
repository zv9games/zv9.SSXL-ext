// ssxl_godot/src/ssxl_signals.rs

//! # SSXLSignals (Communications Hub)
//!
//! Defines a simple, stateless Godot class (`SSXLSignals`) used exclusively
//! to house and expose engine signals to Godot's scripting environment (GDScript/C#).

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Base;

// -----------------------------------------------------------------------------
// SSXLSignals Struct Definition
// -----------------------------------------------------------------------------

/// SSXLSignals: A simple, stateless Node class for defining and emitting custom engine signals.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLSignals {
    /// Base field required for the GDExtension class implementation.
    #[base]
    base: Base<Node>,
}

// -----------------------------------------------------------------------------
// Godot API (Signal Definitions)
// -----------------------------------------------------------------------------

#[godot_api]
impl SSXLSignals {
    /// Constructor logic.
    pub fn init(base: Base<Node>) -> Self {
        SSXLSignals { base }
    }

    // --- Generation Lifecycle Signals ---

    /// Emitted when the engine receives a `build_map` command and starts processing.
    #[signal]
    fn build_map_start();

    /// Emitted when a single **chunk** has finished generation on a worker thread.
    /// Args: x (i32), y (i32)
    #[signal]
    fn chunk_generated(x: i32, y: i32);

    /// Emitted when the entire map generation task has finished.
    #[signal]
    fn build_map_complete();

    /// Emitted when a critical, non-recoverable error occurs during generation.
    /// Args: error_message (GString)
    #[signal]
    fn generation_error(error_message: godot::prelude::GString);

    // --- Animation & Utility Signals ---

    /// CRITICAL: Emitted when a single tile's state flips (e.g., collapsed).
    /// Used to queue animation frames without blocking the engine's generation speed.
    /// Args: tile_id (i32), flip_frame (i32)
    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);

    /// Emitted for high-level animation or loading updates.
    /// Args: percent_done (f32), new_atlas_coords (Vector2i)
    #[signal]
    fn animation_update(percent_done: f32, new_atlas_coords: godot::builtin::Vector2i);

    /// **FIXED:** Emitted periodically to update the Godot client on the engine's internal status (e.g., worker queue size).
    /// Args: status_message (GString)
    #[signal]
    fn engine_status_updated(status_message: godot::prelude::GString);
}