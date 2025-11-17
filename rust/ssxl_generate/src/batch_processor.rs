// ssxl_generate/src/batch_processor.rs
//! Logic for executing large, synchronous batches of chunk generation requests.
//!
//! This module coordinates the parallel generation of a defined rectangular area of the
//! world map. It utilizes the Tokio runtime's `spawn_blocking` to safely move
//! the CPU-intensive work off the async executor, and the Rayon crate for CPU-bound,
//! fine-grained parallelism across available threads, ensuring high-speed completion
//! of generation tasks.

use tokio::runtime::Handle;
use tokio::sync::mpsc::Sender;
use tracing::info;
use std::collections::HashMap;
use std::sync::Arc; // CRITICAL FIX: Removed Mutex import.

use rayon::prelude::*;

use ssxl_math::Vec2i;
use ssxl_cache::ChunkCache; // Assumed to be thread-safe (Send + Sync) via internal locking.

use crate::config_validator::GeneratorConfig;
use crate::task_queue::{handle_chunk_unit, GenerationMessage, CHUNK_SIZE};
use crate::conductor_state::ConductorState;
use crate::generator_manager::DynGenerator;

/// Spawns a new generation task that processes a full batch of chunks in parallel.
///
/// This function is non-blocking to the caller (usually the Conductor) and hands off
/// the CPU-intensive work to a separate thread pool via `spawn_blocking`.
///
/// # Arguments
/// * `runtime_handle`: The handle to the Tokio runtime's executor.
/// * `generators_clone`: Map of all registered generators for selection.
/// * `chunk_cache_clone`: **(FIXED)** The thread-safe cache, shared via Arc. It relies on its
///   internal locking to allow high-speed concurrent read/write operations from Rayon workers.
/// * `active_generator_id`: Identifier of the generator to use for this batch.
/// * `progress_sender_clone`: Channel sender for sending completion/progress messages back to Conductor.
/// * `internal_state_clone`: A copy of the Conductor's shared state (used for queue depth tracking).
/// * `config_clone`: The configuration defining the area and size of the generation request.
pub fn spawn_batch_generation_task(
    runtime_handle: &Handle,
    generators_clone: HashMap<String, Arc<DynGenerator>>,
    // CRITICAL FIX: We changed the type from Arc<Mutex<ChunkCache>> to Arc<ChunkCache>.
    // This allows Rayon's parallel workers to use the cache's concurrent access 
    // mechanism (like RwLock or AtomicResource) instead of fighting over a single Mutex.
    chunk_cache_clone: Arc<ChunkCache>,
    active_generator_id: String,
    progress_sender_clone: Sender<GenerationMessage>,
    internal_state_clone: ConductorState,
    config_clone: GeneratorConfig,
) {
    info!("Conductor spawning BATCH generation task. Config: {}", config_clone);

    // Use `spawn_blocking` to move the synchronous, CPU-bound generation work off
    // of the async runtime's main thread pool, preventing executor starvation.
    runtime_handle.spawn_blocking(move || {
        // Increment the queue depth immediately to track the active task count.
        internal_state_clone.increment_queue_depth();

        // Calculate the grid size for the batch, rounding up to the nearest chunk boundary.
        let chunk_size_i64: i64 = CHUNK_SIZE as i64;
        
        // Ensure map dimensions are used as i64 for the calculation.
        let map_width_i64: i64 = config_clone.width as i64;
        let map_height_i64: i64 = config_clone.height as i64;

        // Calculate the chunk counts using the correct ceiling division formula: (a + b - 1) / b
        let width_in_chunks = (map_width_i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let height_in_chunks = (map_height_i64 + chunk_size_i64 - 1) / chunk_size_i64;

        // Generate a vector of all ChunkKey coordinates (Vec2i) for the entire batch area.
        // The range (0..N) correctly includes coordinates 0 up to N-1.
        let all_chunk_coords: Vec<Vec2i> = (0..width_in_chunks)
            .flat_map(|x| (0..height_in_chunks).map(move |y| Vec2i::new(x, y)))
            .collect();
        
        // Added check for 0x0 map request, although likely prevented by ConfigValidator.
        if all_chunk_coords.is_empty() {
             info!("Batch generation task received a map size of 0x0 chunks. Task finished immediately. Config: {}", config_clone);
        }


        // --- Core Parallel Processing ---
        all_chunk_coords
            // Convert the iterator to a parallel iterator using Rayon.
            .par_iter()
            // Process each chunk in parallel across all available CPU threads.
            .for_each(|&chunk_coords| {
                handle_chunk_unit(
                    chunk_coords,
                    &active_generator_id,
                    &generators_clone,
                    // The Arc reference is passed directly to the worker function.
                    &chunk_cache_clone, 
                    &progress_sender_clone,
                );
            });

        // Send a final message to the Conductor indicating the entire batch is complete.
        // `blocking_send` is used because this closure is running on a synchronous blocking thread.
        let _ = progress_sender_clone.blocking_send(GenerationMessage::GenerationComplete);

        // Decrement the queue depth, signaling to the Conductor that this task is done.
        internal_state_clone.decrement_queue_depth();

        info!("Batch generation task finished processing command: {}", config_clone);
    });
}