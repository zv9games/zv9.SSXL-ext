// ssxl_generate/src/neighbor_check.rs

use ssxl_shared::{
    chunk_data::{ChunkData, CHUNK_SIZE},
    tile_type::TileType,
};

/// Counts the number of 'live' (TileType::Rock) neighbors for a given coordinate (Moore neighborhood).
pub fn count_live_neighbors(chunk_data: &ChunkData, cx: u32, cy: u32) -> u8 {
    let mut count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;

            // Check if neighbor is within chunk bounds
            if nx >= 0 && nx < CHUNK_SIZE as i32 && ny >= 0 && ny < CHUNK_SIZE as i32 {
                let index = (ny as u32 * CHUNK_SIZE + nx as u32) as usize;

                if chunk_data.tiles[index].tile_type == TileType::Rock {
                    count += 1;
                }
            }
        }
    }
    count
}