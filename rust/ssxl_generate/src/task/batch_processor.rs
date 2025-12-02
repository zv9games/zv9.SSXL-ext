// ssxl_generate/src/task/batch_processor.rs

//! Logic for executing large, synchronous batches of chunk generation requests.

use tokio::runtime::Handle;
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
// FIX: Removed use std::panic::{self, AssertUnwindSafe, catch_unwind};

use ssxl_math::Vec2i;
use ssxl_cache::ChunkCache;

// FIX: Corrected import path for GeneratorConfig.
use crate::manager::config_validator::GeneratorConfig;
// FIX: Corrected import paths for ConductorState and DynGenerator.
use crate::conductor::conductor_state::ConductorState;
use crate::manager::generator_manager::DynGenerator;
// FIX: The path to task_queue must now go through the 'task' module.
use crate::task::task_queue::{handle_chunk_unit, GenerationMessage};
use ssxl_shared::CHUNK_SIZE;


/// Spawns a new generation task that processes a full batch of chunks *sequentially* within a 
/// dedicated thread to ensure stability with non-thread-safe generators/caches.
pub fn spawn_batch_generation_task(
	runtime_handle: &Handle,
	generators_clone: HashMap<String, Arc<DynGenerator>>,
	chunk_cache_clone: Arc<ChunkCache>,
	active_generator_id: String,
	progress_sender_clone: Sender<GenerationMessage>,
	// This argument is correct for use in this function, but not the inner call.
	internal_state_clone: Arc<ConductorState>,
	config_clone: GeneratorConfig,
) {
	info!("Conductor spawning SEQUENTIAL BATCH generation task. Config: {}", config_clone);

	runtime_handle.spawn_blocking(move || {
		// Increment the queue depth immediately to track the active task count.
		internal_state_clone.increment_queue_depth();

		// Calculation logic remains the same
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

		// --- Core SEQUENTIAL Processing ---
		let active_generator_id_ref = &active_generator_id;
		
		for &chunk_coords in all_chunk_coords.iter() {
			
			handle_chunk_unit(
				chunk_coords,
				active_generator_id_ref,
				&generators_clone,
				&chunk_cache_clone,
				&progress_sender_clone,
				// The Arc<ConductorState> is passed correctly here.
				&internal_state_clone,
			);
		}

		// Send final completion message.
		if progress_sender_clone.try_send(GenerationMessage::GenerationComplete).is_err() {
			warn!("Batch completion signal dropped (Channel full).");
		}

		// Decrement the queue depth.
		internal_state_clone.decrement_queue_depth();

		info!("Batch generation task finished processing command: {}", config_clone);
	});
}