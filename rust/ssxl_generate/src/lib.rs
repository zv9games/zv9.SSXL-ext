//! Core generation algorithms, runtime orchestration, and task management.

// -------------------------------------------------------------------------------------------------
// MODULE EXPOSURE
// -------------------------------------------------------------------------------------------------
// Expose the Conductor (the validated Runtime/Orchestration core).
pub mod conductor;
pub mod benchmark_logic;
pub mod perlin_generator;
pub mod cellular_automata_generator;

// Direct re-exports of concrete implementations
pub use cellular_automata_generator::CellularAutomataGenerator;
pub use perlin_generator::PerlinGenerator;

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
// Re-export the main components for easy use by other crates (aetherion_godot, aetherion_cli).
pub use conductor::Conductor;
// PHASE 8.3 EXPORT: Expose GeneratorConfig for use by the FFI/Godot wrapper.
pub use conductor::GeneratorConfig;

// EXPOSED BENCHMARK FUNCTION: This resolves the E0432 error in aetherion_cli
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
        // FIX: Properly destructure the 3-element tuple (Conductor, ConductorState, Receiver).
        Ok((conductor, _state, _receiver)) => {
            info!("Runtime created successfully. Testing immediate graceful teardown...");
            
            // Call the consuming teardown method.
            conductor.graceful_teardown();
        }
        Err(e) => {
            error!("Failed to initialize Conductor/Runtime: {:?}", e);
        }
    }
}