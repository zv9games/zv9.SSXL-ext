// FILE: ssxl_shared/src/message/messages.rs

use tokio::sync::mpsc::UnboundedSender;
use crate::{ChunkId, TileCoord};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    TileFlip,
    TweenMove,
    PulseFade(f32),
    CustomScripted(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationPayload {
    FrameUpdate { new_frame: u32 },
    TweenValue { key: String, value: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationUpdate {
    pub coord: TileCoord,
    pub payload: AnimationPayload,
}

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

pub type UpdateSender = UnboundedSender<AnimationUpdate>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationCommand {
    GenerateChunk { coords: TileCoord },
    SetGenerator { id: String },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub success: bool,
    pub message: String,
}

pub type AnimationConductorHandle = UnboundedSender<AnimationCommand>;

pub type CommandResult = Result<(), String>;

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

pub use super::generation_message::GenerationMessage as ChunkMessage;