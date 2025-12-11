// rust/SSXL-ext/src/cache.rs

use dashmap::DashMap; // High-performance concurrent hash map
use crate::shared_chunk::Chunk;
use crate::shared_math::ChunkCoords; 

// --- FIX: Import logging macros from the crate root ---
use crate::{ssxl_info, ssxl_warn}; 

// --------------------------------------------------------------------------
// --- Chunk Cache (Final Results) ---
// --------------------------------------------------------------------------

/// Stores the final, processed Chunk results (post-CA, post-refinement).
/// This is used by the Conductor to check if a chunk needs generation 
/// or is already complete.
pub struct ChunkCache {
    // Key: (ChunkX, ChunkY)
    // Value: The final Chunk struct
    cache: DashMap<ChunkCoords, Chunk>,
}

impl ChunkCache {
    pub fn new() -> Self {
        ChunkCache {
            // Initialize with a default capacity
            cache: DashMap::with_capacity(4096), 
        }
    }

    /// Attempts to retrieve a chunk from the cache.
    /// Returns None if the chunk has not been generated yet.
    pub fn get(&self, coords: ChunkCoords) -> Option<Chunk> {
        // Clone the value out of the DashMap entry
        self.cache.get(&coords).map(|entry| entry.value().clone())
    }

    /// Inserts a newly generated chunk into the cache.
    /// This is called by a worker thread right before sending the chunk to the Conductor.
    pub fn insert(&self, chunk: Chunk) {
        let coords = chunk.position;
        self.cache.insert(coords, chunk);
        ssxl_info!("Chunk Cache: Stored chunk {:?}", coords);
    }
    
    /// Checks if the cache contains the chunk at the given coordinates.
    pub fn contains(&self, coords: ChunkCoords) -> bool {
        self.cache.contains_key(&coords)
    }
    
    /// Clears all entries from the cache (e.g., on world reset).
    pub fn clear(&self) {
        self.cache.clear();
        ssxl_warn!("Chunk Cache: Cleared all stored chunks.");
    }
}

// --------------------------------------------------------------------------
// --- Noise Cache (Intermediate Data) ---
// --------------------------------------------------------------------------

/// Stores raw Perlin noise output for a chunk. 
/// Useful for neighbor queries in CA simulations without recalculating Perlin noise.
pub struct NoiseCache {
    // Key: (ChunkX, ChunkY)
    // Value: A simple vector of raw f64 noise values
    cache: DashMap<ChunkCoords, Vec<f64>>, 
}

impl NoiseCache {
    pub fn new() -> Self {
        NoiseCache {
            cache: DashMap::with_capacity(2048), // Smaller capacity than ChunkCache
        }
    }

    /// Retrieves raw noise values. Used by generate_ca.rs to peek at neighbors' initial state.
    pub fn get_noise_data(&self, coords: ChunkCoords) -> Option<Vec<f64>> {
        // Clone the noise data vector
        self.cache.get(&coords).map(|entry| entry.value().clone())
    }

    /// Stores the raw noise values immediately after generation in generate_perlin.rs.
    pub fn insert_noise_data(&self, coords: ChunkCoords, noise_data: Vec<f64>) {
        self.cache.insert(coords, noise_data);
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.cache.clear();
        ssxl_warn!("Noise Cache: Cleared all stored noise data.");
    }
}