//C:/ZV(/zv9.aetherion/rust/src/zv9_aetherion_pipeline_data_map_grid.rs

use crate::zv9_prelude::*;
use std::collections::HashMap;
use crate::zv9_shared_types::Position;

/// 🧱 Types of tiles that can exist in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Chunk,
    Wall,
    Floor,
    Path,
    Blue,
    Black,
    // Extend with additional types as needed
}

/// 🗺️ Grid structure holding tile data and spatial bounds.
/// Used for procedural generation, placement, and runtime queries.
#[derive(Debug)]
pub struct MapGrid {
    tiles: HashMap<Position, TileType>,
    bounds: GridBounds,
}

impl MapGrid {
    /// Creates a new grid with the given bounds.
    pub fn new(bounds: GridBounds) -> Self {
        Self {
            tiles: HashMap::new(),
            bounds,
        }
    }

    /// Retrieves the tile at the given position.
    /// Returns `None` if the tile is not set.
    pub fn get(&self, pos: Position) -> Option<TileType> {
        self.tiles.get(&pos).copied()
    }

    /// Sets a tile at the given position.
    pub fn set(&mut self, pos: Position, tile: TileType) {
        if self.is_within_bounds(pos) {
            self.tiles.insert(pos, tile);
        }
    }

    /// Removes a tile at the given position.
    pub fn clear(&mut self, pos: Position) {
        self.tiles.remove(&pos);
    }

    /// Returns true if the position is within grid bounds.
    pub fn is_within_bounds(&self, pos: Position) -> bool {
        let sv = SerializableVector2i { x: pos.x, y: pos.y };
        self.bounds.contains(sv)
    }

    /// Returns the bounds of the grid.
    pub fn bounds(&self) -> GridBounds {
        self.bounds
    }

    /// Returns the number of tiles currently stored.
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Returns an iterator over all tile entries.
    pub fn iter(&self) -> impl Iterator<Item = (&Position, &TileType)> {
        self.tiles.iter()
    }

    /// Counts how many tiles match the given type.
    pub fn count_type(&self, target: TileType) -> usize {
        self.tiles.values().filter(|&&t| t == target).count()
    }
}

// the end