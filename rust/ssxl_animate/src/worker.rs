// ssxl_animate/src/worker.rs (Fixed: E0061 and Optimized Imports)

use rayon::prelude::*;
use tracing::info;

// FIX: Added necessary ChunkData/TileData types for the mock data structure.
use ssxl_shared::{
    AnimationCommand, 
    ChunkId, 
    UpdateSender,
    ChunkData,       // REQUIRED for mock structure
    TileData         // REQUIRED for mock structure array
};
use ssxl_shared::message::messages::AnimationUpdate as ChannelUpdate;

use crate::animation_logic;

// --- FIX: MOCK DATA ACCESSOR (TEMPORARY HIGH-EFFICIENCY HACK) ---
// This placeholder simulates a FAST, non-blocking cache lookup for ChunkData.
// In a final production build, this MUST be replaced by a call to the 
// high-performance, thread-safe ssxl_cache API.
fn get_chunk_data_mock(_chunk_id: ChunkId) -> ChunkData {
    ChunkData { 
        id: 0, 
        bounds: Default::default(), 
        tiles: [TileData::default(); 1024], // Fills array with default tile data
        dimension_tag: "Mock".into(),
        generated_at: std::time::SystemTime::now(),
    }
}
// ---------------------------------------------------------------

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
        
        // CRITICAL: Keep this `std::thread::spawn` to run Rayon OFF the async Conductor thread.
        std::thread::spawn(move || {
            chunk_ids.par_iter().for_each(|&chunk_id: &ChunkId| {
                
                // FIX E0061: Retrieve data locally to satisfy execute_for_chunk's 3-argument signature.
                let chunk_data = get_chunk_data_mock(chunk_id);
                
                let tile_updates = 
                    animation_logic::execute_for_chunk(
                        chunk_id, 
                        &chunk_data, // ARGUMENT SUPPLIED: Pass reference to chunk_data
                        anim_type.clone()
                    );
                
                for update in tile_updates {
                    let converted_update = ChannelUpdate {
                        coord: update.coord,
                        payload: update.payload,
                    };

                    let _ = update_tx.send(converted_update).ok(); 
                }
            });

            info!("Finished parallel processing for chunk set.");
        });
    }
}