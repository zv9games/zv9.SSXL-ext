#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::obj::InstanceId;

#[cfg(not(feature = "godot-binding"))]
type RawInstanceId = i64;

use crate::shared_tile::TileData;

#[cfg(feature = "godot-binding")]
const CHUNK_DATA_LAYER: i32 = 0;

#[cfg(feature = "godot-binding")]
use crate::host_tilemap::TileMapDirectWriteExtension;

#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    use godot::classes::TileMap;

    let tilemap_id = InstanceId::from_i64(tilemap_id_raw);

    let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm.cast::<TileMap>(),
        Err(_) => {
            crate::ssxl_error!(
                "SSXL FFI: Failed to retrieve TileMap object for ID {}",
                tilemap_id_raw
            );
            return std::ptr::null_mut();
        }
    };

    let raw_ptr = tilemap.get_raw_chunk_data_ptr(CHUNK_DATA_LAYER, chunk_x, chunk_y);

    if raw_ptr.is_null() {
        crate::ssxl_error!(
            "SSXL FFI: Godot returned NULL pointer for chunk ({}, {})",
            chunk_x,
            chunk_y
        );
    }

    raw_ptr as *mut TileData
}

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

#[cfg(feature = "godot-binding")]
#[no_mangle]
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id_raw: i64,
    chunk_x: i32,
    chunk_y: i32,
) {
    use godot::classes::TileMap;

    let tilemap_id = InstanceId::from_i64(tilemap_id_raw);

    let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
        Ok(tm) => tm.cast::<TileMap>(),
        Err(_) => {
            crate::ssxl_warn!(
                "SSXL FFI: Cannot notify update — invalid TileMap ID {}",
                tilemap_id_raw
            );
            return;
        }
    };

    tilemap.notify_chunk_data_changed(CHUNK_DATA_LAYER, chunk_x, chunk_y);
}

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
