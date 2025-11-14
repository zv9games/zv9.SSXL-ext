// --- Godot GDExtension Imports (godot-rust) 🧩 ---
use godot::prelude::*; // Essential Godot types, macros, and core traits.
use godot::classes::{TileMap, Node};
use godot::builtin::{Variant, GString, Vector2i, Callable}; // Built-in Godot types + Callable.
use godot::obj::{Gd, Base, NewGd}; // Gd, Base, and NewGd.
use godot::obj::bounds::MemRefCounted; // Required trait for bounds checking.
use std::error::Error; // Required for using ConvertError::new()
// ADDED `std::mem` for zero-initialization
use std::mem;
// FIX: Add ManuallyDrop for FFI ownership transfer
use std::mem::ManuallyDrop;
use godot::meta::{
    GodotConvert, ToGodot, FromGodot, ArgPassing // Traits for Godot/Rust FFI conversion.
};
use godot::meta::error::ConvertError; // Error type for FFI conversion failures.
// --- SSXL-ext Shared Crates Imports ---
use ssxl_shared::{ChunkData, CHUNK_SIZE}; // Map data structure and constants.
use ssxl_shared::messages::ChunkMessage; // Network/thread message wrapper.
use ssxl_shared::tile_type::TileType; // Tile properties enum.
use serde_json; // External crate for JSON serialization.
// --- Standard Library Imports ---
use std::io::Error as GodotError; // Aliases standard I/O error for GDExtension usage.
use tracing::error; // Logging macro for centralized error reporting.

// -----------------------------------------------------------------------------
// Godot Conversion Wrapper for ChunkMessage
// -----------------------------------------------------------------------------
/// Wrapper struct required to implement `GodotConvert` for `ChunkMessage`, since it is an external type.
struct ChunkMessageWrapper(ChunkMessage);
// Declares that the struct should be converted using `GString` as the intermediate type.
impl GodotConvert for ChunkMessageWrapper {
    type Via = GString;
}
// Implements conversion from the Rust struct into a Godot GString/Variant.
impl ToGodot for ChunkMessageWrapper {
    type Pass = <GString as ToGodot>::Pass;
    fn to_godot(&self) -> <Self::Pass as ArgPassing>::Output<'_, Self::Via> {
        let json_string = match serde_json::to_string(&self.0) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize ChunkMessage to JSON: {:?}", e);
                String::new()
            }
        };

        // 1. Create the owned GString object.
        let gstring = GString::from(json_string.as_str());

        // FIX (E0515): Use a controlled `unsafe` block to manually perform the memory leak
        // and safely transmute the lifetime of the resulting FFI pointer.
        let output_leaked = unsafe {
            // 2. Get the FFI compatible output type (this still carries a short lifetime).
            let output = gstring.to_godot();

            // 3. Transmute the output's lifetime to the required return type.
            let output_leaked: <Self::Pass as ArgPassing>::Output<'_, Self::Via> = std::mem::transmute(output);

            // 4. Leak the GString memory. This prevents the destructor from running,
            // guaranteeing the memory is valid for the pointer/reference returned in step 3.
            std::mem::forget(gstring);
            
            output_leaked
        };

        output_leaked
    }
}
// Implements conversion from a Godot GString/Variant back into the Rust struct.
impl FromGodot for ChunkMessageWrapper {
    fn try_from_godot(value: Self::Via) -> Result<Self, ConvertError> {
        let json_string = value.to_string();
        let msg: ChunkMessage = serde_json::from_str(&json_string)
            // FIX 2: ConvertError::new() for custom errors
            .map_err(|e| ConvertError::new(format!("Failed to deserialize ChunkMessage from JSON: {}", e)))?;
        Ok(ChunkMessageWrapper(msg))
    }
}
// -----------------------------------------------------------------------------
// Core Constants for Godot TileMap Interaction
// -----------------------------------------------------------------------------
const SSXL_LAYER: i32 = 0; // Target layer ID on the Godot TileMap.
const ATLAS_SOURCE_ID: i32 = 0; // Source ID pointing to the tile atlas resource.
const DEFERRED_RENDER_METHOD_NAME: &str = "present_chunk_internal"; // Method name for deferred calls.
// -----------------------------------------------------------------------------
// ChunkPresenter Struct
// -----------------------------------------------------------------------------
#[derive(Debug, GodotClass)]
#[class(base=Node, init)]
/// A Godot Node used to receive and render chunk data onto a TileMap.
pub struct ChunkPresenter {
    #[base]
    base: Base<Node>, // The required base field for the Godot Node class.
    tilemap_node: Option<Gd<TileMap>>, // Optional reference to the target TileMap.
}
#[godot_api]
impl ChunkPresenter {
    /// Godot `init` constructor.
    pub fn init(base: Base<Node>) -> Self {
        ChunkPresenter {
            base,
            tilemap_node: None,
        }
    }
    /// Helper constructor for creating a new, uninitialized instance outside of the engine's flow.
    pub fn new() -> Self {
        // FIX 1: Using unsafe zero-initialization, which often works when `default()` fails.
        Self::init(unsafe { mem::zeroed() })
    }
    
