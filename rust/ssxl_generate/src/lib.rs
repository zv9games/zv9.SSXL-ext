use ssxl_shared::ChunkData;
use ssxl_math::Vec2i;
use tracing::{info, error};
use std::mem;

// --- Module Declarations (Based on Directory Structure) ---
// Modules that map to subdirectories (containing mod.rs or other files).
pub mod ca;        // Contains cellular_automata_generator.rs
pub mod conductor; // Contains conductor.rs, conductor_state.rs, sync.rs
pub mod manager;   // Contains config_validator.rs, generator_manager.rs, runtime_manager.rs
pub mod perlin;    // Contains perlin_generator.rs
pub mod task;      // Contains batch_processor.rs, benchmark_logic.rs, task_queue.rs

// The original declarations are removed:
// pub mod conductor;
// pub mod benchmark_logic;
// pub mod perlin_generator;
// pub mod cellular_automata_generator;
// pub mod ca; // This was correctly declared as a directory module
// pub mod runtime_manager;
// pub mod config_validator;
// pub mod task_queue;
// pub mod conductor_state;
// pub mod generator_manager;
// pub mod batch_processor;
// pub mod sync;


pub trait Generator {
	fn id(&self) -> &str;
	fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}


// --- Public Exports (Updated to use new module paths) ---

pub use ca::cellular_automata_generator::CellularAutomataGenerator;
pub use perlin::perlin_generator::PerlinGenerator;

// The Conductor is in conductor/conductor.rs
pub use conductor::conductor::Conductor;
// ConfigValidator is in manager/config_validator.rs
pub use manager::config_validator::GeneratorConfig;

// ConductorProgressReceiver/Sender are in conductor/sync.rs
pub use conductor::sync::ConductorProgressReceiver;
pub use conductor::sync::ConductorRequestSender;
// GenerationTask is in task/task_queue.rs
pub use task::task_queue::GenerationTask;

// BenchmarkLogic is in task/benchmark_logic.rs
pub use task::benchmark_logic::benchmark_generation_workload;


pub fn start_runtime_placeholder() {
	match Conductor::new(None) {
		// FIX: The progress_receiver must be kept alive to prevent the channel from closing.
		// Removed the underscore, making the variable `progress_receiver`.
		Ok((conductor, _state, _request_sender, progress_receiver)) => {
			info!("Runtime created successfully. Testing immediate graceful teardown...");
			
			// ARCHITECTURAL FIX: Use std::mem::forget to simulate the FFI/Godot side
			// taking ownership of the receiver, which keeps the progress channel open.
			mem::forget(progress_receiver);
			
			conductor.graceful_teardown();
		}
		Err(e) => {
			error!("Failed to initialize Conductor/Runtime: {:?}", e);
		}
	}
}