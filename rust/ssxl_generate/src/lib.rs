// ssxl_generate/src/lib.rs

//! Core generation algorithms, runtime orchestration, and task management.

// -------------------------------------------------------------------------------------------------
// MODULE EXPOSURE
// -------------------------------------------------------------------------------------------------
// Expose the Conductor (the validated Runtime/Orchestration core).
pub mod conductor;
pub mod benchmark_logic;
pub mod perlin_generator;
pub mod cellular_automata_generator;
pub mod ca; 

// Core orchestration modules required by Conductor.
pub mod runtime_manager; 
pub mod config_validator;
pub mod task_queue;
pub mod conductor_state;
pub mod generator_manager;
pub mod batch_processor;

// FIX: Add the synchronization module required for channel type aliases.
pub mod sync; 


// -------------------------------------------------------------------------------------------------
// CORE TRAIT DEFINITION (Generator Interface)
// -------------------------------------------------------------------------------------------------
use ssxl_shared::chunk_data::ChunkData;
use ssxl_math::Vec2i;

/// Defines the core contract for all procedural generation algorithms.
/// Every generator (Perlin, CA, DiamondSquare, etc.) must implement this trait.
pub trait Generator {
    /// The unique identifier for this specific algorithm (e.g., "perlin_2d_v1").
    fn id(&self) -> &str;

    /// Generates the content for a single Chunk.
    /// It takes a Vec2i which is the world-space coordinate of the chunk.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}


// -------------------------------------------------------------------------------------------------
// PUBLIC EXPORTS
// -------------------------------------------------------------------------------------------------

// Direct re-exports of concrete implementations
pub use cellular_automata_generator::CellularAutomataGenerator;
pub use perlin_generator::PerlinGenerator;

// Main components for FFI/Godot use.
pub use conductor::Conductor;
pub use config_validator::GeneratorConfig; 

// FIX: Re-export the channel types and core Task struct from the `sync` and `task_queue` modules.
pub use sync::ConductorProgressReceiver;
pub use sync::ConductorRequestSender;
pub use task_queue::GenerationTask; // Key type sent to the Conductor

// EXPOSED BENCHMARK FUNCTION
pub use benchmark_logic::benchmark_generation_workload;

// -------------------------------------------------------------------------------------------------
// PUBLIC API FOR CLI/FFI (Validation Entry Points)
// -------------------------------------------------------------------------------------------------
use tracing::{info, error};

/// Starts the ssxl Runtime, creating and immediately shutting down the Conductor.
///
/// NOTE: This is the **structural validation test for CLI Menu [4]** (Start Runtime).
pub fn start_runtime_placeholder() {
    // Pass None as the config_path argument to satisfy the updated Conductor::new signature.
    match Conductor::new(None) {
        // Correctly destructure the 4-element tuple
        Ok((conductor, _state, _progress_receiver, _request_sender)) => {
            info!("Runtime created successfully. Testing immediate graceful teardown...");
            
            // Call the consuming teardown method.
            conductor.graceful_teardown();
        }
        Err(e) => {
            error!("Failed to initialize Conductor/Runtime: {:?}", e);
        }
    }
}