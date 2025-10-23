use crate::zv9_prelude::*;

/// Returns the 4 cardinal neighbors of a tile.
pub fn cardinal_neighbors(pos: SerializableVector2i) -> Vec<SerializableVector2i> {
    vec![
        SerializableVector2i { x: pos.x, y: pos.y - 1 },
        SerializableVector2i { x: pos.x + 1, y: pos.y },
        SerializableVector2i { x: pos.x, y: pos.y + 1 },
        SerializableVector2i { x: pos.x - 1, y: pos.y },
    ]
}

/// Returns the 8 surrounding neighbors of a tile.
pub fn all_neighbors(pos: SerializableVector2i) -> Vec<SerializableVector2i> {
    let mut neighbors = Vec::with_capacity(8);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                neighbors.push(SerializableVector2i {
                    x: pos.x + dx,
                    y: pos.y + dy,
                });
            }
        }
    }
    neighbors
}

/// Checks if a position is within bounds.
pub fn in_bounds(pos: SerializableVector2i, width: i32, height: i32) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < width && pos.y < height
}
