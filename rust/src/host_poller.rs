// rust/SSXL-ext/src/host_poller.rs

use godot::prelude::*;

use crate::generate_conductor::GenerateConductor;
use crate::generate_conductor_state::ConductorState;
use crate::bridge_signals;
use crate::host_state::{get_host_state, HostState};

// --- FIX: Import existing logging macros from the crate root and remove non-existent `ssxl_print` ---
use crate::{ssxl_info, ssxl_error}; 

/// The main polling routine, called once per Godot frame (typically from `host_tick.rs`).
/// Its primary responsibility is to pull completed work from the background thread,
/// render it, and emit the final completion signal.
pub fn poll_conductor_status(conductor: &GenerateConductor) {
    // 1. Process and Render Completed Chunks (The Direct Write)
    // The conductor handles the non-blocking channel polling and rendering internally.
    let (chunks_rendered, is_finished) = conductor.poll_chunks_and_render();

    if chunks_rendered > 0 {
        // --- FIX: Using ssxl_info instead of non-existent ssxl_print ---
        ssxl_info!("Poller: Rendered {} chunks this frame.", chunks_rendered);
        // Optional: Emit a progress update signal here if detailed GDScript tracking is needed.
        // bridge_signals::emit_progress_update(conductor.get_metrics()); 
    }

    // 2. State Transition Monitoring
    if is_finished {
        // The conductor has internally flipped its state to ConductorState::Finished.
        
        // We need to safely retrieve the HostState singleton.
        let host_state = match get_host_state() {
            Ok(state) => state,
            Err(_) => {
                // If HostState isn't ready, we can't emit the signal, so we exit.
                ssxl_error!("HostState is not initialized. Cannot report generation finished.");
                return;
            }
        };

        // Check if the signal has already been sent to prevent duplicates
        if host_state.conductor.get_state_container().get_state() == ConductorState::Finished {
            
            // --- 3. Update Host State and Signal Completion ---
            
            // NOTE: The reconstructed HostState does not have a `reported_conductor_state` 
            // field or a `set_reported_conductor_state` method. 
            // We'll rely on the `conductor`'s internal state which is the source of truth.
            
            ssxl_info!("All chunks rendered. Emitting final signal.");
            
            // Broadcast the signal back to GDScript, closing the loop.
            // Using the target ID from the conductor itself, which should have been set at start.
            let tilemap_id = conductor.tilemap_target_id;
            
            // FIX: Replace the non-existent `is_valid()` method with a check against 0.
            if tilemap_id.to_i64() != 0 {
                bridge_signals::emit_generation_finished(tilemap_id);
            } else {
                ssxl_error!("Cannot emit generation finished signal: TileMap ID not set in Conductor.");
            }
        }
    }
}