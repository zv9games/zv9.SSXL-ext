// ============================================================================
// 🎼 Engine Tick Processing (`crate::engine::tick`)
// ----------------------------------------------------------------------------
// This module defines the `process_engine_tick` function, which is called once
// per frame by Godot. It serves as the heartbeat of the SSXL engine, ensuring
// that generation and animation messages are processed, signals are emitted,
// and the engine state advances consistently.
//
// Purpose:
//   • Poll asynchronous channels for generation and animation updates.
//   • Translate raw engine messages into Godot-compatible signals.
//   • Drive rendering updates (chunks, tile flips) and status reporting.
//   • Emit a tick completion signal to mark the end of each frame.
//
// Key Components:
//   • SSXLEngine
//       - The Godot-facing engine struct wrapping `InternalState`.
//       - Provides access to conductor, poller, and signals node.
//   • SSXLSignals
//       - Godot signal interface used to emit events back to GDScript.
//       - Bridges Rust engine events with Godot script callbacks.
//   • GenerationMessage
//       - Enum representing messages from the generation system.
//       - Variants include `Generated(chunk)`, `StatusUpdate(string)`,
//         and `GenerationComplete`.
//   • create_render_batch_dictionary
//       - Utility function that converts `ChunkData` into a Godot `Dictionary`
//         formatted for TileMap rendering.
//   • AnimationPayload
//       - Represents animation updates (e.g., frame flips).
//
// Function: process_engine_tick
//   • Arguments:
//       - engine: mutable reference to `SSXLEngine`.
//       - tick: current tick counter (frame number).
//   • Workflow:
//       1. Access internal state via `UnsafeCell`.
//       2. Ensure conductor and signals node exist; return early if missing.
//       3. Cast signals node to `SSXLSignals` for emitting signals.
//       4. Poll generation messages:
//           • On `Generated`: convert chunk to render batch and emit signal.
//           • On `StatusUpdate`: emit engine status update signal.
//           • On `GenerationComplete`: emit build completion signal.
//       5. Poll animation messages:
//           • On `FrameUpdate`: emit tile flip update signals.
//       6. Emit `tick_complete` signal with current tick number.
//   • Returns:
//       - No return value; side effects are emitted signals to Godot.
//
// Design Choices:
//   • Defensive programming: early returns if conductor or signals node missing.
//   • Non-blocking polling ensures responsiveness each frame.
//   • Logging (`debug!`) provides visibility into tick activity.
//   • Signals provide a clean, script-friendly interface for Godot integration.
//
// Educational Note:
//   • This function demonstrates how Rust can orchestrate real-time engine
//     updates while integrating seamlessly with Godot’s signal system. By
//     translating internal messages into Godot-native events, it ensures
//     that procedural generation and animation remain synchronized with
//     the game loop.
// ============================================================================


use godot::prelude::*;
use super::state::SSXLEngine;
use super::render_batch::create_render_batch_dictionary;
use crate::ffi::signals::*; 
use godot::builtin::GString; 
use tracing::debug;
use ssxl_shared::message::generation_message::GenerationMessage; 

pub fn process_engine_tick(engine: &mut SSXLEngine, tick: u64) {
    let state = unsafe { &mut *engine._internal_state.get() };
    
    let Some(_conductor) = &state.conductor else { return };
    let Some(signals_node) = state.signals_node.as_mut() else { return }; 
    
    let _signals = signals_node
        .clone()
        .try_cast::<SSXLSignals>()
        .expect("Signals node must be castable to SSXLSignals to emit events.");

    let messages = state.poller.poll_generation();
    for msg in messages {
        match msg {
            GenerationMessage::Generated(_, chunk) => {
                let chunk_x = chunk.bounds.min.x as i32;
                let chunk_y = chunk.bounds.min.y as i32;
                let batch = create_render_batch_dictionary(&chunk, chunk_x, chunk_y);
                
                if !batch.is_empty() {
                    signals_node.emit_signal(
                        "chunk_generated_batch", 
                        &[batch.to_variant()]
                    );
                    debug!("Tick: Emitted chunk_generated_batch ({chunk_x}, {chunk_y})");
                }
            }
            GenerationMessage::StatusUpdate(update) => {
                signals_node.emit_signal(
                    "engine_status_updated", 
                    &[GString::from(update.as_str()).to_variant()]
                );
            }
            GenerationMessage::GenerationComplete => {
                signals_node.emit_signal(
                    "build_map_complete", 
                    &[]
                );
            }
        }
    }

    let anim_msgs = state.poller.poll_animations();
    if !anim_msgs.is_empty() {
        for msg in &anim_msgs {
            if let ssxl_shared::AnimationPayload::FrameUpdate { new_frame } = msg.payload {
                signals_node.emit_signal(
                    "tile_flip_updated", 
                    &[
                        (msg.coord.x as i32).to_variant(), 
                        (msg.coord.y as i32).to_variant(),
                        (new_frame as i32).to_variant()
                    ]
                );
            }
        }
        debug!("Tick: Processed {} animation updates", anim_msgs.len());
    }

    signals_node.emit_signal(
        "tick_complete", 
        &[tick.to_variant()]
    ); 
}
