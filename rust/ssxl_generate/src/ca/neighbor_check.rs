// ssxl_generate/src/neighbor_check.rs

use ssxl_shared::{
    chunk_data::{ChunkData, CHUNK_SIZE},
    tile_type::TileType,
};

/// Counts the number of "live" (TileType::Rock) neighbors in the 3x3 Moore neighborhood
/// surrounding a specific cell within a chunk.
///
/// This function handles boundary conditions, ensuring checks do not exceed the
/// chunk's borders (0 to CHUNK_SIZE - 1). It is crucial for the tempo and accuracy
/// of the Cellular Automata simulation.
///
/// # Arguments
/// * `chunk_data`: The data structure containing the chunk's tiles.
/// * `cx`: The X coordinate of the center cell (0 to CHUNK_SIZE - 1).
/// * `cy`: The Y coordinate of the center cell (0 to CHUNK_SIZE - 1).
///
/// # Returns
/// The count of live neighbors (0-8).
pub fn count_live_neighbors(chunk_data: &ChunkData, cx: u32, cy: u32) -> u8 {
    let mut count = 0;

    // Iterate through the 3x3 Moore neighborhood relative to the center cell (cx, cy).
    for dx in -1..=1 {
        for dy in -1..=1 {
            // 1. Exclude the center cell itself (self-exclusion).
            if dx == 0 && dy == 0 {
                continue;
            }

            // Calculate the neighbor's absolute coordinates.
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;

            // 2. Perform boundary checks. Ensure the neighbor is within the chunk's bounds.
            if nx >= 0 && nx < CHUNK_SIZE as i32 && ny >= 0 && ny < CHUNK_SIZE as i32 {
                // Convert valid (nx, ny) coordinates back to the 1D array index.
                let index = (ny as u32 * CHUNK_SIZE + nx as u32) as usize;

                // 3. Check for the "live" state, defined as TileType::Rock.
                if chunk_data.tiles[index].tile_type == TileType::Rock {
                    count += 1;
                }
            }
        }
    }
    count
}