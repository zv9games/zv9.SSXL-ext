use godot::classes::Engine;
use godot::classes::RenderingServer;
use godot::classes::{CompressedTexture2D, StandardMaterial3D};
use godot::prelude::*;
use godot::builtin::Variant;

use std::collections::HashMap;

use crate::renderer::chunk_mesh::ChunkMesh;
use crate::renderer::mesh_builder::MeshBuilder;
use crate::renderer::mesh_upload::MeshUpload;
use crate::renderer::atlas::SSXLAtlas;
use crate::shared_tile::TileData;

use crate::config::{
    DEBUG_RENDERER,
    DEBUG_MESH_BUILDER,
    DEBUG_MESH_UPLOAD,
};

/// ------------------------------------------------------------
/// SSXLRenderer (Plan B)
/// ------------------------------------------------------------
#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct SSXLRenderer {
    #[base]
    base: Base<Node3D>,

    chunk_tiles: HashMap<(i32, i32), Vec<TileData>>,
    chunk_meshes: HashMap<(i32, i32), ChunkMesh>,

    mesh_builder: MeshBuilder,
    atlas: SSXLAtlas,

    chunk_size: i32,

    world_w: i32,
    world_h: i32,
    chunks_x: i32,
    chunks_y: i32,

    world_origin: Vector3,

    scenario_rid: Option<Rid>,

    material_rid: Option<Rid>,
    atlas_texture_rid: Option<Rid>,

    // Keep the material resource alive to ensure RS material stays valid
    material: Option<Gd<StandardMaterial3D>>,
    // Only allow chunk uploads after ready() has run
    is_ready: bool,
}

#[godot_api]
impl INode3D for SSXLRenderer {
    fn init(base: Base<Node3D>) -> Self {
        let mut this = SSXLRenderer {
            base,

            chunk_tiles: HashMap::new(),
            chunk_meshes: HashMap::new(),

            mesh_builder: MeshBuilder::new(1.0),

            atlas: SSXLAtlas::new_uniform(16, 16),

            chunk_size: 32,

            world_w: 0,
            world_h: 0,
            chunks_x: 0,
            chunks_y: 0,

            world_origin: Vector3::ZERO,

            scenario_rid: None,

            material_rid: None,
            atlas_texture_rid: None,

            material: None,
            is_ready: false,
        };

        // --------------------------------------------------------
        // EDITOR MODE: placeholder RS material
        // --------------------------------------------------------
        if Engine::singleton().is_editor_hint() {
            let mut rs = RenderingServer::singleton();
            let placeholder_mat = rs.material_create();
            this.material_rid = Some(placeholder_mat);

            if DEBUG_RENDERER {
                godot_print!(
                    "SSXLRenderer (editor): placeholder RS material created (RID={:?})",
                    this.material_rid
                );
            }

            return this;
        }

        // --------------------------------------------------------
        // RUNTIME: load real atlas + StandardMaterial3D
        // --------------------------------------------------------
        this.load_hardcoded_atlas();
        this
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            let mut base = self.base_mut();
            base.set_visible(false);
            base.set_notify_local_transform(false);

            if DEBUG_RENDERER {
                godot_print!("SSXLRenderer: editor mode — preview suppressed.");
            }
            return;
        }

        self.base_mut().set_visible(false);

        if let Some(world) = self.base().get_world_3d() {
            self.scenario_rid = Some(world.get_scenario());

            if DEBUG_RENDERER {
                godot_print!(
                    "SSXLRenderer ready, scenario RID = {:?}, material_rid={:?}",
                    self.scenario_rid,
                    self.material_rid
                );
            }
        } else {
            godot_warn!("SSXLRenderer: No World3D available in ready().");
        }

        // Mark renderer fully ready; now it’s safe to upload meshes
        self.is_ready = true;
    }
}

#[godot_api]
impl SSXLRenderer {
    // ------------------------------------------------------------
    // HARD-CODED ATLAS LOADING (runtime only)
    // ------------------------------------------------------------
    fn load_hardcoded_atlas(&mut self) {
        let path = "res://Tileset_atlas_final.png";

        let mut loader = godot::classes::ResourceLoader::singleton();
        let res_opt = loader.load(path);

        let Some(res) = res_opt else {
            godot_error!("Failed to load atlas texture via ResourceLoader at {}", path);
            return;
        };

        let tex: Gd<CompressedTexture2D> = match res.try_cast::<CompressedTexture2D>() {
            Ok(t) => t,
            Err(_) => {
                godot_error!("Resource at {} is not a CompressedTexture2D", path);
                return;
            }
        };

        let tex_rid = tex.get_rid();
        self.atlas_texture_rid = Some(tex_rid);

        // --------------------------------------------------------
        // Create StandardMaterial3D using the atlas as albedo texture
        // --------------------------------------------------------
        let mut mat = StandardMaterial3D::new_gd();
        mat.set("albedo_texture", &Variant::from(tex.clone()));
        mat.set("flags_unshaded", &Variant::from(true));

        // Store the material resource so Godot can keep the RS material alive
        let mat_clone = mat.clone();
        let mat_rid = mat_clone.get_rid();

        self.material = Some(mat_clone);
        self.material_rid = Some(mat_rid);

        if DEBUG_RENDERER {
            godot_print!(
                "Atlas loaded and StandardMaterial3D created (RID={:?}, tex_rid={:?})",
                mat_rid,
                tex_rid
            );
        }
    }

