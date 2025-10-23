
// Handles abstraction between 2D and 3D dimensions.
// Will evolve to support dynamic switching, shared logic, and editor integration.
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;


use std::fmt;

/// Represents the dimensionality of the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    /// Two-dimensional space (tilemaps, flat terrain).
    TwoD,

    /// Three-dimensional space (volumetric terrain, meshes).
    ThreeD,
}

impl Dimension {
    /// Returns true if the dimension is 2D.
    pub fn is_2d(&self) -> bool {
        matches!(self, Dimension::TwoD)
    }

    /// Returns true if the dimension is 3D.
    pub fn is_3d(&self) -> bool {
        matches!(self, Dimension::ThreeD)
    }

    /// Returns a human-readable name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Dimension::TwoD => "2D",
            Dimension::ThreeD => "3D",
        }
    }

    /// Returns the opposite dimension.
    pub fn flipped(&self) -> Self {
        match self {
            Dimension::TwoD => Dimension::ThreeD,
            Dimension::ThreeD => Dimension::TwoD,
        }
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::TwoD
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


// the end