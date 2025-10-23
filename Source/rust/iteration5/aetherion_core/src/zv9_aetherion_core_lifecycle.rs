
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Engine has not been initialized.
    Uninitialized,

    /// Engine is performing startup routines.
    Initializing,

    /// Engine is actively running.
    Running,

    /// Engine is shutting down.
    ShuttingDown,

    /// Engine has completed termination.
    Terminated,
}

/// Tracks the lifecycle state and provides transition methods.
#[derive(Debug, Clone)]
pub struct Lifecycle {
    pub state: LifecycleState,
}

impl Lifecycle {
    /// Creates a new lifecycle in the uninitialized state.
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Uninitialized,
        }
    }

    /// Begins initialization.
    pub fn initialize(&mut self) {
        self.state = LifecycleState::Initializing;
        // Future: emit signal "engine_init_start"
    }

    /// Marks the engine as running.
    pub fn mark_running(&mut self) {
        self.state = LifecycleState::Running;
        // Future: emit signal "engine_initialized"
    }

    /// Begins shutdown.
    pub fn shutdown(&mut self) {
        self.state = LifecycleState::ShuttingDown;
        // Future: emit signal "engine_shutdown_start"
    }

    /// Finalizes termination.
    pub fn terminate(&mut self) {
        self.state = LifecycleState::Terminated;
        // Future: emit signal "engine_terminated"
    }

    /// Returns true if the engine is actively running.
    pub fn is_active(&self) -> bool {
        matches!(self.state, LifecycleState::Running)
    }

    /// Returns true if the engine is in the process of shutting down.
    pub fn is_shutting_down(&self) -> bool {
        matches!(self.state, LifecycleState::ShuttingDown)
    }

    /// Returns true if the engine has completed termination.
    pub fn is_terminated(&self) -> bool {
        matches!(self.state, LifecycleState::Terminated)
    }
}

// the end