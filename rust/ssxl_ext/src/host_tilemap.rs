use godot::prelude::*;
use godot::classes::TileMap;
use crate::shared_tile::TileData;
use crate::shared_error::SSXLCoreError;
use crate::shared_chunk::Chunk;
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};
use crate::{ssxl_info, ssxl_warn, ssxl_error}; // 🔥 FIX 1: Import custom logger macros

pub trait TileMapDirectWriteExtension {
    fn get_raw_chunk_data_ptr(&mut self, layer: i32, chunk_x: i32, chunk_y: i32) -> *mut TileData;
    fn notify_chunk_data_changed(&mut self, layer: i32, chunk_x: i32, chunk_y: i32);
}

impl TileMapDirectWriteExtension for Gd<TileMap> {
    
    fn get_raw_chunk_data_ptr(&mut self, _layer: i32, _chunk_x: i32, _chunk_y: i32) -> *mut TileData {
        let dummy_ptr = 0xDEADBEEF as *mut TileData;
        
        // 🔥 FIX 2: Replaced godot_warn! with ssxl_warn!
        ssxl_warn!("Architectural WARNING: get_raw_chunk_data_ptr is placeholder (0xDEADBEEF). C++ TileMap binding required for runtime stability.");
        dummy_ptr
    }

    fn notify_chunk_data_changed(&mut self, layer: i32, chunk_x: i32, chunk_y: i32) {
        // 🔥 FIX 3: Replaced godot_print! with ssxl_info!
        ssxl_info!("TILEMAP_WRITE_NOTIFY: Chunk ({}, {}) for layer {} marked dirty.", chunk_x, chunk_y, layer);
    }
}

/// The Finisher function responsible for the high-speed data delivery.
/// This must be called from the Godot main thread loop (e.g., host_render.rs).
pub fn render_chunk_direct(tilemap_id: InstanceId, chunk: Chunk) -> Result<(), SSXLCoreError> {
    
    let dest_ptr = unsafe {
        ssxl_get_tilemap_chunk_ptr(
            tilemap_id, 
            chunk.position.0, 
            chunk.position.1
        )
    };

    if dest_ptr.is_null() {
        // 🔥 FIX 4: Replaced godot_print! with ssxl_error!
        ssxl_error!("ERROR: TileMap chunk pointer is NULL for chunk {:?}", chunk.position);
        return Err(SSXLCoreError::InvalidInstance(tilemap_id.to_i64() as u64)); 
    }
    
    let tile_count = chunk.tiles.len();
    
    // Perform the direct, non-overlapping memory copy (the O(1) core optimization).
    unsafe {
        std::ptr::copy_nonoverlapping(
            chunk.tiles.as_ptr() as *const _, // Source: Rust's generated Vector
            dest_ptr as *mut _,               // Destination: Godot's internal TileMap memory
            tile_count
        );
    
        // Notify Godot's renderer that the buffer has been changed manually.
        ssxl_notify_chunk_updated(tilemap_id, chunk.position.0, chunk.position.1);
    }

    Ok(())
}


// ============================================================================
// FFI EXPORTS FOR ssxl_cli (TEST HARNESS)
// ============================================================================

/// EXPORT 1/2: Used by ssxl_cli to stress test single-cell FFI writing.
#[no_mangle]
pub extern "C" fn ssxl_set_cell(x: i32, y: i32, tile_id: i32) {
    // Note: In a real system, this should queue the write.
    // For the CLI test harness, we just log the action and assume success.
    // 🔥 FIX 5: Replaced godot_print! with eprintln! for CLI FFI MOCK.
    eprintln!("FFI_EXPORT: CLI set cell ({}, {}) to Tile ID {}", x, y, tile_id);
}

/// EXPORT 2/2: Used by ssxl_cli to signal the end of a writing batch.
#[no_mangle]
pub extern "C" fn ssxl_notify_tilemap_update() {
    // This signals the host to process the pending tile updates.
    // 🔥 FIX 6: Replaced godot_print! with eprintln! for CLI FFI MOCK.
    eprintln!("FFI_EXPORT: CLI triggered global TileMap update notification.");
    // A real implementation might iterate over a list of dirty chunks and call ssxl_notify_chunk_updated.
}