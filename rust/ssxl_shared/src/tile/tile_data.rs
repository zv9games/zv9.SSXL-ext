//! ============================================================================
//! 🧩 Tile Data Structures (`ssxl_shared::tile::tile_data`)
//! ----------------------------------------------------------------------------
//! This module defines the core representation of a single tile in the SSXL
//! procedural world. Tiles are the smallest unit of world data, and their
//! design emphasizes efficiency, clarity, and compatibility with both Rust
//! and external systems (like Godot).
//!
//! Key responsibilities:
//!   - Provide a lightweight `TileData` struct for representing tile state.
//!   - Support serialization/deserialization for persistence and networking.
//!   - Use bitflags (`u8`) to store multiple boolean properties compactly.
//!   - Define FFI-friendly structures (`AnimationUpdate`) for communication
//!     with external runtimes such as Godot.
//!
//! Design choices:
//!   - `Copy` trait: allows tiles to be duplicated cheaply in tight loops.
//!   - `u8` flags: packs up to 8 booleans into one byte for memory efficiency.
//!   - Noise values: store raw procedural noise for debugging and neighbor checks.
//! ============================================================================

use super::tile_type::TileType;
use serde::{Deserialize, Serialize};
use ssxl_math::prelude::Vec2i; // Used for coordinate-based updates in animations

// -----------------------------------------------------------------------------
// 📦 Core Data Structure: TileData
// -----------------------------------------------------------------------------
// Represents the minimal payload for a single tile in a chunk.
// Fields:
//   - tile_type: categorizes the tile (e.g., grass, rock, void).
//   - noise_value: raw procedural noise used to determine type.
//   - flags: compact bitfield for boolean properties (see tile_flags).
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileData {
    pub tile_type: TileType,
    pub noise_value: f32,
    pub flags: u8,
}

impl Default for TileData {
    /// Provides a default tile: "empty" type, zero noise, no flags.
    /// This ensures predictable initialization when creating new chunks.
    fn default() -> Self {
        TileData {
            tile_type: TileType::default(),
            noise_value: 0.0,
            flags: 0,
        }
    }
}

// -----------------------------------------------------------------------------
// 🔗 Godot FFI Message Structure: AnimationUpdate
// -----------------------------------------------------------------------------
// Used for communication between the Rust engine and the Godot runtime.
// Encapsulates tile animation updates in a format that can be serialized
// and passed across the FFI boundary.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AnimationUpdate {
    pub layer: i32,             // Layer or dimension the tile belongs to
    pub source_id: i32,         // Source ID of the tilemap/animation system
    pub tile_coords: Vec2i,     // World coordinates of the tile to update
    pub new_atlas_coords: Vec2i // New texture atlas coordinates for animation frame
}

// -----------------------------------------------------------------------------
// ⚙️ Bitwise Flag Constants
// -----------------------------------------------------------------------------
// Flags provide a compact way to store multiple boolean properties in one byte.
// Each constant represents a bitmask that can be set, cleared, or checked.
// -----------------------------------------------------------------------------
pub mod tile_flags {
    pub const IS_TRAVERSABLE: u8 = 0b0000_0001; // Entities can move through this tile
    pub const IS_RENDERED:    u8 = 0b0000_0010; // Tile is visible to the client
    pub const IS_MODIFIED:    u8 = 0b0000_0100; // Tile was changed after generation
    pub const HAS_RESOURCE:   u8 = 0b0000_1000; // Tile contains a resource/item
    // Bits 4–7 reserved for future expansion
}

// -----------------------------------------------------------------------------
// 🔧 TileData Methods
// -----------------------------------------------------------------------------
// Utility functions for creating, inspecting, and modifying tile state.
// -----------------------------------------------------------------------------
impl TileData {
    /// Creates a new tile with a given type and noise value.
    /// Flags are initialized to zero (no properties set).
    pub fn new(tile_type: TileType, noise_value: f32) -> Self {
        TileData {
            tile_type,
            noise_value,
            flags: 0,
        }
    }

    /// Determines if the tile is "solid" (non-empty), typically used for collision.
    #[inline(always)]
    pub const fn is_solid(&self) -> bool {
        !self.tile_type.is_empty()
    }
    
    /// Sets or clears a flag property using a bitmask (e.g., `tile_flags::IS_TRAVERSABLE`).
    #[inline(always)]
    pub fn set_flag(&mut self, flag_mask: u8, value: bool) {
        if value {
            self.flags |= flag_mask;   // Set bit(s)
        } else {
            self.flags &= !flag_mask;  // Clear bit(s)
        }
    }

    /// Checks whether a flag property is set.
    #[inline(always)]
    pub const fn has_flag(&self, flag_mask: u8) -> bool {
        (self.flags & flag_mask) != 0
    }
}
