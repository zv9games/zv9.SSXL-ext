// rust/SSXL-ext/src/animate_events.rs (FIXED)

// 🎯 CRITICAL FIX: Gate the Godot import
#[cfg(feature = "godot-binding")]
use godot::prelude::*; // For Vector2i and Color types

// 🎯 CRITICAL FIX: Define mock types for the CLI build (where godot-binding is OFF)
// Vector2i is defined in tools.rs, but we need to define Color here or in a shared place.
#[cfg(not(feature = "godot-binding"))]
type Color = u32; // Placeholder for Godot's Color type

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
        // The type 'Color' here will resolve to 'godot::builtin::Color' when 
        // godot-binding is on, and to the 'u32' alias when it is off.
        color: Color, 
    },
    /// Request to create a new one-shot particle effect.
    SpawnParticleEffect {
        effect_id: u32,
        position: (f32, f32),
    },
}