//! # Core Communication Contracts (`ssxl_shared::messages`)
//!
//! This module defines the essential messaging structures used for asynchronous
//! communication between the Godot FFI layer, the Conductor, and the Rayon worker pool.
//! These public types centralize the data contract to break the cyclic dependency
//! between `ssxl_animate` and `ssxl_sync`.

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

// REMOVED: use godot::meta::GodotConvert;

// Primitives and ChunkData are available via the crate root's public exports.
use crate::chunk_data::ChunkData;
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
}

impl AnimationState {
    /// Implements the required method for the Conductor to update local state.
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }
}


// --- Generic Data Messages (Existing Content) ---

/// A generic, serializable message wrapper for communicating the status of a
/// single chunk data request.
///
/// **FIX:** Removed the `GodotConvert` derive, as it belongs to the Godot-facing
/// code that consumes this shared struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkMessage {
    /// The request was successful, and the complete `ChunkData` is attached.
    Generated(ChunkData),

    /// The request was processed, but the data does not exist (e.g., a cache miss).
    NoData,

    /// An operational error occurred during the request (e.g., I/O failure).
    /// The attached `String` contains the human-readable error description.
    Error(String),
}