// ============================================================================
// 📡 Messaging System
// File: ssxl_shared/src/message/messages.rs
// ----------------------------------------------------------------------------
// This module defines the communication layer for the SSXL engine.
// It provides the data structures used to send instructions (commands)
// and receive feedback (updates/responses) between subsystems such as
// the Conductor, worker threads, and animation pipeline.
//
// Key goals:
//   - Standardize how tasks and updates are represented.
//   - Ensure all messages can be serialized (for persistence, networking, or debugging).
//   - Separate animation-related communication from generation-related communication.
// ============================================================================

use tokio::sync::mpsc::UnboundedSender; // Asynchronous channel for message passing
use crate::{ChunkId, TileCoord};        // Identifiers for chunks and tiles
use serde::{Deserialize, Serialize};    // Traits for serialization and deserialization

// -----------------------------------------------------------------------------
// 🎨 AnimationType
// -----------------------------------------------------------------------------
// Describes the *kind* of animation to apply.
// Each variant represents a different animation strategy:
//   - TileFlip: toggles a tile’s visual state.
//   - TweenMove: smoothly interpolates movement.
//   - PulseFade(f32): fades in/out with a given intensity.
//   - CustomScripted(String): allows user-defined scripted animations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    TileFlip,
    TweenMove,
    PulseFade(f32),
    CustomScripted(String),
}

// -----------------------------------------------------------------------------
// 🎨 AnimationPayload
// -----------------------------------------------------------------------------
// Encapsulates the *data* needed for an animation update.
//   - FrameUpdate: specifies which frame to display.
//   - TweenValue: provides a key/value pair for a tweened property
//     (e.g., "opacity" -> 0.5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationPayload {
    FrameUpdate { new_frame: u32 },
    TweenValue { key: String, value: f32 },
}

// -----------------------------------------------------------------------------
// 🎨 AnimationUpdate
// -----------------------------------------------------------------------------
// Represents a single animation update message.
// Fields:
//   - coord: the tile coordinate being animated.
//   - payload: the animation data to apply at that coordinate.
// This is what worker threads send back to the Conductor to indicate
// progress or changes in animation state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationUpdate {
    pub coord: TileCoord,
    pub payload: AnimationPayload,
}

// -----------------------------------------------------------------------------
// 🎨 AnimationCommand
// -----------------------------------------------------------------------------
// Commands sent *to* the animation subsystem.
// These instruct the system to start, modify, or stop animations.
// Variants:
//   - AnimateChunkSet: apply a chosen animation type to multiple chunks.
//   - StartTestAnimation: run a test/demo animation.
//   - SetTimeScale(f32): adjust global animation speed.
//   - SetEnabled(bool): enable/disable animations globally.
//   - Shutdown: terminate the animation subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationCommand {
    AnimateChunkSet {
        chunk_ids: Vec<ChunkId>,
        anim_type: AnimationType,
    },
    StartTestAnimation,
    SetTimeScale(f32),
    SetEnabled(bool),
    Shutdown,
}

// Type alias for sending animation updates through an async channel.
// This makes it easier to reference the sender type consistently.
pub type UpdateSender = UnboundedSender<AnimationUpdate>;

// -----------------------------------------------------------------------------
// ⚙️ GenerationCommand
// -----------------------------------------------------------------------------
// Commands sent to the *generation subsystem*.
// Variants:
//   - GenerateChunk: request generation of a specific chunk at given coordinates.
//   - SetGenerator: change which generator algorithm is active.
//   - Shutdown: stop the generation subsystem entirely.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationCommand {
    GenerateChunk { coords: TileCoord },
    SetGenerator { id: String },
    Shutdown,
}

// -----------------------------------------------------------------------------
// ⚙️ GenerationResponse
// -----------------------------------------------------------------------------
// Feedback returned after executing a generation command.
// Fields:
//   - success: true if the command succeeded, false otherwise.
//   - message: human-readable explanation of the outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub success: bool,
    pub message: String,
}

// Type alias for sending animation commands to the conductor.
// Simplifies references to the channel type.
pub type AnimationConductorHandle = UnboundedSender<AnimationCommand>;

// Type alias for generic command results.
// Standardizes return values across subsystems.
pub type CommandResult = Result<(), String>;

// -----------------------------------------------------------------------------
// 🎛️ AnimationState
// -----------------------------------------------------------------------------
// Holds the *current global state* of the animation subsystem.
// Fields:
//   - time_scale: multiplier for animation speed (e.g., 2.0 = double speed).
//   - is_enabled: whether animations are globally active.
// Includes helper methods to update these values.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AnimationState {
    pub time_scale: f32,
    pub is_enabled: bool,
}

impl AnimationState {
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

// -----------------------------------------------------------------------------
// 🔗 Re-export
// -----------------------------------------------------------------------------
// Re-exports GenerationMessage from generation_message.rs as ChunkMessage.
// This allows other modules to import generation-related messages
// directly from `messages.rs`, keeping the API surface consistent.
pub use super::generation_message::GenerationMessage as ChunkMessage;
