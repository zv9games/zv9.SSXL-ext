// rust/SSXL-ext/src/bridge_ffi.rs

use godot::prelude::*;
use godot::classes::TileMap;
use crate::shared_tile::TileData;

// --- FFI Function Implementations (Symbols Defined) ---

/// SAFETY: Must only be called on the Godot main thread.
/// Returns a mutable pointer to the internal TileMap storage buffer for a given chunk position.
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    // 1. Attempt to get and cast the TileMap instance.
    let tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) { 
        Ok(tm) => tm.cast::<TileMap>(), 
        Err(_) => {
            godot_error!("ssxl_get_tilemap_chunk_ptr: Invalid TileMap InstanceId: {}", tilemap_id.to_i64());
            return std::ptr::null_mut();
        }
    };
    
    // 2. Access the TileMap data. 
    // ... logic remains as placeholder ...
    std::ptr::null_mut()
}

/// A call to tell Godot's rendering engine to invalidate the area after we manually wrote to its memory.
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) {
    // 1. Attempt to get and cast the TileMap instance (needs to be mutable).
    let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm.cast::<TileMap>(), 
        Err(_) => {
            godot_error!("ssxl_notify_chunk_updated: Invalid TileMap InstanceId: {}", tilemap_id.to_i64());
            return;
        }
    };

    // 2. Call the Godot API equivalent function to force a visual update.
    // FIX 1: Use the correct, argument-less `force_update()` method. 
    // NOTE: This updates the entire TileMap, which is less efficient than a chunk update, 
    // but satisfies the compiler/linker with available methods.
    tilemap.force_update();
}

// --- High-Level Rust Wrapper for Safety/Ergonomics ---

/// WARNING: This function contains UNSAFE FFI calls.
/// Use with extreme caution and only on the main thread.
pub unsafe fn get_raw_chunk_write_ptr(
    tilemap_id: InstanceId, 
    x: i32, 
    y: i32
) -> *mut TileData {
    // This wrapper is now correctly calling the Rust-defined function above.
    ssxl_get_tilemap_chunk_ptr(tilemap_id, x, y)
}