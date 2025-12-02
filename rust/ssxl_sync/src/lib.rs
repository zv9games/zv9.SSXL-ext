// ssxl_sync/src/lib.rs

//! # SSXL Synchronization and Concurrency (`ssxl_sync`)
//!
//! This crate provides the core primitives and manager structs necessary for
//! thread synchronization, communication, and high-performance worker management
//! within the SSXL-ext engine. This includes:
//! 1. **Data Safety:** Atomic wrappers for thread-safe shared resources.
//! 2. **Worker Management:** Thread pools and specialized, dedicated worker threads (Conductors).
//! 3. **Communication:** Channel definitions for command and result passing.

// --------------------------------------------------------------------------------
// --- CRITICAL EXTERNAL IMPORTS ---
// --------------------------------------------------------------------------------

// CRITICAL: Import tokio for the channel types used in the aliases below.
use tokio::sync::mpsc; 
// CRITICAL: Import all core contract types from ssxl_shared.
use ssxl_shared::{
    SSXLError, 
    AnimationUpdate,
    AnimationCommand, 
    // FIX: Removed unused import AnimationState
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

/// Re-exports of generic thread-safe resource wrappers, synchronous channels, and worker placeholders.
pub use primitives::{
    AtomicResource, 
    create_sync_channel, 
    start_sync_worker
};

// --- 2. Animation Conductor Types ---

// FIX: Removed the unused `pub use animation_conductor::{...};` block.
// The primary conductor struct is handled by the `pub use ssxl_animate::AnimationConductor` above.


// --- 3. Thread Pool Manager Types (from pool_manager) ---

/// Re-exports of all public types related to the generic CPU worker thread pool.
pub use pool::{
    WorkerPool, 
    GenerationTask, 
    ConductorResult, 
    Task, 
    TaskResult
};

// --------------------------------------------------------------------------------
// --- Final Type Aliases (CRATE ROOT API) ---
// --------------------------------------------------------------------------------

// The UnboundedSender type is used to send commands to the Conductor's thread.
pub type AnimationConductorHandle = mpsc::UnboundedSender<AnimationCommand>;

// The Receiver half of the command channel for the AnimationConductor.
pub type CommandReceiver = mpsc::UnboundedReceiver<AnimationCommand>;

// The UnboundedSender type is used to send updates (e.g., animation frames) back to the main thread.
pub type UpdateSender = mpsc::UnboundedSender<AnimationUpdate>; 

// The standard result wrapper, using SSXLError.
pub type CommandResult = Result<(), SSXLError>;