// ssxl_math/src/primitives.rs
//! Core types and constants for the Aetherion math foundation layer.

use serde::{Serialize, Deserialize};

// -------------------------------------------------------------------------
// I. PRIMITIVE TYPES AND TRAITS (MUST BE PUB)
// -------------------------------------------------------------------------

/// The canonical signed 2D integer vector for coordinate system logic (Chunk Coords).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

/// A simplified, common result type.
pub type SSXLResult<T> = Result<T, String>;

/// A simplified, common data structure trait implemented by data structs like ChunkData.
pub trait SSXLData: Send + Sync {
    // FIX: Removed 'pub' keyword from trait methods (E0449 resolved)
    fn get_id(&self) -> u64; 
    fn get_value_len(&self) -> usize;
}

// -------------------------------------------------------------------------
// II. CORE CONSTANTS (MUST BE PUB)
// -------------------------------------------------------------------------

/// The standard size (width, height, depth) of a chunk in tiles.
pub const CHUNK_SIZE_I32: i32 = 32;

/// Standard epsilon value for floating-point comparisons (f32).
pub const F32_EPSILON: f32 = 1.0e-6;

// -------------------------------------------------------------------------
// III. IMPLEMENTATION
// -------------------------------------------------------------------------

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
}