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

impl Default for TileData {
    fn default() -> Self {
        // Default "Air" / empty tile
        Self {
            tile_id: 0,
            atlas_coords: 0,
            rotation_flags: 0,
            custom_data: 0,
        }
    }
}

impl TileData {
    /// Returns true if this tile is considered "live" (non-zero tile_id).
    pub fn is_live(&self) -> bool {
        self.tile_id != 0
    }

    /// Sets the tile to live (tile_id = 1) or dead (tile_id = 0).
    pub fn set_live(&mut self, live: bool) {
        if live {
            if self.tile_id == 0 {
                self.tile_id = 1;
            }
        } else {
            self.tile_id = 0;
            self.atlas_coords = 0;
            self.rotation_flags = 0;
            self.custom_data = 0;
        }
    }
}
