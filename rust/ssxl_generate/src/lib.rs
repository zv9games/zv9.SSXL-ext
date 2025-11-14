// ssxl_generate/src/lib.rs

//! The core library crate for the SSXL procedural generation engine.
//!
//! This crate orchestrates the asynchronous runtime, manages generator algorithms,
//! and provides the central Conductor API for world creation.

use ssxl_shared::chunk_data::ChunkData;
use ssxl_math::Vec2i;
use tracing::{info, error};

// --- 1. Internal Module Definitions ---

/// Core command and control center for the generation engine.
pub mod conductor;
/// Logic for stress-testing and profiling generator algorithms.
pub mod benchmark_logic;
/// Perlin noise based generator implementation.
pub mod perlin_generator;
/// Cellular Automata based generator implementation.
pub mod cellular_automata_generator;
/// Module containing Cellular Automata specific rules and utilities.
pub mod ca; 
/// Manages the Tokio asynchronous runtime.
pub mod runtime_manager; 
/// Utilities for validating generation request configurations.
pub mod config_validator;
/// Manages the asynchronous chunk generation task queue.
pub mod task_queue;
/// Thread-safe state tracking for the Conductor.
pub mod conductor_state;
/// Registry for all available generation algorithms.
pub mod generator_manager;
/// Logic for processing large batch generation requests (**Bulldozer** operation).
pub mod batch_processor;
/// Module for synchronous types used for external communication (Senders/Receivers).
pub mod sync; 


// --- 2. Core Generator Trait Definition ---

/// The fundamental contract for all world generation algorithms.
/// (Duplicated here from `generator.rs` for convenience and to define the public contract).
pub trait Generator {
    /// Returns a unique, static string identifier for this generator.
    fn id(&self) -> &str;

    /// Executes the deterministic generation algorithm for a single chunk.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}


// --- 3. Public API Exports (Re-exports for external use) ---

// Concrete Generator implementations
pub use cellular_automata_generator::CellularAutomataGenerator;
pub use perlin_generator::PerlinGenerator;

// Conductor and Configuration Types
pub use conductor::Conductor;
pub use config_validator::GeneratorConfig; 

// Synchronization and Task Types (For FFI and API integration)
pub use sync::ConductorProgressReceiver;
pub use sync::ConductorRequestSender;
pub use task_queue::GenerationTask; 

// Benchmark Utility
pub use benchmark_logic::benchmark_generation_workload;


// --- 4. Initialization Placeholder (Example/Debug) ---

/// A placeholder function to test Conductor initialization and immediate graceful shutdown.
/// This simulates a quick external call to verify runtime setup and teardown.
pub fn start_runtime_placeholder() {
    match Conductor::new(None) {
        Ok((conductor, _state, _progress_receiver, _request_sender)) => {
            info!("Runtime created successfully. Testing immediate graceful teardown...");
            
            // Execute the cleanup logic
            conductor.graceful_teardown();
        }
        Err(e) => {
            error!("Failed to initialize Conductor/Runtime: {:?}", e);
        }
    }
}