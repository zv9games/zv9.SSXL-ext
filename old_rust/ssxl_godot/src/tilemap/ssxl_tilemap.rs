// ============================================================================
// 🧩 SSXLTilemap (`crate::tilemap::ssxl_tilemap`)
// ----------------------------------------------------------------------------
// This module defines the `SSXLTilemap` class, a custom Godot `TileMap` node
// that integrates Rust-based rendering logic with Godot’s scene graph. It
// supports both signal-driven batch rendering and FFI-driven buffered updates,
// ensuring tiles can be placed efficiently from multiple sources.
//
// Purpose:
//   • Provide a Godot-facing `TileMap` implementation controlled by the SSXL engine.
//   • Support bulk tile placement via signals or GDScript (`batch_set_tiles`).
//   • Enable external C-style FFI callbacks to queue and flush tile updates.
//   • Maintain a buffer of pending updates for safe, batched rendering.
//
// Key Components:
//   • Global State
//       - TILEMAP_INSTANCE_ID: stores the Godot instance ID of the active tilemap.
//         Allows external FFI functions to safely access the tilemap.
//       - DEFAULT_LAYER: default layer index used when placing tiles via FFI.
//
//   • SSXLTilemap (struct)
//       - Attributes:
//           • #[derive(GodotClass)] + #[class(base = TileMap)]
//             - Marks SSXLTilemap as a Godot class inheriting from TileMap.
//       - Fields:
//           • base: underlying Godot TileMap node.
//           • tile_source_id: ID of the tile source used when setting cells.
//           • pending_updates: buffer of cell updates queued via FFI calls.
//
//   • ITileMap Implementation
//       - init(base): called when the TileMap node is created.
//       - Stores instance ID globally for FFI access.
//       - Initializes with default tile source and empty update buffer.
//
//   • Godot API Methods
//       - batch_set_tiles(batch):
//           • Primary entrypoint for rendering tiles in bulk.
//           • Expects a Dictionary with keys: "layer", "positions",
//             "atlas_coords", "alt_tiles".
//           • Places tiles on the specified layer using provided coordinates.
//       - get_instance():
//           • Retrieves the current SSXLTilemap instance using its global ID.
//           • Enables safe access for FFI functions.
//       - flush_updates():
//           • Applies all pending updates queued via FFI calls.
//           • Renders them on the default layer in bulk.
//
//   • FFI Host Functions
//       - ssxl_set_cell(x, y, tile_id):
//           • Queues a single cell update into `pending_updates`.
//           • Called from external FFI code.
//       - ssxl_notify_tilemap_update():
//           • Flushes all queued updates to the TileMap.
//           • Called from external FFI code.
//
// Design Choices:
//   • Separation of batch rendering (signals) and buffered updates (FFI)
//     ensures flexibility in how tiles are placed.
//   • Global instance ID allows external systems to interact with the tilemap
//     without unsafe global state manipulation.
//   • Buffered updates prevent unsafe concurrent modifications by batching
//     tile placement into a controlled flush.
//
// Educational Note:
//   • This module demonstrates how Rust can extend Godot’s TileMap with
//     custom rendering logic, while also exposing safe FFI hooks for external
//     systems. By combining signal-driven updates with FFI callbacks,
//     `SSXLTilemap` provides a robust, dual-path rendering pipeline that
//     integrates seamlessly into Godot’s scene graph.
// ============================================================================


use godot::prelude::*;
use godot::classes::{TileMap, ITileMap};
use godot::obj::Base;
use godot::builtin::{Vector2i, PackedVector2Array, PackedInt32Array};
use once_cell::sync::OnceCell;

pub static TILEMAP_INSTANCE_ID: OnceCell<InstanceId> = OnceCell::new(); 
const DEFAULT_LAYER: i32 = 0;

#[derive(GodotClass)]
#[class(base = TileMap)]
pub struct SSXLTilemap {
    base: Base<TileMap>,

    #[export]
    pub tile_source_id: i32,
    pub pending_updates: Vec<(i32, i32, i32)>,
}

#[godot_api]
impl ITileMap for SSXLTilemap {
    fn init(base: Base<TileMap>) -> Self {
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

    fn get_instance() -> Option<Gd<Self>> {
        let id = TILEMAP_INSTANCE_ID.get()?;
        godot::prelude::Gd::try_from_instance_id(*id).ok()
    }

    pub fn flush_updates(&mut self) {
        let updates = std::mem::take(&mut self.pending_updates);
        let len = updates.len();

        if len == 0 {
            return;
        }

        let source_id = self.tile_source_id;
        let mut tilemap = self.base_mut();
        
        tilemap.set_layer_enabled(DEFAULT_LAYER, true);

        for (world_x, world_y, tile_id) in updates {
            let cell = Vector2i::new(world_x, world_y);
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

#[no_mangle]
pub extern "C" fn ssxl_set_cell(x: i32, y: i32, tile_id: i32) {
    if let Some(mut map) = SSXLTilemap::get_instance() {
        map.bind_mut().pending_updates.push((x, y, tile_id));
    } else {
        godot_warn!("ssxl_set_cell: Tilemap instance not available. Update lost.");
    }
}

#[no_mangle]
pub extern "C" fn ssxl_notify_tilemap_update() {
    if let Some(mut map) = SSXLTilemap::get_instance() {
        map.bind_mut().flush_updates();
    }
}
