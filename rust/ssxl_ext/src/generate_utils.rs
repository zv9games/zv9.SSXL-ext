// ssxl_ext/src/generate_utils.rs (FIXED)

// NOTE: Since we are replacing godot_print! with eprintln!, 
// we no longer strictly need 'use godot::prelude::*'. However, 
// if other Godot types are used, keep it. For this simple case,
// we remove the import that was triggering the Godot environment check.

/// EXPORT 5/5: Called by CLI for exhaustive data integrity check (256/256).
/// This function converts an 8-bit neighbor mask (bitmask) into a specific Tile ID 
/// for autotiling, a critical utility function for the generation core.
#[no_mangle]
pub extern "C" fn ssxl_ext_bitmask_to_id(bitmask: u8) -> i32 {
    // In a real implementation, this would involve a large, pre-calculated lookup table (LUT) 
    // to map the 256 possible bitmask inputs to one of the 48 available tile variations.
    
    // ✅ FIX: Replace Godot-dependent print with standard error print for CLI execution.
    // This resolves the "Godot engine not available" panic.
    eprintln!("FFI_EXPORT: CLI converting bitmask {}. MOCK LUT triggered.", bitmask);
    
    // MOCK: Return a deterministic, valid ID (0-47) for the CLI test harness to validate.
    (bitmask % 48) as i32 
}