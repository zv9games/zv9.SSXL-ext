// ssxl_godot/src/engine/query.rs (Optimized Imports)
//
// Pure read-only oracle (Logic Implementation).
// No mutation. No side effects. No legacy.

use godot::prelude::*;
// FIX: Removed unused imports: `godot::builtin::Dictionary` and `ssxl_math::Vec2i`.

// FIX 1: Path changed to reflect module moved to src/tilemap/
use crate::tilemap::status_reporter::StatusReporter;

// FIX 2: Adopt the macro-friendly import pattern to resolve cross-file visibility issues.
use crate::engine::state as state_module; 
// CRITICAL FIX: Explicitly import the SSXLEngine type and the state macro.
use state_module::{SSXLEngine, state}; 

// The entire #[godot_api] impl block is removed to resolve E0119.

/// Returns total number of tiles generated so far (across all chunks).
pub fn get_current_tile_count_logic(engine: &SSXLEngine) -> u64 {
    state!(engine, state);
    StatusReporter::get_current_tile_count_value(state.conductor_state.as_ref())
}

/// Human-readable engine status string
/// e.g. "Generating... 3.7M tiles | Perlin | 12 workers | 142 TPS"
pub fn get_status_logic(engine: &SSXLEngine) -> GString {
    state!(engine, state);
    StatusReporter::get_status_report(
        state.conductor_state.as_ref(),
        state.animation_state.as_ref(),
    )
}

/// Returns the name of the currently active generator
/// e.g. "perlin", "cellular", "simplex", "custom_my_gen"
pub fn get_active_generator_id_logic(engine: &SSXLEngine) -> GString {
    state!(engine, state);
    let Some(arc) = &state.conductor else {
        return "Not Initialized".into();
    };
    arc.lock()
        .map(|c| (&c.get_active_generator_id()).into()) 
        .unwrap_or("Mutex Poisoned".into())
}