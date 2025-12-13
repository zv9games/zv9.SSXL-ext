// rust/SSXL-ext/src/animate_events.rs

use godot::prelude::*; // For Vector2i and Color types

/// Represents a single, final, thread-safe animation instruction for the Godot main thread.
#[derive(Debug, Clone)]
pub enum AnimationEvent {
    /// Change the current frame index of an animated tile.
    SetTileAnimation {
        layer: i32,
        coords: (i32, i32),
        frame_index: i32,
    },
    /// Change the color property of a specific Light2D node.
    SetLightColor {
        light_id: u32,
        color: Color,
    },
    /// Request to create a new one-shot particle effect.
    SpawnParticleEffect {
        effect_id: u32,
        position: (f32, f32),
    },
}