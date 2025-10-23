use serde::{Deserialize, Serialize};
use crate::zv9_prelude::*;

/// 🧱 Metadata for a single tile in the map.
/// Used for procedural generation, chunk streaming, and engine-side placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileInfo {
    /// ID of the source tileset or layer.
    pub source_id: i32,

    /// Coordinates in the atlas (e.g. tile index).
    pub atlas_coords: SerializableVector2i,

    /// Alternate ID for visual variation or animation.
    pub alternate_id: i32,

    /// Rotation in 90° steps (0–3).
    pub rotation: u8,

    /// Layer index for multi-layer maps.
    pub layer: u8,

    /// Bitmask for collision, visibility, or custom flags.
    pub flags: u32,

    /// Optional variant ID for visual diversity.
    pub variant_id: Option<i32>,

    /// Optional animation frame count.
    pub frame_count: Option<u8>,

    /// Optional animation speed in milliseconds.
    pub animation_speed: Option<u32>,
}

impl TileInfo {
    /// Creates a new tile with the given atlas coordinates and optional type.
    pub fn new(atlas_coords: SerializableVector2i, source_id: i32) -> Self {
        Self {
            source_id,
            atlas_coords,
            alternate_id: 0,
            rotation: 0,
            layer: 0,
            flags: tile_flags::VISIBLE,
            variant_id: None,
            frame_count: None,
            animation_speed: None,
        }
    }

    /// Returns true if the tile has the given flag.
    pub fn has_flag(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }

    /// Sets the given flag.
    pub fn set_flag(&mut self, flag: u32) {
        self.flags |= flag;
    }

    /// Clears the given flag.
    pub fn clear_flag(&mut self, flag: u32) {
        self.flags &= !flag;
    }

    /// Returns a debug summary of the tile.
    pub fn describe(&self) -> String {
        format!(
            "TileInfo[src={}, alt={}, rot={}, layer={}, flags={:#06b}, atlas=({}, {})]",
            self.source_id,
            self.alternate_id,
            self.rotation.clamp(0, 3),
            self.layer,
            self.flags,
            self.atlas_coords.x,
            self.atlas_coords.y
        )
    }
}

impl Default for TileInfo {
    fn default() -> Self {
        Self {
            source_id: 0,
            atlas_coords: SerializableVector2i { x: 0, y: 0 },
            alternate_id: 0,
            rotation: 0,
            layer: 0,
            flags: 0,
            variant_id: None,
            frame_count: None,
            animation_speed: None,
        }
    }
}

/// 🎛️ Bitmask flags for tile behavior and rendering.
/// Combine using bitwise OR.
pub mod tile_flags {
    pub const COLLIDABLE: u32     = 0b00001;
    pub const VISIBLE: u32        = 0b00010;
    pub const INTERACTIVE: u32    = 0b00100;
    pub const EMISSIVE: u32       = 0b01000;
    pub const DYNAMIC: u32        = 0b10000;
}
