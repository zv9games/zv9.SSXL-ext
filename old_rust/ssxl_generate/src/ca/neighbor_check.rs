// ============================================================================
// 🧩 Cellular Automata Utility: Neighbor Check (`count_live_neighbors`)
// ----------------------------------------------------------------------------
// This module provides a helper function for the Cellular Automata (CA)
// subsystem of the SSXL engine. It inspects the local neighborhood of a tile
// within a chunk and counts how many surrounding tiles are "live" (defined as
// `TileType::Rock`). This information is critical for applying CA rulesets
// that evolve terrain patterns.
//
// Purpose:
//   • Support CA simulation by providing local neighbor counts.
//   • Implement a 3x3 Moore neighborhood (8 surrounding cells).
//   • Handle boundary conditions safely to avoid out-of-bounds indexing.
//
// Function: count_live_neighbors
//   • Arguments:
//       - `chunk_data`: reference to the chunk’s tile data.
//       - `cx`: X coordinate of the center cell (0..CHUNK_SIZE-1).
//       - `cy`: Y coordinate of the center cell (0..CHUNK_SIZE-1).
//   • Returns:
//       - `u8` count of live neighbors (0–8).
//
// Workflow:
//   1. Convert center coordinates to signed integers for arithmetic.
//   2. Iterate over relative offsets (dx, dy) in the range -1..=1.
//   3. Skip the center cell itself.
//   4. Compute neighbor coordinates and check boundaries.
//   5. If inside bounds, compute linear index into the tile array.
//   6. Increment count if neighbor tile is `TileType::Rock`.
//   7. Return total count.
//
// Design Choices:
//   • Uses signed arithmetic (`i32`) for safe offset calculations.
//   • Boundary checks prevent invalid indexing at chunk edges.
//   • Linear indexing formula (`y * CHUNK_SIZE + x`) ensures efficient lookup.
//   • Returns `u8` since maximum neighbor count is 8.
//
// Educational Note:
//   • This function demonstrates how to implement neighborhood checks in
//     Cellular Automata systems.
//   • By separating neighbor counting from rule application, the design remains
//     modular and extensible (e.g., supporting different neighborhood types).
// ============================================================================


use ssxl_shared::{ChunkData, CHUNK_SIZE, TileType};

pub fn count_live_neighbors(chunk_data: &ChunkData, cx: u32, cy: u32) -> u8 {
    let mut count = 0;

    const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

    let center_x_i32 = cx as i32;
    let center_y_i32 = cy as i32;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = center_x_i32 + dx;
            let ny = center_y_i32 + dy;

            if nx >= 0 && nx < CHUNK_SIZE_I32 && ny >= 0 && ny < CHUNK_SIZE_I32 {
                let neighbor_x = nx as u32;
                let neighbor_y = ny as u32;

                let index = (neighbor_y * CHUNK_SIZE + neighbor_x) as usize;

                if chunk_data.tiles[index].tile_type == TileType::Rock {
                    count += 1;
                }
            }
        }
    }

    count
}
