// ------------------------------------------------------------
// Godot binding imports (only when building the Godot binding).
// ------------------------------------------------------------
#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::obj::InstanceId;

// CLI / non-Godot builds use a simple integer ID.
#[cfg(not(feature = "godot-binding"))]
type RawInstanceId = i64;

use crate::shared_tile::TileData;

// In Plan B, we only have one logical "layer" of chunk data.
#[cfg(feature = "godot-binding")]
const CHUNK_DATA_LAYER: i32 = 0;

// Plan B: use SSXLChunkBuffer, not SSXLTileMap.
#[cfg(feature = "godot-binding")]
use crate::ssxl_chunk_buffer::SSXLChunkBuffer;

//
// ──────────────────────────────────────────────────────────────
//   FFI: ssxl_get_tilemap_chunk_ptr (Plan B: SSXLChunkBuffer)
// ──────────────────────────────────────────────────────────────
//
#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    let instance_id = InstanceId::from_i64(tilemap_id_raw);

    // Retrieve SSXLChunkBuffer by instance ID
    let mut chunk_buffer = match Gd::<SSXLChunkBuffer>::try_from_instance_id(instance_id) {
        Ok(cb) => cb,
        Err(_) => {
            crate::ssxl_error!(
                "SSXL FFI: Failed to retrieve SSXLChunkBuffer for ID {}",
                tilemap_id_raw
            );
            return std::ptr::null_mut();
        }
    };

    // Call the native Rust method directly (Plan B)
    let raw_ptr = chunk_buffer
        .bind_mut()
        .get_raw_chunk_data_ptr(CHUNK_DATA_LAYER, chunk_x, chunk_y);

    if raw_ptr.is_null() {
        crate::ssxl_error!(
            "SSXL FFI: SSXLChunkBuffer returned NULL pointer for chunk ({}, {})",
            chunk_x,
            chunk_y
        );
    }

    raw_ptr as *mut TileData
}

//
// ──────────────────────────────────────────────────────────────
//   CLI fallback (no Godot binding)
// ──────────────────────────────────────────────────────────────
//
#[cfg(not(feature = "godot-binding"))]
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    _tilemap_id_raw: RawInstanceId,
    _chunk_x: i32,
    _chunk_y: i32,
) -> *mut TileData {
    // Silent fallback — no spam
    std::ptr::null_mut()
}

//
// ──────────────────────────────────────────────────────────────
//   FFI: ssxl_notify_chunk_updated
// ──────────────────────────────────────────────────────────────
//
#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) {
    let instance_id = InstanceId::from_i64(tilemap_id_raw);

    // Retrieve SSXLChunkBuffer
    let mut chunk_buffer = match Gd::<SSXLChunkBuffer>::try_from_instance_id(instance_id) {
        Ok(cb) => cb,
        Err(_) => {
            crate::ssxl_warn!(
                "SSXL FFI: Cannot notify update — invalid SSXLChunkBuffer ID {}",
                tilemap_id_raw
            );
            return;
        }
    };

    // Notify SSXLChunkBuffer that data for this chunk has changed.
    chunk_buffer
        .bind_mut()
        .notify_chunk_data_changed(chunk_x, chunk_y);
}

//
// ──────────────────────────────────────────────────────────────
//   CLI fallback (no Godot binding)
// ──────────────────────────────────────────────────────────────
//
#[cfg(not(feature = "godot-binding"))]
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    _tilemap_id_raw: RawInstanceId,
    _chunk_x: i32,
    _chunk_y: i32,
) {
    // Silent fallback — no spam
}
