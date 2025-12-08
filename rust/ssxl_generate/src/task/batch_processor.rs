// ============================================================================
// 🎼 Batch Generation Task Spawner (`crate::task::batch_generation`)
// ----------------------------------------------------------------------------
// This module defines the `spawn_batch_generation_task` function, which is
// responsible for orchestrating sequential batch generation of chunks in the
// SSXL engine. It leverages the Tokio runtime to spawn a blocking task that
// processes all chunks in a map configuration.
//
// Purpose:
//   • Execute full map generation in a controlled, sequential manner.
//   • Ensure stability when working with non-thread-safe generators and caches.
//   • Provide progress and completion signals back to the Conductor.
//   • Track queue depth to monitor active generation tasks.
//
// Key Components:
//   • runtime_handle
//       - A handle to the Tokio runtime.
//       - Used to spawn the blocking batch generation task.
//
//   • generators_clone
//       - Cloned registry of available generators.
//       - Provides access to generator implementations by ID.
//
//   • chunk_cache_clone
//       - Shared cache for storing/retrieving generated chunks.
//       - Prevents redundant computation.
//
//   • active_generator_id
//       - ID of the generator to use for this batch.
//       - Ensures deterministic selection of algorithm.
//
//   • progress_sender_clone
//       - Channel sender for progress/completion messages.
//       - Allows Conductor to track task lifecycle.
//
//   • internal_state_clone
//       - Tracks conductor lifecycle state and queue depth.
//       - Ensures proper accounting of active tasks.
//
//   • config_clone
//       - Generator configuration (map dimensions, seed, etc.).
//       - Defines the scope of the batch generation.
//
// Workflow:
//   1. Log the start of batch generation with configuration details.
//   2. Spawn a blocking task on the runtime to process chunks sequentially.
//   3. Increment queue depth to track active task.
//   4. Calculate map dimensions in terms of chunk counts using ceiling division.
//   5. Build a list of all chunk coordinates to be generated.
//   6. Iterate sequentially over each chunk coordinate:
//        • Call `handle_chunk_unit` to generate and cache chunk data.
//   7. Send a completion message via channel when finished.
//   8. Decrement queue depth to reflect task completion.
//   9. Log completion message with configuration details.
//
// Design Choices:
//   • Sequential processing ensures stability with non-thread-safe components.
//   • Ceiling division guarantees partial chunks are included in generation.
//   • Logging provides visibility into task lifecycle and edge cases.
//   • Queue depth tracking prevents runaway task spawning.
//   • Completion signaling integrates with Conductor’s orchestration layer.
//
// Educational Note:
//   • This function demonstrates how Rust + Tokio can coordinate blocking,
//     sequential workloads within an async runtime.
//   • By combining concurrency primitives (channels, Arc) with structured logging,
//     it ensures deterministic, traceable, and safe batch generation.
// ============================================================================


use tokio::runtime::Handle;
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use ssxl_math::prelude::Vec2i;
use ssxl_cache::ChunkCache;
use crate::manager::config_validator::GeneratorConfig;
use crate::conductor::conductor_state::ConductorState;
use crate::manager::generator_manager::DynGenerator;
use crate::task::task_queue::{handle_chunk_unit, GenerationMessage};
use ssxl_shared::CHUNK_SIZE;

pub fn spawn_batch_generation_task(
    runtime_handle: &Handle,
    generators_clone: HashMap<String, Arc<DynGenerator>>,
    chunk_cache_clone: Arc<ChunkCache>,
    active_generator_id: String,
    progress_sender_clone: Sender<GenerationMessage>,
    internal_state_clone: Arc<ConductorState>,
    config_clone: GeneratorConfig,
) {
    info!("Conductor spawning SEQUENTIAL BATCH generation task. Config: {}", config_clone);

    runtime_handle.spawn_blocking(move || {
        internal_state_clone.increment_queue_depth();

        let chunk_size_i64: i64 = CHUNK_SIZE as i64;
        let map_width_i64: i64 = config_clone.width as i64;
        let map_height_i64: i64 = config_clone.height as i64;

        let width_in_chunks = (map_width_i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let height_in_chunks = (map_height_i64 + chunk_size_i64 - 1) / chunk_size_i64;

        let all_chunk_coords: Vec<Vec2i> = (0..width_in_chunks)
            .flat_map(|x| (0..height_in_chunks).map(move |y| Vec2i::new(x, y)))
            .collect();
        
        if all_chunk_coords.is_empty() {
            info!("Batch generation task received a map size of 0x0 chunks. Task finished immediately. Config: {}", config_clone);
        }

        let active_generator_id_ref = &active_generator_id;
        
        for &chunk_coords in all_chunk_coords.iter() {
            handle_chunk_unit(
                chunk_coords,
                active_generator_id_ref,
                &generators_clone,
                &chunk_cache_clone,
                &progress_sender_clone,
                &internal_state_clone,
            );
        }

        if progress_sender_clone.try_send(GenerationMessage::GenerationComplete).is_err() {
            warn!("Batch completion signal dropped (Channel full).");
        }

        internal_state_clone.decrement_queue_depth();

        info!("Batch generation task finished processing command: {}", config_clone);
    });
}
