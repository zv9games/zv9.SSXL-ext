use godot::prelude::*;
use godot::classes::RenderingServer;
use godot::classes::rendering_server::PrimitiveType;
use godot::builtin::{
    Array as VarArray, PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant,
};

use crate::renderer::chunk_mesh::ChunkMesh;
use crate::config::DEBUG_MESH_UPLOAD;

pub struct MeshUpload;

impl MeshUpload {
    pub fn upload_chunk_mesh(
        chunk_mesh: &mut ChunkMesh,
        chunk_size: i32,
        world_origin: Vector3,
        scenario: Rid,
        material_rid: Rid,
    ) -> (Rid, Rid) {
        let mut rs = RenderingServer::singleton();

        // ------------------------------------------------------------
        // Validate material RID
        // ------------------------------------------------------------
        if !material_rid.is_valid() {
            godot_error!(
                "MeshUpload: INVALID material_rid {:?} for chunk ({}, {})",
                material_rid,
                chunk_mesh.cx,
                chunk_mesh.cy
            );
            return (Rid::new(0), Rid::new(0));
        }

        // ------------------------------------------------------------
        // If empty, free and bail
        // ------------------------------------------------------------
        if chunk_mesh.is_empty() {
            if DEBUG_MESH_UPLOAD {
                godot_print!(
                    "MeshUpload: chunk ({}, {}) EMPTY — freeing RIDs",
                    chunk_mesh.cx, chunk_mesh.cy
                );
            }
            MeshUpload::free_chunk_mesh(chunk_mesh);
            return (Rid::new(0), Rid::new(0));
        }

        // ------------------------------------------------------------
        // Ensure mesh RID
        // ------------------------------------------------------------
        let mesh_rid = match chunk_mesh.mesh_rid {
            Some(rid) => rid,
            None => {
                let rid = rs.mesh_create();
                chunk_mesh.mesh_rid = Some(rid);

                if DEBUG_MESH_UPLOAD {
                    godot_print!(
                        "MeshUpload: created mesh_rid {:?} for chunk ({}, {})",
                        rid, chunk_mesh.cx, chunk_mesh.cy
                    );
                }

                rid
            }
        };

        // ------------------------------------------------------------
        // Build arrays
        // ------------------------------------------------------------
        let verts = PackedVector3Array::from_iter(chunk_mesh.vertices.iter().copied());
        let uvs = PackedVector2Array::from_iter(chunk_mesh.uvs.iter().copied());
        let indices = PackedInt32Array::from_iter(chunk_mesh.indices.iter().map(|i| *i as i32));

        let verts_var = Variant::from(verts);
        let uvs_var = Variant::from(uvs);
        let indices_var = Variant::from(indices);

        const ARRAY_MAX: usize = 13;
        const ARRAY_VERTEX: usize = 0;
        const ARRAY_TEX_UV: usize = 4;
        const ARRAY_INDEX: usize = 12;

        let mut surface = VarArray::new();
        let fill = Variant::from(()); // dummy
        surface.resize(ARRAY_MAX, &fill);
        surface.set(ARRAY_VERTEX, &verts_var);
        surface.set(ARRAY_TEX_UV, &uvs_var);
        surface.set(ARRAY_INDEX, &indices_var);

        // ------------------------------------------------------------
        // Upload surface (no instance yet!)
        // ------------------------------------------------------------
        rs.mesh_clear(mesh_rid);
        rs.mesh_add_surface_from_arrays(mesh_rid, PrimitiveType::TRIANGLES, &surface);

        // ------------------------------------------------------------
        // Assign material BEFORE creating instance RID
        // ------------------------------------------------------------
        if DEBUG_MESH_UPLOAD {
            godot_print!(
                "MeshUpload: setting material_rid={:?} on mesh_rid={:?} surface=0 (chunk {}, {})",
                material_rid,
                mesh_rid,
                chunk_mesh.cx,
                chunk_mesh.cy
            );
        }
        rs.mesh_surface_set_material(mesh_rid, 0, material_rid);

        if DEBUG_MESH_UPLOAD {
            godot_print!(
                "MeshUpload: uploaded mesh for chunk ({}, {}) verts={} indices={}",
                chunk_mesh.cx,
                chunk_mesh.cy,
                chunk_mesh.vertices.len(),
                chunk_mesh.indices.len()
            );
        }

        // ------------------------------------------------------------
        // Create instance RID *after* material is valid
        // ------------------------------------------------------------
        let instance_rid = match chunk_mesh.instance_rid {
            Some(rid) => rid,
            None => {
                let rid = rs.instance_create();
                chunk_mesh.instance_rid = Some(rid);

                rs.instance_set_base(rid, mesh_rid);
                rs.instance_set_scenario(rid, scenario);

                if DEBUG_MESH_UPLOAD {
                    godot_print!(
                        "MeshUpload: created instance_rid {:?} for chunk ({}, {})",
                        rid, chunk_mesh.cx, chunk_mesh.cy
                    );
                }

                rid
            }
        };

        // ------------------------------------------------------------
        // Position chunk
        // ------------------------------------------------------------
        let offset = chunk_mesh.world_offset(chunk_size, world_origin);
        let transform = Transform3D::new(Basis::IDENTITY, offset);
        rs.instance_set_transform(instance_rid, transform);

        if DEBUG_MESH_UPLOAD {
            godot_print!(
                "MeshUpload: positioned chunk ({}, {}) at {:?}",
                chunk_mesh.cx,
                chunk_mesh.cy,
                offset
            );
        }

        chunk_mesh.mark_clean();

        (mesh_rid, instance_rid)
    }

    pub fn free_chunk_mesh(chunk_mesh: &mut ChunkMesh) {
        let mut rs = RenderingServer::singleton();

        if let Some(inst) = chunk_mesh.instance_rid.take() {
            rs.free_rid(inst);

            if DEBUG_MESH_UPLOAD {
                godot_print!(
                    "MeshUpload: freed instance_rid {:?} for chunk ({}, {})",
                    inst, chunk_mesh.cx, chunk_mesh.cy
                );
            }
        }

        if let Some(mesh) = chunk_mesh.mesh_rid.take() {
            rs.free_rid(mesh);

            if DEBUG_MESH_UPLOAD {
                godot_print!(
                    "MeshUpload: freed mesh_rid {:?} for chunk ({}, {})",
                    mesh, chunk_mesh.cx, chunk_mesh.cy
                );
            }
        }
    }
}
