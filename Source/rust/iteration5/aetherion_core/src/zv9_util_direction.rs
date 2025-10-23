

#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction {
    pub dx: i32,
    pub dy: i32,
}

impl Direction {
    pub const UP: Direction = Direction { dx: 0, dy: -1 };
    pub const RIGHT: Direction = Direction { dx: 1, dy: 0 };
    pub const DOWN: Direction = Direction { dx: 0, dy: 1 };
    pub const LEFT: Direction = Direction { dx: -1, dy: 0 };

    /// Returns all four cardinal directions.
    pub fn all() -> [Direction; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }

    /// Returns the direction to the left (counter-clockwise).
    pub fn left(&self) -> Self {
        match *self {
            Self::UP => Self::LEFT,
            Self::LEFT => Self::DOWN,
            Self::DOWN => Self::RIGHT,
            Self::RIGHT => Self::UP,
            _ => *self,
        }
    }

    /// Returns the direction to the right (clockwise).
    pub fn right(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => *self,
        }
    }

    /// (Optional) Returns the opposite direction.
    pub fn reverse(&self) -> Self {
        match *self {
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
            Self::RIGHT => Self::LEFT,
            _ => *self,
        }
    }
}
