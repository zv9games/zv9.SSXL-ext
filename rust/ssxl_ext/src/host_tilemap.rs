use godot::prelude::*;
use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::shared_tile::TileData;
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};

/// NOTE:
/// The old TileMapDirectWriteExtension trait is removed.
/// SSXLTileMap is now a native Godot class, so we call the FFI bridge directly.

/// Render a chunk by:
/// 1. Asking SSXLTileMap for a raw pointer to the chunk buffer
/// 2. Copying TileData into that buffer
/// 3. Notifying SSXLTileMap that the chunk changed
pub fn render_chunk_direct(tilemap_id_raw: i64, chunk: Chunk) -> Result<(), SSXLCoreError> {
    // SAFETY: FFI call into Godot to retrieve the chunk pointer
    let dest_ptr = unsafe {
        ssxl_get_tilemap_chunk_ptr(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        )
    };

    if dest_ptr.is_null() {
        return Err(SSXLCoreError::InvalidInstance(tilemap_id_raw));
    }

    let tile_count = chunk.tiles.len();

    if tile_count > 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(
                chunk.tiles.as_ptr() as *const TileData,
                dest_ptr,
                tile_count,
            );
        }
    }

    // SAFETY: Notify Godot that the chunk buffer was updated
    unsafe {
        ssxl_notify_chunk_updated(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        );
    }

    Ok(())
}
