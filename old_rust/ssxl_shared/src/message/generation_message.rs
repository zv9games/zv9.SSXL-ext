// ============================================================================
// 📡 Generation Messaging
// File: ssxl_shared/src/message/generation_message.rs
// ----------------------------------------------------------------------------
// Purpose:
//   - Defines the communication protocol between the Conductor (main thread)
//     and worker threads in the SSXL engine.
//   - Provides structures for sending generation tasks into the pipeline and
//     receiving results or status updates back.
//   - Ensures messages are serializable for persistence, networking, or debugging.
// ============================================================================

use crate::chunk::chunk_data::ChunkData;   // Core chunk payload type
use ssxl_math::prelude::Vec2i;             // 2D integer vector for chunk coordinates
use std::sync::Arc;                        // Atomic reference-counted pointer for safe sharing
use serde::{Serialize, Deserialize};       // Serialization traits for message passing

// -----------------------------------------------------------------------------
// 🛠️ Work Request Structure: GenerationTask
// -----------------------------------------------------------------------------
// Represents a single unit of work to be performed by a worker thread.
// Inserted into the engine’s TaskQueue by the Conductor.
// Fields:
//   - chunk_coords: identifies which chunk in chunk-space should be generated.
//   - generator_id: specifies which generator algorithm to use (e.g. "cellular_automata").
// Derives:
//   - Debug, Clone: for inspection and duplication.
//   - Serialize, Deserialize: for persistence and cross-thread communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationTask {
    pub chunk_coords: Vec2i,     // Target chunk coordinates in grid space
    pub generator_id: String,    // Generator identifier string
}

// -----------------------------------------------------------------------------
// 📬 Communication Message Enumeration: GenerationMessage
// -----------------------------------------------------------------------------
// Represents messages sent back from worker threads to the Conductor.
// Each variant signals a different type of pipeline outcome.
// Variants:
//   - Generated: A chunk has been successfully produced.
//   - StatusUpdate: Informational message about progress or internal state.
//   - GenerationComplete: Signals that all tasks in a batch are finished.
// -----------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub enum GenerationMessage {
    // 🌟 Generated: A new chunk is ready.
    // Payload:
    //   - Vec2i: coordinates of the chunk.
    //   - Arc<ChunkData>: reference-counted chunk data for safe sharing.
    Generated(Vec2i, Arc<ChunkData>),

    // 📊 StatusUpdate: Provides progress or state information.
    // Example: "50% complete" or "Switching generator mode".
    StatusUpdate(String),

    // ✅ GenerationComplete: Marks the end of a batch of tasks.
    // Used by the Conductor to update global generation state.
    GenerationComplete,
}
