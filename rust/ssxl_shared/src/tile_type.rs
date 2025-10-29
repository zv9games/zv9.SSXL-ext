//! Defines the canonical set of fundamental types that a Tile can represent.
//!
//! This enum is used by the generation modules to assign meaning to raw noise values,
//! and by the rendering engine (Godot) to select the correct visual asset.

use serde::{Serialize, Deserialize};

/// The fundamental, physical classification of a tile, aligned with the MVG schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)] // Ensures compact storage for cache serialization
pub enum TileType {
    /// 0: The default, empty, or uninitialized state (Void/Air).
    Void = 0,
    /// 1: Represents a water body (V < 0.3).
    Water = 1,
    /// 2: Represents standard ground/plains (0.3 <= V < 0.6).
    Grass = 2,
    /// 3: Represents high-elevation terrain (V >= 0.6).
    Mountain = 3,
    /// 4: Represents a boundary or special-condition tile that cannot be traversed or modified easily.
    Boundary = 4,
    /// 5: Reserved for future structured objects/built environment (Structure/Roads).
    Structure = 5,
    /// 6: **(CA GENERATOR)** Represents solid ground or a wall, used as the 'live' cell by the Cellular Automata Generator.
    Rock = 6,
    /// 7: Reserved for future expansion or custom user types.
    Custom1 = 7,
    /// 8: Reserved for future expansion or custom user types.
    Custom2 = 8,
}

// ---------------------------
// IMPL: Default and Conversion
// ---------------------------

impl Default for TileType {
    /// The default state of a tile is non-solid and empty.
    fn default() -> Self {
        TileType::Void
    }
}

impl TileType {
    /// Helper function to convert the enum variant into its underlying u8 representation.
    #[inline] // Hint to the compiler for potential inlining for performance
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Attempts to convert a u8 into a TileType. Returns None if the value is outside the defined range.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TileType::Void),
            1 => Some(TileType::Water),
            2 => Some(TileType::Grass),
            3 => Some(TileType::Mountain),
            4 => Some(TileType::Boundary),
            5 => Some(TileType::Structure),
            // FIX: Added Rock
            6 => Some(TileType::Rock), 
            // FIX: Updated Custom variants' values
            7 => Some(TileType::Custom1),
            8 => Some(TileType::Custom2),
            _ => None,
        }
    }
}

// ---------------------------
// IMPL: Gameplay Logic Helpers
// ---------------------------

impl TileType {
    /// Checks if the tile type indicates a ground-based, traversable surface.
    pub const fn is_walkable(self) -> bool {
        // FIX: Added Rock as a walkable surface
        matches!(self, TileType::Grass | TileType::Mountain | TileType::Structure | TileType::Rock)
    }

    /// Checks if the tile type indicates a liquid that typically requires fluid dynamics simulation.
    pub const fn is_fluid(self) -> bool {
        matches!(self, TileType::Water)
    }

    /// Checks if the tile is a placeholder state that hasn't been generated yet.
    pub const fn is_empty(self) -> bool {
        matches!(self, TileType::Void)
    }
}