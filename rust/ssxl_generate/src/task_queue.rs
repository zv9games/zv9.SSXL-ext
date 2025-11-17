// ssxl_generate/src/task_queue.rs

//! Manages the asynchronous request loop for chunk generation and the core logic
//! for handling a single chunk request (cache check, generation, and storage).

use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::collections::HashMap;
// FIX: Removed Mutex import. Arc is sufficient, as ChunkCache handles its own locking.
use std::sync::Arc; 

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
    // CRITICAL FIX: Changed from &Arc<Mutex<ChunkCache>> to &Arc<ChunkCache>.
    // We rely on ChunkCache's internal AtomicResource for thread-safe concurrent access.
    chunk_cache: &Arc<ChunkCache>,
    progress_sender: &mpsc::Sender<GenerationMessage>,
) {
    let key_vec3 = I64Vec3::new(chunk_coords.x, chunk_coords.y, 0);
    let chunk_key = ChunkKey(key_vec3);
    let chunk_data: ChunkData;
    
    // We now interact directly with the ChunkCache which manages its internal lock (RwLock/AtomicResource).
    // This removes the redundant and bottlenecking Mutex::lock() call.

    // 1. Check Cache (Crypto-coded memory)
    match chunk_cache.load_chunk(&chunk_key) {
        Ok(Some(data)) => {
            info!("Chunk Unit: Retrieved chunk {:?} from cache (Gen: {}).", chunk_coords, generator_name);
            // 'data' is Arc<ChunkData>, 'chunk_data' is ChunkData.
            chunk_data = (*data).clone();
        },
        Ok(None) => {
            // 2. Generate if Cache Miss
            info!("Chunk Unit: Chunk {:?} not found in cache. Generating with {}.", chunk_coords, generator_name);
            let generator_arc = generators
                .get(generator_name)
                .expect("Generator ID must be registered in Conductor.");
            
            chunk_data = generator_arc.generate_chunk(chunk_coords);
            
            // 3. Save to Cache
            // ChunkCache::save_chunk expects Arc<ChunkData>.
            if let Err(e) = chunk_cache.save_chunk(&chunk_key, Arc::new(chunk_data.clone())) {
                error!("Chunk Unit: Failed to save chunk {:?} to cache: {:?}", chunk_coords, e);
            } else {
                info!("Chunk Unit: Saved chunk {:?} to cache.", chunk_coords);
            }
        },
        Err(e) => {
            // Cache system error (e.g., I/O or internal AtomicResource error), generate without caching.
            warn!("Chunk Unit: Cache access failed for {:?}: {:?}. Falling back to generation without caching.", chunk_coords, e);
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
    // FIX: Changed from Arc<Mutex<ChunkCache>> to Arc<ChunkCache> to align with Conductor/batch_processor.
    chunk_cache: Arc<ChunkCache>,
    conductor_state: Arc<ConductorState>,
) {
    rt_handle.spawn(async move {
        info!("Generation Task Queue active. Listening for requests.");
        
        // Loop while the sender side of the channel is open.
        while let Some(task) = request_rx.recv().await {
            // Check Conductor State (Pause/Shutdown)
            if !conductor_state.as_ref().is_active() { 
                warn!("Request received while Conductor is paused or shutting down. Dropping task: {:?}", task.chunk_coords);
                // Note: Task queue depth management (if any) is assumed to happen externally or based on the drop.
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
                    &chunk_cache_clone, // Now the correct Arc<ChunkCache> type
                    &progress_tx_clone,
                );
            });
        }
        
        // Loop exited (sender was dropped), signifying engine shutdown.
        info!("Generation Task Queue shutting down.");
        // FIX: Ensure the final message is sent asynchronously.
        if progress_tx.send(GenerationMessage::GenerationComplete).await.is_err() { 
            error!("Failed to send final GenerationComplete message.");
        }
    });
}