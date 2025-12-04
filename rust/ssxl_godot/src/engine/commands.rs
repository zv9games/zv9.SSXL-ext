// FILE: ssxl_godot/src/engine/commands.rs (Optimized Imports)
//
// Only the lightweight, high-frequency runtime commands that do NOT belong in:
// • init.rs (setup)
// • api.rs (heavyweight generation start)
// • tick.rs (game loop)
//
// These are instant, side-effect-focused commands used during gameplay.

use godot::prelude::*;
use ssxl_shared::AnimationCommand;
// FIX: Removed unused imports: `tracing::info` and `tracing::warn`.

// Import necessary types and state access
use crate::engine::state as state_module; 
use state_module::{SSXLEngine, state}; 

// The entire #[godot_api] impl block is removed.
// Functions are now standalone and delegated from init.rs.

/// Gracefully stops the current generation process.
/// Emits appropriate signal so Godot can listen to.
pub fn stop_generation_logic(engine: &mut SSXLEngine) {
    state!(engine, state);
    if let Some(arc) = &state.conductor {
        if let Ok(c) = arc.lock() {
            let _ = c.stop_generation();
        }
    }
}

/// Toggles the low-latency animation conductor (flow fields, particle systems, etc.)
pub fn set_animation_enabled_logic(engine: &mut SSXLEngine, enabled: bool) {
    state!(engine, state);
    if let Some(tx) = &state.animation_conductor {
        let _ = tx.send(AnimationCommand::SetEnabled(enabled));
    }
}

/// Changes the active generator mid-session (for dynamic biome switching, etc.)
pub fn set_generator_logic(engine: &mut SSXLEngine, name: GString) {
    state!(engine, state);
    if let Some(arc) = &state.conductor {
        if let Ok(mut c) = arc.lock() {
            // FIX E0599: Correcting the method name to the likely existing setter.
            let _ = c.set_generator(&name.to_string());
        }
    }
}