// ssxl_math/src/generation_utils.rs

//! # Generation Utilities
//!
//! Provides common mathematical and random-sampling utilities used throughout the
//! procedural generation pipeline (e.g., `ssxl_generate` crate).
//!
//! Functions here are designed for rapid, stateless generation logic, such as
//! calculating percentage chance rolls and generating unique identifiers for data structures.

use crate::primitives::SSXLData;
use rand::Rng;

// -----------------------------------------------------------------------------
// Randomness and Chance Utilities
// -----------------------------------------------------------------------------

/// Rolls a chance check against a given percentage.
///
/// This function is vital for injecting controlled **randomness** and **balance**
/// into the generation process (e.g., probability of a resource spawning, or a
/// cellular automata rule firing).
///
/// # Arguments
/// * `target_percent` - The probability of success, expressed as a whole percentage (0-100).
///
/// # Returns
/// * `0`: Success (The random number was less than `target_percent`).
/// * `1`: Failure (The random number was greater than or equal to `target_percent`).
///
/// # Example
/// A 25% chance of spawning:
/// ```ignore
/// if generate_percent_roll(25) == 0 {
///     // spawn item
/// }
/// ```
pub fn generate_percent_roll(target_percent: u8) -> u32 {
    // Generate a random value in the inclusive range [0, 99], which is a 0-100 scale.
    let rand_val = rand::thread_rng().gen_range(0..100) as u8;

    // Success occurs if the random value falls within the target range.
    if rand_val < target_percent {
        0 // Success / Hit
    } else {
        1 // Failure / Miss
    }
}

// -----------------------------------------------------------------------------
// Data Processing Utilities
// -----------------------------------------------------------------------------

/// Processes an object implementing the `SSXLData` trait to derive a simple aggregate identifier.
///
/// This is a basic utility that combines the object's unique ID with its data payload size.
/// It is often used for creating quick, unique signatures or simple hashing/checksums
/// for data chunks across worker threads.
///
/// # Arguments
/// * `data` - A reference to any structure that implements the `SSXLData` trait.
///
/// # Returns
/// * `u64` - The sum of the data's ID and the length of its associated value.
pub fn process_data(data: &impl SSXLData) -> u64 {
    // Combine the inherent ID (u64) with the value length (u64 after casting).
    let processed_value = data.get_id() + data.get_value_len() as u64;
    processed_value
}