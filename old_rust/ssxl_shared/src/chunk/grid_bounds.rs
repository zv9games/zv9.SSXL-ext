use crate::Serialize;
use crate::Deserialize;
// -----------------------------------------------------------------------------
// Grid Bounds Module Overview
// -----------------------------------------------------------------------------
// This module defines the fundamental 2D coordinate and bounding box structures
// used throughout the SSXL engine. These are essential for representing world-space
// positions and rectangular regions (chunks, tiles, or arbitrary areas).
//
// Key Components:
// - Coord2D: A single point in 2D world space.
// - GridBounds: A rectangular bounding box defined by min and max coordinates.
// -----------------------------------------------------------------------------
//
// Coord2D
// -----------------------------------------------------------------------------
// Purpose:
//   - Represents a single 2D coordinate in world space.
//   - Uses i64 to support extremely large coordinate ranges (beyond i32 limits).
// Derives:
//   - Debug, Clone, Copy: for easy inspection and duplication.
//   - PartialEq, Eq, PartialOrd, Ord, Hash: for comparisons and use in collections.
//   - Serialize, Deserialize: for persistence and networking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Coord2D {
    pub x: i64, // X component of the coordinate
    pub y: i64, // Y component of the coordinate
}

// -----------------------------------------------------------------------------
// GridBounds
// -----------------------------------------------------------------------------
// Purpose:
//   - Represents a rectangular region in 2D world space.
//   - Defined by inclusive minimum (min) and exclusive maximum (max) coordinates.
// Convention:
//   - Half-open range: [min, max)
//     * min is inclusive
//     * max is exclusive
//   - This ensures correct size calculation and avoids off-by-one errors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GridBounds {
    pub min: Coord2D, // Inclusive minimum coordinate (bottom-left corner)
    pub max: Coord2D, // Exclusive maximum coordinate (one past top-right corner)
}

impl GridBounds {
    // -------------------------------------------------------------------------
    // Constructor: new
    // -------------------------------------------------------------------------
    // Creates a new GridBounds from explicit min and max coordinates.
    // Arguments:
    //   - min_x, min_y: inclusive minimum coordinates
    //   - max_x, max_y: exclusive maximum coordinates
    pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Self {
        GridBounds {
            min: Coord2D { x: min_x, y: min_y },
            max: Coord2D { x: max_x, y: max_y },
        }
    }

    // -------------------------------------------------------------------------
    // Method: size
    // -------------------------------------------------------------------------
    // Calculates the width and height of the bounds.
    // Formula:
    //   size.x = max.x - min.x
    //   size.y = max.y - min.y
    // Works correctly with half-open ranges, yielding the number of integer
    // coordinates contained within the bounds.
    pub fn size(&self) -> Coord2D {
        Coord2D {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
        }
    }

    // -------------------------------------------------------------------------
    // Method: contains
    // -------------------------------------------------------------------------
    // Checks if a given coordinate lies within the bounds.
    // Follows half-open range convention:
    //   - min.x <= coord.x < max.x
    //   - min.y <= coord.y < max.y
    // Returns:
    //   - true if inside bounds
    //   - false otherwise
    pub fn contains(&self, coord: Coord2D) -> bool {
        coord.x >= self.min.x && coord.x < self.max.x &&
        coord.y >= self.min.y && coord.y < self.max.y
    }
}

// -----------------------------------------------------------------------------
// Default Implementation
// -----------------------------------------------------------------------------
// Provides a default GridBounds instance.
//   - min = (0,0)
//   - max = (0,0)
// Represents a zero-sized bounds at the origin.
impl Default for GridBounds {
    fn default() -> Self {
        GridBounds {
            min: Coord2D { x: 0, y: 0 },
            max: Coord2D { x: 0, y: 0 },
        }
    }
}
