// ssxl_animate/src/worker.rs (Optimized Imports)

use rayon::prelude::*;
use tracing::info;

// FIX: Removed unused imports: `AnimationPayload`, `TileCoord`, and `AnimationUpdate as AnimationResultUpdate`.
// Only keeping the types used explicitly as standalone types: `AnimationCommand`, `ChunkId`, and `UpdateSender`.
use ssxl_shared::{AnimationCommand, ChunkId, UpdateSender};
use ssxl_shared::message::messages::AnimationUpdate as ChannelUpdate;

use crate::animation_logic;

/// Initializes the global Rayon thread pool for maximal data parallelism.
#[allow(dead_code)]
pub fn initialize_worker_pool(count: usize) {
    if rayon::ThreadPoolBuilder::new()
        .num_threads(count)
        .build_global()
        .is_ok()
    {
        info!("Animation worker pool (Rayon) initialized with {} threads.", count);
    } else {
        info!("Animation worker pool (Rayon) found or failed to initialize.");
    }
}

/// Executes the animation command in parallel, delegating work immediately off
/// the Conductor's async thread to the Rayon pool.
pub fn process_command_parallel(
    command: AnimationCommand, 
    update_tx: UpdateSender
) {
    if let AnimationCommand::AnimateChunkSet { chunk_ids, anim_type } = command {
        
        std::thread::spawn(move || {
            chunk_ids.par_iter().for_each(|&chunk_id: &ChunkId| {
                
                let tile_updates = 
                    animation_logic::execute_for_chunk(chunk_id, anim_type.clone());
                
                // ssxl_shared::AnimationUpdate (the element type of tile_updates)
                // fields are used directly to construct the ChannelUpdate.
                for update in tile_updates {
                    
                    let converted_update = ChannelUpdate {
                        // The field is now `coord`.
                        coord: update.coord,
                        // The payload is now fully formed.
                        payload: update.payload,
                    };

                    let _ = update_tx.send(converted_update).ok(); 
                }
            });

            info!("Finished parallel processing for chunk set.");
        });
    }
}