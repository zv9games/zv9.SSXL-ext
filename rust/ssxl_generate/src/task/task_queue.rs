// ssxl_generate/src/task/task_queue.rs

//! Manages the asynchronous request loop for chunk generation and the core logic
//! for handling a single chunk request (cache check, generation, and storage).

use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle; 
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::Arc;

use ssxl_math::Vec2i;

use ssxl_cache::ChunkCache;
use crate::Generator;

// FIX: Corrected import path for conductor_state module.
use crate::conductor::conductor_state; 

// Corrected imports from the ssxl_shared crate
use ssxl_shared::ChunkData; 
use ssxl_shared::CHUNK_SIZE as SHARED_CHUNK_SIZE; 
// FIX: Corrected import path to include the 'generation_message' submodule.
pub use ssxl_shared::message::generation_message::{GenerationMessage, GenerationTask}; 


type DynGenerator = Box<dyn Generator + Send + Sync>;

pub const CHUNK_SIZE: i64 = SHARED_CHUNK_SIZE as i64;


// --- 1. Single Chunk Processing Unit (FIXED for tile counting) ---

pub fn handle_chunk_unit(
	chunk_coords: Vec2i,
	generator_name: &str,
	generators: &HashMap<String, Arc<DynGenerator>>,
	_chunk_cache: &Arc<ChunkCache>,
	progress_sender: &mpsc::Sender<GenerationMessage>,
	// FIX: Use the corrected, fully qualified path
	conductor_state: &Arc<conductor_state::ConductorState>, 
) {
	let chunk_data: ChunkData;
	
	// Using non-blocking println! to bypass logger deadlock on the tokio::task::spawn_blocking thread.
	println!("DEBUG: Chunk Unit: Starting generation for chunk {:?} with {}.", chunk_coords, generator_name);
	
	let generator_arc = generators
		.get(generator_name)
		.expect("Generator ID must be registered in Conductor.");
	
	// Execute the generation (safe in sequential context)
	chunk_data = generator_arc.generate_chunk(chunk_coords);
	
	// FIX: Call .len() on the `tiles` field within ChunkData, not on ChunkData itself.
    let tile_count = chunk_data.tiles.len() as u64; 
	conductor_state.increment_tile_count(tile_count);
	
	let chunk_data_arc = Arc::new(chunk_data);
	
	// Log success/failure of sending the message.
	if let Err(e) = progress_sender.try_send(
        // FIX: Changed variant name from `ChunkGenerated` to `Generated`
		GenerationMessage::Generated(chunk_coords, chunk_data_arc)
	) {
		// Using eprintln! for critical debug information since tracing::warn! is blocked.
		eprintln!("CRITICAL WARN: Chunk Unit: FAILED to send ChunkGenerated message for {:?} (Channel full or disconnected): {:?}", chunk_coords, e);
	} else {
		println!("DEBUG: Chunk Unit: SUCCESSFULLY sent ChunkGenerated message for {:?}.", chunk_coords);
	}
}

// --- 2. Asynchronous Request Loop (FIXED for graceful shutdown and worker call) ---

pub fn start_request_loop(
	rt_handle: Handle,
	mut request_rx: mpsc::UnboundedReceiver<GenerationTask>,
	progress_tx: mpsc::Sender<GenerationMessage>,
	generators: Arc<HashMap<String, Arc<DynGenerator>>>,
	chunk_cache: Arc<ChunkCache>,
	// FIX: Use corrected, fully-qualified path for ConductorState
	conductor_state: Arc<conductor_state::ConductorState>,
) {
	rt_handle.spawn(async move {
		info!("Generation Task Queue active. Listening for requests.");
		
        // FIX: Collect handles of all spawned blocking tasks to ensure they finish before shutdown.
        let mut active_tasks: Vec<JoinHandle<()>> = Vec::new();
        
		while let Some(task) = request_rx.recv().await {
			if !conductor_state.as_ref().is_active() {
				warn!("Request received while Conductor is paused or shutting down. Dropping task: {:?}", task.chunk_coords);
				continue;
			}

			let progress_tx_clone = progress_tx.clone();
			let generators_clone = generators.clone();
			let chunk_cache_clone = chunk_cache.clone();
            // NEW: Clone ConductorState to move into the spawned blocking task.
            let conductor_state_clone = conductor_state.clone();

			// Spawn the blocking task for sequential chunk processing
			let handle = tokio::task::spawn_blocking(move || { // Capture the JoinHandle
				handle_chunk_unit(
					task.chunk_coords,
					&task.generator_id,
					&generators_clone,
					&chunk_cache_clone,
					&progress_tx_clone,
                    // FIX: Pass the ConductorState reference
                    &conductor_state_clone, 
				);
			});
            
            active_tasks.push(handle);
		}
		
		// Loop exited (sender was dropped), signifying engine shutdown.
		info!("Generation Task Queue shutting down. Waiting for {} in-flight tasks...", active_tasks.len());
		
        // FIX: Wait for all in-flight tasks to complete before signaling GenerationComplete.
        for handle in active_tasks {
            // Await each blocking task. This is the crucial step that prevents early receiver drop.
            if let Err(e) = handle.await {
                error!("A spawned blocking task panicked: {:?}", e);
            }
        }
        
        info!("All in-flight generation tasks completed.");
        
		// CRITICAL FIX: Ensure the final message is sent asynchronously inside the async block.
		if progress_tx.send(GenerationMessage::GenerationComplete).await.is_err() {
			error!("Failed to send final GenerationComplete message. Receiver may have already closed.");
		}
	});
}