// ============================================================================
// 🎼 SSXL Engine Crate Root (`lib.rs`)
// ----------------------------------------------------------------------------
// This file defines the core structure of the SSXL engine crate. It organizes
// all subsystems (generators, conductor, manager, tasks) and exposes a clean,
// unified public API for external use.
//
// Purpose:
//   • Serve as the entry point for the SSXL engine library.
//   • Define the `Generator` trait, the contract for all procedural generators.
//   • Organize submodules for different generation strategies and orchestration.
//   • Re-export key components to simplify external imports.
//   • Provide a placeholder runtime entry point for testing initialization.
//
// Submodules:
//   • ca
//       - Implements cellular automata generators.
//       - Provides rule-based cave/maze generation algorithms.
//   • conductor
//       - Orchestrates generator execution, task queue, and runtime lifecycle.
//       - Tracks conductor state and manages async task flow.
//   • manager
//       - Validates generator configuration (map dimensions, chunk limits).
//       - Maintains registry of available generators.
//   • perlin
//       - Implements Perlin noise generator for smooth terrain generation.
//   • task
//       - Provides task queue, batch processor, and benchmarking utilities.
//
// Trait:
//   • Generator
//       - Contract for all procedural generation algorithms.
//       - Methods:
//           • id(): returns unique identifier string.
//           • generate_chunk(): generates a chunk of terrain at given coordinates.
//
// Re-exports:
//   • CellularAutomataGenerator, PerlinGenerator
//       - Concrete generator implementations exposed at crate root.
//   • Conductor
//       - Orchestrator for runtime and task execution.
//   • GeneratorConfig
//       - Configuration validator for generator settings.
//   • ConductorProgressReceiver, ConductorRequestSender
//       - Sync interfaces for conductor communication.
//   • GenerationTask
//       - Represents a single chunk generation request.
//   • benchmark_generation_workload
//       - Utility for simulating heavy workloads.
//
// Function:
//   • start_runtime_placeholder
//       - Demonstrates runtime initialization and immediate teardown.
//       - Workflow:
//           1. Attempt to create a new Conductor with no configuration.
//           2. If successful:
//                • Log runtime creation.
//                • Forget progress_receiver to avoid drop side effects.
//                • Call graceful_teardown() on Conductor.
//           3. If failed:
//                • Log error with failure details.
//
// Design Choices:
//   • Modular organization ensures separation of concerns.
//   • Re-exports flatten hierarchy for ergonomic external use.
//   • Placeholder runtime provides a safe test harness for initialization logic.
//
// Educational Note:
//   • This file demonstrates how Rust crates can be structured to balance
//     modularity and usability. By centralizing traits, modules, and re-exports,
//     SSXL provides a clean API surface while maintaining internal organization.
// ============================================================================


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
