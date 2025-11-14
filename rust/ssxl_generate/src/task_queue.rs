// ssxl_generate/src/task_queue.rs

//! Manages the asynchronous request loop for chunk generation and the core logic
//! for handling a single chunk request (cache check, generation, and storage).

use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use glam::I64Vec3;
use ssxl_math::{Vec2i, prelude::ChunkKey};

use ssxl_cache::ChunkCache;
use crate::Generator;
use crate::conductor_state::ConductorState;

use ssxl_shared::chunk_data::ChunkData;
// Import the canonical size from ssxl_shared and cast it locally for coordinate math.
use ssxl_shared::config::CHUNK_SIZE as SHARED_CHUNK_SIZE;
pub use ssxl_shared::generation_message::{GenerationMessage, GenerationTask};


/// Type alias for a thread-safe, dynamically dispatched Generator trait object.
type DynGenerator = Box<dyn Generator + Send + Sync>;

/// Canonical chunk size in i64 format for coordinate math.
pub const CHUNK_SIZE: i64 = SHARED_CHUNK_SIZE as i64;


// --- 1. Single Chunk Processing Unit ---

/// The atomic unit of work for chunk generation.
///
/// This synchronous function performs the cache lookup, calls the generator if needed,
/// saves the result to the cache, and sends a completion message.
///
/// **Crucially, this must be run on a blocking thread pool (e.g., via `spawn_blocking`)
/// because generator functions are CPU-intensive and synchronous.**
pub fn handle_chunk_unit(
    chunk_coords: Vec2i,
    generator_name: &str,
    generators: &HashMap<String, Arc<DynGenerator>>,
    chunk_cache: &Arc<Mutex<ChunkCache>>,
    progress_sender: &mpsc::Sender<GenerationMessage>,
) {
    let key_vec3 = I64Vec3::new(chunk_coords.x, chunk_coords.y, 0);
    let chunk_key = ChunkKey(key_vec3);
    let chunk_data: ChunkData;

    // Attempt to acquire the lock for the chunk cache
    match chunk_cache.lock() {
        Ok(cache_lock) => { // <-- REMOVED `mut` HERE
            // 1. Check Cache (Crypto-coded memory)
            match cache_lock.load_chunk(&chunk_key) {
                Ok(Some(data)) => {
                    info!("Chunk Unit: Retrieved chunk {:?} from cache (Gen: {}).", chunk_coords, generator_name);
                    chunk_data = data;
                },
                Ok(None) => {
                    // 2. Generate if Cache Miss
                    info!("Chunk Unit: Chunk {:?} not found in cache. Generating with {}.", chunk_coords, generator_name);
                    let generator_arc = generators
                        .get(generator_name)
                        .expect("Generator ID must be registered in Conductor.");
                    
                    chunk_data = generator_arc.generate_chunk(chunk_coords);
                    
                    // 3. Save to Cache
                    if let Err(e) = cache_lock.save_chunk(&chunk_key, &chunk_data) {
                        error!("Chunk Unit: Failed to save chunk {:?} to cache: {:?}", chunk_coords, e);
                    } else {
                        info!("Chunk Unit: Saved chunk {:?} to cache.", chunk_coords);
                    }
                },
                Err(e) => {
                    // Cache system error, generate without caching
                    warn!("Chunk Unit: Cache load failed for {:?}: {:?}. Falling back to generation without caching.", chunk_coords, e);
                    let generator_arc = generators
                        .get(generator_name)
                        .expect("Generator ID must be registered in Conductor.");
                    chunk_data = generator_arc.generate_chunk(chunk_coords);
                }
            }
        },
        Err(e) => {
            // Cache Mutex Poisoned: Indicates a critical, unrecoverable state. Generate without caching.
            error!("Chunk Unit: Cache Mutex poisoned: {}. Falling back to generation without caching.", e);
            let generator_arc = generators
                .get(generator_name)
                .expect("Generator ID must be registered in Conductor.");
            chunk_data = generator_arc.generate_chunk(chunk_coords);
        }
    }
    
    // 4. Send Completion/Progress Message
    let chunk_data_arc = Arc::new(chunk_data);
    
    // Use blocking_send as this function is already running on a blocking thread.
    if let Err(e) = progress_sender.blocking_send(
        GenerationMessage::ChunkGenerated(chunk_coords, chunk_data_arc)
    ) {
        error!("Chunk Unit: Failed to send ChunkGenerated message for {:?}: {:?}", chunk_coords, e);
    }
}


// --- 2. Asynchronous Request Loop ---

/// Spawns the main request processing loop onto the Tokio runtime handle.
///
/// This loop listens for new generation tasks and offloads the synchronous work
/// to the dedicated `tokio::task::spawn_blocking` thread pool, maintaining the
/// asynchronous core's **tempo**.
pub fn start_request_loop(
    rt_handle: Handle,
    mut request_rx: mpsc::UnboundedReceiver<GenerationTask>,
    progress_tx: mpsc::Sender<GenerationMessage>,
    generators: Arc<HashMap<String, Arc<DynGenerator>>>,
    chunk_cache: Arc<Mutex<ChunkCache>>,
    conductor_state: Arc<ConductorState>,
) {
    rt_handle.spawn(async move {
        info!("Generation Task Queue active. Listening for requests.");
        
        // Loop while the sender side of the channel is open.
        while let Some(task) = request_rx.recv().await {
            // Check Conductor State (Pause/Shutdown)
            if !conductor_state.as_ref().is_active() { 
                warn!("Request received while Conductor is paused or shutting down. Dropping task: {:?}", task.chunk_coords);
                // Decrement queue depth is omitted here as it's assumed to be handled by the caller/request loop logic elsewhere.
                continue;
            }

            // Prepare thread-safe clones for the blocking task
            let progress_tx_clone = progress_tx.clone();
            let generators_clone = generators.clone();
            let chunk_cache_clone = chunk_cache.clone();

            // Offload CPU-intensive work to the dedicated blocking thread pool.
            tokio::task::spawn_blocking(move || {
                handle_chunk_unit(
                    task.chunk_coords,
                    &task.generator_id,
                    &generators_clone,
                    &chunk_cache_clone,
                    &progress_tx_clone,
                );
            });
        }
        
        // Loop exited (sender was dropped), signifying engine shutdown.
        info!("Generation Task Queue shutting down.");
        // FIX: Replaced illegal blocking call with the correct asynchronous send.
        if progress_tx.send(GenerationMessage::GenerationComplete).await.is_err() { 
            error!("Failed to send final GenerationComplete message.");
        }
    });
}