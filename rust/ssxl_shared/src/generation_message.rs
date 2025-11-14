// ssxl_shared/src/generation_message.rs

//! # Generation Messaging (`ssxl_shared::generation_message`)
//!
//! This module defines the structures and enumerations used for communication
//! between the main **Conductor** thread and the asynchronous **Worker Threads**
//! (managed by the `RuntimeManager` in the `ssxl_generate` crate).
//!
//! These structures manage the flow of work requests (`GenerationTask`) and
//! completed results (`GenerationMessage`).

use crate::chunk_data::ChunkData;
use ssxl_math::Vec2i;
use std::sync::Arc;


// --- Work Request Structure ---

/// Defines a single unit of work (a task) to be processed by a worker thread.
///
/// This structure is put into the engine's `TaskQueue` by the `Conductor`.
#[derive(Debug, Clone)]
pub struct GenerationTask {
    /// The chunk-space coordinates of the chunk that needs to be generated.
    pub chunk_coords: Vec2i,
    /// The unique identifier of the generator to be used for this task (e.g., "cellular_automata").
    pub generator_id: String,
}


// --- Communication Message Enumeration ---

/// An enumeration of messages sent from the worker threads back to the
/// main thread or the Conductor to signal task completion or pipeline status.
#[derive(Debug)]
pub enum GenerationMessage {
    /// Signals that a chunk has been successfully generated.
    ///
    /// Uses `Arc<ChunkData>` (Atomic Reference Counted) to allow multiple parts
    /// of the engine (e.g., the cache and the Godot presenter) to hold a reference
    /// to the generated data simultaneously without deep copying, which is critical
    /// for high-performance data sharing.
    ChunkGenerated(Vec2i, Arc<ChunkData>),

    /// Signals that all current tasks related to a specific generation batch
    /// or request have been finalized. Used by the `Conductor` to update
    /// the generation state.
    GenerationComplete,
}