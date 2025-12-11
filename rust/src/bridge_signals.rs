// rust/SSXL-ext/src/bridge_signals.rs

use godot::prelude::*;
use godot::classes::Engine; 
use godot::classes::Object; // Add Object to clarify Gd<Object> type

// We need a way to get the GDExtension class instance (the singleton)
// to call emit_signal on it. This assumes the existence of a global
// accessor `get_ssxl_instance()`.

/// Emits the final signal back to the Godot GDScript orchestration layer.
pub fn emit_generation_finished(tilemap_id: InstanceId) {
    // 1. Get the main GDExtension instance handle
    // let ssxl_instance = get_ssxl_instance(); 

    // FIX: Use the infallible, dedicated Engine::singleton() method which returns Gd<Engine>.
    // This resolves the missing function error and the previous argument/unwrap errors.
    
    let mut engine_singleton: Gd<Engine> = Engine::singleton(); 
    
    // NOTE: The `call` method often requires a mutable receiver (`&mut self`), 
    // so we make `engine_singleton` mutable.
    
    engine_singleton.call("emit_signal", &[
        "generation_complete".to_variant(), 
        tilemap_id.to_variant()
    ]);
}