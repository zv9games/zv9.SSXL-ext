// ssxl_generate/src/conductor_state.rs

//! Defines the ConductorState, which is the thread-safe, shared state of the SSXL engine.
//!
//! This structure provides real-time status, queue depth, and active generator tracking
//! for all components, functioning as the **crypto-coded memory** for the Conductor.

use tracing::error;
// Added AtomicU64 to support u64 return type for tile count
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};

// --- 1. Conductor Status Enum ---

/// Enumerates the operational states of the SSXL Conductor runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    /// The runtime is starting up and initializing core components.
    Initializing,
    /// The runtime is fully operational and waiting for requests.
    Running,
    /// Request processing and generation are temporarily halted.
    Paused,
    /// A large batch generation task is actively running (**Bulldozer** mode).
    Generating,
    /// A graceful shutdown has been initiated.
    ShuttingDown,
    /// A critical, non-recoverable error has occurred.
    Error,
}

// --- 2. Conductor State Structure ---

/// Thread-safe structure holding the shared state for the Conductor and all worker threads.
#[derive(Clone)]
pub struct ConductorState {
    /// The current operational status. Protected by a Mutex as updates are infrequent.
    status: Arc<Mutex<ConductorStatus>>,
    /// The number of pending chunk generation requests. Uses AtomicUsize for efficient, lock-free updates.
    queue_depth: Arc<AtomicUsize>,
    /// The ID string of the currently selected generator. Protected by a Mutex.
    active_generator_id: Arc<Mutex<String>>,
    /// ⭐ **FIX:** The total number of tiles successfully placed during the current generation run.
    tile_counter: Arc<AtomicU64>,
}
#[allow(dead_code)]
impl ConductorState {
    /// Creates a new state instance, initialized to `Initializing`.
    pub fn new(initial_generator_id: String) -> Self {
        ConductorState {
            status: Arc::new(Mutex::new(ConductorStatus::Initializing)),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: Arc::new(Mutex::new(initial_generator_id)),
            tile_counter: Arc::new(AtomicU64::new(0)), // Initialize the new counter
        }
    }

    /// Retrieves the current operational status of the Conductor.
    /// Logs an error and returns `ConductorStatus::Error` if the mutex is poisoned.
    pub fn get_status(&self) -> ConductorStatus {
        match self.status.lock() {
            Ok(guard) => *guard,
            Err(e) => {
                error!("Mutex poisoned when reading status: {}", e);
                ConductorStatus::Error
            }
        }
    }
    
    /// Returns true if the Conductor is in a state where it can actively process or accept tasks.
    pub fn is_active(&self) -> bool {
        let current_status = self.get_status();
        current_status == ConductorStatus::Running || current_status == ConductorStatus::Generating
    }

    /// Retrieves the current depth (number of pending requests) of the task queue.
    /// Uses relaxed memory ordering for maximum **tempo**, as strict ordering isn't critical.
    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    /// ⭐ **FIX:** Retrieves the total number of tiles successfully placed.
    /// This resolves the need for `tile_count` access in the consumer crate.
    pub fn get_tiles_placed(&self) -> u64 {
        self.tile_counter.load(Ordering::Relaxed)
    }

    /// Retrieves the ID of the currently selected generator.
    pub fn get_active_generator_id(&self) -> String {
        // Using unwrap() assumes the Mutex will not be poisoned under normal engine operation.
        self.active_generator_id.lock().unwrap().clone()
    }

    /// Atomically increases the queue depth count. Used when a new task is posted.
    pub fn increment_queue_depth(&self) {
        self.queue_depth.fetch_add(1, Ordering::Relaxed);
    }

    /// Atomically decreases the queue depth count. Used after a task is completed.
    pub fn decrement_queue_depth(&self) {
        // Note: fetch_sub handles underflow if used incorrectly, but should typically remain >= 0.
        self.queue_depth.fetch_sub(1, Ordering::Relaxed);
    }
    
    /// ⭐ **FIX:** Atomically increases the tile count by the specified amount.
    /// Used by worker threads after placing a batch of tiles.
    pub(crate) fn increment_tile_count(&self, amount: u64) {
        self.tile_counter.fetch_add(amount, Ordering::Relaxed);
    }

    /// Updates the operational status. Only visible within the `ssxl_generate` crate.
    pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.lock().unwrap() = new_status;
    }

    /// Updates the active generator ID string. Only visible within the `ssxl_generate` crate.
    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.lock().unwrap() = id.to_string();
    }
}