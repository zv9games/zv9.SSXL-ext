use godot::prelude::*;
use std::collections::HashMap;

use crate::renderer::chunk_mesh::ChunkMesh;
use crate::renderer::mesh_builder::MeshBuilder;
use crate::renderer::mesh_upload::MeshUpload;
use crate::renderer::atlas::SSXLAtlas;

use crate::ssxl_chunk_buffer::SSXLChunkBuffer;

/// ------------------------------------------------------------
/// SSXLRenderer (Plan B)
/// ------------------------------------------------------------
#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct SSXLRenderer {
    #[base]
    base: Base<Node3D>,

    /// Source of tile data (Plan B buffer).
    chunk_buffer: Option<Gd<SSXLChunkBuffer>>,
    /// CPU-side meshes per chunk coord.
    chunk_meshes: HashMap<(i32, i32), ChunkMesh>,

    /// CPU-side mesh builder.
    mesh_builder: MeshBuilder,
    /// UV atlas for tiles.
    atlas: SSXLAtlas,

    /// Chunk size in tiles.
    chunk_size: i32,
    /// Scenario RID used for instances (lazy-initialized).
    scenario_rid: Option<Rid>,
}

#[godot_api]
impl INode3D for SSXLRenderer {
    fn init(base: Base<Node3D>) -> Self {
        SSXLRenderer {
            base,
            chunk_buffer: None,
            chunk_meshes: HashMap::new(),
            mesh_builder: MeshBuilder::new(1.0),
            atlas: SSXLAtlas::new_uniform(32, 32),
            chunk_size: 32,
            scenario_rid: None,
        }
    }

    fn ready(&mut self) {
        if let Some(world) = self.base().get_world_3d() {
            self.scenario_rid = Some(world.get_scenario());
            godot_print!(
                "SSXLRenderer ready, scenario RID = {:?}",
                self.scenario_rid
            );
        } else {
            godot_warn!("SSXLRenderer: No World3D available in ready().");
        }
    }
}

#[godot_api]
impl SSXLRenderer {
    // ------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------
    #[func]
    pub fn set_chunk_buffer(&mut self, buffer: Gd<SSXLChunkBuffer>) {
        self.chunk_buffer = Some(buffer);
    }

    #[func]
    pub fn set_chunk_size(&mut self, size: i32) {
        self.chunk_size = size;
        self.mesh_builder = MeshBuilder::new(1.0);
    }

    #[func]
    pub fn set_uniform_atlas(&mut self, tiles_x: i32, tiles_y: i32) {
        self.atlas = SSXLAtlas::new_uniform(tiles_x, tiles_y);
    }

    // ------------------------------------------------------------
    // Expose instance RID (first valid chunk)
    // ------------------------------------------------------------
    #[func]
    pub fn get_instance_rid(&self) -> Rid {
        self.chunk_meshes
            .values()
            .find_map(|m| m.instance_rid)
            .unwrap_or(Rid::new(0))
    }

    // ------------------------------------------------------------
    // Expose mesh RID (first valid chunk)
    // ------------------------------------------------------------
    #[func]
    pub fn get_mesh_rid(&self) -> Rid {
        self.chunk_meshes
            .values()
            .find_map(|m| m.mesh_rid)
            .unwrap_or(Rid::new(0))
    }

    // ------------------------------------------------------------
    // PLAN B: Reset world
    // ------------------------------------------------------------
    #[func]
    pub fn reset_world(&mut self) {
        for (_, mesh) in self.chunk_meshes.iter_mut() {
            MeshUpload::free_chunk_mesh(mesh);
        }

        self.chunk_meshes.clear();
        godot_print!("SSXLRenderer: world reset.");
    }

    // ------------------------------------------------------------
    // PLAN B: Begin world
    // ------------------------------------------------------------
    #[func]
    pub fn begin_world(&mut self, world_w: i32, world_h: i32, chunk_size: i32) {
        self.chunk_size = chunk_size;
        godot_print!(
            "SSXLRenderer: begin world {}x{} (chunk_size={})",
            world_w,
            world_h,
            chunk_size
        );
    }

    // ------------------------------------------------------------
    // PLAN B: Apply chunk data
    // ------------------------------------------------------------
    #[func]
    pub fn apply_chunk(&mut self, cx: i32, cy: i32) {
        let buffer = match &self.chunk_buffer {
            Some(b) => b,
            None => {
                godot_error!("SSXLRenderer: No chunk buffer set.");
                return;
            }
        };

        let binding = buffer.bind();
        let tiles = match binding.get_chunk_slice(cx, cy) {
            Some(slice) => slice,
            None => {
                godot_warn!("SSXLRenderer: No tiles for chunk ({}, {}).", cx, cy);
                return;
            }
        };

        // Ensure scenario RID exists
        if self.scenario_rid.is_none() {
            if let Some(world) = self.base().get_world_3d() {
                self.scenario_rid = Some(world.get_scenario());
            } else {
                godot_error!("SSXLRenderer: No World3D at apply_chunk.");
                return;
            }
        }

        let scenario = self.scenario_rid.unwrap();

        // Build mesh
        let mesh_key = (cx, cy);
        let mesh = self
            .chunk_meshes
            .entry(mesh_key)
            .or_insert_with(|| ChunkMesh::new(cx, cy));

        self.mesh_builder
            .build_chunk_mesh(mesh, tiles, self.chunk_size, &self.atlas);

        // DEBUG: print geometry info
        godot_print!(
            "DEBUG: Chunk ({}, {}) verts={} indices={}",
            cx,
            cy,
            mesh.vertices.len(),
            mesh.indices.len()
        );

        // Upload via RenderingServer
        MeshUpload::upload_chunk_mesh(mesh, self.chunk_size, scenario);

        godot_print!(
            "SSXLRenderer: applied chunk ({}, {}), mesh uploaded.",
            cx,
            cy
        );
    }

    // ------------------------------------------------------------
    // PLAN B: Finalize world
    // ------------------------------------------------------------
    #[func]
    pub fn finalize_world(&mut self) {
        godot_print!("SSXLRenderer: finalize world.");
    }
}
