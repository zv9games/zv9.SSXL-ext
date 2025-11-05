// ssxl_generate/src/conductor_state.rs

//! Contains the thread-safe, shared state objects used by the Conductor for
//! monitoring and status reporting to external systems (like Godot FFI).

use tracing::error;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};

// -----------------------------------------------------------------------------
// CONDUCTOR STATUS ENUM
// -----------------------------------------------------------------------------

/// Represents the operational state of the Conductor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    Initializing,
    Running,
    Paused,
    // CRITICAL FIX: Add the missing 'Generating' state.
    Generating, 
    ShuttingDown,
    Error,
}

// -----------------------------------------------------------------------------
// CONDUCTOR STATE STRUCT
// -----------------------------------------------------------------------------

/// Shared, thread-safe state exposed to the FFI consumer (Godot) for monitoring.
#[derive(Clone)]
pub struct ConductorState {
    status: Arc<Mutex<ConductorStatus>>,
    queue_depth: Arc<AtomicUsize>,
    active_generator_id: Arc<Mutex<String>>,
}

impl ConductorState {
    pub fn new(initial_generator_id: String) -> Self {
        ConductorState {
            status: Arc::new(Mutex::new(ConductorStatus::Initializing)),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: Arc::new(Mutex::new(initial_generator_id)),
        }
    }

    // --- Public Getters (Read Access for FFI/CLI) ---

    pub fn get_status(&self) -> ConductorStatus {
        match self.status.lock() {
            Ok(guard) => *guard,
            Err(e) => {
                error!("Mutex poisoned when reading status: {}", e);
                ConductorStatus::Error
            }
        }
    }
    
    // ✅ FIX: Added the missing public method to check for active states.
    pub fn is_active(&self) -> bool {
        let current_status = self.get_status();
        current_status == ConductorStatus::Running || current_status == ConductorStatus::Generating
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    pub fn get_active_generator_id(&self) -> String {
        self.active_generator_id.lock().unwrap().clone()
    }

    // --- Public Mutators (Used by task_queue and batch_processor) ---

    /// Increments the current count of active generation tasks.
    pub fn increment_queue_depth(&self) {
        self.queue_depth.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrements the current count of active generation tasks.
    pub fn decrement_queue_depth(&self) {
        self.queue_depth.fetch_sub(1, Ordering::Relaxed);
    }

    // --- Internal Mutators (For Conductor-only state changes) ---

    pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.lock().unwrap() = new_status;
    }

    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.lock().unwrap() = id.to_string();
    }
}