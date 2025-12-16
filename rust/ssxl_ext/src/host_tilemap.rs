// 🎯 CRITICAL FIX: Gate Godot imports
#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]

#[cfg(feature = "godot-binding")]
use godot::classes::TileMap;

#[cfg(feature = "godot-binding")]
use crate::shared_tile::TileData;
use crate::shared_error::SSXLCoreError;
use crate::shared_chunk::Chunk;
#[cfg(feature = "godot-binding")]
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};

#[cfg(feature = "godot-binding")]
pub trait TileMapDirectWriteExtension {
    fn get_raw_chunk_data_ptr(
        &mut self,
        layer: i32,
        chunk_x: i32,
        chunk_y: i32,
    ) -> *mut TileData;
    fn notify_chunk_data_changed(&mut self, layer: i32, chunk_x: i32, chunk_y: i32);
}

#[cfg(feature = "godot-binding")]
impl TileMapDirectWriteExtension for Gd<TileMap> {
    fn get_raw_chunk_data_ptr(
        &mut self,
        _layer: i32,
        _chunk_x: i32,
        _chunk_y: i32,
    ) -> *mut TileData {
        let dummy_ptr = 0xDEADBEEF as *mut TileData;
        crate::ssxl_warn!(
            "Architectural WARNING: get_raw_chunk_data_ptr is placeholder (0xDEADBEEF). \
             C++ TileMap binding required for runtime stability."
        );
        dummy_ptr
    }

    fn notify_chunk_data_changed(&mut self, layer: i32, chunk_x: i32, chunk_y: i32) {
        crate::ssxl_info!(
            "TILEMAP_WRITE_NOTIFY: Chunk ({}, {}) for layer {} marked dirty.",
            chunk_x,
            chunk_y,
            layer
        );
    }
}

/// High-speed data delivery into Godot's TileMap.
/// Called from the Godot main thread (e.g., host_render.rs).
#[cfg(feature = "godot-binding")]
pub fn render_chunk_direct(tilemap_id_raw: i64, chunk: Chunk) -> Result<(), SSXLCoreError> {
    // Unified ID model: tilemap_id_raw is already the numeric ID used at the FFI boundary.

    let dest_ptr = unsafe {
        ssxl_get_tilemap_chunk_ptr(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        )
    };

    if dest_ptr.is_null() {
        crate::ssxl_error!(
            "ERROR: TileMap chunk pointer is NULL for chunk {:?}",
            chunk.position
        );
        // InvalidInstance now takes i64, not u64
        return Err(SSXLCoreError::InvalidInstance(tilemap_id_raw));
    }

    let tile_count = chunk.tiles.len();

    unsafe {
        std::ptr::copy_nonoverlapping(
            chunk.tiles.as_ptr() as *const _,
            dest_ptr as *mut _,
            tile_count,
        );

        ssxl_notify_chunk_updated(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        );
    }

    Ok(())
}

// 🎯 CLI fallback: no Godot, no InstanceId.
#[cfg(not(feature = "godot-binding"))]
pub fn render_chunk_direct(_tilemap_id: i64, _chunk: Chunk) -> Result<(), SSXLCoreError> {
    Ok(())
}

// ============================================================================
// FFI EXPORTS FOR ssxl_cli (TEST HARNESS)
// ============================================================================

#[no_mangle]
#[cfg(not(feature = "godot-binding"))]
pub extern "C" fn ssxl_set_cell(x: i32, y: i32, tile_id: i32) {
    eprintln!(
        "FFI_EXPORT: CLI set cell ({}, {}) to Tile ID {}",
        x, y, tile_id
    );
}

#[cfg(feature = "godot-binding")]
pub extern "C" fn ssxl_set_cell(_x: i32, _y: i32, _tile_id: i32) {}

#[no_mangle]
#[cfg(not(feature = "godot-binding"))]
pub extern "C" fn ssxl_notify_tilemap_update() {
    eprintln!("FFI_EXPORT: CLI triggered global TileMap update notification.");
}

#[cfg(feature = "godot-binding")]
pub extern "C" fn ssxl_notify_tilemap_update() {}
