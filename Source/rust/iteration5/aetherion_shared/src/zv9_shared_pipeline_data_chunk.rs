use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::zv9_prelude::*;


/// 🧱 MapDataChunk — grid-aligned tile metadata used in procedural generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MapDataChunk {
    pub tiles: HashMap<SerializableVector2i, TileInfo>,
}

impl MapDataChunk {
    /// Creates an empty chunk.
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Inserts a tile at the given position.
    pub fn insert(&mut self, pos: SerializableVector2i, info: TileInfo) {
        self.tiles.insert(pos, info);
    }

    /// Returns the number of tiles in the chunk.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns true if the chunk is empty.
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Consumes the chunk and returns its inner map.
    pub fn into_inner(self) -> HashMap<SerializableVector2i, TileInfo> {
        self.tiles
    }

    /// Returns an iterator over all tile entries.
    pub fn iter(&self) -> impl Iterator<Item = (&SerializableVector2i, &TileInfo)> {
        self.tiles.iter()
    }

    /// Merges another chunk into this one, overwriting overlapping positions.
    pub fn merge(&mut self, other: MapDataChunk) {
        self.tiles.extend(other.tiles);
    }

    /// Returns a reference to a tile at the given position, if it exists.
    pub fn get(&self, pos: &SerializableVector2i) -> Option<&TileInfo> {
        self.tiles.get(pos)
    }

    /// Places a tile at the given (x, y) position using default metadata.
    /// Used for benchmarking or placeholder logic.
    pub fn place_tile(&mut self, _x: u32, _y: u32, _info: TileInfo) {
        // no-op for benchmarking
    }
}
