// rust/SSXL-ext/src/shared_math.rs

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

/// The number of neighbors checked by the Cellular Automata algorithm (3x3 grid minus center).
pub const CA_NEIGHBOR_COUNT: u8 = 8;

/// A constant representing the world's gravity factor in the simulation, 
/// used by systems like animate_worker.rs.
pub const WORLD_GRAVITY_FACTOR: f32 = 9.81;

/// A small epsilon value used for floating-point comparisons to maintain precision.
pub const F32_EPSILON: f32 = 0.00001;


// --------------------------------------------------------------------------
// --- Shared Structures ---
// --------------------------------------------------------------------------

/// A simple structure to represent a position and a direction vector, 
/// used by animate_worker.rs for entities.
/// #[repr(C)] ensures FFI compatibility if this is passed directly to the Godot side.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)] 
pub struct EntityMovementState {
    pub position_x: f32,
    pub position_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}