//! Utility module for core functions used by the generation pipeline.

use crate::primitives::SSXLData; 
// FIX: Simplified imports. thread_rng is now used, removing the need for SeedableRng, Pcg64, etc.
use rand::Rng; 

// -----------------------------------------------------------------------------
// I. RANDOMNESS UTILITIES
// -----------------------------------------------------------------------------

/// A simple, thread-local generator for random values.
/// This uses the standard library's thread-local RNG (`rand::thread_rng()`)
/// to efficiently generate random numbers, bypassing the complex manual
/// setup that caused trait resolution errors.
/// 
/// Returns `0` if a generated random value (0-99) is less than `target_percent`.
pub fn generate_percent_roll(target_percent: u8) -> u32 {
    // thread_rng() returns a thread-local, seeded generator.
    let rand_val = rand::thread_rng().gen_range(0..100) as u8;

    if rand_val < target_percent {
        0
    } else {
        1
    }
}

// -----------------------------------------------------------------------------
// II. CORE PROCESS UTILITIES
// -----------------------------------------------------------------------------

/// Processes the input data, applying a placeholder mathematical transformation.
///
/// # Arguments
///
/// * `data` - A reference to the core `AetherionData` structure containing relevant state.
///
/// # Returns
///
/// A simple derived u64 value.
pub fn process_data(data: &impl SSXLData) -> u64 {
    // Uses the trait methods defined on SSXLData in ssxl_math/src/primitives.rs
    let processed_value = data.get_id() + data.get_value_len() as u64;
    processed_value
}