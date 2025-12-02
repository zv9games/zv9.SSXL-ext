// ssxl_generate/src/ca/neighbor_check.rs

// FIX: Import ChunkData, CHUNK_SIZE, and TileType directly from the ssxl_shared root, 
// as the inner modules (`chunk_data` and `tile_type`) are no longer public.
use ssxl_shared::{ChunkData, CHUNK_SIZE, TileType};

/// Counts the number of "live" (TileType::Rock) neighbors in the 3x3 Moore neighborhood
/// surrounding a specific cell within a chunk.
///
/// This function handles boundary conditions, ensuring checks do not exceed the
/// chunk's borders (0 to CHUNK_SIZE - 1). It is crucial for the tempo and accuracy
/// of the Cellular Automata simulation.
///
/// **Safety Focus:** Explicitly checks signed coordinates (`i32`) against bounds before
/// casting to unsigned types (`u32`) for indexing, preventing potential wraparound bugs.
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
    // Cache the chunk size as i32 for direct comparison against signed coordinates.
    const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

    // Convert center coordinates to i32 for safe arithmetic with dx/dy.
    let center_x_i32 = cx as i32;
    let center_y_i32 = cy as i32;

    // Iterate through the 3x3 Moore neighborhood relative to the center cell (cx, cy).
    for dx in -1..=1 {
        for dy in -1..=1 {
            // 1. Exclude the center cell itself (self-exclusion).
            if dx == 0 && dy == 0 {
                continue;
            }

            // Calculate the neighbor's absolute coordinates using i32.
            let nx = center_x_i32 + dx;
            let ny = center_y_i32 + dy;

            // 2. CRITICAL BOUNDARY CHECK: Ensure the neighbor is within the chunk's bounds [0, CHUNK_SIZE_I32 - 1].
            if nx >= 0 && nx < CHUNK_SIZE_I32 && ny >= 0 && ny < CHUNK_SIZE_I32 {
                
                // Now that bounds are guaranteed, safely cast back to u32 for indexing.
                let neighbor_x = nx as u32;
                let neighbor_y = ny as u32;
                
                // Use a helper function for safer access if available, otherwise calculate index.
                // Assuming ChunkData provides a tiles array indexed [y * size + x].
                let index = (neighbor_y * CHUNK_SIZE + neighbor_x) as usize;

                // 3. Check for the "live" state, defined as TileType::Rock.
                // NOTE: This assumes `chunk_data.tiles` is publicly accessible.
                if chunk_data.tiles[index].tile_type == TileType::Rock {
                    count += 1;
                }
            }
        }
    }
    count
}