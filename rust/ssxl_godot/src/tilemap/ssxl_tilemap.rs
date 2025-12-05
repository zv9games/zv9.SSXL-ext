// ssxl_godot/src/tilemap/ssxl_tilemap.rs
//
// The final render sink.
// Receives render batches from the async core (via signal) or FFI calls.
// Pure, fast, panic-safe, zero bloat.

use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};
use godot::obj::Base;
use godot::builtin::{Vector2i, PackedVector2Array, PackedInt32Array};

use once_cell::sync::OnceCell;

// --- FFI HOST GLOBALS ---
/// Global storage for the TileMap instance ID, allowing static C-functions to access it.
// HIGH: OnceCell is used here because this file lives in ssxl_godot, which is a GDExtension
// that links to the ssxl_engine_ffi library. The std::OnceLock is available in ssxl_engine_ffi.
pub static TILEMAP_INSTANCE_ID: OnceCell<InstanceId> = OnceCell::new(); 
const DEFAULT_LAYER: i32 = 0; // The layer where FFI calls place tiles.

// --- CLASS DEFINITION ---
#[derive(GodotClass)]
#[class(base = TileMap)]
pub struct SSXLTilemap {
    base: Base<TileMap>,

    #[export]
    pub tile_source_id: i32, // Source ID used when setting cells.

    /// Buffer to collect individual cell updates made via FFI calls (`ssxl_set_cell`).
    /// The updates are flushed to Godot when `ssxl_notify_tilemap_update` is called.
    pub pending_updates: Vec<(i32, i32, i32)>, // (world_x, world_y, tile_id)
}

#[godot_api]
impl ITileMap for SSXLTilemap {
    fn init(base: Base<TileMap>) -> Self {
        // Store the InstanceId globally when the node is initialized.
        let id = base.to_init_gd().instance_id();
        let _ = TILEMAP_INSTANCE_ID.set(id);

        Self {
            base,
            tile_source_id: 1,
            pending_updates: Vec::new(),
        }
    }
}

#[godot_api]
impl SSXLTilemap {
    // --- BATCH RENDER (Signal/GDScript) ---

    /// Primary render entrypoint — called from Rust via signal
    /// Expects the exact format from render_batch.rs
    #[func]
    pub fn batch_set_tiles(&mut self, batch: Dictionary) {
        let Some(layer) = batch.get("layer").and_then(|v| v.try_to::<i64>().ok()) else {
            godot_warn!("SSXLTilemap: missing or invalid 'layer'");
            return;
        };
        let layer = layer as i32;

        let positions = match batch.get("positions").and_then(|v| v.try_to::<PackedVector2Array>().ok()) {
            Some(p) => p,
            None => {
                godot_warn!("SSXLTilemap: missing 'positions'");
                return;
            }
        };

        let atlas_coords = match batch.get("atlas_coords").and_then(|v| v.try_to::<PackedVector2Array>().ok()) {
            Some(a) => a,
            None => {
                godot_warn!("SSXLTilemap: missing 'atlas_coords'");
                return;
            }
        };

        let alt_tiles = batch
            .get("alt_tiles")
            .and_then(|v| v.try_to::<PackedInt32Array>().ok())
            .unwrap_or_default();

        let source_id = self.tile_source_id;
        let mut tilemap = self.base_mut();

        tilemap.set_layer_enabled(layer, true);

        let len = positions.len();
        if len == 0 {
            return;
        }

        for i in 0..len {
            let pos = positions.get(i).unwrap();
            let atlas = atlas_coords.get(i).unwrap();
            let alt = alt_tiles.get(i).unwrap_or(0);

            let cell = Vector2i::new(pos.x as i32, pos.y as i32);
            let atlas = Vector2i::new(atlas.x as i32, atlas.y as i32);

            tilemap
                .set_cell_ex(layer, cell)
                .source_id(source_id)
                .atlas_coords(atlas)
                .alternative_tile(alt)
                .done();
        }

        godot_print!("SSXLTilemap: Rendered {len} tiles on layer {layer}");
    }

    // --- FFI RENDER (Callback) ---

    /// Helper function to get the current TileMap instance from its global ID.
    fn get_instance() -> Option<Gd<Self>> {
        let id = TILEMAP_INSTANCE_ID.get()?;
        // WARNING FIX: Removed unnecessary `unsafe` block.
        // `try_from_instance_id` is safe to call.
        godot::prelude::Gd::try_from_instance_id(*id).ok()
    }

    /// Flushes the pending update buffer to the Godot TileMap.
    pub fn flush_updates(&mut self) {
        // Must take ownership of the buffer *before* acquiring the mutable borrow to `self.base`.
        let updates = std::mem::take(&mut self.pending_updates);
        let len = updates.len();

        if len == 0 {
            return;
        }

        let source_id = self.tile_source_id;
        let mut tilemap = self.base_mut();
        
        tilemap.set_layer_enabled(DEFAULT_LAYER, true);

        // Apply all updates from the buffer
        for (world_x, world_y, tile_id) in updates {
            let cell = Vector2i::new(world_x, world_y);
            // Assuming tile_id maps to (0, tile_id) in the atlas for a placeholder
            let atlas = Vector2i::new(0, tile_id); 
            
            tilemap
                .set_cell_ex(DEFAULT_LAYER, cell)
                .source_id(source_id)
                .atlas_coords(atlas)
                .alternative_tile(0)
                .done();
        }

        godot_print!("FFI Host: Batch rendered {len} tiles via FFI callback.");
    }
}

// --- FFI HOST IMPLEMENTATION ---
// These functions are the "unresolved externals" from ssxl_engine_ffi.
// They are implemented here to satisfy the linker.

/// C-style function that queues a cell update.
#[no_mangle]
pub extern "C" fn ssxl_set_cell(x: i32, y: i32, tile_id: i32) {
    if let Some(mut map) = SSXLTilemap::get_instance() {
        // Queue the update to the instance's buffer
        map.bind_mut().pending_updates.push((x, y, tile_id));
    } else {
        godot_warn!("ssxl_set_cell: Tilemap instance not available. Update lost.");
    }
}

/// C-style function that triggers the buffered updates to be rendered.
#[no_mangle]
pub extern "C" fn ssxl_notify_tilemap_update() {
    if let Some(mut map) = SSXLTilemap::get_instance() {
        // Flush the buffered updates to the Godot API
        map.bind_mut().flush_updates();
    }
}