use godot::prelude::*;

/// ------------------------------------------------------------
/// SSXL Chunk Mesh (CPU-side)
///
/// This struct represents the mesh for a single chunk in the
/// SSXL Plan B renderer. It contains:
///
/// ✅ CPU-side vertex data
/// ✅ CPU-side UV data
/// ✅ CPU-side index buffer
/// ✅ Chunk world position
/// ✅ RenderingServer RIDs (mesh + instance)
/// ✅ Dirty flag (needs rebuild)
///
/// RenderingServer upload happens in mesh_upload.rs.
/// Mesh building happens in mesh_builder.rs.
/// ------------------------------------------------------------
#[derive(Debug)]
pub struct ChunkMesh {
    /// Chunk coordinate (cx, cy)
    pub cx: i32,
    pub cy: i32,

    /// CPU-side mesh buffers
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub indices: Vec<u32>,

    /// RenderingServer resources
    pub mesh_rid: Option<Rid>,
    pub instance_rid: Option<Rid>,

    /// Whether this chunk needs to be rebuilt or re-uploaded
    pub dirty: bool,
}

impl ChunkMesh {
    /// Create an empty chunk mesh at chunk coordinate (cx, cy)
    pub fn new(cx: i32, cy: i32) -> Self {
        let mesh = ChunkMesh {
            cx,
            cy,
            vertices: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            mesh_rid: None,
            instance_rid: None,
            dirty: true,
        };

        godot_print!(
            "DEBUG ChunkMesh::new -> ({}, {}) created (dirty=true, no RIDs yet)",
            cx,
            cy
        );

        mesh
    }

    /// Clear CPU-side buffers (used before rebuilding)
    pub fn clear(&mut self) {
        godot_print!(
            "DEBUG ChunkMesh::clear -> ({}, {}) clearing geometry (verts={}, indices={})",
            self.cx,
            self.cy,
            self.vertices.len(),
            self.indices.len()
        );

        self.vertices.clear();
        self.uvs.clear();
        self.indices.clear();
        self.dirty = true;

        godot_print!(
            "DEBUG ChunkMesh::clear -> ({}, {}) now empty, dirty=true",
            self.cx,
            self.cy
        );
    }

    /// Mark this chunk as needing a GPU upload
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        godot_print!(
            "DEBUG ChunkMesh::mark_dirty -> ({}, {}) marked dirty",
            self.cx,
            self.cy
        );
    }

    /// Mark this chunk as clean after GPU upload
    pub fn mark_clean(&mut self) {
        self.dirty = false;
        godot_print!(
            "DEBUG ChunkMesh::mark_clean -> ({}, {}) marked clean (mesh_rid={:?}, instance_rid={:?})",
            self.cx,
            self.cy,
            self.mesh_rid,
            self.instance_rid
        );
    }

    /// Returns true if this chunk has no geometry
    pub fn is_empty(&self) -> bool {
        let empty = self.vertices.is_empty();
        if empty {
            godot_print!(
                "DEBUG ChunkMesh::is_empty -> ({}, {}) EMPTY (verts=0)",
                self.cx,
                self.cy
            );
        }
        empty
    }

    /// Compute world-space offset for this chunk
    /// (used by renderer_node.rs when placing instances)
    ///
    /// Now matches MeshBuilder's XZ-plane layout: X/Z = chunk grid, Y = up.
    pub fn world_offset(&self, chunk_size: i32) -> Vector3 {
        Vector3::new(
            (self.cx * chunk_size) as f32,
            0.0,
            (self.cy * chunk_size) as f32,
        )
    }

    /// ------------------------------------------------------------
    /// Debug helper: print full state of this chunk mesh
    /// ------------------------------------------------------------
    pub fn debug_print_state(&self) {
        godot_print!(
            "DEBUG ChunkMesh STATE -> ({}, {}) verts={} uvs={} indices={} mesh_rid={:?} instance_rid={:?} dirty={}",
            self.cx,
            self.cy,
            self.vertices.len(),
            self.uvs.len(),
            self.indices.len(),
            self.mesh_rid,
            self.instance_rid,
            self.dirty
        );
    }
}
