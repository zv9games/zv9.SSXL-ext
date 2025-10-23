
/// ✅ Suggestions for shared/grid2d.rs

// 🔧 Add bounds-checking utilities:
//     - `fn in_bounds(&self, x: usize, y: usize) -> bool`
//     - Improves clarity and reuse across `get()` and `set()`

// 🧩 Add iteration and mapping support:
//     - `fn iter(&self) -> impl Iterator<Item = &T>`
//     - `fn map<U: Clone>(...) -> Grid2D<U>`
//     - Enables functional-style transformations and traversal

// 🚦 Add coordinate-based accessors:
//     - `fn get_at(&self, pos: SerializableVector2i)`
//     - `fn set_at(&mut self, pos: SerializableVector2i, value: T)`
//     - Useful for grid systems already using vector types

// 📚 Document indexing assumptions:
//     - Clarify row-major layout and how (x, y) maps to `data[y * width + x]`
//     - Prevent confusion in procedural or rendering logic

// 🧪 Add unit tests for access and mutation:
//     - Validate edge cases, out-of-bounds behavior, and fill logic

// 🧼 Optional: Add debug visualization:
//     - `fn summary(&self) -> String` or `impl Display`
//     - Useful for logging or visual inspection of grid contents

// 🚀 Future: Add resizing or cropping:
//     - `fn resize(&mut self, new_width, new_height, default: T)`
//     - Enables dynamic terrain or map expansion

// 🧠 Consider exposing grid metadata:
//     - e.g. `fn total_cells(&self) -> usize`, `fn is_empty(&self) -> bool`
//     - Useful for diagnostics, analytics, or conditional logic
#[allow(unused_imports)]
use crate::zv9_prelude::*;


#[derive(Debug, Clone)]
pub struct Grid2D<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Grid2D<T> {
    pub fn filled(width: usize, height: usize, value: T) -> Self {
        let data = vec![value; width * height];
        Grid2D { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.data.get(y * self.width + x)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }
}
