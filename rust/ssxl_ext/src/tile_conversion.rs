// ssxl_ext/src/tile_conversion.rs

// We only need Godot prelude if other code in this file uses Godot types.
// Since this file contains an FFI export and the only other logic is math/logging,
// we can remove or comment out the unused import for the CLI build.
// use godot::prelude::*; // Commented out as it's not strictly necessary for the fixed logic below

/// A utility function that converts an 8-bit neighbor mask (bitmask)
/// into a specific Tile ID for autotiling.
///
/// This is a pure-data conversion function, critical for the GPU-like parallel
/// processing that SSXL-ext performs.
#[no_mangle]
pub extern "C" fn ssxl_ext_bitmask_to_id(bitmask: u8) -> i32 {
    // In a real implementation, this would involve a large, pre-calculated lookup table (LUT)
    // to map the 256 possible bitmask inputs to one of the 48 available tile variations.
    
    // 🔥 FIX: Replaced godot_print! with eprintln! for CLI FFI MOCK.
    eprintln!("FFI_EXPORT: CLI converting bitmask {}. MOCK LUT triggered.", bitmask);
    
    // MOCK: Return a deterministic, valid ID (0-47) for the CLI test harness to validate.
    (bitmask % 48) as i32
}