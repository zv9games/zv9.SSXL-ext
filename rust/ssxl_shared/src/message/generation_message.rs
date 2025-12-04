// ssxl_shared/src/message/generation_message.rs (Fixed Imports & Variants)

//! # Generation Messaging (`ssxl_shared::message::generation_message`)

use crate::chunk::chunk_data::ChunkData;
use ssxl_math::prelude::Vec2i;
use std::sync::Arc;
// FIX 1: Import the serialization traits from serde.
use serde::{Serialize, Deserialize};


// --- Work Request Structure ---

/// Defines a single unit of work (a task) to be processed by a worker thread.
///
/// This structure is put into the engine's `TaskQueue` by the `Conductor`.
// FIX 2: Add Serialize and Deserialize derives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationTask {
    /// The chunk-space coordinates of the chunk that needs to be generated.
    pub chunk_coords: Vec2i,
    /// The unique identifier of the generator to be used for this task (e.g., "cellular_automata").
    pub generator_id: String,
}


// --- Communication Message Enumeration ---

/// An enumeration of messages sent from the worker threads back to the
/// main thread or the Conductor to signal task completion or pipeline status.
// FIX 3: Add Serialize and Deserialize derives.
#[derive(Debug, Serialize, Deserialize)]
pub enum GenerationMessage {
    /// Signals that a chunk has been successfully generated.
    ///
    /// The payload includes the chunk coordinates (for tracking) and the
    /// atomic reference-counted data.
    Generated(Vec2i, Arc<ChunkData>),

    /// Signals a change in the internal generation status or progress update.
    StatusUpdate(String),

    /// Signals that all current tasks related to a specific generation batch
    /// or request have been finalized. Used by the `Conductor` to update
    /// the generation state.
    GenerationComplete,
}