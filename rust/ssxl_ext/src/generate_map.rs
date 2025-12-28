use crate::shared_chunk::Chunk;
use crate::shared_tile::TileData;

pub fn apply_tile_mapping(mut chunk: Chunk) -> Chunk {
    let size = chunk.size as usize;

    // chunk.position is a tuple: (i32, i32)
    let cx = chunk.position.0;
    let cy = chunk.position.1;

    for y in 0..size {
        for x in 0..size {
            let idx = y * size + x;

            // Convert local tile coords → global tile coords
            let gx = cx * size as i32 + x as i32;
            let gy = cy * size as i32 + y as i32;

            // Global checkerboard pattern
            let is_black = (gx + gy) % 2 == 0;

            let mut tile = TileData::default();
            tile.tile_id = 1;

            // Correct atlas indices for (14,13) and (15,13)
            tile.atlas_coords = if is_black { 222 } else { 223 };

            chunk.tiles[idx] = tile;
        }
    }

    chunk
}
