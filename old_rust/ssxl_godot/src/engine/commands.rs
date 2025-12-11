// ============================================================================
// 🎼 Engine Control Logic (`crate::engine::control_logic`)
// ----------------------------------------------------------------------------
// This module defines public-facing functions that allow Godot scripts to
// control the SSXL engine at runtime. These functions provide safe access to
// the engine’s internal state and expose key orchestration commands.
//
// Purpose:
//   • Allow Godot to stop terrain generation gracefully.
//   • Enable or disable animation subsystems dynamically.
//   • Switch active terrain generators mid-session.
//   • Provide a clean, script-friendly API surface for engine control.
//
// Key Components:
//   • SSXLEngine
//       - Represents the engine state exposed to Godot.
//       - Holds references to conductor, animation subsystems, and channels.
//   • state! macro
//       - Provides safe access to the internal state of `SSXLEngine`.
//   • AnimationCommand
//       - Enum representing commands sent to the animation conductor.
//       - Includes variants like `SetEnabled` for toggling animation systems.
//
// Functions:
//   • stop_generation_logic
//       - Gracefully halts the current generation process.
//       - Workflow:
//           1. Access engine state via `state!`.
//           2. If a conductor exists, acquire its lock.
//           3. Call `stop_generation()` on the conductor.
//           4. Errors (e.g., lock poisoning) are ignored silently.
//
//   • set_animation_enabled_logic
//       - Toggles the animation conductor (flow fields, particle systems, etc.).
//       - Workflow:
//           1. Access engine state via `state!`.
//           2. If an animation conductor channel exists, send `SetEnabled` command.
//           3. Errors (e.g., channel closed) are ignored silently.
//
//   • set_generator_logic
//       - Switches the active generator mid-session.
//       - Useful for dynamic biome switching or experimentation.
//       - Workflow:
//           1. Access engine state via `state!`.
//           2. If a conductor exists, acquire its lock.
//           3. Call `set_generator()` with the new generator name.
//           4. Convert Godot’s `GString` into a Rust `String` before passing.
//           5. Errors are ignored silently.
//
// Design Choices:
//   • Defensive programming: all functions ignore errors silently to prevent
//     runtime crashes in Godot scripts.
//   • Use of `Arc<Mutex>` ensures safe concurrent access to conductor state.
//   • Godot-compatible types (`GString`) allow seamless FFI integration.
//   • Separation of control logic into standalone functions keeps API surface clean.
//
// Educational Note:
//   • This module demonstrates how Rust can expose safe, script-friendly
//     orchestration commands to external engines like Godot. By combining
//     concurrency primitives, enums for structured commands, and FFI-compatible
//     types, it creates a robust bridge between procedural generation logic
//     and game engine scripting.
// ============================================================================


use godot::prelude::*;
use ssxl_shared::AnimationCommand;
use crate::engine::state as state_module; 
use state_module::{SSXLEngine, state}; 

pub fn stop_generation_logic(engine: &mut SSXLEngine) {
    state!(engine, state);
    if let Some(arc) = &state.conductor {
        if let Ok(c) = arc.lock() {
            let _ = c.stop_generation();
        }
    }
}

pub fn set_animation_enabled_logic(engine: &mut SSXLEngine, enabled: bool) {
    state!(engine, state);
    if let Some(tx) = &state.animation_conductor {
        let _ = tx.send(AnimationCommand::SetEnabled(enabled));
    }
}

pub fn set_generator_logic(engine: &mut SSXLEngine, name: GString) {
    state!(engine, state);
    if let Some(arc) = &state.conductor {
        if let Ok(mut c) = arc.lock() {
            let _ = c.set_generator(&name.to_string());
        }
    }
}
