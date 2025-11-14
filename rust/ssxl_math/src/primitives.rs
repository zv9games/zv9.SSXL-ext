//! # Primitives Module (`ssxl_math::primitives`)
//!
//! Defines the essential, low-level data types, type aliases, traits, and global
//! constants used throughout the SSXL-ext procedural generation engine.

use serde::{Deserialize, Serialize};

// --- Data Structures ---

/// A 2D vector for integer coordinates, typically used for tile offsets or local
/// coordinate mapping within a chunk.
///
/// Uses `i64` to maintain compatibility with the large coordinate space of `I64Vec3`
/// from the `glam` crate, preventing silent overflow issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2i {
    /// The X component of the 2D vector.
    pub x: i64,
    /// The Y component of the 2D vector.
    pub y: i64,
}

impl Vec2i {
    /// Creates a new `Vec2i` instance.
    pub fn new(x: i64, y: i64) -> Self {
        Vec2i { x, y }
    }
}

// --- Type Aliases ---

/// The standard 2D coordinate type for referencing a tile in the world
/// or within a chunk. Aliased to `Vec2i`.
pub type TileCoord = Vec2i; 

/// The standard 2D coordinate type for referencing a spatial chunk in the world grid.
/// Aliased to `Vec2i` for memory layout consistency with `TileCoord`.
pub type ChunkId = Vec2i; 

/// A specialized `Result` type for the SSXL-ext project.
///
/// The error type is fixed as `String`, providing a simple, high-level way to
/// convey error messages across the engine's various crates.
pub type SSXLResult<T> = Result<T, String>;

// --- Traits for Engine Data Management ---

/// A trait defining the requirements for any data structure that will be managed
/// or processed by the SSXL engine (e.g., in the task queue or cache).
///
/// The bounds `Send + Sync` are mandatory, ensuring all implementors can be
/// safely sent between worker threads and shared across thread boundaries.
pub trait SSXLData: Send + Sync {
    /// Retrieves a unique 64-bit ID for the data. Used primarily for cache keys
    /// and tracking within the `task_queue`.
    fn get_id(&self) -> u64;

    /// Returns the length or size of the data's core value in bytes or elements.
    /// Used for diagnostics, memory management, or processing limits.
    fn get_value_len(&self) -> usize;
}

// --- Global Constants ---

/// The canonical side length of a procedural chunk in the world.
///
/// This value is cast to `i64` to match the coordinate system of the engine,
/// ensuring consistent type usage for chunk-related calculations.
pub const CHUNK_SIZE_I64: i64 = 32;

/// A small constant used for floating-point comparisons to account for
/// precision errors (e.g., in perlin noise interpolation or physics-related math).
pub const F32_EPSILON: f32 = 1.0e-6;