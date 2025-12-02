// ssxl_shared/src/message/messages.rs (Final Optimized Imports)

//! # Core Communication Contracts (`ssxl_shared::message::messages`)
//!
//! This module defines the essential messaging structures used for asynchronous
//! communication between the Godot FFI layer, the Conductor, and the Rayon worker pool.
//! These public types centralize the data contract to break the cyclic dependency
//! between `ssxl_animate` and `ssxl_sync`.

// FIX: Removed the last unused import: `super::generation_message::GenerationMessage`
// The public alias `ChunkMessage` still functions as it uses a fully qualified path
// to resolve `GenerationMessage` at compile time, thus the `use` statement is not required
// and was correctly flagged as unused.
use tokio::sync::mpsc::UnboundedSender;
use crate::{ChunkId, TileCoord};


// --- Core Animation Communication (Cycle-Breaking Types) ---

/// Defines the various types of animation behaviors that workers can execute.
#[derive(Debug, Clone)]
pub enum AnimationType {
    TileFlip,
    TweenMove,
    PulseFade(f32),
    CustomScripted(String),
}

/// Defines the change to a tile (e.g., new frame ID, new tween value).
#[derive(Debug, Clone)]
pub enum AnimationPayload {
    FrameUpdate { new_frame: u32 },
    TweenValue { key: String, value: f32 },
}

/// A structure representing a single update to be applied by the Godot main thread.
#[derive(Debug, Clone)]
pub struct AnimationUpdate {
    pub coord: TileCoord,
    pub payload: AnimationPayload,
}

/// The command sent from the Godot FFI layer (via ssxl_sync) to the async workers.
#[derive(Debug, Clone)]
pub enum AnimationCommand {
    /// Command 1: Delegate heavy animation work to the Rayon thread pool.
    AnimateChunkSet {
        chunk_ids: Vec<ChunkId>,
        anim_type: AnimationType,
    },
    
    /// Command 2: System command to kick off a dedicated test or demo animation.
    StartTestAnimation,
    
    /// Command 3: State command to adjust the overall simulation speed.
    SetTimeScale(f32),

    /// State command to enable or disable the animation conductor thread.
    SetEnabled(bool),
    
    /// Command 4: System command to trigger a graceful shutdown of the Conductor.
    Shutdown,
}

/// The sender handle for the main update channel (Tokio/MPSC).
/// Used by Rayon workers to send results back to the main thread poller.
pub type UpdateSender = UnboundedSender<AnimationUpdate>;


// --- FFI / Conductor Control Types (Completing the Contract) ---

/// The handle returned to the FFI layer to send AnimationCommands to the Conductor.
/// This type lives here to break the package dependency cycle (ssxl_sync -> ssxl_animate).
pub type AnimationConductorHandle = UnboundedSender<AnimationCommand>;

/// A simple Result type for command processing success/failure.
pub type CommandResult = Result<(), String>;

/// The current runtime state of the Animation Conductor (e.g., Running, Paused).
#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationState {
    pub time_scale: f32, // The state field required by the SetTimeScale command
    // FIX: Added to track the enabled/disabled state, resolving E0599.
    pub is_enabled: bool,
}

impl AnimationState {
    /// Implements the required method for the Conductor to update local state.
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }

    /// FIX: Implements the required setter for the `AnimationCommand::SetEnabled` command.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}


// --- Generic Data Messages (REMOVED DUPLICATION) ---

// NOTE: Since the Godot code was using ChunkMessage, we will ensure that 
// GenerationMessage is exported as a public alias for compatibility.
// This resolves the `unused import` warning by removing the direct import,
// while keeping the public re-export which uses the fully qualified path.
pub use super::generation_message::GenerationMessage as ChunkMessage;