// ============================================================================
// 🗄️ Chunk Cache System (`ssxl_cache`)
// ----------------------------------------------------------------------------
// This module implements a thread-safe, region-aware cache for chunk data in
// the SSXL engine. It combines an LRU (Least Recently Used) eviction policy
// with region indexing to balance performance, memory usage, and spatial
// organization.
//
// Key Concepts:
//   • ChunkCache:
//       - Stores chunks in an LRU cache, automatically evicting the least
//         recently used entries when capacity is exceeded.
//       - Provides methods to load, save, and remove chunks safely.
//       - Tracks cache metrics (hits, misses, evictions) for performance tuning.
//   • RegionIndex:
//       - Maps regions (groups of chunks) to sets of chunk keys.
//       - Regions are defined by dividing world coordinates by REGION_SIZE.
//       - Supports insertion and removal of chunk keys, automatically cleaning
//         up empty regions.
//   • CacheMetrics:
//       - Atomic counters for hits, misses, and evictions.
//       - Provides lightweight tracking of cache performance without locking.
//   • REGION_SIZE:
//       - Defines the granularity of regions (here, 64 units).
//       - Used to group chunks spatially for efficient lookup.
//
// Workflow:
//   1. Initialization (`ChunkCache::new`):
//      - Creates an LRU cache with a non-zero capacity.
//      - Initializes region index and metrics tracker.
//   2. Loading (`load_chunk`):
//      - Attempts to retrieve a chunk by key.
//      - Increments hit/miss counters accordingly.
//   3. Saving (`save_chunk`):
//      - Inserts a chunk into the cache and updates the region index.
//      - If capacity is exceeded, evicts the least recently used chunk.
//      - Tracks eviction metrics and cleans up region index.
//   4. Removal (`remove_chunk`):
//      - Manually removes a chunk and updates the region index.
//   5. Inspection (`len`, `capacity`):
//      - Provides current cache size and maximum capacity.
//
// Design Choices:
//   • `parking_lot::Mutex` and `RwLock` provide high-performance locking
//     primitives for concurrent access.
//   • `Arc` ensures safe shared ownership of chunks and metrics across threads.
//   • `AtomicResource` wraps the region index for ergonomic thread-safe access.
//   • `lru::LruCache` provides a proven eviction strategy for bounded memory.
//
// Educational Note:
//   • This module demonstrates a layered caching strategy:
//       - LRU eviction ensures memory bounds.
//       - Region indexing provides spatial organization.
//       - Metrics tracking enables runtime tuning.
//   • By combining these techniques, the engine achieves efficient chunk
//     management in large, procedurally generated worlds.
// ============================================================================


use ssxl_math::coordinate_system::ChunkKey;
use ssxl_shared::ChunkData;
use ssxl_sync::AtomicResource;

use std::collections::{HashMap, HashSet};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering}
};
use std::io;
use std::num::NonZeroUsize;

use parking_lot::{Mutex, RwLock};
use tracing::info;
use glam::I64Vec3;
use lru::LruCache;

const REGION_SIZE: i64 = 64;
type RegionKey = ChunkKey;
type RegionList = RwLock<HashSet<ChunkKey>>;

#[derive(Debug, Default)]
pub struct CacheMetrics {
    pub hits: AtomicUsize,
    pub misses: AtomicUsize,
    pub evictions: AtomicUsize,
}

impl CacheMetrics {
    #[inline(always)]
    pub fn hit(&self) { self.hits.fetch_add(1, Ordering::Relaxed); }
    #[inline(always)]
    pub fn miss(&self) { self.misses.fetch_add(1, Ordering::Relaxed); }
    #[inline(always)]
    pub fn evict(&self) { self.evictions.fetch_add(1, Ordering::Relaxed); }
}

#[derive(Debug, Clone)]
pub struct RegionIndex {
    storage: AtomicResource<HashMap<RegionKey, Arc<RegionList>>>,
}

impl RegionIndex {
    pub fn new() -> Self {
        Self {
            storage: AtomicResource::new(HashMap::new()),
        }
    }

    #[inline(always)]
    fn chunk_to_region_key(chunk_key: ChunkKey) -> RegionKey {
        let p = chunk_key.0;
        let rx = p.x / REGION_SIZE;
        let ry = p.y / REGION_SIZE;
        let rz = p.z / REGION_SIZE;
        ChunkKey(I64Vec3::new(rx, ry, rz))
    }

    pub fn insert_key(&self, chunk_key: ChunkKey) {
        let region_key = Self::chunk_to_region_key(chunk_key);
        let list_arc = {
            let mut map = self.storage.write();
            map.entry(region_key)
                .or_insert_with(|| Arc::new(RwLock::new(HashSet::new())))
                .clone()
        };
        let mut list = list_arc.write();
        list.insert(chunk_key);
    }

    pub fn remove_key(&self, chunk_key: ChunkKey) -> bool {
        let region_key = Self::chunk_to_region_key(chunk_key);
        let list_arc = self.storage.read().get(&region_key).cloned();

        if let Some(list_arc) = list_arc {
            let mut list = list_arc.write();
            if list.remove(&chunk_key) {
                if list.is_empty() {
                    drop(list);
                    self.storage.write().remove(&region_key);
                    info!("Removed empty region from index: {:?}", region_key);
                }
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct ChunkCache {
    storage: Mutex<LruCache<ChunkKey, Arc<ChunkData>>>,
    region_index: RegionIndex,
    capacity: NonZeroUsize,
    pub metrics: Arc<CacheMetrics>,
}

impl ChunkCache {
    pub fn new(max_chunks: usize) -> io::Result<Self> {
        let capacity = NonZeroUsize::new(max_chunks.max(1))
            .unwrap_or(NonZeroUsize::new(1024).unwrap());
        
        info!("ChunkCache initialized with LRU eviction (capacity: {})", capacity);

        Ok(Self {
            storage: Mutex::new(LruCache::new(capacity)),
            region_index: RegionIndex::new(),
            capacity,
            metrics: Arc::new(CacheMetrics::default()),
        })
    }

    pub fn load_chunk(&self, key: &ChunkKey) -> Option<Arc<ChunkData>> {
        let mut guard = self.storage.lock();
        let result = guard.get(key).map(Arc::clone);
        
        if result.is_some() {
            self.metrics.hit();
        } else {
            self.metrics.miss();
        }

        result
    }

    pub fn save_chunk(&self, key: &ChunkKey, data: Arc<ChunkData>) -> io::Result<()> {
        let key = *key;
        let mut guard = self.storage.lock();

        self.region_index.insert_key(key);

        let _old_data = guard.put(key, data);
        
        if let Some((evicted_key, _)) = guard.pop_lru() {
            self.metrics.evict();
            self.region_index.remove_key(evicted_key);
            info!(
                "LRU evicted chunk: {:?} (cache size: {})",
                evicted_key,
                guard.len()
            );
        }

        Ok(())
    }

    pub fn remove_chunk(&self, key: &ChunkKey) -> Option<Arc<ChunkData>> {
        let removed = self.storage.lock().pop(key);
        if removed.is_some() {
            self.region_index.remove_key(*key);
            info!("Manually removed chunk from cache: {:?}", key);
        }
        removed
    }

    pub fn len(&self) -> usize {
        self.storage.lock().len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity.get()
    }
}
