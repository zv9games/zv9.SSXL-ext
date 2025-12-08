// ============================================================================
// 🧮 Math Module Index
// File: src/math/mod.rs
// ----------------------------------------------------------------------------
// In Rust, a `mod.rs` file serves as the "table of contents" for a directory.
// This file declares which submodules belong to the `math/` namespace.
//
// Purpose of the `math` module:
//   • Organize math-related helpers and primitives in one place.
//   • Provide deterministic serialization utilities for types like `SystemTime`.
//   • Keep the crate structure clean by grouping math logic separately from
//     chunks, tiles, and messaging.
//
// Why this matters:
//   • Other parts of the engine (e.g., chunk generation, caching) rely on
//     math primitives for consistent behavior.
//   • Declaring `pub mod math_primitives;` here makes the file
//     `src/math/math_primitives.rs` available as `crate::math::math_primitives`.
//   • This ensures imports like `use crate::math::math_primitives;` compile
//     correctly without unresolved path errors.
// ============================================================================

// Declare the `math_primitives` submodule.
// This corresponds to the file: src/math/math_primitives.rs
pub mod math_primitives;
