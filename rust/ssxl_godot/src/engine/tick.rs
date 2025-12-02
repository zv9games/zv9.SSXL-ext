// File: ssxl_godot/src/engine/tick.rs (Final Optimized Imports and Logic)

use godot::prelude::*;
use super::state::SSXLEngine;
use super::render_batch::create_render_batch_dictionary;

use crate::ffi::signals::*; 

use godot::builtin::GString; 

use tracing::debug;

use ssxl_shared::message::generation_message::GenerationMessage; 

pub fn process_engine_tick(engine: &mut SSXLEngine, tick: u64) {
    let state = unsafe { &mut *engine._internal_state.get() };
    
    // FIX 1: Prefix `conductor` with `_` to suppress the unused variable warning.
    let Some(_conductor) = &state.conductor else { return };
    
    // Using `as_mut()` here allows for `emit_signal` calls later.
    let Some(signals_node) = state.signals_node.as_mut() else { return }; 
    
    let _signals = signals_node
        .clone()
        .try_cast::<SSXLSignals>()
        .expect("Signals node must be castable to SSXLSignals to emit events.");

    // --- Generation Message Polling ---
    let messages = state.poller.poll_generation();
    for msg in messages {
        // FIX 2: Removed the unreachable `_ => {}` pattern.
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
            // Note: If GenerationMessage has other variants, they should be added here.
            // Since the compiler stated the previous `_` was unreachable, this list is assumed complete.
        }
    }

    // --- Animation Message Polling ---
    let anim_msgs = state.poller.poll_animations();
    if !anim_msgs.is_empty() {
        for msg in &anim_msgs {
            // FIX: Uses fully qualified path `ssxl_shared::AnimationPayload` now.
            if let ssxl_shared::AnimationPayload::FrameUpdate { new_frame } = msg.payload {
                
                signals_node.emit_signal(
                    "tile_flip_updated", 
                    &[
                        // FIX: use `msg.coord` instead of `msg.tile_coords`.
                        (msg.coord.x as i32).to_variant(), 
                        (msg.coord.y as i32).to_variant(), // Assuming y-coordinate is also needed
                        (new_frame as i32).to_variant()
                    ]
                );
            }
        }
        debug!("Tick: Processed {} animation updates", anim_msgs.len());
    }

    // --- Tick Completion Signal ---
    signals_node.emit_signal(
        "tick_complete", 
        &[tick.to_variant()]
    ); 
}