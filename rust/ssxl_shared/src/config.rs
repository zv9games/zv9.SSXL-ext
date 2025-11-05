// ssxl_shared/src/config.rs

/// The canonical size for all chunks in the SSXL Engine (32x32 tiles).
pub const CHUNK_SIZE: u32 = 32;

/// The total number of tiles in a single chunk (CHUNK_SIZE * CHUNK_SIZE = 1024).
pub const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;