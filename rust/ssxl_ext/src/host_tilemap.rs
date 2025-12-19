use godot::prelude::*;
use godot::classes::TileMap;
use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::shared_tile::TileData;
use crate::bridge_ffi::{ssxl_get_tilemap_chunk_ptr, ssxl_notify_chunk_updated};

pub trait TileMapDirectWriteExtension {
    fn get_raw_chunk_data_ptr(&mut self, layer: i32, cx: i32, cy: i32) -> *mut u8;
    fn notify_chunk_data_changed(&mut self, layer: i32, cx: i32, cy: i32);
}

impl TileMapDirectWriteExtension for Gd<TileMap> {
    fn get_raw_chunk_data_ptr(&mut self, layer: i32, cx: i32, cy: i32) -> *mut u8 {
        self.call(
            "get_raw_chunk_data_ptr",
            &[
                Variant::from(layer as i64),
                Variant::from(cx as i64),
                Variant::from(cy as i64),
            ],
        )
        .to::<*mut u8>()
    }

    fn notify_chunk_data_changed(&mut self, layer: i32, cx: i32, cy: i32) {
        let _ = self.call(
            "notify_chunk_data_changed",
            &[
                Variant::from(layer as i64),
                Variant::from(cx as i64),
                Variant::from(cy as i64),
            ],
        );
    }
}

pub fn render_chunk_direct(tilemap_id_raw: i64, chunk: Chunk) -> Result<(), SSXLCoreError> {
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

    unsafe {
        ssxl_notify_chunk_updated(
            tilemap_id_raw,
            chunk.position.0,
            chunk.position.1,
        );
    }

    Ok(())
}
