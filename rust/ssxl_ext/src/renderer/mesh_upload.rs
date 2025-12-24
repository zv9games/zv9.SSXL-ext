use godot::prelude::*;
use godot::classes::RenderingServer;
use godot::classes::rendering_server::PrimitiveType;
use godot::builtin::{
    Array as VarArray, PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant,
};

use crate::renderer::chunk_mesh::ChunkMesh;

pub struct MeshUpload;

impl MeshUpload {
    pub fn upload_chunk_mesh(
        chunk_mesh: &mut ChunkMesh,
        chunk_size: i32,
        scenario: Rid,
    ) -> Rid {
        let mut rs = RenderingServer::singleton();

        // If this mesh has no geometry, free any existing RIDs and return an invalid RID.
        if chunk_mesh.is_empty() {
            godot_warn!(
                "DEBUG MeshUpload: chunk ({}, {}) EMPTY — freeing RIDs",
                chunk_mesh.cx, chunk_mesh.cy
            );
            MeshUpload::free_chunk_mesh(chunk_mesh);
            return Rid::new(0);
        }

        // ------------------------------------------------------------
        // Ensure ArrayMesh RID exists
        // ------------------------------------------------------------
        let mesh_rid = match chunk_mesh.mesh_rid {
            Some(rid) => {
                godot_print!(
                    "DEBUG MeshUpload: reusing mesh_rid={:?} for chunk ({}, {})",
                    rid, chunk_mesh.cx, chunk_mesh.cy
                );
                rid
            }
            None => {
                let rid = rs.mesh_create();
                chunk_mesh.mesh_rid = Some(rid);
                godot_print!(
                    "DEBUG MeshUpload: created NEW mesh_rid={:?} for chunk ({}, {})",
                    rid, chunk_mesh.cx, chunk_mesh.cy
                );
                rid
            }
        };

        // ------------------------------------------------------------
        // Build PackedArrays
        // ------------------------------------------------------------
        let verts = PackedVector3Array::from_iter(chunk_mesh.vertices.iter().copied());
        let uvs = PackedVector2Array::from_iter(chunk_mesh.uvs.iter().copied());
        let indices = PackedInt32Array::from_iter(chunk_mesh.indices.iter().map(|i| *i as i32));

        let verts_var = Variant::from(verts);
        let uvs_var = Variant::from(uvs);
        let indices_var = Variant::from(indices);
        let nil = Variant::nil();

        // ------------------------------------------------------------
        // Build surface array for mesh_add_surface_from_arrays
        // ------------------------------------------------------------
        const ARRAY_MAX: usize = 13;
        const ARRAY_VERTEX: usize = 0;
        const ARRAY_TEX_UV: usize = 4;
        const ARRAY_INDEX: usize = 12;

        let mut surface = VarArray::new();
        surface.resize(ARRAY_MAX, &nil);
        surface.set(ARRAY_VERTEX, &verts_var);
        surface.set(ARRAY_TEX_UV, &uvs_var);
        surface.set(ARRAY_INDEX, &indices_var);

        // ------------------------------------------------------------
        // Upload to GPU
        // ------------------------------------------------------------
        rs.mesh_clear(mesh_rid);
        rs.mesh_add_surface_from_arrays(mesh_rid, PrimitiveType::TRIANGLES, &surface);

        // DEBUG: surface count after upload
        let surf_count = rs.mesh_get_surface_count(mesh_rid);
        godot_print!(
            "DEBUG MeshUpload: mesh_rid={:?} now has {} surfaces (verts={}, indices={})",
            mesh_rid,
            surf_count,
            chunk_mesh.vertices.len(),
            chunk_mesh.indices.len()
        );

        if surf_count == 0 {
            godot_warn!(
                "DEBUG MeshUpload: WARNING — mesh_rid={:?} has NO SURFACES. Material will appear NULL.",
                mesh_rid
            );
        }

        // ------------------------------------------------------------
        // Ensure instance RID exists
        // ------------------------------------------------------------
        let instance_rid = match chunk_mesh.instance_rid {
            Some(rid) => {
                godot_print!(
                    "DEBUG MeshUpload: reusing instance_rid={:?} for chunk ({}, {})",
                    rid, chunk_mesh.cx, chunk_mesh.cy
                );
                rid
            }
            None => {
                let rid = rs.instance_create();
                chunk_mesh.instance_rid = Some(rid);
                godot_print!(
                    "DEBUG MeshUpload: created NEW instance_rid={:?} for chunk ({}, {})",
                    rid, chunk_mesh.cx, chunk_mesh.cy
                );
                rs.instance_set_base(rid, mesh_rid);
                rs.instance_set_scenario(rid, scenario);
                rid
            }
        };

        // ------------------------------------------------------------
        // Position chunk
        // ------------------------------------------------------------
        let offset = chunk_mesh.world_offset(chunk_size);
        let transform = Transform3D::new(Basis::IDENTITY, offset);
        rs.instance_set_transform(instance_rid, transform);

        godot_print!(
            "DEBUG MeshUpload: instance_rid={:?} transform set to {:?}",
            instance_rid,
            offset
        );

        chunk_mesh.mark_clean();
        mesh_rid
    }

    pub fn free_chunk_mesh(chunk_mesh: &mut ChunkMesh) {
        let mut rs = RenderingServer::singleton();

        if let Some(inst) = chunk_mesh.instance_rid.take() {
            godot_print!(
                "DEBUG MeshUpload: freeing instance_rid={:?} for chunk ({}, {})",
                inst, chunk_mesh.cx, chunk_mesh.cy
            );
            rs.free_rid(inst);
        }
        if let Some(mesh) = chunk_mesh.mesh_rid.take() {
            godot_print!(
                "DEBUG MeshUpload: freeing mesh_rid={:?} for chunk ({}, {})",
                mesh, chunk_mesh.cx, chunk_mesh.cy
            );
            rs.free_rid(mesh);
        }
    }
}
