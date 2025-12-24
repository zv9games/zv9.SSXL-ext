// ------------------------------------------------------------
// SSXL Renderer (Plan B)
// ------------------------------------------------------------
//
// This module owns the entire rendering pipeline for SSXL.
//
// Responsibilities:
// ✅ Read raw chunk buffers from SSXLChunkBuffer
// ✅ Build per‑chunk meshes (CPU)
// ✅ Upload meshes to Godot's RenderingServer (GPU)
// ✅ Manage chunk instances, visibility, and culling
// ✅ Replace TileMap entirely
//
// Submodules:
// - chunk_mesh.rs:     CPU-side mesh representation
// - mesh_builder.rs:   Converts TileData → vertices/UVs/indices
// - atlas.rs:          UV atlas mapping for tiles
// - mesh_upload.rs:    RenderingServer mesh creation + updates
// - renderer_node.rs:  Godot-facing node that owns the renderer
//
// ------------------------------------------------------------

pub mod chunk_mesh;
pub mod mesh_builder;
pub mod atlas;
pub mod mesh_upload;
pub mod renderer_node;

// Re-export the main renderer node for easy access
pub use renderer_node::SSXLRenderer;
