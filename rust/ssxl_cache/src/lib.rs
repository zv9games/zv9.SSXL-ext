// ssxl_cache/src/lib.rs
use ssxl_math::coordinate_system::ChunkKey;
use ssxl_shared::ChunkData;
use ssxl_sync::AtomicResource;

use std::collections::{HashMap, HashSet};
use std::sync::{
    Arc,
    // HIGH: Added Atomic for O(1) non-blocking metric updates
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

// HIGH: O(1) Metric collection
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
        // The lock on `self.storage` is automatically dropped.
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
    pub metrics: Arc<CacheMetrics>, // HIGH: Added metrics tracker
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
            metrics: Arc::new(CacheMetrics::default()), // HIGH: Initialize metrics
        })
    }

    pub fn load_chunk(&self, key: &ChunkKey) -> Option<Arc<ChunkData>> {
        let mut guard = self.storage.lock();
        let result = guard.get(key).map(Arc::clone);
        
        // HIGH: Track hit/miss
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

        // RegionIndex insertion is O(1) and idempotent (HashSet). Always update.
        self.region_index.insert_key(key);

        // Insert the new chunk. `put` returns `Option<V>` (the old value) if the key existed.
        let _old_data = guard.put(key, data);
        
        // FIX E0308: Use `pop_lru()` to get the evicted key/value pair when the cache capacity is exceeded.
        // `pop_lru()` correctly returns `Option<(K, V)>`.
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