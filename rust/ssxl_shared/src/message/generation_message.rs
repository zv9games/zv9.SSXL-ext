// ssxl_shared/src/message/generation_message.rs (Fixed Imports & Variants)

//! # Generation Messaging (`ssxl_shared::message::generation_message`)
//!
//! This module defines the structures and enumerations used for communication
//! between the main **Conductor** thread and the asynchronous **Worker Threads**
//! (managed by the `RuntimeManager` in the `ssxl_generate` crate).
//!
//! These structures manage the flow of work requests (`GenerationTask`) and
//! completed results (`GenerationMessage`).

// FIX: Update the path from crate::chunk_data to the new subdirectory path.
use crate::chunk::chunk_data::ChunkData;
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
    // FIX 1: Renamed from `ChunkGenerated` to `Generated` to match the name 
    // expected by `ssxl_godot/src/engine/tick.rs`.
    // FIX 3: Re-introducing the `Vec2i` coordinate to match the 
    // previous pattern expectation in `ssxl_generate/src/conductor/conductor.rs`.
    /// Signals that a chunk has been successfully generated.
    ///
    /// The payload includes the chunk coordinates (for tracking) and the
    /// atomic reference-counted data.
    Generated(Vec2i, Arc<ChunkData>),

    // FIX 2: Added the missing `StatusUpdate` variant, which the Godot code requires.
    /// Signals a change in the internal generation status or progress update.
    StatusUpdate(String),

    /// Signals that all current tasks related to a specific generation batch
    /// or request have been finalized. Used by the `Conductor` to update
    /// the generation state.
    GenerationComplete,
}