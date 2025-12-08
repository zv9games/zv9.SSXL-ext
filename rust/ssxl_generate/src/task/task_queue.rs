// ============================================================================
// 🎼 Task Queue System (`crate::task::task_queue`)
// ----------------------------------------------------------------------------
// This module defines the core task queue logic for the SSXL engine. It manages
// chunk generation requests, integrates with caching, and ensures that tasks
// are processed safely and efficiently within the Tokio runtime.
//
// Purpose:
//   • Provide an asynchronous task queue for chunk generation.
//   • Handle cache lookups and generation logic in a unified flow.
//   • Track conductor state (tile counts, lifecycle, active status).
//   • Ensure clean startup and shutdown of generation tasks.
//
// Key Components:
//   • DynGenerator (type alias)
//       - Dynamically dispatched `Generator` trait object.
//       - Send + Sync for concurrency safety.
//   • CHUNK_SIZE (constant)
//       - Local i64 constant derived from shared crate.
//       - Defines chunk dimensions for generation and caching.
//
// Functions:
//   • handle_chunk_unit
//       - Handles a single chunk request end-to-end.
//       - Workflow:
//           1. Build a unique `ChunkKey` from coordinates.
//           2. Check cache for existing chunk.
//           3. If cache hit → send cached chunk via channel.
//           4. If cache miss → generate new chunk using selected generator.
//           5. Update conductor state with tile count.
//           6. Save generated chunk to cache.
//           7. Send generated chunk via progress channel.
//       - Provides structured logging for cache hits/misses, errors, and results.
//
//   • start_request_loop
//       - Main async loop for processing incoming generation tasks.
//       - Workflow:
//           1. Spawn async loop on runtime.
//           2. Log start of task queue.
//           3. Maintain list of active blocking tasks.
//           4. For each incoming task:
//                a. Check conductor active state.
//                b. Clone necessary resources.
//                c. Spawn blocking task to handle chunk via `handle_chunk_unit`.
//                d. Track task handle.
//           5. When channel closes, drain all active tasks.
//           6. Send `GenerationComplete` message.
//           7. Log clean shutdown.
//       - Ensures graceful lifecycle management of generation tasks.
//
// Design Choices:
//   • Separation of concerns: `handle_chunk_unit` focuses on single chunk logic,
//     while `start_request_loop` orchestrates task queue lifecycle.
//   • Arc + HashMap ensures safe concurrent access to generators and cache.
//   • Structured logging (info, warn, debug, error) provides visibility into
//     task execution, cache behavior, and error handling.
//   • Blocking tasks are spawned for generation to isolate heavy workloads
//     while preserving async responsiveness.
//
// Educational Note:
//   • This module demonstrates how Rust + Tokio can integrate async orchestration
//     with blocking workloads, ensuring both performance and safety.
//   • By combining caching, atomic state tracking, and structured logging,
//     it provides a robust foundation for procedural generation pipelines.
// ============================================================================


use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{info, error, warn, debug};
use std::collections::HashMap;
use std::sync::Arc;
use ssxl_math::prelude::Vec2i;
use ssxl_math::coordinate_system::ChunkKey;
use glam::I64Vec3;
use ssxl_cache::ChunkCache;
use ssxl_shared::CHUNK_SIZE as SHARED_CHUNK_SIZE;
use crate::Generator;
use crate::conductor::conductor_state;
pub use ssxl_shared::message::generation_message::{GenerationMessage, GenerationTask};

type DynGenerator = Box<dyn Generator + Send + Sync>;
pub const CHUNK_SIZE: i64 = SHARED_CHUNK_SIZE as i64;

pub fn handle_chunk_unit(
    chunk_coords: Vec2i,
    generator_name: &str,
    generators: &HashMap<String, Arc<DynGenerator>>,
    chunk_cache: &Arc<ChunkCache>,
    progress_sender: &mpsc::Sender<GenerationMessage>,
    conductor_state: &Arc<conductor_state::ConductorState>,
) {
    let chunk_key = ChunkKey(I64Vec3 {
        x: chunk_coords.x,
        y: chunk_coords.y,
        z: 0,
    });

    if let Some(chunk_data_arc) = chunk_cache.load_chunk(&chunk_key) {
        debug!(?chunk_coords, "Cache HIT");

        let msg = GenerationMessage::Generated(chunk_coords, chunk_data_arc);
        if progress_sender.try_send(msg).is_err() {
            warn!(?chunk_coords, "Failed to send cached chunk (channel full/closed)");
        }
        return;
    }

    debug!(?chunk_coords, generator = %generator_name, "Cache MISS → generating");

    let generator = generators
        .get(generator_name)
        .expect("Generator must exist in map");

    let chunk_data = generator.generate_chunk(chunk_coords);

    let tile_count = chunk_data.tiles.len() as u64;
    conductor_state.increment_tile_count(tile_count);

    let chunk_data_arc = Arc::new(chunk_data);

    if chunk_cache.save_chunk(&chunk_key, chunk_data_arc.clone()).is_err() {
        error!(?chunk_coords, "Failed to save generated chunk to cache");
    }

    let msg = GenerationMessage::Generated(chunk_coords, chunk_data_arc);
    if progress_sender.try_send(msg).is_err() {
        warn!(?chunk_coords, "Failed to send generated chunk (channel full/closed)");
    } else {
        debug!(?chunk_coords, "Sent newly generated chunk");
    }
}

pub fn start_request_loop(
    rt_handle: Handle,
    mut request_rx: mpsc::UnboundedReceiver<GenerationTask>,
    progress_tx: mpsc::Sender<GenerationMessage>,
    generators: Arc<HashMap<String, Arc<DynGenerator>>>,
    chunk_cache: Arc<ChunkCache>,
    conductor_state: Arc<conductor_state::ConductorState>,
) {
    rt_handle.spawn(async move {
        info!("Generation Task Queue started");

        let mut active_tasks: Vec<JoinHandle<()>> = Vec::new();

        while let Some(task) = request_rx.recv().await {
            if !conductor_state.as_ref().is_active() {
                warn!(?task.chunk_coords, "Dropping task — Conductor not active");
                continue;
            }

            let progress_tx = progress_tx.clone();
            let generators = generators.clone();
            let cache = chunk_cache.clone();
            let state = conductor_state.clone();

            let handle = tokio::task::spawn_blocking(move || {
                handle_chunk_unit(
                    task.chunk_coords,
                    &task.generator_id,
                    &generators,
                    &cache,
                    &progress_tx,
                    &state,
                );
            });

            active_tasks.push(handle);
        }

        info!("Request channel closed. Draining {} active tasks...", active_tasks.len());

        for handle in active_tasks {
            if let Err(e) = handle.await {
                error!("Generation task panicked: {:?}", e);
            }
        }

        let _ = progress_tx.send(GenerationMessage::GenerationComplete).await;

        info!("Generation Task Queue shut down cleanly");
    });
}
