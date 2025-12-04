use ssxl_shared::ChunkData;
use ssxl_math::prelude::Vec2i;
use tracing::{info, error};
use std::mem;

pub mod ca;
pub mod conductor;
pub mod manager;
pub mod perlin;
pub mod task;

pub trait Generator {
	fn id(&self) -> &str;
	fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}

pub use ca::cellular_automata_generator::CellularAutomataGenerator;
pub use perlin::perlin_generator::PerlinGenerator;

pub use conductor::conductor::Conductor;
pub use manager::config_validator::GeneratorConfig;

pub use conductor::sync::ConductorProgressReceiver;
pub use conductor::sync::ConductorRequestSender;
pub use task::task_queue::GenerationTask;

pub use task::benchmark_logic::benchmark_generation_workload;

pub fn start_runtime_placeholder() {
	match Conductor::new(None) {
		Ok((conductor, _state, _request_sender, progress_receiver)) => {
			info!("Runtime created successfully. Testing immediate graceful teardown...");
			
			mem::forget(progress_receiver);
			
			conductor.graceful_teardown();
		}
		Err(e) => {
			error!("Failed to initialize Conductor/Runtime: {:?}", e);
		}
	}
}