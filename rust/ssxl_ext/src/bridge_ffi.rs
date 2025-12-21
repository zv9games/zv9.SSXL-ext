// Only compile this when building the Godot binding.
#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::obj::InstanceId;

#[cfg(not(feature = "godot-binding"))]
type RawInstanceId = i64;

use crate::shared_tile::TileData;

#[cfg(feature = "godot-binding")]
const CHUNK_DATA_LAYER: i32 = 0;

// ✅ We now import the native SSXLTileMap class, not the old trait.
#[cfg(feature = "godot-binding")]
use crate::ssxl_tilemap::SSXLTileMap;

//
// ──────────────────────────────────────────────────────────────────────────────
//   FFI: ssxl_get_tilemap_chunk_ptr
// ──────────────────────────────────────────────────────────────────────────────
//
#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    let tilemap_id = InstanceId::from_i64(tilemap_id_raw);

    // ✅ Try to retrieve the SSXLTileMap instance
    let mut tilemap = match Gd::<SSXLTileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm,
        Err(_) => {
            crate::ssxl_error!(
                "SSXL FFI: Failed to retrieve SSXLTileMap object for ID {}",
                tilemap_id_raw
            );
            return std::ptr::null_mut();
        }
    };

    // ✅ Call the native Rust method directly
    let raw_ptr = tilemap.bind_mut().get_raw_chunk_data_ptr(
        CHUNK_DATA_LAYER,
        chunk_x,
        chunk_y,
    );

    if raw_ptr.is_null() {
        crate::ssxl_error!(
            "SSXL FFI: SSXLTileMap returned NULL pointer for chunk ({}, {})",
            chunk_x,
            chunk_y
        );
    }

    raw_ptr as *mut TileData
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   CLI fallback (no Godot binding)
// ──────────────────────────────────────────────────────────────────────────────
//
#[cfg(not(feature = "godot-binding"))]
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id_raw: RawInstanceId,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    let _id = tilemap_id_raw;
    let _cx = chunk_x;
    let _cy = chunk_y;

    eprintln!("FFI_EXPORT: CLI MOCK: get_tilemap_chunk_ptr() called (Godot disabled)");
    std::ptr::null_mut()
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   FFI: ssxl_notify_chunk_updated
// ──────────────────────────────────────────────────────────────────────────────
//
#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) {
    let tilemap_id = InstanceId::from_i64(tilemap_id_raw);

    // ✅ Retrieve SSXLTileMap, not TileMap
    let mut tilemap = match Gd::<SSXLTileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm,
        Err(_) => {
            crate::ssxl_warn!(
                "SSXL FFI: Cannot notify update — invalid SSXLTileMap ID {}",
                tilemap_id_raw
            );
            return;
        }
    };

    // ✅ Call the native Rust method
    tilemap
        .bind_mut()
        .notify_chunk_data_changed(CHUNK_DATA_LAYER, chunk_x, chunk_y);
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   CLI fallback
// ──────────────────────────────────────────────────────────────────────────────
//
#[cfg(not(feature = "godot-binding"))]
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id_raw: RawInstanceId,
    chunk_x: i32,
    chunk_y: i32,
) {
    let _id = tilemap_id_raw;
    let _cx = chunk_x;
    let _cy = chunk_y;

    eprintln!(
        "FFI_EXPORT: CLI MOCK: notify_chunk_updated({}, {})",
        _cx, _cy
    );
}
