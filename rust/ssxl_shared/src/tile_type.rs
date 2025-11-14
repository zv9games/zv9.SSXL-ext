// ssxl_shared/src/tile_type.rs

//! # Tile Type Enumeration (`ssxl_shared::tile_type`)
//!
//! This module defines the canonical `TileType` enum, which identifies the material
//! or nature of a single tile. It is a fundamental data structure shared across
//! the entire SSXL-ext engine.
//!
//! **Critical Feature:** The use of `#[repr(u8)]` is a **memory optimization**
//! that guarantees the enum is stored as a single byte, drastically reducing the
//! memory footprint of the massive `ChunkData` tile arrays.

use serde::{Deserialize, Serialize};

/// Enumeration of all primary tile materials available in the procedural world.
///
/// Ensures strict memory layout as a single byte for efficiency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)] 
pub enum TileType {
    /// The default, empty state (e.g., air or background).
    Void = 0,
    /// A fluid, typically used for rivers, oceans, or lower areas.
    Water = 1,
    /// Base terrain, usually flat and traversable.
    Grass = 2,
    /// Elevated terrain, potentially difficult to traverse.
    Mountain = 3,
    /// An artificial boundary used to cap chunk edges or isolate generation regions.
    Boundary = 4,
    /// An engineered or placed structure (e.g., a ruin, road, or wall).
    Structure = 5,
    /// A solid, rocky, non-soil terrain type.
    Rock = 6,
    /// Reserved for generator-specific customization or future expansion.
    Custom1 = 7,
    /// Reserved for generator-specific customization or future expansion.
    Custom2 = 8,
}


impl Default for TileType {
    /// The default state of a tile is always `TileType::Void` (empty).
    fn default() -> Self {
        TileType::Void
    }
}

impl TileType {
    /// Converts the `TileType` instance into its underlying `u8` representation.
    #[inline] 
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Attempts to convert a raw `u8` value back into a `TileType`.
    ///
    /// Returns `None` if the input value does not correspond to a defined variant,
    /// ensuring safe deserialization.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TileType::Void),
            1 => Some(TileType::Water),
            2 => Some(TileType::Grass),
            3 => Some(TileType::Mountain),
            4 => Some(TileType::Boundary),
            5 => Some(TileType::Structure),
            6 => Some(TileType::Rock),
            7 => Some(TileType::Custom1),
            8 => Some(TileType::Custom2),
            _ => None,
        }
    }
    
    /// Provides a simple, default unique ID for this tile type (equal to its u8 value).
    pub const fn get_default_tile_id(self) -> u16 {
        self.to_u8() as u16
    }

    /// Provides default atlas coordinates (X, Y) for initial rendering/visual representation.
    ///
    /// This is a common lookup function used by the rendering component (`ssxl_godot`).
    pub const fn get_default_atlas_coords(self) -> (u16, u16) {
        match self {
            // All default tiles are expected to be on the first row (Y=0) of the atlas.
            TileType::Water    => (1, 0),
            TileType::Grass    => (2, 0),
            TileType::Mountain => (3, 0),
            TileType::Boundary => (4, 0),
            TileType::Structure=> (5, 0),
            TileType::Rock     => (6, 0),
            // Void and unspecified types default to the origin (0, 0).
            _ => (0, 0), 
        }
    }
}


impl TileType {
    /// Checks if the tile type is one that a character can typically traverse (walk on).
    pub const fn is_walkable(self) -> bool {
        matches!(self, 
            TileType::Grass 
            | TileType::Mountain 
            | TileType::Structure 
            | TileType::Rock
        )
    }

    /// Checks if the tile type is considered a fluid (e.g., for flow/buoyancy simulation).
    pub const fn is_fluid(self) -> bool {
        matches!(self, TileType::Water)
    }

    /// Checks if the tile is empty (the default/void state).
    pub const fn is_empty(self) -> bool {
        matches!(self, TileType::Void)
    }
}