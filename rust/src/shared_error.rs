// rust/SSXL-ext/src/shared_error.rs

use thiserror::Error; // Recommended for ergonomic error handling
use godot::prelude::Gd;
use std::fmt;

/// The canonical error type for all operations within the SSXL-ext GDExtension core.
// FIX: Added 'Clone' trait derivation to resolve the E0277 error in shared_message.rs.
#[derive(Debug, Error, Clone)] 
pub enum SSXLCoreError {
    // ---------------------- STATE & LIFECYCLE ERRORS -------------------------
    /// The global HostState singleton was not initialized (via host_init.rs).
    #[error("Core state uninitialized: HostState singleton is not yet set.")]
    UninitializedState,
    
    // FIX: Added missing InitializationError variant to resolve errors in src/host_state.rs
    /// Catch-all for state setup failures (like double initialization or invalid setup).
    #[error("Core initialization failed: {0}")]
    InitializationError(String),
    
    /// An operation was attempted while the conductor was already busy with a generation task.
    // FIX: Added missing ConductorBusy variant
    #[error("Conductor lifecycle error: Conductor is currently busy and cannot accept new jobs.")]
    ConductorBusy, 
    
    /// An operation was attempted while the conductor was in the wrong state (e.g., trying to start while paused).
    #[error("Conductor lifecycle error: System is currently in state '{0}'.")]
    InvalidConductorState(String),
    
    /// A required configuration value was missing or invalid.
    #[error("Configuration error: Invalid value for '{0}'.")]
    InvalidConfig(String),
    
    // ---------------------- CHANNEL & CONCURRENCY ERRORS ---------------------
    /// Failed to send a message to a channel (e.g., worker failed to send chunk).
    #[error("Channel sending failed: {0}")]
    ChannelSendError(String),
    /// Failed to receive a message from a channel (e.g., Conductor failed to receive task).
    #[error("Channel receiving failed: {0}")]
    ChannelRecvError(String),
    /// A worker thread panicked or failed to join during shutdown.
    #[error("Thread management error: Worker thread join failed.")]
    ThreadJoinError,

    // ---------------------- GENERATION & MATH ERRORS -------------------------
    /// A procedural generation step (Perlin, CA) produced invalid data.
    #[error("Generation data error: {0}")]
    GenerationDataError(String),
    /// A math operation resulted in an unexpected boundary condition (e.g., zero division, coordinate overflow).
    #[error("Mathematical boundary error: {0}")]
    MathError(String),
    
    // ---------------------- GODOT & FFI ERRORS -------------------------------
    /// An InstanceId failed to resolve to a valid Godot object (e.g., TileMap deleted).
    #[error("Godot instance error: ID '{0}' is invalid or null.")]
    InvalidInstance(u64),
    /// The direct memory write FFI operation failed due to invalid pointers or sizes.
    #[error("FFI Bridge error: Direct memory write failed: {0}")]
    FFIWriteError(String),
    /// Godot API call failed (e.g., failed to set a tile property).
    #[error("Godot API failure on '{0}': {1}")]
    GodotAPIFailure(String, String),
}

// rust/SSXL-ext/src/shared_error.rs

// Convert the standard Flume channel errors into our unified type.
impl<T> From<flume::SendError<T>> for SSXLCoreError where T: fmt::Debug {
    fn from(e: flume::SendError<T>) -> Self {
        SSXLCoreError::ChannelSendError(format!("{:?}", e))
    }
}

impl From<flume::RecvError> for SSXLCoreError {
    fn from(e: flume::RecvError) -> Self {
        SSXLCoreError::ChannelRecvError(e.to_string())
    }
}

// Convert the standard thread Join error (when a thread panics)
impl From<Box<dyn std::any::Any + Send>> for SSXLCoreError {
    fn from(_: Box<dyn std::any::Any + Send>) -> Self {
        SSXLCoreError::ThreadJoinError
    }
}