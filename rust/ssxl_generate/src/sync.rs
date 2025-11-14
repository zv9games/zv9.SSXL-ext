// ssxl_generate/src/sync.rs

//! Defines public type aliases for synchronization primitives used to communicate
//! with the Conductor from external systems (like FFI wrappers).
//!
//! This module simplifies the public API by wrapping complex Tokio channel types.

use tokio::sync::mpsc;
use crate::task_queue::GenerationMessage;
use crate::task_queue::GenerationTask;

// --- Public Synchronization Type Aliases ---

/// An unbounded sender for posting new chunk generation requests to the Conductor's task queue.
///
/// This is the primary input channel for the SSXL generation engine, allowing users to
/// queue up work (`GenerationTask`) without blocking.
pub type ConductorRequestSender = mpsc::UnboundedSender<GenerationTask>;

/// The receiver for all progress updates and completion messages from the Conductor.
///
/// This channel delivers structured updates (`GenerationMessage`) such as `ChunkGenerated`
/// and `GenerationComplete` back to the external caller.
pub type ConductorProgressReceiver = mpsc::Receiver<GenerationMessage>;