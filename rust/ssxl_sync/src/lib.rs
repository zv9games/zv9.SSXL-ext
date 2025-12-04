// FILE: ssxl_sync/src/lib.rs

//! # SSXL Synchronization and Concurrency (`ssxl_sync`)
//! 
//! This crate defines core thread-safe primitives, thread pool management, 
//! and high-level FFI channel handles for communication across the engine's asynchronous boundary.

use tokio::sync::mpsc;
use std::ops::{Deref, DerefMut}; // Required for implementing the FFI Handle wrapper

// --------------------------------------------------------------------------------
// --- CRITICAL EXTERNAL IMPORTS ---
// --------------------------------------------------------------------------------

// CRITICAL: Import all core contract types from ssxl_shared.
use ssxl_shared::{
    SSXLError,
    AnimationUpdate,
    AnimationCommand,
};
// FIX: Import and public re-export the core struct in one step to avoid E0252/E0365.
pub use ssxl_animate::AnimationConductor;

// --------------------------------------------------------------------------------
// --- Internal Modules ---
// --------------------------------------------------------------------------------

/// Defines core thread-safe types, enums, and channel aliases used for concurrency and communication.
pub mod primitives;

/// Manages the generic, fixed-size thread pool used for synchronous, CPU-intensive tasks.
pub mod pool;

/// Manages the dedicated worker thread and state for calculating visual animation frames.
pub mod animation_conductor;


// --------------------------------------------------------------------------------
// --- Public Re-exports (Crate Facade) ---
// --------------------------------------------------------------------------------

// --- 1. Generic Primitives and Utility Functions (from primitives) ---

/// Re-exports of generic thread-safe resource wrappers and synchronous channels.
pub use primitives::{
    AtomicResource,
    create_unbounded_channel,
};

// --- 2. Thread Pool Manager Types (from pool_manager) ---

/// Re-exports of all public types related to the generic CPU worker thread pool.
pub use pool::{
    WorkerPool,
    GenerationTask,
    ConductorResult,
    Task,
    TaskResult
};

// --------------------------------------------------------------------------------
// --- Final FFI Wrappers and Type Aliases (CRATE ROOT API) ---
// --------------------------------------------------------------------------------

// The Receiver half of the command channel for the AnimationConductor.
pub type CommandReceiver = mpsc::UnboundedReceiver<AnimationCommand>;

// The UnboundedSender type is used to send updates (e.g., animation frames) back to the main thread.
pub type UpdateSender = mpsc::UnboundedSender<AnimationUpdate>;

// The standard result wrapper, using SSXLError.
pub type CommandResult = Result<(), SSXLError>;


/// The **Animation Conductor Handle**. 
/// 
/// **FIX:** Changed from a type alias to a public struct wrapping the channel sender. 
/// This allows us to implement the `::new()` constructor required by the FFI layer (`ssxl_godot`), 
/// resolving `error[E0599]`.
pub struct AnimationConductorHandle {
    inner: mpsc::UnboundedSender<AnimationCommand>,
}

impl AnimationConductorHandle {
    /// Public constructor required by the FFI layer (`ssxl_godot/api_initializers.rs`).
    /// Allows the FFI layer to wrap the raw channel sender into this opaque handle.
    pub fn new(inner: mpsc::UnboundedSender<AnimationCommand>) -> Self {
        AnimationConductorHandle { inner }
    }
    
    // FIX (E0599): Adds the method to access the inner sender, allowing the FFI layer
    // to call `.inner().clone()` and re-wrap the result in ssxl_godot/src/engine/init.rs.
    pub fn inner(&self) -> &mpsc::UnboundedSender<AnimationCommand> {
        &self.inner
    }
}

// Allows the struct to be used as if it were the underlying `mpsc::UnboundedSender`.
impl Deref for AnimationConductorHandle {
    type Target = mpsc::UnboundedSender<AnimationCommand>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Provides mutable access to the underlying sender.
impl DerefMut for AnimationConductorHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}