// ssxl_shared/src/grid_bounds.rs
//! Defines the spatial boundaries for a Chunk or any defined area within the procedural world.
//!
//! Uses i64 coordinates to support extremely large, super-massive, expansive arcs.

use serde::{Serialize, Deserialize};

/// Represents a simple 2D integer coordinate (X, Y).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Coord2D {
    pub x: i64,
    pub y: i64,
}

/// Defines the minimum (min) and maximum (max) corners of a rectangular area
/// in the Aetherion world space, typically used to bound a Chunk or Region.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GridBounds {
    /// The inclusive minimum corner of the bounding box (bottom-left).
    pub min: Coord2D,
    /// The exclusive maximum corner of the bounding box (top-right + 1).
    pub max: Coord2D,
}

impl GridBounds {
    /// Creates a new GridBounds instance.
    pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Self {
        GridBounds {
            min: Coord2D { x: min_x, y: min_y },
            max: Coord2D { x: max_x, y: max_y },
        }
    }

    /// Calculates the size of the bounded area along the X and Y axes.
    /// The size is calculated as max - min.
    pub fn size(&self) -> Coord2D {
        Coord2D {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
        }
    }

    /// Checks if a given coordinate is contained within these bounds (inclusive minimum, exclusive maximum).
    pub fn contains(&self, coord: Coord2D) -> bool {
        coord.x >= self.min.x && coord.x < self.max.x &&
        coord.y >= self.min.y && coord.y < self.max.y
    }
}

impl Default for GridBounds {
    /// Provides a default (zero-sized) GridBounds instance.
    fn default() -> Self {
        GridBounds {
            min: Coord2D { x: 0, y: 0 },
            max: Coord2D { x: 0, y: 0 },
        }
    }
}
