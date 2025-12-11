// rust/SSXL-ext/src/shared_chunk.rs

use crate::shared_math::ChunkCoords;
use crate::shared_tile::TileData;

/// Represents a single, self-contained, generated block of the world map.
/// This structure is the primary payload sent from worker threads to the conductor.
// FIX: Add #[derive(Default)] to satisfy the requirement for std::mem::take in generate_batch_processor.rs
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

// rust/SSXL-ext/src/shared_chunk.rs

impl Chunk {
    /// Initializes a new, empty chunk with the required size and position.
    pub fn new(position: ChunkCoords, size: u32) -> Self {
        // Pre-allocate the vector capacity immediately for performance.
        let capacity = (size * size) as usize;
        Self {
            position,
            size,
            // Initialize with the correct size (will be filled during generation).
            tiles: Vec::with_capacity(capacity), 
            contains_assets: false,
        }
    }

    /// Calculates the 1D index from 2D local coordinates (X, Y).
    /// This ensures consistent Row-Major ordering across all modules.
    /// Note: This does not perform bounds checking; it assumes inputs (x, y) are 0..size.
    pub fn get_index(&self, x: u32, y: u32) -> usize {
        // Index = Y * Size + X (Row-Major Ordering)
        (y * self.size + x) as usize
    }

    /// Gets an immutable reference to the TileData at the specified local coordinates.
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        let index = self.get_index(x, y);
        self.tiles.get(index)
    }

    /// Gets a mutable reference to the TileData for modification during generation.
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        let index = self.get_index(x, y);
        self.tiles.get_mut(index)
    }
}