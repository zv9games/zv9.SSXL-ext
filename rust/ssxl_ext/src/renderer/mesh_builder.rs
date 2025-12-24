use godot::prelude::*;

use crate::renderer::chunk_mesh::ChunkMesh;
use crate::renderer::atlas::SSXLAtlas;
use crate::shared_tile::TileData;

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
        chunk_mesh.clear();

        let tile_size = self.tile_size;
        let tiles_per_side = chunk_size as usize;

        let mut printed_first = false;

        for local_y in 0..tiles_per_side {
            for local_x in 0..tiles_per_side {
                let idx = local_y * tiles_per_side + local_x;
                if idx >= tiles.len() {
                    continue;
                }

                let tile = tiles[idx];

                if tile.tile_id == 0 {
                    continue;
                }

                // ------------------------------------------------------------
                // DEBUG: tile ID sanity check
                // ------------------------------------------------------------
                if tile.tile_id < 0 {
                    godot_warn!(
                        "DEBUG MeshBuilder: INVALID tile_id={} at ({}, {})",
                        tile.tile_id, local_x, local_y
                    );
                }

                // ------------------------------------------------------------
                // Compute world-space quad position (XZ plane)
                // ------------------------------------------------------------
                let x0 = (local_x as f32) * tile_size;
                let z0 = (local_y as f32) * tile_size;
                let x1 = x0 + tile_size;
                let z1 = z0 + tile_size;

                // ------------------------------------------------------------
                // Fetch UVs from atlas
                // ------------------------------------------------------------
                let uv = atlas.get_uv(tile.tile_id as usize);

                // ------------------------------------------------------------
                // DEBUG: UV sanity check
                // ------------------------------------------------------------
                if uv.uv_min.x < 0.0 || uv.uv_min.y < 0.0 ||
                   uv.uv_max.x > 1.0 || uv.uv_max.y > 1.0 {
                    godot_warn!(
                        "DEBUG MeshBuilder: UV OUT OF RANGE for tile_id={} -> min={:?}, max={:?}",
                        tile.tile_id, uv.uv_min, uv.uv_max
                    );
                }

                // ------------------------------------------------------------
                // Push quad vertices (floor at y = 0)
                // ------------------------------------------------------------
                let base_index = chunk_mesh.vertices.len() as u32;

                let v0 = Vector3::new(x0, 0.0, z0);
                let v1 = Vector3::new(x1, 0.0, z0);
                let v2 = Vector3::new(x1, 0.0, z1);
                let v3 = Vector3::new(x0, 0.0, z1);

                chunk_mesh.vertices.push(v0);
                chunk_mesh.vertices.push(v1);
                chunk_mesh.vertices.push(v2);
                chunk_mesh.vertices.push(v3);

                let uv0 = uv.uv_min;
                let uv1 = Vector2::new(uv.uv_max.x, uv.uv_min.y);
                let uv2 = uv.uv_max;
                let uv3 = Vector2::new(uv.uv_min.x, uv.uv_max.y);

                chunk_mesh.uvs.push(uv0);
                chunk_mesh.uvs.push(uv1);
                chunk_mesh.uvs.push(uv2);
                chunk_mesh.uvs.push(uv3);

                // ------------------------------------------------------------
                // DEBUG: Print the first tile's geometry + UVs
                // ------------------------------------------------------------
                if !printed_first {
                    printed_first = true;

                    godot_print!(
                        "DEBUG MeshBuilder: first tile at ({}, {}), tile_id={}",
                        local_x, local_y, tile.tile_id
                    );

                    godot_print!(
                        "DEBUG MeshBuilder: atlas UV rect = min={:?}, max={:?}",
                        uv.uv_min, uv.uv_max
                    );

                    godot_print!(
                        "DEBUG MeshBuilder: first 4 vertices = {:?}, {:?}, {:?}, {:?}",
                        v0, v1, v2, v3
                    );

                    godot_print!(
                        "DEBUG MeshBuilder: first 4 UVs = {:?}, {:?}, {:?}, {:?}",
                        uv0, uv1, uv2, uv3
                    );
                }

                // ------------------------------------------------------------
                // Push indices (two triangles)
                // ------------------------------------------------------------
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
        // DEBUG: Summary of geometry
        // ------------------------------------------------------------
        godot_print!(
            "DEBUG MeshBuilder: chunk ({}, {}) built -> verts={} uvs={} indices={}",
            chunk_mesh.cx,
            chunk_mesh.cy,
            chunk_mesh.vertices.len(),
            chunk_mesh.uvs.len(),
            chunk_mesh.indices.len()
        );

        if chunk_mesh.vertices.is_empty() {
            godot_warn!(
                "DEBUG MeshBuilder: chunk ({}, {}) has NO GEOMETRY — material will appear NULL",
                chunk_mesh.cx,
                chunk_mesh.cy
            );
        }

        if chunk_mesh.indices.is_empty() {
            godot_warn!(
                "DEBUG MeshBuilder: chunk ({}, {}) has NO INDICES — surface will be empty",
                chunk_mesh.cx,
                chunk_mesh.cy
            );
        }

        chunk_mesh.mark_dirty();
    }
}
