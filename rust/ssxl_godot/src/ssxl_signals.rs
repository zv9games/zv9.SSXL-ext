// ssxl_godot/src/ssxl_signals.rs
// This GDExtension class serves as a dedicated signal bus for the SSXL-ext engine,
// allowing the Rust core to communicate asynchronous events back to Godot's GDScript layer.

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Base;

// -------------------------------------------------------------------------------------------------
// SSXL SIGNALS GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// A dedicated Node for holding and emitting signals related to the SSXL-ext engine's operations.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl SSXLSignals {
    pub fn init(base: Base<Node>) -> Self {
        SSXLSignals {
            base,
        }
    }

    // ---------------------------------------------------------------------
    // 📢 Signal Declarations
    // ---------------------------------------------------------------------

    /// Emitted when the map generation process starts.
    #[signal]
    fn build_map_start();

    /// Emitted when a new chunk of data has been generated and applied to the TileMap.
    #[signal]
    fn chunk_generated(x: i32, y: i32);

    /// Emitted when the entire map generation process is complete.
    #[signal]
    fn build_map_complete();

    /// Emitted when the AnimationConductor sends a tile update for progressive visual loading.
    #[signal]
    fn animation_update(percent_done: f32, new_atlas_coords: godot::builtin::Vector2i);
    
    /// Emitted when the generation worker reports an unrecoverable error.
    #[signal]
    fn generation_error(error_message: godot::prelude::GString); 
}