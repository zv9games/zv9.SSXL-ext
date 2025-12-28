use std::collections::HashMap;

use godot::prelude::*;
use crate::shared_tile::TileData;

/// ------------------------------------------------------------
/// PLAN B VERSION:
/// SSXLChunkBuffer (formerly SSXLTileMap)
///
/// This node:
/// - Owns per‑chunk TileData buffers
/// - Exposes raw pointers for SSXL core writes
/// - Emits signals *only after data is written*, not on allocation
/// ------------------------------------------------------------
#[derive(GodotClass)]
#[class(base = Node)]
pub struct SSXLChunkBuffer {
    #[base]
    base: Base<Node>,

    /// Map of (cx, cy) -> flat tile buffer for that chunk.
    chunk_buffers: HashMap<(i32, i32), Vec<TileData>>,

    /// Cached chunk size in cells (must match SSXL config / host_state).
    chunk_size: i32,
}

#[godot_api]
impl INode for SSXLChunkBuffer {
    fn init(base: Base<Node>) -> Self {
        SSXLChunkBuffer {
            base,
            chunk_buffers: HashMap::new(),
            chunk_size: 32,
        }
    }
}

#[godot_api]
impl SSXLChunkBuffer {
    // ------------------------------------------------------------
    // Signals
    // ------------------------------------------------------------
    #[signal]
    fn chunk_ready(cx: i32, cy: i32);

    #[signal]
    fn chunk_updated(cx: i32, cy: i32);

    #[signal]
    fn chunk_cleared(cx: i32, cy: i32);

    #[signal]
    fn all_chunks_cleared();

    // ------------------------------------------------------------
    // Chunk size configuration
    // ------------------------------------------------------------
    #[func]
    pub fn set_chunk_size(&mut self, size: i32) {
        if size > 0 {
            self.chunk_size = size;
        }
    }

    #[func]
    pub fn get_chunk_size(&self) -> i32 {
        self.chunk_size
    }

    // ------------------------------------------------------------
    // Raw pointer access for SSXL core
    // (NO SIGNALS EMITTED HERE)
    // ------------------------------------------------------------
    #[func]
    pub fn get_raw_chunk_data_ptr(&mut self, _layer: i32, cx: i32, cy: i32) -> *mut u8 {
        let key = (cx, cy);
        let chunk_area = (self.chunk_size * self.chunk_size) as usize;

        let buf = self
            .chunk_buffers
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(chunk_area));

        if buf.len() < chunk_area {
            buf.resize(chunk_area, TileData::default());
        }

        buf.as_mut_ptr() as *mut u8
    }

    // ------------------------------------------------------------
    // Post-copy notification (called from ssxl_notify_chunk_updated)
    // ------------------------------------------------------------
    #[func]
    pub fn notify_chunk_data_changed(&mut self, cx: i32, cy: i32) {
        // Emit chunk_ready for first-time creation,
        // or chunk_updated for subsequent writes.
        if self.chunk_buffers.contains_key(&(cx, cy)) {
            self.base_mut().emit_signal(
                "chunk_updated",
                &[Variant::from(cx), Variant::from(cy)],
            );
        } else {
            self.base_mut().emit_signal(
                "chunk_ready",
                &[Variant::from(cx), Variant::from(cy)],
            );
        }
    }

    // ------------------------------------------------------------
    // Clear all chunk buffers
    // ------------------------------------------------------------
    #[func]
    pub fn clear_chunks(&mut self) {
        self.chunk_buffers.clear();
        self.base_mut().emit_signal("all_chunks_cleared", &[]);
    }

    // ------------------------------------------------------------
    // Optional: Clear a single chunk
    // ------------------------------------------------------------
    #[func]
    pub fn clear_chunk(&mut self, cx: i32, cy: i32) {
        if self.chunk_buffers.remove(&(cx, cy)).is_some() {
            self.base_mut().emit_signal(
                "chunk_cleared",
                &[Variant::from(cx), Variant::from(cy)],
            );
        }
    }
}

impl SSXLChunkBuffer {
    // ------------------------------------------------------------
    // Rust-only accessors for the renderer
    // ------------------------------------------------------------
    pub fn get_chunk_slice(&self, cx: i32, cy: i32) -> Option<&[TileData]> {
        self.chunk_buffers.get(&(cx, cy)).map(|v| v.as_slice())
    }

    pub fn get_chunk_slice_mut(&mut self, cx: i32, cy: i32) -> Option<&mut [TileData]> {
        let existed = self.chunk_buffers.contains_key(&(cx, cy));

        if existed {
            self.base_mut().emit_signal(
                "chunk_updated",
                &[Variant::from(cx), Variant::from(cy)],
            );
        }

        self.chunk_buffers
            .get_mut(&(cx, cy))
            .map(|v| v.as_mut_slice())
    }
}
