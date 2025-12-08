// ============================================================================
// 🧮 SSXL Math Module (`crate::ssxl_math`)
// ----------------------------------------------------------------------------
// This module defines the mathematical foundation of the SSXL engine. It
// provides core coordinate types, generation utilities, hashing functions,
// and primitive constants, along with specialized math routines for fast
// vector operations.
//
// Purpose:
//   • Organize math-related functionality into cohesive submodules.
//   • Provide efficient algorithms for vector normalization and hashing.
//   • Offer a prelude for convenient re-exports of essential math utilities.
//
// Module Structure:
//   • coordinate_system
//       - Defines core coordinate types (WorldPos, ChunkKey, TileOffset).
//       - Handles conversion between global world positions and chunk-local offsets.
//   • generation_utils
//       - Provides utility functions for procedural generation.
//       - Includes randomness utilities (percent rolls) and data processing helpers.
//   • hashing
//       - Provides deterministic SHA-256 hashing functions.
//       - Used for chunk IDs, cache keys, and ensuring stable procedural generation.
//   • primitives
//       - Defines core mathematical constants, types, and result/error handling.
//       - Acts as the foundation for other modules.
//
// Special Functions:
//   • q_rsqrt(number: f32) -> f32
//       - Fast inverse square root approximation.
//       - Famous algorithm from Quake III Arena, adapted here for safe use.
//       - Returns an approximation of 1/sqrt(number).
//       - Uses bit-level manipulation and one Newton-Raphson refinement.
//       - Safety guard: returns 0.0 if input <= 0.0.
//
//   • normalize_vector_3d(x: f32, y: f32, z: f32) -> (f32, f32, f32)
//       - Computes the unit vector (normalized vector) of a 3D vector.
//       - Uses q_rsqrt for fast inverse square root approximation.
//       - Returns a tuple representing the normalized vector.
//
// Prelude Module:
//   • Provides a convenience re-export of all essential types and functions.
//   • Allows other crates to import `ssxl_math::prelude::*` for quick access.
//   • Re-exports coordinate_system, generation_utils, hashing, primitives,
//     along with q_rsqrt and normalize_vector_3d.
//
// Educational Note:
//   • This module demonstrates how to combine classical math algorithms
//     (like fast inverse square root) with modern Rust abstractions.
//   • By organizing math utilities into submodules and exposing a prelude,
//     SSXL ensures both clarity and convenience for developers working on
//     large-scale procedural generation and simulation systems.
// ============================================================================


pub mod coordinate_system;
pub mod generation_utils;
pub mod hashing;
pub mod primitives;

#[inline(always)]
pub fn q_rsqrt(number: f32) -> f32 {
    if number <= 0.0 { 
        return 0.0;
    }

    const THREEHALFS: f32 = 1.5;

    let x2 = number * 0.5;
    let y = number;
    
    let i = y.to_bits();
    let j_bits = 0x5f3759df_u32.wrapping_sub(i >> 1);

    let mut y = f32::from_bits(j_bits);
    
    y = y * (THREEHALFS - (x2 * y * y));
    y
}

pub fn normalize_vector_3d(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let mag_sq = x * x + y * y + z * z;
    let inv_mag = q_rsqrt(mag_sq); 
    
    (x * inv_mag, y * inv_mag, z * inv_mag)
}

pub mod prelude {
    pub use super::coordinate_system::*;
    pub use super::generation_utils::*;
    pub use super::hashing::*;
    pub use super::primitives::*;
    
    pub use super::q_rsqrt;
    pub use super::normalize_vector_3d;
}
