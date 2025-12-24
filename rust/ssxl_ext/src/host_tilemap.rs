use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::shared_tile::TileData;
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};

/// Render a chunk by:
/// 1. Asking SSXLChunkBuffer for a raw pointer to the chunk buffer
/// 2. Copying TileData into that buffer
/// 3. Notifying SSXLChunkBuffer that the chunk changed
pub fn render_chunk_direct(tilemap_id_raw: i64, chunk: Chunk) -> Result<(), SSXLCoreError> {
    // Debug: inspect first source tile in the Chunk coming from the worker
    if let Some(first) = chunk.tiles.get(0) {
        eprintln!(
            "[host_tilemap] SRC first tile for chunk ({}, {}): {:?}",
            chunk.position.0,
            chunk.position.1,
            first
        );
    } else {
        eprintln!(
            "[host_tilemap] SRC chunk ({}, {}) has EMPTY tiles vec!",
            chunk.position.0,
            chunk.position.1
        );
    }

    // SAFETY: FFI call into Godot to retrieve the chunk pointer
    let dest_ptr = unsafe {
        ssxl_get_tilemap_chunk_ptr(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        )
    };

    if dest_ptr.is_null() {
        eprintln!(
            "[host_tilemap] ERROR: ssxl_get_tilemap_chunk_ptr returned NULL for chunk ({}, {})",
            chunk.position.0,
            chunk.position.1
        );
        return Err(SSXLCoreError::InvalidInstance(tilemap_id_raw));
    }

    let tile_count = chunk.tiles.len();

    eprintln!(
        "[host_tilemap] DEST ptr for chunk ({}, {}) = {:?}, copying {} tiles",
        chunk.position.0,
        chunk.position.1,
        dest_ptr,
        tile_count
    );

    if tile_count > 0 {
        unsafe {
            // Interpret the destination pointer as a TileData slice
            let dest_slice_before: &[TileData] =
                std::slice::from_raw_parts(dest_ptr as *const TileData, tile_count);

            eprintln!(
                "[host_tilemap] DEST first tile BEFORE copy for chunk ({}, {}): {:?}",
                chunk.position.0,
                chunk.position.1,
                dest_slice_before.get(0)
            );

            // Perform the copy
            std::ptr::copy_nonoverlapping(
                chunk.tiles.as_ptr() as *const TileData,
                dest_ptr,
                tile_count,
            );

            let dest_slice_after: &[TileData] =
                std::slice::from_raw_parts(dest_ptr as *const TileData, tile_count);

            eprintln!(
                "[host_tilemap] DEST first tile AFTER copy for chunk ({}, {}): {:?}",
                chunk.position.0,
                chunk.position.1,
                dest_slice_after.get(0)
            );
        }
    } else {
        eprintln!(
            "[host_tilemap] WARNING: chunk ({}, {}) has 0 tiles; nothing to copy.",
            chunk.position.0,
            chunk.position.1
        );
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
