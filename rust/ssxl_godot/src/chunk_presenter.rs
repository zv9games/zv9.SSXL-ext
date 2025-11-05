// ssxl_godot/src/chunk_presenter.rs (FIXED)

use godot::prelude::*;
use godot::classes::TileMap;

use ssxl_shared::{ChunkData, CHUNK_SIZE};
use ssxl_shared::tile_type::TileType;

use ssxl_shared::messages::ChunkMessage;
use ssxl_shared::tile_data::AnimationUpdate;

use std::io::Error as GodotError;

// Define the TileMap layer ID used by SSXL.
const SSXL_LAYER: i32 = 0;
// Define the atlas source ID for the TileSet, common for all tiles.
const ATLAS_SOURCE_ID: i32 = 0;

/// The ChunkPresenter is responsible for applying generated chunk data
/// to the Godot TileMap instance.
#[derive(Clone, Default, Debug)]
pub struct ChunkPresenter {}

impl ChunkPresenter {
    /// Creates a new presenter instance.
    pub fn new() -> Self {
        ChunkPresenter {}
    }

    // --------------------------------------------------------------------------
    // MESSAGE HANDLERS
    // --------------------------------------------------------------------------

    /// Implements the present_chunk method expected by ChannelHandler.
    pub fn present_chunk(&self, tile_map: &mut Gd<TileMap>, msg: ChunkMessage) {
        if let ChunkMessage::Generated(chunk_data) = msg {
            if let Err(e) = self.apply_chunk_data(tile_map, chunk_data) {
                godot_print!("Error applying chunk data: {:?}", e);
            }
        }
    }

    /// Implements the update_animated_tile method expected by ChannelHandler.
    pub fn update_animated_tile(&self, tile_map: &mut Gd<TileMap>, update: AnimationUpdate) {
        // Convert Vec2i tile_coords (i64) to Vector2i (i32)
        let cell_pos = Vector2i::new(
            update.tile_coords.x as i32, 
            update.tile_coords.y as i32
        );
        // Convert Vec2i new_atlas_coords (i64) to Vector2i (i32)
        let atlas_coords = Vector2i::new(
            update.new_atlas_coords.x as i32, 
            update.new_atlas_coords.y as i32
        );

        // Using the fluent set_cell_ex pattern to apply the new tile frame
        tile_map.set_cell_ex(
            SSXL_LAYER, 		// The target layer
            cell_pos 		    // FIX: Removed No-Break Space // The cell position
        )
        .source_id(ATLAS_SOURCE_ID)	
        .atlas_coords(atlas_coords)
        .alternative_tile(0);
    }
    
    // --------------------------------------------------------------------------
    // GENERATION DATA APPLICATION
    // --------------------------------------------------------------------------

    /// Applies a single, fully generated ChunkData structure to the Godot TileMap.
    pub fn apply_chunk_data(&self, tile_map: &mut Gd<TileMap>, chunk_data: ChunkData) -> Result<(), GodotError> {
        
        let chunk_size_i32 = CHUNK_SIZE as i32;
        
        // 🛑 CRITICAL FIX: Convert World Coordinate (bounds.min) back to TileMap Cell Coordinate
        // chunk_coords = world_coords / chunk_size
        let chunk_grid_x = (chunk_data.bounds.min.x / CHUNK_SIZE as i64) as i32;
        let chunk_grid_y = (chunk_data.bounds.min.y / CHUNK_SIZE as i64) as i32;

        // The starting GLOBAL cell position in the TileMap grid.
        let chunk_pos_grid = Vector2i::new(
            chunk_grid_x * chunk_size_i32,
            chunk_grid_y * chunk_size_i32,
        );

        // Step 1: Iterate over all tiles and set them on the TileMap
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let index = (y * CHUNK_SIZE + x) as usize;
                // De-reference the fixed array to get the TileData
                let tile = &chunk_data.tiles[index]; 

                let tile_type = tile.tile_type;
                
                let (_tile_id_render) = tile_type.get_default_tile_id();
                let (atlas_x, atlas_y) = tile_type.get_default_atlas_coords();

                // Skip drawing empty tiles (Void type)
                if tile_type.is_empty() {
                    continue;
                }
                
                // Calculate the final GLOBAL TileMap cell position
                let cell_pos = chunk_pos_grid + Vector2i::new(x as i32, y as i32);

                // Get the coordinates within the TileSet atlas texture
                let atlas_coords = Vector2i::new(
                    atlas_x as i32,
                    atlas_y as i32
                );

                // Use the set_cell_ex builder pattern with the correct chained methods.
                tile_map.set_cell_ex(
                    SSXL_LAYER, 		// The target layer
                    cell_pos 		    // FIX: Removed No-Break Space // The cell position
                )
                .source_id(ATLAS_SOURCE_ID) 	// Set the TileSet source (i32)
                .atlas_coords(atlas_coords) 	// Set coordinates within the TileSet atlas (Vector2i)
                .alternative_tile(0);			// Set the alternative tile index (i32, 0 for default)
            }
        }

        // ✅ FINAL FIX: Use the standard, public API for forcing a TileMap update.
        // Replaces the unstable tile_map.call("notify_runtime_tile_data_update", ...)
        // The TileMap::force_update() method in Godot's Rust bindings takes no arguments.
        tile_map.force_update();
        
        Ok(())
    }
}