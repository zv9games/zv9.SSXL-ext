// File: ssxl_math/src/lib.rs

// --- 1. Module Declarations (CRITICAL: MUST be at the root) ---
/// Defines the global and local coordinate system structures (`ChunkKey`, `WorldPos`).
pub mod coordinate_system;

/// Houses various utility functions for procedural generation algorithms.
pub mod generation_utils;

/// Provides deterministic hashing functions (e.g., `hash_chunk_coords`).
pub mod hashing;

/// Core mathematical primitives, custom types, and error handling results.
pub mod primitives;

// --------------------------------------------------------------------------------

// --- 2. Cold Quantum Fast Implementation (FTL, Safety-Optimized) ---

/// FTL Inverse Square Root: **O(1)** Approximation for vector normalization.
///
#[inline(always)]
pub fn q_rsqrt(number: f32) -> f32 {
    // O(1) Safety Guard: Prevents NaN/UB when magnitude is zero or negative.
    if number <= 0.0 { 
        return 0.0;
    }

    const THREEHALFS: f32 = 1.5;
    let x2 = number * 0.5;
    let y = number;
    
    // Use u32 for bitwise operation.
    let i = y.to_bits();
    let j_bits = 0x5f3759df_u32.wrapping_sub(i >> 1);

    let mut y = f32::from_bits(j_bits);
    
    // Single Newton's iteration.
    y = y * (THREEHALFS - (x2 * y * y));
    y
}

/// Computes the normalized (unit) vector of a 3D float vector.
pub fn normalize_vector_3d(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let mag_sq = x * x + y * y + z * z;
    let inv_mag = q_rsqrt(mag_sq); 
    
    (x * inv_mag, y * inv_mag, z * inv_mag)
}

// --------------------------------------------------------------------------------

// --- 3. Prelude for Internal Engine Use (Minimalist API) ---

/// A convenience module that re-exports all essential types and traits
/// from the `ssxl_math` crate.
pub mod prelude {
    // Use 'super::' consistently to access items defined in the parent scope (lib.rs).
    pub use super::coordinate_system::*;
    pub use super::generation_utils::*;
    pub use super::hashing::*;
    pub use super::primitives::*;
    
    pub use super::q_rsqrt;
    pub use super::normalize_vector_3d;
}