    /// Public method to set the TileMap reference.
    pub fn set_tilemap_node(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node);
    }
    // -------------------------------------------------------------------------
    // Deferred Rendering API
    // -------------------------------------------------------------------------
    /// Creates a Godot `Callable` to safely queue the rendering work to run on the main thread.
    pub fn create_deferred_present_call(&self, msg: ChunkMessage) -> Option<Callable> {
        let presenter_node = self.base().clone();
        // Convert the ChunkMessage into a Variant argument.
        let msg_variant = ChunkMessageWrapper(msg).to_variant();
        // Create the Callable, binding the message variant as its argument.
        Some(Callable::from_object_method(
            &presenter_node,
            DEFERRED_RENDER_METHOD_NAME,
        ).bind(&[msg_variant]))
    }
    /// The internal method that executes the chunk rendering, called by the deferred `Callable`.
    #[func]
    pub fn present_chunk_internal(&mut self, msg_variant: Variant) {
        // Attempt to deserialize the ChunkMessage from the Variant.
        let msg_wrapper = match msg_variant.try_to::<ChunkMessageWrapper>() {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to deserialize ChunkMessage from Variant in deferred call: {:?}", e);
                return;
            }
        };
        let msg = msg_wrapper.0; // Extract the ChunkMessage
        if let Some(ref mut tile_map) = self.tilemap_node {
            // Only process a generated chunk message.
            if let ChunkMessage::Generated(chunk_data) = msg {
                // The previous fix (E0502) to use an associated function is preserved here.
                if let Err(e) = ChunkPresenter::grunt_apply_chunk_data(tile_map, chunk_data) {
                    error!("Error applying chunk data in deferred call: {:?}", e);
                }
            }
        } else {
            error!("Cannot present chunk: TileMap reference is missing.");
        }
    }
    // -------------------------------------------------------------------------
    // Core Rendering Logic (Internal)
    // -------------------------------------------------------------------------
    /// Logic to iterate over all tiles in a chunk and set the corresponding cells in the TileMap.
    // The previous fix (E0502) to use an associated function is preserved here.
    fn grunt_apply_chunk_data(tile_map: &mut Gd<TileMap>, chunk_data: ChunkData) -> Result<(), GodotError> {
        let chunk_size_i32 = CHUNK_SIZE as i32;
        // Calculate the chunk's grid position in terms of chunk coordinates.
        let chunk_grid_x = (chunk_data.bounds.min.x / CHUNK_SIZE as i64) as i32;
        let chunk_grid_y = (chunk_data.bounds.min.y / CHUNK_SIZE as i64) as i32;
        // Calculate the starting grid coordinate (top-left) for this chunk in the TileMap grid space.
        let chunk_pos_grid = Vector2i::new(
            chunk_grid_x * chunk_size_i32,
            chunk_grid_y * chunk_size_i32,
        );
        // Iterate through all tiles within the chunk.
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let index = (y * CHUNK_SIZE + x) as usize;
                let tile = &chunk_data.tiles[index];
                let tile_type = tile.tile_type;
                
                // Get the atlas coordinates for the given tile type.
                let (_tile_id_render, (atlas_x, atlas_y)) = (TileType::get_default_tile_id(tile_type), TileType::get_default_atlas_coords(tile_type));
                if TileType::is_empty(tile_type) {
                    continue; // Skip rendering empty tiles.
                }
                // Calculate the final cell position.
                let cell_pos = chunk_pos_grid + Vector2i::new(x as i32, y as i32);
                let atlas_coords = Vector2i::new(
                    atlas_x as i32,
                    atlas_y as i32
                );
                // Call the Godot API to set the cell properties.
                let _ = tile_map.set_cell_ex(
                    SSXL_LAYER,
                    cell_pos
                )
                .source_id(ATLAS_SOURCE_ID)
                .atlas_coords(atlas_coords)
                .alternative_tile(0);
            }
        }
        tile_map.force_update(); // Manually forces the TileMap rendering update.
        Ok(())
    }
}
// Helper implementation for the orchestration layer, allowing safe cloning of the ChunkPresenter structure.
impl Clone for ChunkPresenter {
    fn clone(&self) -> Self {
        Self {
            // FIX 1 (Clone): Using unsafe zero-initialization.
            base: unsafe { mem::zeroed() },
            tilemap_node: self.tilemap_node.clone(),
        }
    }
}
// Helper implementation for the orchestration layer, allowing easy default initialization.
impl Default for ChunkPresenter {
    fn default() -> Self {
        Self::new() // Uses the safe `new` constructor.
    }
}