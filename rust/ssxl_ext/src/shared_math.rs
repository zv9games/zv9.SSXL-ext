// --------------------------------------------------------------------------
// --- Type Aliases ---
// --------------------------------------------------------------------------

/// Type alias for 2D chunk coordinates (X, Y).
/// Used as the primary key for chunk addressing across the system (e.g., in cache.rs).
pub type ChunkCoords = (i32, i32);

/// Type alias for 2D coordinates used to identify a single tile within a chunk.
pub type LocalTileCoords = (i32, i32);

/// Type alias for 2D coordinates identifying a single tile in the entire world.
pub type WorldTileCoords = (i32, i32);


// --------------------------------------------------------------------------
// --- Constants ---
// --------------------------------------------------------------------------

pub const CA_NEIGHBOR_COUNT: u8 = 8;
pub const WORLD_GRAVITY_FACTOR: f32 = 9.81;
pub const F32_EPSILON: f32 = 0.00001;


// --------------------------------------------------------------------------
// --- Shared Structures ---
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct EntityMovementState {
    pub position_x: f32,
    pub position_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}


// --------------------------------------------------------------------------
// --- NEW: Chunk / Tile Coordinate Helpers (for halo CA) ---
// --------------------------------------------------------------------------

/// Extension helpers for ChunkCoords.
/// These do not change any existing behavior; they simply provide
/// the math needed for halo-based CA and neighbor lookup.
pub trait ChunkCoordsExt {
    /// Returns the coordinates of a neighboring chunk.
    /// Example: (cx, cy).neighbor(1, 0) → (cx+1, cy)
    fn neighbor(self, dx: i32, dy: i32) -> ChunkCoords;

    /// Converts world tile coordinates into chunk coordinates.
    /// Example: world (65, 10) with chunk_size 32 → chunk (2, 0)
    fn world_to_chunk(world: WorldTileCoords, chunk_size: i32) -> ChunkCoords;

    /// Converts world tile coordinates into local tile coordinates inside a chunk.
    /// Returns None if the world coordinate is not inside this chunk.
    fn world_to_local(self, world: WorldTileCoords, chunk_size: i32) -> Option<LocalTileCoords>;

    /// Computes the world-space origin (top-left tile) of this chunk.
    fn world_origin(self, chunk_size: i32) -> WorldTileCoords;
}

impl ChunkCoordsExt for ChunkCoords {
    #[inline]
    fn neighbor(self, dx: i32, dy: i32) -> ChunkCoords {
        (self.0 + dx, self.1 + dy)
    }

    #[inline]
    fn world_to_chunk(world: WorldTileCoords, chunk_size: i32) -> ChunkCoords {
        (
            world.0.div_euclid(chunk_size),
            world.1.div_euclid(chunk_size),
        )
    }

    #[inline]
    fn world_to_local(self, world: WorldTileCoords, chunk_size: i32) -> Option<LocalTileCoords> {
        let origin_x = self.0 * chunk_size;
        let origin_y = self.1 * chunk_size;

        let lx = world.0 - origin_x;
        let ly = world.1 - origin_y;

        if lx >= 0 && ly >= 0 && lx < chunk_size && ly < chunk_size {
            Some((lx, ly))
        } else {
            None
        }
    }

    #[inline]
    fn world_origin(self, chunk_size: i32) -> WorldTileCoords {
        (self.0 * chunk_size, self.1 * chunk_size)
    }
}
