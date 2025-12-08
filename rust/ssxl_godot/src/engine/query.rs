// ============================================================================
// 🎼 Engine Query Logic (`crate::engine::query`)
// ----------------------------------------------------------------------------
// This module defines read-only query functions that expose engine state to
// Godot scripts. These functions provide safe, side-effect-free access to
// information about the SSXL engine’s current status, tile counts, and active
// generator identity.
//
// Purpose:
//   • Allow Godot scripts to query engine state without mutating it.
//   • Provide human-readable status reports and numeric summaries.
//   • Safely handle concurrency when accessing conductor state.
//   • Return Godot-compatible types (`GString`, `u64`) for seamless FFI.
//
// Key Components:
//   • get_current_tile_count_logic
//       - Returns the total number of tiles generated so far.
//       - Workflow:
//           1. Access engine state via `state!` macro.
//           2. Call `StatusReporter::get_current_tile_count_value` with conductor state.
//           3. Return tile count as `u64`.
//
//   • get_status_logic
//       - Returns a human-readable engine status string.
//       - Example: "Generating... 3.7M tiles | Perlin | 12 workers | 142 TPS"
//       - Workflow:
//           1. Access engine state via `state!` macro.
//           2. Call `StatusReporter::get_status_report` with conductor and animation state.
//           3. Return result as `GString`.
//
//   • get_active_generator_id_logic
//       - Returns the name of the currently active generator.
//       - Examples: "perlin", "cellular", "simplex", "custom_my_gen".
//       - Workflow:
//           1. Access engine state via `state!` macro.
//           2. Check if conductor exists (`Option<Arc<Mutex<Conductor>>>`).
//           3. If None → return `"Not Initialized"`.
//           4. If Some → attempt to acquire lock on conductor.
//           5. On success → call `get_active_generator_id()` and convert to `GString`.
//           6. On failure (mutex poisoned) → return `"Mutex Poisoned"`.
//
// Design Choices:
//   • Queries are side-effect-free, ensuring safe read-only access.
//   • Defensive handling of `Option` and `Mutex` prevents runtime crashes.
//   • Integration with `StatusReporter` centralizes reporting logic.
//   • Returning Godot-native types ensures compatibility with GDScript.
//
// Educational Note:
//   • This module demonstrates how Rust can expose safe, read-only queries
//     to external engines like Godot. By combining concurrency primitives,
//     defensive programming, and FFI-compatible types, it provides a robust
//     interface for monitoring engine state without risking mutation.
// ============================================================================


use godot::prelude::*;
use crate::tilemap::status_reporter::StatusReporter;
use crate::engine::state as state_module; 
use state_module::{SSXLEngine, state}; 

pub fn get_current_tile_count_logic(engine: &SSXLEngine) -> u64 {
    state!(engine, state);
    StatusReporter::get_current_tile_count_value(state.conductor_state.as_ref())
}

pub fn get_status_logic(engine: &SSXLEngine) -> GString {
    state!(engine, state);
    StatusReporter::get_status_report(
        state.conductor_state.as_ref(),
        state.animation_state.as_ref(),
    )
}

pub fn get_active_generator_id_logic(engine: &SSXLEngine) -> GString {
    state!(engine, state);
    let Some(arc) = &state.conductor else {
        return "Not Initialized".into();
    };
    arc.lock()
        .map(|c| (&c.get_active_generator_id()).into()) 
        .unwrap_or("Mutex Poisoned".into())
}
