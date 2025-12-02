// ssxl_shared/src/grid_bounds.rs

//! # Grid Bounds Structures (`ssxl_shared::grid_bounds`)
//!
//! This module defines the core structures for representing 2D world-space points
//! and rectangular bounding boxes (bounds) used throughout the SSXL-ext engine.

use serde::{Deserialize, Serialize};

// --- Coordinate Structure ---

/// Represents a single 2D world-space coordinate point.
///
/// Uses `i64` to support the massive scale of the SSXL world coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Coord2D {
    /// The X component of the coordinate.
    pub x: i64,
    /// The Y component of the coordinate.
    pub y: i64,
}

// --- Bounding Box Structure ---

/// Defines a rectangular region in the world by its minimum and maximum coordinate points.
///
/// **Convention:** `GridBounds` uses a **half-open range** (`[min, max)`), meaning
/// the minimum coordinates are **inclusive** and the maximum coordinates are **exclusive**.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GridBounds {
    /// The inclusive minimum coordinate (e.g., the bottom-left point of the region).
    pub min: Coord2D,
    /// The exclusive maximum coordinate (one unit past the top-right point of the region).
    pub max: Coord2D,
}

impl GridBounds {
    /// Creates a new `GridBounds` instance from four explicit coordinate components.
    pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Self {
        GridBounds {
            min: Coord2D { x: min_x, y: min_y },
            max: Coord2D { x: max_x, y: max_y },
        }
    }

    /// Calculates the size (width and height) of the bounds.
    ///
    /// For a half-open range, the size is simply `max - min`, which correctly
    /// yields the number of unique integer coordinates contained within the bounds.
    pub fn size(&self) -> Coord2D {
        Coord2D {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
        }
    }

    /// Checks if a given coordinate is contained within the bounds.
    ///
    /// Follows the half-open range convention: `[min.x, max.x)` and `[min.y, max.y)`.
    pub fn contains(&self, coord: Coord2D) -> bool {
        coord.x >= self.min.x && coord.x < self.max.x &&
        coord.y >= self.min.y && coord.y < self.max.y
    }
}

impl Default for GridBounds {
    /// Returns a default, zero-sized bounds at the origin (0, 0) to (0, 0).
    fn default() -> Self {
        GridBounds {
            min: Coord2D { x: 0, y: 0 },
            max: Coord2D { x: 0, y: 0 },
        }
    }
}