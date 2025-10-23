// ssxl_math/src/generation_utils.rs
//! Utility module for core functions used by the generation pipeline.

// FIX: Corrected the module path from 'math_primitives' to 'primitives'
use crate::primitives::SSXLData; 
use std::time::SystemTime; // Required for seeding fast_rand

/// 🛠️ Implements a basic placeholder fast random utility for CA seeding.
///
/// Returns `0` if a generated random value (0-99) is less than `target_percent`.
/// This is used to randomly select tiles for the initial Cellular Automata seed.
pub fn fast_rand(target_percent: u8) -> u32 {
    // Uses time/memory address hash for a quick, non-cryptographic pseudo-random seed
    let seed = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
        
    // Simple modulo 100 to get a value for percent chance check
    let rand_val = (seed % 100) as u8;

    if rand_val < target_percent {
        0
    } else {
        1
    }
}

/// Processes the input data, applying a placeholder mathematical transformation.
///
/// In a real engine, this would perform preliminary calculations (e.g., noise sampling,
/// terrain height adjustments) before final structure generation.
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