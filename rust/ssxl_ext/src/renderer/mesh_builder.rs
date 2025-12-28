use godot::prelude::*;

use crate::renderer::chunk_mesh::ChunkMesh;
use crate::renderer::atlas::SSXLAtlas;
use crate::shared_tile::TileData;
use crate::config::DEBUG_MESH_BUILDER;   // <-- global debug toggle

/// ------------------------------------------------------------
/// SSXL Mesh Builder (Plan B)
/// Tiles now built on the XZ plane (Y = up).
/// ------------------------------------------------------------
pub struct MeshBuilder {
    pub tile_size: f32,   // world-space tile size (usually 1.0)
}

impl MeshBuilder {
    pub fn new(tile_size: f32) -> Self {
        MeshBuilder { tile_size }
    }

    pub fn build_chunk_mesh(
        &self,
        chunk_mesh: &mut ChunkMesh,
        tiles: &[TileData],
        chunk_size: i32,
        atlas: &SSXLAtlas,
    ) {
        // ------------------------------------------------------------
        // Debug print
        // ------------------------------------------------------------
        if DEBUG_MESH_BUILDER {
            godot_print!(
                "MeshBuilder: building chunk ({}, {}) with {} tiles",
                chunk_mesh.cx,
                chunk_mesh.cy,
                tiles.len()
            );
        }

        chunk_mesh.clear();

        let tile_size = self.tile_size;
        let tiles_per_side = chunk_size as usize;

        for local_y in 0..tiles_per_side {
            for local_x in 0..tiles_per_side {
                let idx = local_y * tiles_per_side + local_x;
                if idx >= tiles.len() {
                    continue;
                }

                let tile = tiles[idx];

                // ------------------------------------------------------------
                // FIXED: skip only if atlas_coords == 0 (true empty tile)
                // ------------------------------------------------------------
                if tile.atlas_coords == 0 {
                    continue;
                }

                // Compute world-space quad position (XZ plane)
                let x0 = (local_x as f32) * tile_size;
                let z0 = (local_y as f32) * tile_size;
                let x1 = x0 + tile_size;
                let z1 = z0 + tile_size;

                // UV lookup uses atlas_coords
                let atlas_index = tile.atlas_coords as usize;
                let uv = atlas.get_uv(atlas_index);

                // Push quad vertices (floor at y = 0)
                let base_index = chunk_mesh.vertices.len() as u32;

                let v0 = Vector3::new(x0, 0.0, z0);
                let v1 = Vector3::new(x1, 0.0, z0);
                let v2 = Vector3::new(x1, 0.0, z1);
                let v3 = Vector3::new(x0, 0.0, z1);

                chunk_mesh.vertices.push(v0);
                chunk_mesh.vertices.push(v1);
                chunk_mesh.vertices.push(v2);
                chunk_mesh.vertices.push(v3);

                // UVs
                let uv0 = uv.uv_min;
                let uv1 = Vector2::new(uv.uv_max.x, uv.uv_min.y);
                let uv2 = uv.uv_max;
                let uv3 = Vector2::new(uv.uv_min.x, uv.uv_max.y);

                chunk_mesh.uvs.push(uv0);
                chunk_mesh.uvs.push(uv1);
                chunk_mesh.uvs.push(uv2);
                chunk_mesh.uvs.push(uv3);

                // Push indices (two triangles)
                chunk_mesh.indices.extend_from_slice(&[
                    base_index,
                    base_index + 1,
                    base_index + 2,
                    base_index,
                    base_index + 2,
                    base_index + 3,
                ]);
            }
        }

        // ------------------------------------------------------------
        // Debug print
        // ------------------------------------------------------------
        if DEBUG_MESH_BUILDER {
            godot_print!(
                "MeshBuilder: finished chunk ({}, {}) verts={} indices={}",
                chunk_mesh.cx,
                chunk_mesh.cy,
                chunk_mesh.vertices.len(),
                chunk_mesh.indices.len()
            );
        }

        chunk_mesh.mark_dirty();
    }
}
