// rust/SSXL-ext/src/host_poller.rs

use crate::generate_conductor::GenerateConductor;
// REMOVED: unused import `crate::generate_conductor_state::ConductorState;`
use crate::bridge_signals;
// Removed unused import: use crate::host_state::get_host_state;
// Reason: Avoids unnecessary locking and retrieval of the global HostState 

use crate::ssxl_info; 
// REMOVED: unused import `ssxl_error` (error logging is now handled inside Conductor)

/// The main polling routine, called once per Godot frame (typically from `host_tick.rs`).
/// Its primary responsibility is to pull completed work from the background thread,
/// render it, and emit the final completion signal.
pub fn poll_conductor_status(conductor: &GenerateConductor) {
    // 1. Process and Render Completed Chunks (The Direct Write)
    // The non-blocking channel polling and rendering are handled internally by the Conductor.
    let (chunks_rendered, generation_completed) = conductor.poll_chunks_and_render();

    if chunks_rendered > 0 {
        ssxl_info!("Poller: Rendered {} chunks this frame.", chunks_rendered);
        // Optional: Emit a progress update signal here if detailed GDScript tracking is needed.
        // bridge_signals::emit_progress_update(conductor.get_metrics());
    }

    // 2. State Transition Monitoring and Signal Broadcast (Lifecycle Guard)
    
    // `generation_completed` indicates the Conductor has internally flipped its state to Finished,
    // and all chunks are written. Now, we must emit the signal exactly once.
    if generation_completed {
        // This call implements the CRITICAL single-emission guard.
        // It uses Conductor's internal AtomicBool to ensure the signal is broadcast only on the 
        // *first* frame it finishes, providing robust lifecycle management.
        if let Some(tilemap_id) = conductor.try_finalize_and_get_target_id() {
            ssxl_info!("Poller: All chunks rendered. Emitting final signal for ID: {}", tilemap_id.to_i64());
            
            // Broadcast the signal back to GDScript, closing the loop.
            // This is the signal-driven hook for GDScript orchestration.
            bridge_signals::emit_generation_finished(tilemap_id);
        }
        // If try_finalize_and_get_target_id() returns None, the signal was already sent 
        // or an internal error (logged by the Conductor) occurred.
    }
}