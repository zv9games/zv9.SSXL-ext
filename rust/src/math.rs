// rust/SSXL-ext/src/math.rs

use crate::shared_config::MapSettingsConfig; // To get chunk size

/// Converts continuous world coordinates (global tile coordinates) into 
/// discrete chunk coordinates.
/// 
/// This is crucial for determining which worker needs to process a given location.
pub fn world_to_chunk_coords(world_x: i32, world_y: i32, map_settings: &MapSettingsConfig) -> (i32, i32) {
    let chunk_size = map_settings.chunk_size as i32;
    
    // Use floor division (integer division in Rust for positive numbers)
    // For proper handling of negative coordinates (common in large worlds), 
    // we use a correction for negative numbers.
    let chunk_x = (world_x as f32 / chunk_size as f32).floor() as i32;
    let chunk_y = (world_y as f32 / chunk_size as f32).floor() as i32;
    
    (chunk_x, chunk_y)
}

/// Converts discrete chunk coordinates and local chunk coordinates back into 
/// global world coordinates.
pub fn chunk_to_world_coords(chunk_x: i32, chunk_y: i32, local_x: i32, local_y: i32, map_settings: &MapSettingsConfig) -> (i32, i32) {
    let chunk_size = map_settings.chunk_size as i32;
    
    let world_x = (chunk_x * chunk_size) + local_x;
    let world_y = (chunk_y * chunk_size) + local_y;
    
    (world_x, world_y)
}

/// Calculates the local tile coordinate within a chunk from a global coordinate.
pub fn world_to_local_coords(world_x: i32, world_y: i32, map_settings: &MapSettingsConfig) -> (i32, i32) {
    let chunk_size = map_settings.chunk_size as i32;
    
    // The modulo operation handles the wrap-around within the chunk boundary.
    // Ensure the result is non-negative for proper indexing.
    let local_x = world_x.rem_euclid(chunk_size);
    let local_y = world_y.rem_euclid(chunk_size);
    
    (local_x, local_y)
}

// rust/SSXL-ext/src/math.rs

/// Fast, safe 32-bit integer clamping. Useful for setting bounds on noise or CA values.
pub fn clamp_i32(val: i32, min: i32, max: i32) -> i32 {
    val.max(min).min(max)
}

/// Fast, safe float clamping. Used for normalizing noise output to a [0.0, 1.0] range.
pub fn clamp_f64(val: f64, min: f64, max: f64) -> f64 {
    val.max(min).min(max)
}

/// Linearly interpolates between 'a' and 'b' by the factor 't'.
/// Used in various generation algorithms for blending values.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.max(0.0).min(1.0) // Clamp t to [0.0, 1.0]
}

/// Calculates the distance squared between two points. 
/// Used for fast proximity checks without the overhead of a square root.
pub fn distance_squared_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}

// rust/SSXL-ext/src/math.rs

/// Generates a deterministic u64 seed from a pair of chunk coordinates.
/// Ensures that the same coordinates always produce the same seed, 
/// guaranteeing continuity across chunk borders.
pub fn hash_chunk_coords(chunk_x: i32, chunk_y: i32, world_seed: u64) -> u64 {
    // A simple, fast XOR-based hash mixing method:
    let mut hash: u64 = world_seed;
    hash = hash.wrapping_add(chunk_x as u64);
    hash = hash.wrapping_mul(31); // Simple prime multiplier
    hash ^= (chunk_y as u64).wrapping_shl(32);
    hash
}