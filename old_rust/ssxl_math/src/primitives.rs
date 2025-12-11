// ============================================================================
// 🔧 Primitives Module (`crate::primitives`)
// ----------------------------------------------------------------------------
// This module defines the low-level building blocks of the SSXL engine.
// It provides core types, traits, and constants that form the mathematical
// and structural foundation for higher-level systems like coordinate handling,
// generation utilities, and hashing.
//
// Purpose:
//   • Define canonical vector types for consistent coordinate representation.
//   • Provide semantic type aliases for clarity in APIs.
//   • Establish a lightweight, project-wide result type for error handling.
//   • Define a minimal trait (`SSXLData`) for generic data management.
//   • Expose global constants for chunk sizing and floating-point tolerance.
//
// Key Components:
//   • Vec2i
//       - A 2D integer vector using i64 components.
//       - Represents tile or chunk coordinates in 2D space.
//       - Aligns with the engine’s 64-bit world math (I64Vec3).
//       - Prevents overflow when combined with large-scale coordinates.
//       - Provides a constructor `new(x, y)` for explicit initialization.
//
//   • Type Aliases
//       - TileCoord: semantic alias for a tile’s coordinate in 2D space.
//       - ChunkId: semantic alias for a chunk’s identifier in a world grid.
//       - SSXLResult<T>: project-wide result type using `Result<T, String>`,
//         favoring human-readable error messages over custom enums.
//
//   • Trait: SSXLData
//       - Minimal contract for data managed by the engine (task queues, caches).
//       - Requires `Send + Sync` for safe concurrency across threads.
//       - Methods:
//           • get_id() -> u64: stable identifier for indexing and deduplication.
//           • get_value_len() -> usize: size of payload, useful for diagnostics.
//
//   • Constants
//       - CHUNK_SIZE_I64: canonical cubic side length for procedural chunks (32).
//         Uses i64 to align with engine math and avoid casting pitfalls.
//       - F32_EPSILON: small tolerance (1e-6) for floating-point comparisons,
//         used in math-heavy routines like normalization or interpolation.
//
// Design Choices:
//   • Using i64 for coordinates ensures compatibility with infinite world math.
//   • Semantic type aliases improve readability without runtime overhead.
//   • SSXLResult standardizes error handling across modules.
//   • SSXLData provides a minimal, flexible trait for generic data processing.
//
// Educational Note:
//   • This module demonstrates how to establish a clean foundation for an engine.
//     By defining primitives here, higher-level systems can build on consistent,
//     reusable abstractions, ensuring clarity, safety, and scalability across
//     the entire codebase.
// ============================================================================


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
}

impl Vec2i {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2i { x, y }
    }
}

pub type TileCoord = Vec2i; 
pub type ChunkId = Vec2i; 
pub type SSXLResult<T> = Result<T, String>;

pub trait SSXLData: Send + Sync {
    fn get_id(&self) -> u64;
    fn get_value_len(&self) -> usize;
}

pub const CHUNK_SIZE_I64: i64 = 32;
pub const F32_EPSILON: f32 = 1.0e-6;
