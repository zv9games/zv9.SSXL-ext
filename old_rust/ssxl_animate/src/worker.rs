// ============================================================================
// ⚡ Parallel Animation Worker (`ssxl_animate::worker`)
// ----------------------------------------------------------------------------
// This module defines the parallel execution layer for animation commands.
// It leverages Rayon’s thread pool to distribute heavy, CPU-bound animation
// work across multiple cores, keeping the conductor’s async loop lightweight
// and responsive.
//
// Key Concepts:
//   • Worker Pool Initialization:
//       - `initialize_worker_pool(count)` sets up a global Rayon thread pool
//         with a fixed number of threads.
//       - Ensures predictable parallelism and efficient use of CPU resources.
//   • Mock Data Accessor:
//       - `get_chunk_data_mock` simulates retrieving `ChunkData` for a given
//         chunk ID.
//       - In production, this would be replaced by a thread-safe cache or
//         database lookup (e.g., `ssxl_cache`).
//   • Parallel Command Processing:
//       - `process_command_parallel` offloads heavy animation work from the
//         conductor’s async thread into Rayon’s pool.
//       - Handles `AnimateChunkSet` commands by iterating over chunk IDs in
//         parallel, retrieving chunk data, executing animation logic, and
//         sending updates back to Godot via channels.
//       - Uses `std::thread::spawn` to ensure Rayon work runs outside the
//         async runtime, preventing event loop blocking.
//
// Workflow:
//   1. Conductor receives an `AnimateChunkSet` command.
//   2. `process_command_parallel` spawns a new OS thread.
//   3. Rayon distributes chunk IDs across worker threads.
//   4. For each chunk:
//        a. Retrieve chunk data (mock accessor).
//        b. Execute animation logic (`execute_for_chunk`).
//        c. Convert results into `ChannelUpdate` messages.
//        d. Send updates back through the channel.
//   5. Log completion of parallel processing.
//
// Design Choices:
//   • Rayon provides ergonomic parallel iterators (`par_iter`) for data-parallel
//     workloads, reducing boilerplate compared to manual thread management.
//   • Channels decouple worker threads from the conductor, ensuring safe,
//     asynchronous communication with Godot.
//   • Logging via `tracing::info` provides visibility into worker pool
//     initialization and task completion.
//
// Educational Note:
//   • This module demonstrates a hybrid concurrency model:
//       - Async conductor loop for responsiveness.
//       - Rayon thread pool for throughput on CPU-heavy tasks.
//   • By separating orchestration (conductor) from execution (workers),
//     the engine achieves both scalability and maintainability.
// ============================================================================


use rayon::prelude::*;
use tracing::info;

use ssxl_shared::{
    AnimationCommand,
    ChunkId,
    UpdateSender,
    ChunkData,
    TileData,
};
use ssxl_shared::message::messages::AnimationUpdate as ChannelUpdate;

use crate::animation_logic;

fn get_chunk_data_mock(_chunk_id: ChunkId) -> ChunkData {
    ChunkData { 
        id: 0,
        bounds: Default::default(),
        tiles: [TileData::default(); 1024],
        dimension_tag: "Mock".into(),
        generated_at: std::time::SystemTime::now(),
    }
}

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

pub fn process_command_parallel( 
    command: AnimationCommand,
    update_tx: UpdateSender,
) {
    if let AnimationCommand::AnimateChunkSet { chunk_ids, anim_type } = command {
        std::thread::spawn(move || {
            chunk_ids.par_iter().for_each(|&chunk_id: &ChunkId| {
                let chunk_data = get_chunk_data_mock(chunk_id);
                
                let tile_updates = 
                    animation_logic::execute_for_chunk(
                        chunk_id, 
                        &chunk_data,
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
