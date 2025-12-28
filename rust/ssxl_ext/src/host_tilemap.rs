use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::config::DEBUG_HOST_TILEMAP;

use godot::prelude::*;
use godot::obj::{Gd, InstanceId};

use crate::renderer::renderer_node::SSXLRenderer;

/// Render a chunk by calling into the SSXLRenderer node.
/// This replaces the old TileMap pointer-copy pipeline entirely.
pub fn render_chunk_direct(renderer_id_raw: i64, chunk: Chunk) -> Result<(), SSXLCoreError> {
    if DEBUG_HOST_TILEMAP {
        eprintln!(
            "[host_tilemap] ENTER render_chunk_direct: chunk=({}, {}), tiles={}",
            chunk.position.0,
            chunk.position.1,
            chunk.tiles.len()
        );
    }

    // Convert raw i64 → InstanceId → Gd<Object>
    let instance_id = InstanceId::from_i64(renderer_id_raw);
    let obj: Gd<Object> = Gd::from_instance_id(instance_id);

    // Your Godot-Rust version: cast() returns Gd<SSXLRenderer> directly
    let mut renderer: Gd<SSXLRenderer> = obj.cast();

    let cx = chunk.position.0;
    let cy = chunk.position.1;

    if DEBUG_HOST_TILEMAP {
        eprintln!(
            "[host_tilemap] Calling SSXLRenderer.apply_chunk({}, {}), tiles={}",
            cx,
            cy,
            chunk.tiles.len()
        );
    }

    // Pass tile data as a slice (CRITICAL)
    renderer.bind_mut().apply_chunk(cx, cy, &chunk.tiles);

    if DEBUG_HOST_TILEMAP {
        eprintln!(
            "[host_tilemap] apply_chunk({}, {}) completed",
            cx, cy
        );
    }

    Ok(())
}
