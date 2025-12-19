use crate::shared_math::ChunkCoords;
use crate::shared_tile::TileData;

/// Represents a single, self-contained, generated block of the world map.
/// This structure is the primary payload sent from worker threads to the conductor.
#[derive(Debug, Clone, Default)]
pub struct Chunk {
    /// The discrete coordinates (X, Y) identifying this chunk in the world grid.
    pub position: ChunkCoords,

    /// The edge length of the chunk (e.g., 32 for a 32x32 chunk).
    pub size: u32,

    /// The flat, contiguous array of tile data for the chunk.
    /// Storage is in Row-Major order (Y is outer loop, X is inner loop).
    pub tiles: Vec<TileData>,

    /// Metadata flag indicating if this chunk contains dynamic assets (entities, lights, etc.).
    pub contains_assets: bool,
}

impl Chunk {
    /// Initializes a new, empty chunk with the required size and position.
    pub fn new(position: ChunkCoords, size: u32) -> Self {
        let capacity = (size * size) as usize;
        Self {
            position,
            size,
            tiles: Vec::with_capacity(capacity),
            contains_assets: false,
        }
    }

    /// Calculates the 1D index from 2D local coordinates (X, Y).
    #[inline]
    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.size + x) as usize
    }

    /// Gets an immutable reference to the TileData at the specified local coordinates.
    #[inline]
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        self.tiles.get(self.get_index(x, y))
    }

    /// Gets a mutable reference to the TileData for modification during generation.
    #[inline]
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        let idx = self.get_index(x, y);
        self.tiles.get_mut(idx)
    }

    // -------------------------------------------------------------------------
    // ✅ NEW HELPERS FOR HALO-BASED CA
    // -------------------------------------------------------------------------

    /// Converts local chunk coordinates (0..size) into world coordinates.
    #[inline]
    pub fn local_to_world(&self, lx: i32, ly: i32) -> (i32, i32) {
        let base_x = self.position.0 * self.size as i32;
        let base_y = self.position.1 * self.size as i32;
        (base_x + lx, base_y + ly)
    }

    /// Converts world coordinates back into local chunk coordinates.
    /// Returns None if the world coordinate is not inside this chunk.
    #[inline]
    pub fn world_to_local(&self, wx: i32, wy: i32) -> Option<(u32, u32)> {
        let base_x = self.position.0 * self.size as i32;
        let base_y = self.position.1 * self.size as i32;

        let lx = wx - base_x;
        let ly = wy - base_y;

        if lx >= 0 && ly >= 0 && lx < self.size as i32 && ly < self.size as i32 {
            Some((lx as u32, ly as u32))
        } else {
            None
        }
    }

    /// Computes the world coordinate of the chunk’s top-left tile.
    #[inline]
    pub fn world_origin(&self) -> (i32, i32) {
        (
            self.position.0 * self.size as i32,
            self.position.1 * self.size as i32,
        )
    }
}
