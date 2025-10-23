use aetherion_shared::zv9_prelude::*;
use std::ops::AddAssign;
use crate::zv9_util_direction::Direction;
use crate::zv9_util_velocity::Velocity;


/// A 2D grid position used for tile placement, movement, and spatial queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Creates a new position from coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a new position stepped in the given direction.
    pub fn step(&self, direction: Direction) -> Self {
        Self {
            x: self.x + direction.dx,
            y: self.y + direction.dy,
        }
    }

    /// Returns the minimum of two positions (component-wise).
    pub fn min(self, other: Position) -> Position {
        Position {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the maximum of two positions (component-wise).
    pub fn max(self, other: Position) -> Position {
        Position {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Returns the Euclidean distance to another position.
    pub fn distance_to(&self, other: Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Converts this position to a serializable vector.
    pub fn to_serializable(&self) -> SerializableVector2i {
        SerializableVector2i { x: self.x, y: self.y }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.dx as i32;
        self.y += rhs.dy as i32;
    }
}

impl From<Position> for SerializableVector2i {
    fn from(pos: Position) -> Self {
        SerializableVector2i { x: pos.x, y: pos.y }
    }
}
