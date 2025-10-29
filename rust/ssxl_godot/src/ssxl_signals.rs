// ssxl_godot/src/ssxl_signals.rs (Final, Clean Code)

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Base;
// FIX: Removed unused imports `GString`, `Dictionary`, and `Array`

// -------------------------------------------------------------------------------------------------
// SSXL SIGNALS GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// A dedicated Godot Node class used purely for emitting signals from the Rust core back to GDScript.
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
    
    // --- Signal Declarations ---

    #[signal] 
    fn build_map_start();

    #[signal]
    fn chunk_generated(x: i32, y: i32);
    
    #[signal]
    fn build_map_complete();

    // --- Signal Emitter Functions ---

    /// Emits the signal that the map build process has started.
    #[func]
    pub fn emit_build_map_start(&mut self) {
        self.base_mut().emit_signal("build_map_start", &[]);
    }

    /// Emits the signal that a new chunk has been generated and is ready to be loaded.
    #[func]
    pub fn emit_chunk_generated(&mut self, x: i32, y: i32) {
        self.base_mut().emit_signal("chunk_generated", &[x.to_variant(), y.to_variant()]);
    }

    /// Emits the signal that the entire map build process is complete.
    #[func]
    pub fn emit_build_map_complete(&mut self) {
        self.base_mut().emit_signal("build_map_complete", &[]);
    }
}