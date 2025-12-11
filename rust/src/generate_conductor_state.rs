// rust/SSXL-ext/src/generate_conductor_state.rs

use std::fmt;
use std::sync::{Mutex, Arc};
// --- FIX: Import logging macros from the crate root ---
use crate::ssxl_info;

// --------------------------------------------------------------------------
// --- ConductorState Enum ---
// --------------------------------------------------------------------------

/// Defines the current operational status of the Generation Conductor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConductorState {
    /// Initial state, awaiting a command to start. (Previously Idle)
    Ready, // <<< FIX: Renamed 'Idle' to 'Ready' to satisfy generate_conductor.rs
    /// The worker pool is actively generating chunks.
    Generating,
    /// Generation is complete, but the final cleanup/signals are pending.
    Finished,
    /// An unrecoverable error occurred (e.g., worker panic, channel failure).
    Error,
    /// Generation has been manually paused or terminated.
    Paused,
}

impl fmt::Display for ConductorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// --------------------------------------------------------------------------
// --- GenerationMetrics Struct ---
// --------------------------------------------------------------------------

/// Holds the volatile metrics of the current generation job.
#[derive(Debug, Clone, Copy)]
pub struct GenerationMetrics {
    /// The total number of chunks requested for generation.
    pub total_chunks: u32,
    /// The number of chunks successfully completed and rendered.
    pub completed_chunks: u32,
    /// The number of jobs that failed during processing.
    pub failed_jobs: u32,
    /// The instantaneous speed of processing (e.g., chunks per second).
    pub current_throughput: f32, 
}

impl Default for GenerationMetrics {
    fn default() -> Self {
        GenerationMetrics {
            total_chunks: 0,
            completed_chunks: 0,
            failed_jobs: 0,
            current_throughput: 0.0,
        }
    }
}

// --------------------------------------------------------------------------
// --- ConductorStateContainer Struct (Thread-Safe Wrapper) ---
// --------------------------------------------------------------------------

/// A thread-safe container holding the Conductor's current state and metrics.
/// This is typically wrapped in an Arc<T> for shared, concurrent access.
pub struct ConductorStateContainer {
    // The current state (requires Mutex protection).
    state: Mutex<ConductorState>,
    // The current metrics (requires Mutex protection).
    metrics: Mutex<GenerationMetrics>,
}

impl ConductorStateContainer {
    pub fn new() -> Self {
        ConductorStateContainer {
            // FIX: Initialize to the 'Ready' state
            state: Mutex::new(ConductorState::Ready),
            metrics: Mutex::new(GenerationMetrics::default()),
        }
    }

    /// Safely updates the state, logging the transition. (Main thread only)
    pub fn transition_to(&self, new_state: ConductorState) {
        let mut state = self.state.lock().unwrap();
        if *state != new_state {
            ssxl_info!("Conductor State Transition: {} -> {}", *state, new_state);
            *state = new_state;
        }
    }

    /// Safely reads the current state.
    pub fn get_state(&self) -> ConductorState {
        *self.state.lock().unwrap()
    }

    /// Safely reads the current metrics.
    pub fn get_metrics(&self) -> GenerationMetrics {
        *self.metrics.lock().unwrap()
    }

    /// Atomic update: Sets the total number of chunks to be generated.
    pub fn set_total_chunks(&self, count: u32) {
        self.metrics.lock().unwrap().total_chunks = count;
    }

    /// Atomic update: Increments the count of completed chunks.
    pub fn increment_completed_chunks(&self) {
        self.metrics.lock().unwrap().completed_chunks += 1;
        // Logic for throughput calculation would also be here
    }

    /// Atomic update: Increments the count of failed jobs.
    pub fn increment_failed_jobs(&self) {
        self.metrics.lock().unwrap().failed_jobs += 1;
    }
}