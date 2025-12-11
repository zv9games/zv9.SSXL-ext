//! ============================================================================
//! 🧩 Tile Type Enumeration (`ssxl_shared::tile::tile_type`)
//! ----------------------------------------------------------------------------
//! This module defines the `TileType` enum, which categorizes the material or
//! nature of a single tile in the SSXL engine. It is one of the most fundamental
//! data structures, used everywhere from chunk generation to rendering.
//!
//! Key design choices:
//!   - `#[repr(u8)]`: Forces the enum to be stored as a single byte. This is a
//!     critical memory optimization because each chunk contains thousands of tiles.
//!     Using one byte per tile type keeps memory usage predictable and efficient.
//!   - Serialization derives: Enables saving/loading tile data across FFI boundaries
//!     and persistent caches.
//!
//! Educational note:
//!   - Enums with `#[repr(u8)]` are often used in game engines to pack large grids
//!     of state into memory without wasting space.
//!   - Conversion helpers (`to_u8`, `from_u8`) make it easy to move between raw
//!     byte values and strongly typed variants.
//! ============================================================================

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// 📦 Core Enum: TileType
// -----------------------------------------------------------------------------
// Represents all canonical tile categories in the procedural world.
// Each variant corresponds to a material or terrain type.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)] 
pub enum TileType {
    Void = 0,       // Empty space (air/background)
    Water = 1,      // Fluid terrain (rivers, oceans)
    Grass = 2,      // Base terrain, flat and walkable
    Mountain = 3,   // Elevated terrain, harder to traverse
    Boundary = 4,   // Artificial boundary (chunk edges, isolation zones)
    Structure = 5,  // Engineered/placed structures (walls, ruins, roads)
    Rock = 6,       // Solid rocky terrain
    Custom1 = 7,    // Reserved for generator-specific customization
    Custom2 = 8,    // Reserved for generator-specific customization
}

// Maximum valid value for TileType when represented as a raw u8.
pub const MAX_TILE_TYPE_VALUE: u8 = 8;

// -----------------------------------------------------------------------------
// 🔧 Default Implementation
// -----------------------------------------------------------------------------
// Provides a predictable default: all tiles start as `Void` unless specified.
// -----------------------------------------------------------------------------
impl Default for TileType {
    fn default() -> Self {
        TileType::Void
    }
}

// -----------------------------------------------------------------------------
// 🔧 Conversion & Lookup Methods
// -----------------------------------------------------------------------------
// Utility functions for working with TileType in raw form or rendering contexts.
// -----------------------------------------------------------------------------
impl TileType {
    /// Converts the enum into its underlying `u8` representation.
    /// Useful for serialization or compact storage.
    #[inline(always)] 
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Converts a raw `u8` back into a `TileType`, if valid.
    /// Returns `None` if the value is outside the defined range.
    #[inline(always)]
    pub fn from_u8(value: u8) -> Option<Self> {
        if value <= MAX_TILE_TYPE_VALUE {
            // SAFETY: We checked that the value is within [0, 8].
            Some(unsafe { std::mem::transmute(value) })
        } else {
            None
        }
    }
    
    /// Provides a default unique ID for this tile type.
    /// By convention, this is just its `u8` value.
    #[inline(always)]
    pub const fn get_default_tile_id(self) -> u16 {
        self.to_u8() as u16
    }

    /// Returns default atlas coordinates (X, Y) for rendering.
    /// These are used by the rendering system to look up textures.
    #[inline(always)]
    pub const fn get_default_atlas_coords(self) -> (u16, u16) {
        match self {
            TileType::Water    => (1, 0),
            TileType::Grass    => (2, 0),
            TileType::Mountain => (3, 0),
            TileType::Boundary => (4, 0),
            TileType::Structure=> (5, 0),
            TileType::Rock     => (6, 0),
            _ => (0, 0), // Void and custom types default to origin
        }
    }
}

// -----------------------------------------------------------------------------
// 🔧 Semantic Helper Methods
// -----------------------------------------------------------------------------
// These methods provide higher-level meaning for tile types, used in gameplay
// logic such as collision, traversal, and fluid simulation.
// -----------------------------------------------------------------------------
impl TileType {
    /// Returns true if the tile can typically be walked on.
    #[inline(always)]
    pub const fn is_walkable(self) -> bool {
        matches!(self, TileType::Grass | TileType::Mountain | TileType::Structure | TileType::Rock)
    }

    /// Returns true if the tile is a fluid (e.g., water).
    #[inline(always)]
    pub const fn is_fluid(self) -> bool {
        matches!(self, TileType::Water)
    }

    /// Returns true if the tile is empty (Void).
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        matches!(self, TileType::Void)
    }
}
