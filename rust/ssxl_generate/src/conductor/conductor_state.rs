// ============================================================================
// 🎼 Conductor State Management (`crate::conductor::conductor_state`)
// ----------------------------------------------------------------------------
// This module defines the lifecycle state and shared state container for the
// Conductor, the central orchestrator of SSXL’s procedural generation system.
// It provides thread-safe, atomic access to conductor status, queue depth,
// active generator ID, and tile counters.
//
// Purpose:
//   • Represent the lifecycle phases of the Conductor (initializing, running,
//     generating, stopping, shutting down, error).
//   • Maintain shared state across async tasks and threads safely.
//   • Provide atomic counters and resources for concurrent mutation.
//   • Expose getters and mutators for monitoring and controlling conductor state.
//
// Key Components:
//   • ConductorStatus (enum)
//       - Represents lifecycle phases:
//           • Initializing: conductor is starting up.
//           • Running: actively processing tasks.
//           • Paused: temporarily halted.
//           • Generating: actively generating chunks.
//           • Stopping: graceful stop requested.
//           • ShuttingDown: final shutdown in progress.
//           • Error: fault state encountered.
//
//   • ConductorState (struct)
//       - Holds shared state with atomic safety:
//           • status: current lifecycle status.
//           • queue_depth: number of pending tasks.
//           • active_generator_id: ID of currently active generator.
//           • tile_counter: total number of tiles placed/generated.
//       - Designed for concurrent access across async tasks and threads.
//
//   • Implementation Methods
//       - new: constructs initial state with defaults.
//       - get_status: returns current lifecycle status.
//       - is_active: checks if conductor is in an active state (Running/Generating).
//       - get_queue_depth: returns number of tasks in queue.
//       - get_tiles_placed: returns total tiles placed/generated.
//       - get_active_generator_id: returns active generator ID.
//       - increment_queue_depth / decrement_queue_depth: adjust queue depth atomically.
//       - increment_tile_count: increase tile counter by specified amount.
//       - set_status: update lifecycle status.
//       - set_active_generator_id: update active generator ID.
//
// Workflow:
//   1. ConductorState is created with initial generator ID.
//   2. Async tasks increment/decrement queue depth as tasks are added/completed.
//   3. Tile counter tracks total tiles generated across tasks.
//   4. Status transitions reflect conductor lifecycle (e.g., Running → Stopping).
//   5. Active generator ID can be switched dynamically.
//
// Design Choices:
//   • AtomicResource provides safe concurrent read/write for complex types.
//   • Arc + AtomicUsize/AtomicU64 ensure thread-safe counters.
//   • Relaxed ordering is used for performance where strict ordering isn’t required.
//   • Clear separation of getters and mutators improves readability and control.
//
// Educational Note:
//   • This module demonstrates how to design thread-safe state containers in Rust,
//     combining atomic primitives with Arc for shared ownership.
//   • By encapsulating lifecycle and counters, it provides a robust foundation
//     for orchestrating async procedural generation workflows.
// ============================================================================


use std::sync::{Arc, atomic::{AtomicUsize, AtomicU64, Ordering}};
use ssxl_sync::AtomicResource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    Initializing,
    Running,
    Paused,
    Generating,
    Stopping,
    ShuttingDown,
    Error,
}

#[derive(Clone)]
pub struct ConductorState {
    status: AtomicResource<ConductorStatus>,
    queue_depth: Arc<AtomicUsize>,
    active_generator_id: AtomicResource<String>,
    tile_counter: Arc<AtomicU64>,
}

impl ConductorState {
    pub fn new(initial_generator_id: String) -> Self {
        ConductorState {
            status: AtomicResource::new(ConductorStatus::Initializing),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: AtomicResource::new(initial_generator_id),
            tile_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_status(&self) -> ConductorStatus {
        *self.status.read()
    }
    
    pub fn is_active(&self) -> bool {
        let current_status = self.get_status();
        current_status == ConductorStatus::Running || current_status == ConductorStatus::Generating
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    pub fn get_tiles_placed(&self) -> u64 {
        self.tile_counter.load(Ordering::Relaxed)
    }

    pub fn get_active_generator_id(&self) -> String {
        self.active_generator_id.read().clone()
    }

    pub fn increment_queue_depth(&self) {
        self.queue_depth.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_queue_depth(&self) {
        self.queue_depth.fetch_sub(1, Ordering::Relaxed);
    }
    
    pub(crate) fn increment_tile_count(&self, amount: u64) {
        self.tile_counter.fetch_add(amount, Ordering::Relaxed);
    }

    pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.write() = new_status;
    }

    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.write() = id.to_string();
    }
}
