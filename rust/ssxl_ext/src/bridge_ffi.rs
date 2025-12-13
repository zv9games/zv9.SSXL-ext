use godot::prelude::*;
use godot::classes::TileMap;
use crate::shared_tile::TileData;
use crate::host_tilemap::TileMapDirectWriteExtension;

// --- Finisher Transparency & Robustness Improvements ---

/// The default TileMap layer used for all procedural chunk data.
/// Using a named constant instead of a magic number (0) improves transparency.
const CHUNK_DATA_LAYER: i32 = 0;

// --------------------------------------------------------

#[no_mangle]
/// # Safety
///
/// This function is the Finisher's hook for direct memory writing.
/// It is inherently unsafe because it hands a raw pointer to a Rust thread,
/// allowing it to mutate Godot's internal memory without Godot's direct knowledge.
///
/// The caller (the Rust generation core) *must* adhere to the following guarantees:
/// 1. **Validity:** The returned pointer must be used only if it is non-null.
/// 2. **Lifetime Guard:** The pointer must *not* be used after the Godot `TileMap` object
///    identified by `tilemap_id` is destroyed. The Finisher's lifecycle management
///    (e.g., in the Conductor) must guard against this use-after-free scenario.
/// 3. **Concurrency Guard:** Only one thread/task is allowed to write to the memory
///    region represented by this pointer at any given time.
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm.cast::<TileMap>(),
        Err(_) => {
            godot_error!("SSXL FFI: Failed to retrieve TileMap object for ID {}", tilemap_id.to_i64());
            return std::ptr::null_mut();
        }
    };
    
    // We use the explicit, named constant for transparency.
    let raw_ptr: *mut TileData = tilemap.get_raw_chunk_data_ptr(CHUNK_DATA_LAYER, chunk_x, chunk_y);

    if raw_ptr.is_null() {
        godot_error!("SSXL FFI: Godot failed to return raw pointer for chunk ({}, {}) on layer {}", chunk_x, chunk_y, CHUNK_DATA_LAYER);
    }
    
    // raw_ptr is either the valid memory location or std::ptr::null_mut()
    raw_ptr
}

#[no_mangle]
/// # Safety
///
/// This function must only be called **after** a successful write operation
/// using a pointer previously returned by `ssxl_get_tilemap_chunk_ptr`.
/// It signals Godot to redraw and re-evaluate the chunk's visual state.
/// This prevents a data race where Godot tries to read the chunk while it's being written to.
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) {
    let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm.cast::<TileMap>(),
        Err(_) => {
            // Note: Use 'warn' here as the corresponding write operation might have failed,
            // or the TileMap was destroyed, and we don't need to return a value.
            godot_warn!("SSXL FFI: Cannot notify update, Invalid TileMap InstanceId: {}", tilemap_id.to_i64());
            return;
        }
    };

    // Use the explicit constant for consistency and clarity.
    tilemap.notify_chunk_data_changed(CHUNK_DATA_LAYER, chunk_x, chunk_y);
}

// --------------------------------------------------------
// Internal Rust API (Can remain, but should also use the constant)
// --------------------------------------------------------

#[allow(dead_code)] // Assuming this is used elsewhere in the Rust core
pub unsafe fn get_raw_chunk_write_ptr(
    tilemap_id: InstanceId,
    x: i32,
    y: i32
) -> *mut TileData {
    // This wrapper is fine, but it inherits all the unsafety from the FFI call.
    ssxl_get_tilemap_chunk_ptr(tilemap_id, x, y)
}