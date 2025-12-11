// rust/SSXL-ext/src/shared_tile.rs

/// The minimal data required to represent a single tile in the Godot TileMap.
///
/// #[repr(C)] ensures FFI-compatibility for direct memory writing 
/// to Godot's C++ data structures, making the chunk rendering extremely fast.
/// 
/// Total size: 6 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileData {
    pub tile_id: u16,        // ID corresponding to a tile in the Godot TileSet (source ID)
    pub atlas_coords: u16,   // Packed coords within the TileSet Atlas
    pub rotation_flags: u8,  // Rotation, flip, and custom flags
    pub custom_data: u8,     // Used for dynamic data (e.g., fluid level, density score)
}

// Add a default implementation for easy initialization (e.g., for empty chunks)
impl Default for TileData {
    fn default() -> Self {
        // Defaulting to an "Air" or "Empty" tile (ID 0)
        Self {
            tile_id: 0, 
            atlas_coords: 0, 
            rotation_flags: 0, 
            custom_data: 0,
        }
    }
}

// --- FIX: Implement the is_live and set_live methods for Cellular Automata logic ---
impl TileData {
    /// Checks if the tile is in a 'live' state. 
    /// In the context of a tile map, this usually means the tile is present (tile_id != 0).
    pub fn is_live(&self) -> bool {
        // A tile is 'live' if it has been assigned a valid tile_id (i.e., not the default 0).
        self.tile_id != 0
    }

    /// Sets the live/dead state of the tile.
    /// To set 'live' (true), we use a placeholder tile_id (1).
    /// To set 'dead' (false), we use the default 'empty' tile_id (0).
    pub fn set_live(&mut self, live: bool) {
        if live {
            // Set to a placeholder 'live' tile ID (assuming ID 1 is the default solid tile)
            if self.tile_id == 0 {
                self.tile_id = 1; 
            }
        } else {
            // Set to the 'dead' (empty) tile ID
            self.tile_id = 0;
            // Optionally clear other related fields for an empty tile
            self.atlas_coords = 0;
            self.rotation_flags = 0;
            self.custom_data = 0;
        }
    }
}
// --- END FIX ---


/// A structure to hold the generated data for a single TileMap chunk.
/// This is the payload delivered from the worker threads to the main thread.
#[derive(Debug, Clone)] // Clone is necessary for safe job/message passing
pub struct Chunk {
    pub position: (i32, i32), // Grid position of the chunk
    pub tiles: Vec<TileData>, // The raw tile data array
    pub size: u32,             // Edge length (e.g., 16x16 chunk -> size = 16)
}

// Default implementation for Chunk (e.g., for empty/uninitialized chunks)
impl Default for Chunk {
    fn default() -> Self {
        Self {
            position: (0, 0),
            tiles: Vec::new(),
            size: 0,
        }
    }
}