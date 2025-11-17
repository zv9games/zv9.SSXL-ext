use ssxl_math::coordinate_system::ChunkKey;
use ssxl_sync::AtomicResource;
use ssxl_shared::chunk_data::ChunkData;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, RwLock}; 
use tracing::{info, warn};
use glam::I64Vec3; 

type CacheMap = HashMap<ChunkKey, Arc<ChunkData>>;

const REGION_SIZE: i64 = 64;

type RegionKey = ChunkKey;
type RegionList = RwLock<Vec<ChunkKey>>;
type RegionMap = HashMap<RegionKey, Arc<RegionList>>;


#[derive(Debug, Clone)]
pub struct RegionIndex {
    storage: AtomicResource<RegionMap>,
}

impl RegionIndex {
    fn new() -> Self {
        Self {
            storage: AtomicResource::new(RegionMap::new()),
        }
    }
    
    #[inline]
    fn chunk_to_region_key(chunk_key: ChunkKey) -> RegionKey {
        let rx = chunk_key.0.x / REGION_SIZE; 
        let ry = chunk_key.0.y / REGION_SIZE; 
        let rz = chunk_key.0.z / REGION_SIZE; 

        ChunkKey(I64Vec3 { x: rx, y: ry, z: rz })
    }

    pub fn get_chunks_in_region(&self, region_key: &RegionKey) -> Option<Arc<RegionList>> {
        let map = self.storage.read();
        map.get(region_key).map(Arc::clone)
    }

    pub fn insert_key(&self, chunk_key: ChunkKey) {
        let region_key = Self::chunk_to_region_key(chunk_key);
        let mut map_lock = self.storage.write();

        let list_lock_arc = map_lock.entry(region_key)
            .or_insert_with(|| Arc::new(RwLock::new(Vec::new())))
            .clone();
        
        drop(map_lock); 

        {
            // CRITICAL FIX: The use of an explicit scope block forces the guard to drop.
            // Using `if let` here instead of `match` makes the expression end early.
            if let Ok(mut list) = list_lock_arc.write() {
                list.push(chunk_key);
            } else {
                warn!("Region list lock poisoned for {:?}: {}", region_key, "write error");
            };
        }
    }
    
    pub fn remove_key(&self, chunk_key: ChunkKey) -> bool {
        let region_key = Self::chunk_to_region_key(chunk_key);
        
        let list_lock_arc = {
            let map = self.storage.read();
            map.get(&region_key).map(Arc::clone)
        };

        if let Some(list_lock_arc) = list_lock_arc {
            
            match list_lock_arc.write() {
                Ok(mut list) => {
                    if let Some(pos) = list.iter().position(|k| *k == chunk_key) {
                        list.remove(pos);
                        
                        if list.is_empty() {
                            drop(list); 
                            let mut map_write = self.storage.write();
                            map_write.remove(&region_key);
                            info!("Removed empty region key from index: {:?}", region_key);
                        }
                        return true;
                    }
                },
                Err(e) => {
                    warn!("Region list lock poisoned during removal for {:?}: {}", region_key, e);
                }
            }
        }
        
        false
    }
}

#[derive(Debug, Clone)]
pub struct ChunkCache {
    storage: AtomicResource<CacheMap>,
    region_index: RegionIndex,
    max_capacity: usize,
}

impl ChunkCache {
    pub fn new(max_chunks: usize) -> Result<Self, io::Error> {
        info!("Initializing ChunkCache and RegionIndex with max_chunks: {}", max_chunks);
        
        Ok(ChunkCache { 
            storage: AtomicResource::new(CacheMap::new()), 
            region_index: RegionIndex::new(),
            max_capacity: max_chunks,
        })
    }

    pub fn load_chunk(&self, key: &ChunkKey) -> Result<Option<Arc<ChunkData>>, io::Error> {
        let map = self.storage.read();
        Ok(map.get(key).map(Arc::clone))
    }

    pub fn save_chunk(&self, key: &ChunkKey, data: Arc<ChunkData>) -> Result<(), io::Error> {
        let mut map = self.storage.write();

        if map.len() >= self.max_capacity {
            warn!("ChunkCache capacity limit ({}) reached. LRU eviction is required to prevent OOM.", self.max_capacity);
        }
        
        if map.insert(*key, data).is_none() {
            info!("Saved new chunk to cache: {:?}", key);
            
            self.region_index.insert_key(*key);
        } else {
            warn!("Overwrote existing chunk in cache: {:?}", key);
        }
        Ok(())
    }
}