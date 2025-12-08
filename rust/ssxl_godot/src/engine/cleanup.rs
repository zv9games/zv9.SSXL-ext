// ============================================================================
// 🎼 Engine Shutdown Logic (`crate::engine::shutdown`)
// ----------------------------------------------------------------------------
// This module defines the `shutdown_logic` function, which provides a clean,
// public-facing API for Godot to explicitly shut down the SSXL engine. It ensures
// that all resources, background threads, and subsystems are gracefully released.
//
// Purpose:
//   • Allow Godot scripts to trigger a controlled shutdown of the SSXL engine.
//   • Safely tear down the Conductor and related subsystems.
//   • Clear references to Godot nodes to prevent dangling pointers.
//   • Return a confirmation message compatible with Godot (`GString`).
//
// Key Components:
//   • SSXLEngine
//       - Represents the engine state exposed to Godot.
//       - Holds references to conductor, animation subsystems, and Godot nodes.
//   • state! macro
//       - Provides safe access to the internal state of `SSXLEngine`.
//   • Arc + Mutex
//       - Used to manage shared ownership of the Conductor across threads.
//       - `Arc::try_unwrap` ensures teardown only occurs if the Conductor is
//         uniquely owned, preventing unsafe shutdowns.
//
// Workflow:
//   1. Access the engine’s internal state via the `state!` macro.
//   2. If a Conductor exists:
//        • Attempt to unwrap the `Arc<Mutex<Conductor>>`.
//        • If successful, call `graceful_teardown()` on the Conductor.
//        • If not unique or poisoned, skip teardown safely.
//   3. Clear other subsystems:
//        • conductor_state
//        • animation_conductor
//        • animation_state
//   4. Clear Godot node references:
//        • signals_node
//        • tilemap_node
//   5. Return a Godot-compatible confirmation message (`GString`).
//
// Design Choices:
//   • Graceful teardown ensures background tasks and channels are closed cleanly.
//   • Clearing subsystems prevents memory leaks and dangling references.
//   • Returning a `GString` integrates seamlessly with Godot’s scripting layer.
//   • Defensive use of `Arc::try_unwrap` avoids unsafe shutdown in multi-threaded contexts.
//
// Educational Note:
//   • This function demonstrates how Rust can safely manage engine lifecycle
//     when integrated with external systems like Godot. By combining concurrency
//     primitives, structured teardown, and Godot-compatible types, it ensures
//     stability and reliability during engine shutdown.
// ============================================================================


use godot::prelude::*;
use std::sync::Arc;

use crate::engine::state as state_module;
use state_module::{SSXLEngine, state};

pub fn shutdown_logic(engine: &mut SSXLEngine) -> GString {
    let state = state!(engine);
    
    if let Some(conductor_arc) = state.conductor.take() {
        if let Some(conductor) = Arc::try_unwrap(conductor_arc)
            .ok()
            .and_then(|m| m.into_inner().ok())
        {
            conductor.graceful_teardown();
        }
    }
    
    state.conductor_state.take();
    state.animation_conductor.take();
    state.animation_state.take();
    
    state.signals_node.take();
    state.tilemap_node.take();

    "SSXLEngine resources shut down and released.".into()
}
