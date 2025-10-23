use crate::zv9_prelude::*;
use std::fmt;

/// Defines a rectangular region in grid space.
/// Used for placement, chunking, and spatial queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridBounds {
    pub origin: SerializableVector2i,
    pub width: i32,
    pub height: i32,
}

impl GridBounds {
    /// Creates a new GridBounds from origin and size.
    pub fn new(origin: SerializableVector2i, size: SerializableVector2i) -> Self {
        Self {
            origin,
            width: size.x.max(0),
            height: size.y.max(0),
        }
    }

    /// Returns true if the given position is inside the bounds.
    pub fn contains(&self, pos: SerializableVector2i) -> bool {
        let rel_x = pos.x - self.origin.x;
        let rel_y = pos.y - self.origin.y;
        rel_x >= 0 && rel_y >= 0 && rel_x < self.width && rel_y < self.height
    }

    /// Returns the center point of the bounds.
    pub fn center(&self) -> SerializableVector2i {
        SerializableVector2i {
            x: self.origin.x + self.width / 2,
            y: self.origin.y + self.height / 2,
        }
    }

    /// Returns the four corners of the bounds.
    pub fn corners(&self) -> [SerializableVector2i; 4] {
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.width;
        let h = self.height;

        [
            SerializableVector2i { x: ox, y: oy },
            SerializableVector2i { x: ox + w, y: oy },
            SerializableVector2i { x: ox, y: oy + h },
            SerializableVector2i { x: ox + w, y: oy + h },
        ]
    }

    /// Expands the bounds by the given delta.
    pub fn expand(&mut self, dx: i32, dy: i32) {
        self.width = (self.width + dx).max(0);
        self.height = (self.height + dy).max(0);
    }

    /// Shifts the origin by the given offset.
    pub fn shift_origin(&mut self, offset: SerializableVector2i) {
        self.origin.x += offset.x;
        self.origin.y += offset.y;
    }

    /// Returns true if this bounds intersects another.
    pub fn intersects(&self, other: &GridBounds) -> bool {
        let ax1 = self.origin.x;
        let ay1 = self.origin.y;
        let ax2 = ax1 + self.width;
        let ay2 = ay1 + self.height;

        let bx1 = other.origin.x;
        let by1 = other.origin.y;
        let bx2 = bx1 + other.width;
        let by2 = by1 + other.height;

        ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1
    }

    /// Returns true if this bounds fully contains another.
    pub fn contains_bounds(&self, other: &GridBounds) -> bool {
        let ax1 = self.origin.x;
        let ay1 = self.origin.y;
        let ax2 = ax1 + self.width;
        let ay2 = ay1 + self.height;

        let bx1 = other.origin.x;
        let by1 = other.origin.y;
        let bx2 = bx1 + other.width;
        let by2 = by1 + other.height;

        bx1 >= ax1 && bx2 <= ax2 && by1 >= ay1 && by2 <= ay2
    }

    /// Returns an iterator over all positions within the bounds.
    pub fn iter(&self) -> impl Iterator<Item = SerializableVector2i> {
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.width;
        let h = self.height;

        (0..h).flat_map(move |dy| {
            (0..w).map(move |dx| SerializableVector2i {
                x: ox + dx,
                y: oy + dy,
            })
        })
    }
}

impl fmt::Display for GridBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GridBounds(origin=({}, {}), size={}x{})",
            self.origin.x, self.origin.y, self.width, self.height
        )
    }
}