    // ------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------
    #[func]
    pub fn set_chunk_size(&mut self, size: i32) {
        self.chunk_size = size;
        self.mesh_builder = MeshBuilder::new(1.0);
    }

    #[func]
    pub fn get_instance_rid(&self) -> Rid {
        self.chunk_meshes
            .values()
            .find_map(|m| m.instance_rid)
            .unwrap_or(Rid::new(0))
    }

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
        self.chunk_tiles.clear();

        self.world_w = 0;
        self.world_h = 0;
        self.chunks_x = 0;
        self.chunks_y = 0;
        self.world_origin = Vector3::ZERO;

        if DEBUG_RENDERER {
            godot_print!("SSXLRenderer: world reset.");
        }

        if !Engine::singleton().is_editor_hint() {
            self.base_mut().set_visible(false);
        }
    }

    // ------------------------------------------------------------
    // PLAN B: Begin world
    // ------------------------------------------------------------
    #[func]
    pub fn begin_world(&mut self, world_w: i32, world_h: i32, chunk_size: i32) {
        self.chunk_size = chunk_size;
        self.world_w = world_w.max(0);
        self.world_h = world_h.max(0);

        if chunk_size > 0 {
            self.chunks_x = (self.world_w + chunk_size - 1) / chunk_size;
            self.chunks_y = (self.world_h + chunk_size - 1) / chunk_size;
        } else {
            self.chunks_x = 0;
            self.chunks_y = 0;
            godot_error!(
                "SSXLRenderer: begin_world called with invalid chunk_size {}",
                chunk_size
            );
        }

        self.world_origin = Vector3::new(
            -(self.world_w as f32) * 0.5,
            0.0,
            -(self.world_h as f32) * 0.5,
        );

        if DEBUG_RENDERER {
            godot_print!(
                "SSXLRenderer: begin world {}x{} tiles (chunk_size={}) → {}x{} chunks, world_origin={:?}",
                self.world_w,
                self.world_h,
                self.chunk_size,
                self.chunks_x,
                self.chunks_y,
                self.world_origin
            );
        }
    }

    // ------------------------------------------------------------
    // PLAN B: Apply chunk data
    // ------------------------------------------------------------
    pub fn apply_chunk(&mut self, cx: i32, cy: i32, tiles: &[TileData]) {
        // Hard guard: don’t upload anything until ready() has run
        if !self.is_ready {
            if DEBUG_RENDERER {
                godot_print!(
                    "SSXLRenderer: not ready; skipping apply_chunk({}, {})",
                    cx, cy
                );
            }
            return;
        }

        let Some(mat_rid) = self.material_rid else {
            if DEBUG_RENDERER {
                godot_print!("SSXLRenderer: material_rid None; skipping chunk ({}, {})", cx, cy);
            }
            return;
        };

        if !mat_rid.is_valid() {
            godot_error!(
                "SSXLRenderer: INVALID material_rid {:?} for chunk ({}, {})",
                mat_rid, cx, cy
            );
            return;
        }

        let Some(scenario) = self.scenario_rid else {
            if DEBUG_RENDERER {
                godot_print!("SSXLRenderer: scenario None; skipping chunk ({}, {})", cx, cy);
            }
            return;
        };

        if DEBUG_RENDERER {
            godot_print!(
                "SSXLRenderer: ENTER apply_chunk({}, {}), tiles={}, material_rid={:?}",
                cx, cy, tiles.len(), mat_rid
            );
        }

        if tiles.is_empty() {
            godot_warn!("SSXLRenderer: No tiles passed for chunk ({}, {}).", cx, cy);
            return;
        }

        self.chunk_tiles.insert((cx, cy), tiles.to_vec());

        let mesh_key = (cx, cy);
        let mesh = self
            .chunk_meshes
            .entry(mesh_key)
            .or_insert_with(|| ChunkMesh::new(cx, cy));

        let stored_tiles = self.chunk_tiles.get(&(cx, cy)).unwrap();

        self.mesh_builder
            .build_chunk_mesh(mesh, stored_tiles, self.chunk_size, &self.atlas);

        if DEBUG_MESH_BUILDER {
            godot_print!(
                "DEBUG: Chunk ({}, {}) verts={} indices={}",
                cx, cy, mesh.vertices.len(),
                mesh.indices.len()
            );
        }

        let (mesh_rid, instance_rid) = MeshUpload::upload_chunk_mesh(
            mesh,
            self.chunk_size,
            self.world_origin,
            scenario,
            mat_rid,
        );

        mesh.mesh_rid = Some(mesh_rid);
        mesh.instance_rid = Some(instance_rid);

        if !Engine::singleton().is_editor_hint() && !self.base().is_visible() {
            self.base_mut().set_visible(true);
        }

        if DEBUG_MESH_UPLOAD {
            godot_print!(
                "SSXLRenderer: applied chunk ({}, {}), mesh uploaded (mesh_rid={:?}, instance_rid={:?}).",
                cx, cy, mesh_rid, instance_rid
            );
        }
    }

    // ------------------------------------------------------------
    // PLAN B: Finalize world
    // ------------------------------------------------------------
    #[func]
    pub fn finalize_world(&mut self) {
        if DEBUG_RENDERER {
            godot_print!(
                "SSXLRenderer: finalize world; world={}x{} tiles, chunks={}x{}, origin={:?}",
                self.world_w,
                self.world_h,
                self.chunks_x,
                self.chunks_y,
                self.world_origin
            );
        }
    }
}
