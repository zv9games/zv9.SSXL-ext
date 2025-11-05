// ssxl_generate/src/task_queue.rs

//! The core manager for asynchronous generation tasks, chunk requests,
//! and the coordination of the blocking work pool.

use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Imports updated to support i64 (64-bit) coordinates
use glam::I64Vec3;
use ssxl_math::{Vec2i, prelude::ChunkKey};

// Internal Crate Dependencies
use ssxl_cache::ChunkCache;
use crate::Generator; // Generator is correctly at crate root (lib.rs)
use crate::conductor_state::ConductorState;

// External Crate Dependencies
use ssxl_shared::chunk_data::ChunkData;
// ✅ Imports fixed to resolve E0432 for ssxl_shared modules
pub use ssxl_shared::config::CHUNK_SIZE as SHARED_CHUNK_SIZE;
pub use ssxl_shared::generation_message::{GenerationMessage, GenerationTask};


// Define a type alias for the generator trait object
type DynGenerator = Box<dyn Generator + Send + Sync>;

// --- CONSTANTS (Moved from Conductor) ---
/// The size of a chunk in tiles.
// ✅ Made public to resolve E0603 in batch_processor.rs
pub const CHUNK_SIZE: i64 = SHARED_CHUNK_SIZE as i64; // Use the canonical size, cast to i64 for coordinate math

// -----------------------------------------------------------------------------
// ASYNCHRONOUS TASK QUEUE LOGIC
// -----------------------------------------------------------------------------

/// Encapsulates the core logic for loading from cache, generating, and saving a single chunk.
/// This function is thread-safe and blocking-pool safe.
// ✅ Made public to resolve E0432 in conductor.rs (via the missing start_request_loop)
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

    match chunk_cache.lock() {
        Ok(cache_lock) => { 
            match cache_lock.load_chunk(&chunk_key) {
                Ok(Some(data)) => {
                    info!("Chunk Unit: Retrieved chunk {:?} from cache (Gen: {}).", chunk_coords, generator_name);
                    chunk_data = data;
                },
                Ok(None) => {
                    info!("Chunk Unit: Chunk {:?} not found in cache. Generating with {}.", chunk_coords, generator_name);
                    let generator_arc = generators
                        .get(generator_name)
                        .expect("Generator ID must be registered in Conductor.");
                    
                    chunk_data = generator_arc.generate_chunk(chunk_coords);
                    
                    // Save to cache (requires mutable cache_lock)
                    if let Err(e) = cache_lock.save_chunk(&chunk_key, &chunk_data) {
                        error!("Chunk Unit: Failed to save chunk {:?} to cache: {:?}", chunk_coords, e);
                    } else {
                        info!("Chunk Unit: Saved chunk {:?} to cache.", chunk_coords);
                    }
                },
                Err(e) => {
                    warn!("Chunk Unit: Cache load failed for {:?}: {:?}. Falling back to generation without caching.", chunk_coords, e);
                    let generator_arc = generators
                        .get(generator_name)
                        .expect("Generator ID must be registered in Conductor.");
                    chunk_data = generator_arc.generate_chunk(chunk_coords);
                }
            }
        },
        Err(e) => {
            error!("Chunk Unit: Cache Mutex poisoned: {}. Falling back to generation without caching.", e);
            let generator_arc = generators
                .get(generator_name)
                .expect("Generator ID must be registered in Conductor.");
            chunk_data = generator_arc.generate_chunk(chunk_coords);
        }
    }
    
    // Emit signal with the zero-copy bulk data payload
    let chunk_data_arc = Arc::new(chunk_data);
    
    // 🛑 IMPROVEMENT: Add proper error handling for channel send.
    if let Err(e) = progress_sender.blocking_send(
        GenerationMessage::ChunkGenerated(chunk_coords, chunk_data_arc)
    ) {
        // If the main thread's receiver is dropped, this error occurs.
        error!("Chunk Unit: Failed to send ChunkGenerated message for {:?}: {:?}", chunk_coords, e);
    }
}


/// Starts the asynchronous request loop on the provided Tokio runtime handle.
/// This loop continuously processes incoming GenerationTask requests.
pub fn start_request_loop(
    rt_handle: Handle,
    request_rx: mpsc::UnboundedReceiver<GenerationTask>,
    progress_tx: mpsc::Sender<GenerationMessage>,
    generators: Arc<HashMap<String, Arc<DynGenerator>>>,
    chunk_cache: Arc<Mutex<ChunkCache>>,
    conductor_state: Arc<ConductorState>,
) {
    // Note: We move the request receiver into the async block, 
    // where it will be polled for new tasks.
    rt_handle.spawn(async move {
        info!("Generation Task Queue active.");

        let mut request_rx = request_rx;
        
        while let Some(task) = request_rx.recv().await {
            // ✅ FIX: Replaced non-existent `is_running()` with the new `is_active()` method from ConductorState.
            if !conductor_state.as_ref().is_active() { 
                warn!("Request received while Conductor is paused. Dropping task: {:?}", task.chunk_coords);
                continue;
            }

            // Spawn the blocking work unit onto the dedicated blocking thread pool
            // This prevents long-running generation tasks from blocking the Tokio executor.
            let progress_tx_clone = progress_tx.clone();
            let generators_clone = generators.clone();
            let chunk_cache_clone = chunk_cache.clone();

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
        
        // When the command sender is dropped and request_rx closes, we signal completion.
        info!("Generation Task Queue shutting down.");
        // FIX (E0599): Use the correct variant name: GenerationComplete
        if progress_tx.blocking_send(GenerationMessage::GenerationComplete).is_err() { 
            error!("Failed to send final GenerationComplete message.");
        }
    });
}