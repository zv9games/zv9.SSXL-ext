// rust/SSXL-ext/src/host_render.rs

use godot::prelude::*;
use flume::Receiver;
use crate::shared_chunk::Chunk;
use crate::host_tilemap::render_chunk_direct;
use crate::host_state::HostState;
// FIX: Import the ConductorState enum
use crate::generate_conductor_state::{ConductorStateContainer, ConductorState}; 

// Define a safe maximum number of chunk writes per frame.
const MAX_CHUNKS_PER_FRAME: u32 = 16; 

/// Manages the rendering budget for completed chunks.
/// This function is called by the Host Poller on the Godot main thread.
pub fn render_available_chunks(
    tilemap_id: InstanceId,
    chunk_receiver: &Receiver<Chunk>,
    state_container: &ConductorStateContainer,
) -> (u32, bool) {
    
    let mut chunks_rendered_this_frame = 0;

    // Loop, but break if we hit the frame budget
    while chunks_rendered_this_frame < MAX_CHUNKS_PER_FRAME {
        
        // Non-blocking channel receive check
        match chunk_receiver.try_recv() {
            Ok(chunk) => {
                // 1. Execute the HIGH-PERFORMANCE DIRECT WRITE
                match render_chunk_direct(tilemap_id, chunk) {
                    Ok(_) => {
                        chunks_rendered_this_frame += 1;
                        state_container.increment_completed_chunks();
                    },
                    Err(e) => {
                        godot_error!("Render Pacing Layer: Direct write failed: {:?}", e);
                        // We continue, but might want to transition the state to Error
                    }
                }
            },
            Err(flume::TryRecvError::Empty) => {
                // No more chunks available in the channel. Exit loop.
                break;
            },
            Err(flume::TryRecvError::Disconnected) => {
                // Channel is closed (workers are shut down).
                godot_warn!("Render Pacing Layer: Worker channel disconnected unexpectedly.");
                break;
            }
        }
    }
    
    // Check for overall generation completion
    let metrics = state_container.get_metrics();
    let is_finished = metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;
    
    if is_finished && state_container.get_state() == ConductorState::Generating {
        state_container.transition_to(ConductorState::Finished);
    }
    
    (chunks_rendered_this_frame, is_finished)
}