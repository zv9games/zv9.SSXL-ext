// ssxl_animate/src/worker.rs

use rayon::prelude::*;
use tracing::info;

// --- Imports from ssxl_shared ---
use ssxl_shared::{AnimationCommand, UpdateSender, ChunkId, AnimationType};

// FIX: Explicitly import the correct message type. The function 'animation_logic::execute_for_chunk' 
// now correctly returns this type, making the old aliases unnecessary.
use ssxl_shared::messages::AnimationUpdate; 

use crate::animation_logic;

/// Initializes the global Rayon thread pool for maximal data parallelism.
/// This should be called once during engine initialization.
pub fn initialize_worker_pool(count: usize) {
    // Only attempt to configure if not already configured.
    if rayon::ThreadPoolBuilder::new()
        .num_threads(count)
        .build_global()
        .is_ok()
    {
        info!("Animation worker pool (Rayon) initialized with {} threads.", count);
    } else {
        // This is normal if called multiple times or configured elsewhere.
        info!("Animation worker pool (Rayon) found or failed to initialize.");
    }
}

/// Executes the animation command in parallel, delegating work immediately off
/// the Conductor's async thread to the Rayon pool.
///
/// This function *must* return instantly to maintain the Conductor's fast tempo.
pub fn process_command_parallel(command: AnimationCommand, update_tx: UpdateSender) {
    if let AnimationCommand::AnimateChunkSet { chunk_ids, anim_type } = command {
        
        // Use std::thread::spawn to move execution off the Conductor's single async
        // task and safely into a closure that executes the parallel computation.
        std::thread::spawn(move || {
            
            // CRITICAL: The parallel iterator ensures the workload is split across all
            // available Rayon threads, processing chunks concurrently.
            chunk_ids.par_iter().for_each(|&chunk_id: &ChunkId| {
                
                // 1. Perform animation calculation (pure, math-heavy logic)
                // The return type now automatically matches the channel's type.
                let tile_updates = animation_logic::execute_for_chunk(
                    chunk_id, 
                    anim_type.clone()
                );
                
                // 2. Send all results back to the main thread's poller via the channel.
                for update in tile_updates {
                    // Type check now passes seamlessly.
                    let _ = update_tx.send(update).ok(); 
                }
            });

            info!("Finished parallel processing for chunk set.");
        });
    }
}