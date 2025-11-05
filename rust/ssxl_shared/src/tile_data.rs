// ssxl_shared/src/tile_data.rs

//! Defines the TileData structure: a compact, serializable unit holding all
//! generated and runtime data for a single tile in the world grid.
//!
//! Total size is 6 bytes (1 + 4 + 1), which is highly memory-efficient for bulk storage.

use super::tile_type::TileType;
use serde::{Serialize, Deserialize};

// CRITICAL FIX: Required import for AnimationUpdate coordinates.
// Assumes Vec2i is available from the ssxl_math crate.
use ssxl_math::Vec2i;

// --------------------------------------------------------------------------------
// CORE TILE DATA STRUCTURE
// --------------------------------------------------------------------------------

/// A compact structure holding the state of a single tile in the world.
///
/// This type is designed to be small, cheap to copy, and highly serializable for
/// use in the aetherion_cache.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileData {
    /// The fundamental type of terrain/feature, determined by the MVG.
    /// This uses the u8 discriminant of the TileType enum.
    pub tile_type: TileType,
    
    /// The raw noise value V ∈ [0.0, 1.0] used to determine the `tile_type`.
    /// This serves as the primary elevation/metric field, combining the roles of 
    /// the previous `height_value` and potentially `aux_value` for simplicity.
    pub noise_value: f32, // 4 bytes
    
    /// An 8-bit field for general-purpose runtime flags (e.g., IsModified, IsTraversable).
    pub flags: u8, // 1 byte
}

impl Default for TileData {
    /// Provides a default, uninitialized state.
    fn default() -> Self {
        TileData {
            tile_type: TileType::default(), // Defaults to Void
            noise_value: 0.0,
            flags: 0,
        }
    }
}

// --------------------------------------------------------------------------------
// ANIMATION & COMMUNICATION DATA STRUCTURE (Fixes E0432)
// --------------------------------------------------------------------------------

/// Represents a simple tile update message for the animation worker.
/// This struct is sent from `ssxl_sync` (worker) to `ssxl_godot` (main thread)
/// to apply a single tile graphic change without affecting the underlying `TileData`.
#[derive(Debug, Clone)]
pub struct AnimationUpdate {
    pub layer: i32,
    pub source_id: i32,
    pub tile_coords: Vec2i,
    pub new_atlas_coords: Vec2i,
}


// ---------------------------
// IMPL: TileData Constructor, Flags Utility and Logic
// ---------------------------

/// Bitmask constants for the `flags` field
pub mod tile_flags { 
    pub const IS_TRAVERSABLE: u8 = 0b0000_0001; // Bit 0: Is the tile physically passable?
    pub const IS_RENDERED:    u8 = 0b0000_0010; // Bit 1: Is the tile currently visible/rendered?
    pub const IS_MODIFIED:    u8 = 0b0000_0100; // Bit 2: Has this tile been manually changed post-generation?
    pub const HAS_RESOURCE:   u8 = 0b0000_1000; // Bit 3: Does this tile contain a gatherable resource?
    // Bits 4-7 are available for future logic.
}

impl TileData {
    /// Creates a new TileData instance with the generated type and noise value.
    /// This is the primary constructor used by the aetherion_generate MVG.
    pub fn new(tile_type: TileType, noise_value: f32) -> Self {
        // Initialize other fields to their defaults when constructed by the generator
        TileData {
            tile_type,
            noise_value,
            flags: 0, 
        }
    }

    /// Checks if the tile is considered solid or non-traversable (Air/Void).
    /// This relies on the `TileType`'s inherent logic for better consistency.
    pub const fn is_solid(&self) -> bool {
        !self.tile_type.is_empty()
    }

    /// Sets a specific bit flag by index (0-7).
    pub fn set_flag_by_index(&mut self, flag_index: u8) {
        if flag_index < 8 {
            self.flags |= 1 << flag_index;
        }
    }

    /// Clears a specific bit flag by index (0-7).
    pub fn clear_flag_by_index(&mut self, flag_index: u8) {
        if flag_index < 8 {
            self.flags &= !(1 << flag_index);
        }
    }

    /// Checks if a specific bit flag is set by index (0-7).
    pub fn check_flag_by_index(&self, flag_index: u8) -> bool {
        if flag_index < 8 {
            (self.flags & (1 << flag_index)) != 0
        } else {
            false
        }
    }
    
    /// Sets or clears a flag using the public constant masks (e.g., tile_flags::IS_TRAVERSABLE).
    pub fn set_flag(&mut self, flag_mask: u8, value: bool) {
        if value {
            self.flags |= flag_mask;
        } else {
            self.flags &= !flag_mask;
        }
    }

    /// Checks if a specific flag is set using the public constant masks.
    pub const fn has_flag(&self, flag_mask: u8) -> bool {
        (self.flags & flag_mask) != 0
    }
}