// ============================================================================
// 🎲 Utility Functions (`crate::utils::probability`)
// ----------------------------------------------------------------------------
// This module provides lightweight utility functions for probabilistic rolls
// and generic data processing. These helpers are designed to support procedural
// generation and quick uniqueness checks in the SSXL engine.
//
// Purpose:
//   • `generate_percent_roll`: perform probabilistic chance rolls for events.
//   • `process_data`: derive a simple aggregate identifier from SSXLData objects.
//
// Key Functions:
//   • generate_percent_roll(target_percent: u8) -> u32
//       - Performs a probabilistic roll against a given percentage chance.
//       - Arguments:
//           • target_percent: u8 (0–100), representing the probability of success.
//       - Behavior:
//           • Generates a random integer between 0 and 99 inclusive.
//           • Compares it against target_percent.
//           • Returns 0 if success (random < target_percent).
//           • Returns 1 if failure (random >= target_percent).
//       - Example:
//           • generate_percent_roll(25) → 25% chance of returning 0 (success).
//
//   • process_data(data: &impl SSXLData) -> u64
//       - Processes an object implementing the `SSXLData` trait.
//       - Arguments:
//           • data: reference to any object implementing SSXLData.
//       - Behavior:
//           • Calls `get_id()` to retrieve the object’s unique identifier.
//           • Calls `get_value_len()` to retrieve the length of its payload.
//           • Adds them together to produce a u64 aggregate value.
//       - Returns:
//           • u64 representing the combined identifier + payload length.
//       - Use Cases:
//           • Lightweight hashing.
//           • Quick uniqueness checks.
//           • Simple aggregate identifiers.
//
// Design Choices:
//   • Using `rand::Rng` ensures efficient and flexible random number generation.
//   • Returning `u32` for rolls keeps results lightweight and script-friendly.
//   • Trait-based `process_data` allows generic handling of any SSXLData object.
//   • Separation of probability and data utilities keeps the module cohesive.
//
// Educational Note:
//   • These functions demonstrate how small, focused utilities can support
//     larger systems. By abstracting probability rolls and data aggregation,
//     the engine gains reusable building blocks for procedural generation,
//     resource spawning, and uniqueness validation.
// ============================================================================


use crate::primitives::SSXLData;
use rand::Rng;

pub fn generate_percent_roll(target_percent: u8) -> u32 {
    let rand_val = rand::thread_rng().gen_range(0..100) as u8;

    if rand_val < target_percent {
        0
    } else {
        1
    }
}

pub fn process_data(data: &impl SSXLData) -> u64 {
    let processed_value = data.get_id() + data.get_value_len() as u64;
    processed_value
}
