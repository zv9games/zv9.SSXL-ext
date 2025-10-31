//! Core types and constants for the Aetherion math foundation layer.

use serde::{Serialize, Deserialize};

// -------------------------------------------------------------------------
// I. PRIMITIVE TYPES AND TRAITS (MUST BE PUB)
// -------------------------------------------------------------------------

/// 📐 **BULLDOZER FIX:** The canonical signed 2D integer vector for coordinate system logic (Chunk Coords).
/// Must use i64 to support a world scale that exceeds 2.1 billion chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2i {
    pub x: i64, // Updated from i32
    pub y: i64, // Updated from i32
}

/// A simplified, common result type.
pub type SSXLResult<T> = Result<T, String>;

/// A simplified, common data structure trait implemented by data structs like ChunkData.
pub trait SSXLData: Send + Sync {
    fn get_id(&self) -> u64; 
    fn get_value_len(&self) -> usize;
}

// -------------------------------------------------------------------------
// II. CORE CONSTANTS (MUST BE PUB)
// -------------------------------------------------------------------------

/// **BULLDOZER FIX:** The standard size (width, height, depth) of a chunk in tiles,
/// defined as i64 to ensure seamless arithmetic with the new i64-based Vec2i and GridBounds.
pub const CHUNK_SIZE_I64: i64 = 32;

/// Standard epsilon value for floating-point comparisons (f32).
pub const F32_EPSILON: f32 = 1.0e-6;

// -------------------------------------------------------------------------
// III. IMPLEMENTATION
// -------------------------------------------------------------------------

impl Vec2i {
    pub fn new(x: i64, y: i64) -> Self { // Updated arguments to i64
        Vec2i { x, y }
    }
}