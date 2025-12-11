// rust/SSXL-ext/src/host_tilemap.rs

use godot::prelude::*;
// FIX 1: Import the correct, custom error type.
use crate::shared_error::SSXLCoreError; 
use crate::shared_chunk::Chunk;
// FIX 4: Correct the import name to match the external symbol name required by the linker.
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};

/// The Finisher function responsible for the high-speed data delivery.
/// This must be called from the Godot main thread loop (e.g., host_render.rs).
// FIX 2: Change the return type to the correct custom error enum.
pub fn render_chunk_direct(tilemap_id: InstanceId, chunk: Chunk) -> Result<(), SSXLCoreError> {
    
    // 1. Determine the memory location using the FFI bridge.
    // FIX 5: Wrap the call to the unsafe FFI function ssxl_get_tilemap_chunk_ptr in an unsafe block.
    let dest_ptr = unsafe {
        // FIX 4: Use the corrected function name, ssxl_get_tilemap_chunk_ptr.
        ssxl_get_tilemap_chunk_ptr(
            tilemap_id, 
            chunk.position.0, 
            chunk.position.1
        )
    };

    if dest_ptr.is_null() {
        // Handle the error if the TileMap or chunk doesn't exist
        godot_print!("ERROR: TileMap chunk pointer is NULL for chunk {:?}", chunk.position);
        
        // FIX 3: Use the public `to_i64()` method and cast the result to the required `u64`.
        return Err(SSXLCoreError::InvalidInstance(tilemap_id.to_i64() as u64)); 
    }
    
    // 2. Perform the direct, non-overlapping memory copy (THE CORE OPTIMIZATION).
    let tile_count = chunk.tiles.len();
    
    // This existing block handles the copy and notification, which are already unsafe operations.
    // The previous fix ensures the pointer acquisition is also covered.
    unsafe {
        // Since dest_ptr is an *mut c_void, we trust the FFI side to manage the memory access.
        // We assume the size of the TileData struct is correctly accounted for in the copy length.
        std::ptr::copy_nonoverlapping(
            chunk.tiles.as_ptr() as *const _, // Source: Rust's generated Vector
            dest_ptr as *mut _,               // Destination: Godot's internal TileMap memory
            tile_count                        // The number of TileData structs to copy
        );
    
        // 3. Notify Godot's renderer that the buffer has been changed manually.
        // This call must also remain unsafe, as it interacts with the C FFI layer.
        ssxl_notify_chunk_updated(tilemap_id, chunk.position.0, chunk.position.1);
    }

    Ok(())
}