use std::ops::AddAssign;
use serde::{Serialize, Deserialize};

#[allow(unused_imports)]
use crate::zv9_prelude::*;

// ─── Type Aliases ─────────────────────────────────────────────
pub type Coord = (i32, i32);
pub type Timestamp = u64;

// ─── Entity ID ────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn value(self) -> u64 {
        self.0
    }

    pub fn from_raw(id: u64) -> Self {
        EntityId(id)
    }
}

// ─── Serializable Vector ──────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

// ─── Direction ────────────────────────────────────────────────
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

    pub fn all() -> [Direction; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }

    pub fn left(&self) -> Self {
        match *self {
            Self::UP => Self::LEFT,
            Self::LEFT => Self::DOWN,
            Self::DOWN => Self::RIGHT,
            Self::RIGHT => Self::UP,
            _ => *self,
        }
    }

    pub fn right(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => *self,
        }
    }

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

// ─── Velocity ─────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

// ─── Position ─────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(&self, direction: Direction) -> Self {
        Self {
            x: self.x + direction.dx,
            y: self.y + direction.dy,
        }
    }

    pub fn min(self, other: Position) -> Position {
        Position {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Position) -> Position {
        Position {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn distance_to(&self, other: Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

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
