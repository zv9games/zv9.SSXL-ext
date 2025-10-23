// ssxl_cache/src/lib.rs
//! In-memory caching layer for storing and retrieving procedural generation data (Chunks).

use ssxl_math::coordinate_system::ChunkKey;
use ssxl_sync::AtomicResource;
use ssxl_shared::chunk_data::ChunkData; // NEW: Use the final data structure
use std::collections::HashMap;
use std::io; // NEW: Required for Result type in new(), load_chunk, and save_chunk
use tracing::{info, warn};

// --- 1. Obsolete ChunkDataPlaceholder structure removed for Phase 6. ---

// --- 2. Cache Structure ---

// Type alias for the thread-safe core map
type CacheMap = HashMap<ChunkKey, ChunkData>; // FIX: Switched to ChunkData

/// The thread-safe, in-memory cache for generated Chunk data.
/// Uses AtomicResource to allow safe concurrent read/write access across worker threads.
#[derive(Debug, Clone)]
pub struct ChunkCache {
    storage: AtomicResource<CacheMap>,
}

impl ChunkCache {
    /// Creates a new, empty, thread-safe cache instance.
    // FIX: Changed return type to Result<Self, io::Error> to satisfy Conductor's initialization.
    pub fn new() -> Result<Self, io::Error> {
        info!("SSXL ChunkCache initialized: Ready for thread-safe storage.");
        Ok(ChunkCache {
            storage: AtomicResource::new(HashMap::new()),
        })
    }

    /// Attempts to retrieve a ChunkData by its ChunkKey (Cache Load implementation).
    // FIX: Implemented load_chunk as required by the Conductor (Fixes E0599).
    pub fn load_chunk(&self, key: &ChunkKey) -> Result<Option<ChunkData>, io::Error> {
        let map = self.storage.read();
        // Returns a clone of the ChunkData, or None. Wrapped in Ok.
        Ok(map.get(key).cloned()) 
    }

    /// Inserts a ChunkData into the cache (Cache Save implementation).
    // FIX: Implemented save_chunk as required by the Conductor (Fixes E0599).
    pub fn save_chunk(&self, key: &ChunkKey, data: &ChunkData) -> Result<(), io::Error> {
        let mut map = self.storage.write();
        
        if map.insert(*key, data.clone()).is_none() {
            info!("Saved new chunk to cache: {:?}", key);
        } else {
            warn!("Overwrote existing chunk in cache: {:?}", key);
        }
        Ok(())
    }

    /// Reports the current number of chunks stored in the cache.
    pub fn len(&self) -> usize {
        self.storage.read().len()
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.storage.write().clear();
        info!("SSXL ChunkCache cleared.");
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ---------------------------
// IMPL: Unit Tests (Removed obsolete tests)
// ---------------------------
// Note: Obsolete unit tests relying on ChunkDataPlaceholder and old methods (get/insert) 
// have been removed for project cleanup.