

#[allow(unused_imports)]
use crate::zv9_prelude::*;

use serde::{Deserialize, Serialize};

/// Serializable wrapper for Godot's `Vector2i`.
/// Used in map data, tile metadata, and chunk streaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vector2i> for SerializableVector2i {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl SerializableVector2i {
    /// Converts this wrapper into a Godot-native `Vector2i`.
    pub fn to_vector2i(&self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }
}

// the end