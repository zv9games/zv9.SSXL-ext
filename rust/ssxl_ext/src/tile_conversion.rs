// ssxl_ext/src/tile_conversion.rs

/// Pure Rust implementation of bitmask → tile ID conversion.
/// This is what the CLI should call directly.
pub fn bitmask_to_id(bitmask: u8) -> i32 {
    // Real implementation would use a LUT.
    (bitmask % 48) as i32
}

/// FFI wrapper for Godot.
/// This is what the DLL exports.
#[no_mangle]
pub extern "C" fn ssxl_ext_bitmask_to_id(bitmask: u8) -> i32 {
    // Optional: debug print for Godot-side calls
    eprintln!("FFI_EXPORT: converting bitmask {} via FFI wrapper", bitmask);

    bitmask_to_id(bitmask)
}
