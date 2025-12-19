#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::classes::Engine;

use crate::shared_types::InstanceType;   // ✅ FIXED: correct import

#[cfg(feature = "godot-binding")]
pub fn emit_generation_finished(tilemap_id: InstanceType) {
    let mut engine_singleton: Gd<Engine> = Engine::singleton();
    engine_singleton.call("emit_signal", &[
        "generation_complete".to_variant(),
        tilemap_id.to_variant(),
    ]);
}

#[cfg(not(feature = "godot-binding"))]
pub fn emit_generation_finished(tilemap_id: InstanceType) {
    eprintln!(
        "INFO: CLI Mode - Skipped emitting generation_finished signal for ID: {}",
        tilemap_id
    );
}
