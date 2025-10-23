use godot::prelude::*;
use aetherion_core::zv9_prelude::*;

/// Local wrapper for converting SerializableVector2i into Godot's native Vector2i.
#[derive(Debug, Clone, Copy)]
pub struct GodotVec2i(pub SerializableVector2i);

impl From<GodotVec2i> for Vector2i {
    fn from(value: GodotVec2i) -> Self {
        Vector2i::new(value.0.x, value.0.y)
    }
}
