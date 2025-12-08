//! ============================================================================
//! ⚡ SSXL Synchronization and Concurrency (`ssxl_sync`)
//! ----------------------------------------------------------------------------
//! This crate provides the concurrency backbone of the SSXL engine.
//! It defines thread-safe primitives, manages worker pools, and exposes
//! high-level FFI channel handles for communication across asynchronous
//! boundaries (e.g., between Rust workers and Godot).
//!
//! Educational notes:
//!   • Synchronization: ensures safe sharing of resources across threads.
//!   • Concurrency: organizes work into pools and tasks for efficient parallelism.
//!   • FFI channels: provide a standardized way to send/receive messages
//!     between Rust and external runtimes.
//!
//! By centralizing these utilities, `ssxl_sync` acts as the glue layer
//! that keeps the engine’s multi-threaded systems coordinated.
//! ============================================================================

use tokio::sync::mpsc; // Tokio channels for async communication

// -----------------------------------------------------------------------------
// 🔗 External Imports
// -----------------------------------------------------------------------------
// Import core contract types from `ssxl_shared`. These are the fundamental
// message and error types used across the engine.
// -----------------------------------------------------------------------------
use ssxl_shared::{
    SSXLError,        // Standardized error type for engine operations
    AnimationUpdate,  // Struct representing animation update messages
    AnimationCommand, // Enum representing animation commands
    // AnimationConductorHandle could be imported directly if needed
};

// FIX: Import and re-export the core struct in one step.
// This avoids duplicate definitions (E0252) or conflicting re-exports (E0365).
pub use ssxl_animate::AnimationConductor;

// -----------------------------------------------------------------------------
// 📂 Internal Modules
// -----------------------------------------------------------------------------
// These modules implement the building blocks of synchronization:
//   • primitives: atomic resources and channel helpers
//   • pool: worker pool and task scheduling
//   • animation_conductor: FFI-facing wrapper for animation threads
// -----------------------------------------------------------------------------
pub mod primitives;
pub mod pool;
pub mod animation_conductor;

// -----------------------------------------------------------------------------
// 🌐 Public Re-exports (Crate Facade)
// -----------------------------------------------------------------------------
// Re-export commonly used types so downstream crates can access them directly
// from `ssxl_sync` without needing deep paths.
// -----------------------------------------------------------------------------
pub use primitives::{AtomicResource, create_unbounded_channel};

pub use pool::{
    WorkerPool,       // Thread pool manager
    GenerationTask,   // Task definition for chunk generation
    ConductorResult,  // Result type for conductor operations
    Task,             // Generic task abstraction
    TaskResult,       // Result type for tasks
};

// -----------------------------------------------------------------------------
// 🛠️ FFI Wrappers and Type Aliases (Crate Root API)
// -----------------------------------------------------------------------------
// These type aliases define the public API surface for FFI communication.
// They standardize the channel types used to send commands and updates
// across the Rust ↔ Godot boundary.
// -----------------------------------------------------------------------------
pub type CommandReceiver = mpsc::UnboundedReceiver<AnimationCommand>; // Receives animation commands
pub type UpdateSender   = mpsc::UnboundedSender<AnimationUpdate>;     // Sends animation updates
pub type CommandResult  = Result<(), SSXLError>;                     // Standardized command result

// ✅ No struct wrapper here — we rely on the alias from `ssxl_shared`
// to keep the API surface consistent and avoid duplication.
