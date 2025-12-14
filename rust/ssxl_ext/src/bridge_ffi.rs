// rust/SSXL-ext/src/bridge_ffi.rs

use godot::prelude::*;
// WARNING FIX: TileMap is only used in the GDExtension impl, so we can keep it here, 
// but we remove the unused imports below.

// REMOVED: use godot::classes::TileMap; // Unused in this module's root scope
// REMOVED: use crate::host_tilemap::TileMapDirectWriteExtension; // Unused in this module's root scope
// REMOVED: use crate::{ssxl_error, ssxl_warn}; // Unused, as they are #[macro_export]

use crate::shared_tile::TileData;
// Import the macros at the crate root to make them available:

// --- Finisher Transparency & Robustness Improvements ---

/// The default TileMap layer used for all procedural chunk data.
// WARNING FIX: Prefix with _ to silence unused constant warning.
const _CHUNK_DATA_LAYER: i32 = 0;

// --------------------------------------------------------

#[no_mangle]
/// # Safety
/// This is the Finisher's hook for direct memory writing.
pub unsafe extern "C" fn ssxl_get_tilemap_chunk_ptr(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) -> *mut TileData {
    // WARNING FIX: Prefix unused CLI variables with underscores.
    let _tilemap_id = tilemap_id;
    let _chunk_x = chunk_x;
    let _chunk_y = chunk_y;

    // --- GDExtension Implementation (Runs only when Godot is present) ---
    #[cfg(not(feature = "ssxl_cli"))] // Now correctly defined in Cargo.toml
    {
        // Re-import the necessary Godot types only within the scope where they are used.
        use godot::classes::TileMap;
        use crate::host_tilemap::TileMapDirectWriteExtension;
        
        // Use the macro path for calls:
        use crate::ssxl_error; 

        let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
            Ok(tm) => tm.cast::<TileMap>(),
            Err(_) => {
                // Use the macro from the crate root
                ssxl_error!("SSXL FFI: Failed to retrieve TileMap object for ID {}", tilemap_id.to_i64());
                return std::ptr::null_mut();
            }
        };

        // WARNING FIX: Use the constant, but without the underscore for the GDExtension path
        let layer = 0; // Or define CHUNK_DATA_LAYER here if needed
        let raw_ptr: *mut TileData = tilemap.get_raw_chunk_data_ptr(layer, chunk_x, chunk_y);

        if raw_ptr.is_null() {
            ssxl_error!("SSXL FFI: Godot failed to return raw pointer for chunk ({}, {})", chunk_x, chunk_y);
        }

        return raw_ptr;
    }

    // --- CLI MOCK Implementation (Runs only in CLI, linker needs the function) ---
    #[cfg(feature = "ssxl_cli")]
    {
        eprintln!("FFI_EXPORT: CLI MOCK: Attempted to get TileMap pointer (Godot FFI disabled)");
        return std::ptr::null_mut();
    }
}

#[no_mangle]
/// # Safety
/// Signals Godot to redraw and re-evaluate the chunk's visual state.
pub unsafe extern "C" fn ssxl_notify_chunk_updated(
    tilemap_id: InstanceId,
    chunk_x: i32,
    chunk_y: i32,
) {
    // WARNING FIX: Prefix unused CLI variables with underscores.
    let _tilemap_id = tilemap_id;
    let _chunk_x = chunk_x;
    let _chunk_y = chunk_y;

    // --- GDExtension Implementation ---
    #[cfg(not(feature = "ssxl_cli"))]
    {
        // Re-import the necessary Godot types only within the scope where they are used.
        use godot::classes::TileMap;
        use crate::host_tilemap::TileMapDirectWriteExtension;
        
        // Use the macro path for calls:
        use crate::{ssxl_warn, ssxl_info}; // ssxl_warn is used here
        
        let mut tilemap = match Gd::<TileMap>::try_from_instance_id(tilemap_id) {
            Ok(tm) => tm.cast::<TileMap>(),
            Err(_) => {
                // Use the macro from the crate root
                ssxl_warn!("SSXL FFI: Cannot notify update, Invalid TileMap InstanceId: {}", tilemap_id.to_i64());
                return;
            }
        };

        let layer = 0; // Or define CHUNK_DATA_LAYER here if needed
        tilemap.notify_chunk_data_changed(layer, chunk_x, chunk_y);
    }
    
    // --- CLI MOCK Implementation ---
    #[cfg(feature = "ssxl_cli")]
    {
        eprintln!("FFI_EXPORT: CLI MOCK: TileMap update notification for chunk ({}, {}) (Godot FFI disabled)", _chunk_x, _chunk_y);
    }
}