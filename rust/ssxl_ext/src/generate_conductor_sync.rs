// rust/SSXL-ext/src/generate_conductor_sync.rs

use flume::Receiver;
use crate::shared_message::{GenerationDataMessage, GenerationControlMessage};
use crate::generate_conductor_state::{ConductorStateContainer, ConductorState};
use crate::shared_chunk::Chunk;
use crate::tools::{ssxl_info, ssxl_error, Profiler};

// The maximum number of messages (completed chunks/errors) to process from the channel per frame.
// This is separate from the rendering budget but necessary to prevent channel backlog.
const MAX_MESSAGES_PER_FRAME: u32 = 32;

/// Polls the completed work channel, updates metrics, and stages chunks for rendering.
/// 
/// This is the synchronization point between the worker threads and the main thread state.
/// 
/// Returns: A vector of completed Chunks that are ready to be passed to the host_render.rs pacing layer.
pub fn poll_and_stage_completed_work(
    chunk_receiver: &Receiver<GenerationDataMessage>,
    state_container: &ConductorStateContainer,
) -> Vec<Chunk> {
    
    let mut completed_chunks_staged = Vec::new();
    let mut messages_processed = 0;
    
    // Use a profiler to track the time spent syncing channels
    let _p = Profiler::start("Conductor_Sync_Poll");

    // Loop until the channel is empty or the frame budget is hit
    while messages_processed < MAX_MESSAGES_PER_FRAME {
        
        match chunk_receiver.try_recv() {
            Ok(message) => {
                messages_processed += 1;
                
                match message {
                    GenerationDataMessage::CompletedChunk(chunk) => {
                        // 1. Update Metrics
                        state_container.increment_completed_chunks();
                        
                        // 2. Stage for Rendering
                        // The chunk is now safe on the main thread and ready for Godot API calls.
                        completed_chunks_staged.push(chunk);
                    },
                    
                    GenerationDataMessage::JobFailure(e) => {
                        // 1. Update Metrics (may or may not count as completed, depending on policy)
                        state_container.increment_failed_jobs();
                        
                        // 2. Transition State
                        state_container.transition_to(ConductorState::Error);
                        
                        ssxl_error!("Generation Worker Failed: {}", e);
                    },
                    
                    GenerationDataMessage::Ack => {
                        // Worker status acknowledgement (e.g., worker started)
                        // Ignore but count towards the budget.
                    }
                }
            },
            
            Err(flume::TryRecvError::Empty) => {
                // Channel is temporarily empty. Exit the loop.
                break;
            },
            
            Err(flume::TryRecvError::Disconnected) => {
                // Worker pool channel has been closed (workers shut down unexpectedly).
                ssxl_error!("Worker pool channel disconnected. Generation likely failed or was terminated.");
                state_container.transition_to(ConductorState::Error);
                break;
            }
        }
    }
    
    completed_chunks_staged
}