// ssxl_shared/src/tile_data.rs

//! # Tile Data Structures (`ssxl_shared::tile_data`)
//!
//! This module defines the `TileData` structure, which holds the minimal necessary
//! information for a single tile in the procedural world. It also includes utility
//! structures and bitmask constants (`tile_flags`) for efficient property management.
//! The use of `Copy` and `u8` flags is key to maintaining the high-speed "Tempo"
//! of the multi-threaded engine.

use super::tile_type::TileType;
use serde::{Deserialize, Serialize};

// Re-exports a core math primitive for use in related structures (e.g., AnimationUpdate).
use ssxl_math::Vec2i;


// --- Core Data Structure ---

/// Represents the data payload for a single tile in a chunk.
///
/// Marked `Copy` for high performance, as tiles are frequently cloned/copied
/// within generation and rendering loops.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileData {
    /// The material or type of tile (e.g., grass, rock, empty).
    pub tile_type: TileType,
    
    /// The raw noise value (e.g., Perlin, Simplex) used to determine this tile's type.
    /// This is crucial for seamless neighbor checks and debugging the procedural generation.
    pub noise_value: f32,
    
    /// A compact bitfield storing up to 8 boolean properties about the tile.
    pub flags: u8,
}

impl Default for TileData {
    /// Returns the default state for a tile: `TileType::Void` with zero noise and no flags set.
    fn default() -> Self {
        TileData {
            tile_type: TileType::default(),
            noise_value: 0.0,
            flags: 0,
        }
    }
}


// --- Godot FFI Message Structure ---

/// Data structure used specifically for communication between the Rust engine
/// and the Godot runtime regarding tile animations.
///
/// This message is typically used by a dedicated rendering or animation system.
#[derive(Debug, Clone, Serialize, Deserialize)] // FIX: Added Serialize and Deserialize
pub struct AnimationUpdate {
    /// The layer or dimension the tile belongs to.
    pub layer: i32,
    /// The source ID of the tile map or animation source.
    pub source_id: i32,
    /// The world coordinates of the tile to update.
    pub tile_coords: Vec2i,
    /// The new coordinates within the texture atlas for the tile's animation frame.
    pub new_atlas_coords: Vec2i,
}


// --- Bitwise Flag Constants ---

/// Bitmasks used to set, check, or clear properties in the `TileData::flags` field.
/// This maximizes data density by storing 8 booleans in a single byte (`u8`).
pub mod tile_flags {
    /// Bit 0: Is the tile physically passable by entities? (e.g., air, water surface)
    pub const IS_TRAVERSABLE: u8 = 0b0000_0001;
    /// Bit 1: Is the tile currently visible/rendered by the client?
    pub const IS_RENDERED:    u8 = 0b0000_0010;
    /// Bit 2: Has this tile been manually changed post-generation by a player/editor?
    pub const IS_MODIFIED:    u8 = 0b0000_0100;
    /// Bit 3: Does this tile contain a gatherable resource or item?
    pub const HAS_RESOURCE:    u8 = 0b0000_1000;
    // Bits 4-7 are reserved for future properties.
}


impl TileData {
    /// Creates a new `TileData` instance with specified type and noise value.
    pub fn new(tile_type: TileType, noise_value: f32) -> Self {
        TileData {
            tile_type,
            noise_value,
            flags: 0, // Flags are initialized to zero (all false)
        }
    }

    /// Checks if the tile is considered a "solid" block, typically used for collision.
    pub const fn is_solid(&self) -> bool {
        // Relies on a method in TileType to define "solidity."
        !self.tile_type.is_empty()
    }

    /// Sets a flag based on its **bit index** (0-7).
    pub fn set_flag_by_index(&mut self, flag_index: u8) {
        if flag_index < 8 {
            // Bitwise OR assignment: sets the bit at the given index to 1.
            self.flags |= 1 << flag_index;
        }
    }

    /// Clears (sets to 0) a flag based on its **bit index** (0-7).
    pub fn clear_flag_by_index(&mut self, flag_index: u8) {
        if flag_index < 8 {
            // Bitwise AND assignment with the inverted mask: sets the bit to 0.
            self.flags &= !(1 << flag_index);
        }
    }

    /// Checks the status of a flag based on its **bit index** (0-7).
    pub fn check_flag_by_index(&self, flag_index: u8) -> bool {
        if flag_index < 8 {
            // Bitwise AND with the mask: returns true if the bit is set.
            (self.flags & (1 << flag_index)) != 0
        } else {
            false
        }
    }
    
    /// Sets or clears a property using a predefined **flag mask** (e.g., `tile_flags::IS_TRAVERSABLE`).
    pub fn set_flag(&mut self, flag_mask: u8, value: bool) {
        if value {
            // Set the bit(s) (|= is bitwise OR assignment).
            self.flags |= flag_mask;
        } else {
            // Clear the bit(s) (&! is bitwise AND with the inverted mask).
            self.flags &= !flag_mask;
        }
    }

    /// Checks the status of a property using a predefined **flag mask**.
    pub const fn has_flag(&self, flag_mask: u8) -> bool {
        // Returns true if any of the bits in the mask are set in self.flags.
        (self.flags & flag_mask) != 0
    }
}