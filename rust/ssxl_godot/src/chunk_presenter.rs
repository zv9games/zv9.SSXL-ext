// ssxl_godot/src/chunk_presenter.rs (Final Cleaned and Optimized)
// --- Godot GDExtension Imports (godot-rust) 🧩 ---
use godot::prelude::*;
// FIX: Replace generic TileMap with our custom class for typed batch call
use godot::classes::Node;
// ASSUMPTION: SSXLTileMap is available in a sibling module within the crate
use crate::ssxl_tilemap::SSXLTileMap;
use godot::builtin::{Variant, GString, Vector2i, Callable, Array, Dictionary};
use godot::obj::{Gd, Base};
use std::mem;
use godot::meta::{
    GodotConvert, ToGodot, FromGodot, ArgPassing
};
use godot::meta::error::ConvertError;
// --- SSXL-ext Shared Crates Imports ---
use ssxl_shared::{ChunkData, CHUNK_SIZE};
use ssxl_shared::messages::ChunkMessage;
use ssxl_shared::tile_type::TileType;
use serde_json;
// --- Standard Library Imports ---
use std::io::Error as GodotError;
use tracing::{error, info}; // Added info for logging optimization details
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
// REMOVED: CXX_BATCH_METHOD_NAME is no longer needed, using typed method call.
// -----------------------------------------------------------------------------
// ChunkPresenter Struct
// -----------------------------------------------------------------------------
#[derive(Debug, GodotClass)]
#[class(base=Node, init)]
/// A Godot Node used to receive and render chunk data onto a TileMap.
pub struct ChunkPresenter {
    #[base]
    base: Base<Node>, // The required base field for the Godot Node class.
    // FIX: Use the specific typed Godot class SSXLTileMap
    tilemap_node: Option<Gd<SSXLTileMap>>, // Optional reference to the target TileMap.
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
    /// Helper constructor for creating a new instance outside of the engine's flow (for use in internal Rust modules).
    // FIX E0599: Restored `new()` for use in `ssxl_engine.rs` / internal code.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Public method to set the TileMap reference.
    // FIX: Update signature to accept the new typed class
    pub fn set_tilemap_node(&mut self, tilemap_node: Gd<SSXLTileMap>) {
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
                // CALLING THE REVISED BATCH FUNCTION
                // NOTE: `tile_map` is now Gd<SSXLTileMap>
                if let Err(e) = ChunkPresenter::grunt_apply_chunk_data(tile_map, chunk_data) {
                    error!("Error applying chunk data in deferred call: {:?}", e);
                }
            }
        } else {
            error!("Cannot present chunk: SSXLTileMap reference is missing.");
        }
    }
    // -------------------------------------------------------------------------
    // Core Rendering Logic (Internal) - REVISED FOR SINGLE-CALL C++ BATCH
    // -------------------------------------------------------------------------
    /// Logic to iterate over all tiles in a chunk, collect data into a single batch Dictionary,
    /// and dispatch it in a single FFI call for a custom C++ TileMap function to process.
    // FIX: Update signature to use our custom SSXLTileMap
    fn grunt_apply_chunk_data(tile_map: &mut Gd<SSXLTileMap>, chunk_data: ChunkData) -> Result<(), GodotError> {
        let chunk_size_i32 = CHUNK_SIZE as i32;
        // Calculate chunk grid position.
        let chunk_grid_x = (chunk_data.bounds.min.x / CHUNK_SIZE as i64) as i32;
        let chunk_grid_y = (chunk_data.bounds.min.y / CHUNK_SIZE as i64) as i32;
        // Calculate the starting grid coordinate (top-left) for this chunk.
        let chunk_pos_grid = Vector2i::new(
            chunk_grid_x * chunk_size_i32,
            chunk_grid_y * chunk_size_i32,
        );

        // --- FFI BATCH OPTIMIZATION: Collect data into Godot typed arrays ---
        // NOTE: These `Array<T>` collections automatically convert to `PackedArray` when converted to Variant
        let mut pos_array = Array::<Vector2i>::new();
        let mut atlas_array = Array::<Vector2i>::new();
        let mut source_id_array = Array::<i32>::new();
        let mut alt_tile_array = Array::<i32>::new();

        // Iterate through all tiles within the chunk.
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let index = (y * CHUNK_SIZE + x) as usize;
                let tile_type = chunk_data.tiles[index].tile_type;
                
                // Calculate the final cell position for this tile.
                let cell_pos = chunk_pos_grid + Vector2i::new(x as i32, y as i32);
                
                // Push the position regardless of content, as we need to either place or clear the tile.
                pos_array.push(cell_pos);

                if TileType::is_empty(tile_type) {
                    // To CLEAR a cell, set source_id to -1.
                    source_id_array.push(-1);
                    atlas_array.push(Vector2i::new(-1, -1));
                    alt_tile_array.push(0);
                } else {
                    // Place a tile.
                    let (_tile_id_render, (atlas_x, atlas_y)) = 
                        (TileType::get_default_tile_id(tile_type), TileType::get_default_atlas_coords(tile_type));
                    
                    let atlas_coords = Vector2i::new(atlas_x as i32, atlas_y as i32);
                    
                    source_id_array.push(ATLAS_SOURCE_ID);
                    atlas_array.push(atlas_coords);
                    alt_tile_array.push(0); // Assuming 0 is the default alternative tile
                }
            }
        }
        
        // Single Batch FFI Call
        if pos_array.is_empty() {
            return Ok(());
        }

        // 1. Bundle all arrays and the layer ID into a single Dictionary.
        let mut batch_data = Dictionary::new();
        batch_data.set("layer", SSXL_LAYER.to_variant());
        batch_data.set("positions", pos_array.to_variant());
        batch_data.set("atlas_coords", atlas_array.to_variant());
        batch_data.set("source_ids", source_id_array.to_variant());
        batch_data.set("alt_tiles", alt_tile_array.to_variant());
        
        info!(
            "ChunkPresenter: Dispatching single-call batch update of {} tiles to SSXLTileMap (Layer {}).",
            pos_array.len(),
            SSXL_LAYER
        );

        // 2. Call the C++-backed function on the SSXLTileMap using a typed method call.
        // Since `present_chunk_internal` is executed on the main thread (via `call_deferred` Callable),
        // we call the function directly for maximum performance, bypassing string lookup overhead.
        tile_map.bind_mut().batch_set_tiles_v4(batch_data);
        
        Ok(())
    }
}
// Helper implementation for the orchestration layer, allowing safe cloning of the ChunkPresenter structure.
impl Clone for ChunkPresenter {
    fn clone(&self) -> Self {
        // FIX 1: Use attribute to suppress invalid_value warning for zero-initialized Base<Node>
        #[allow(invalid_value)]
        Self {
            // Use unsafe zero-initialization for the Base<Node> field, as it is only a temporary internal state.
            base: unsafe { mem::zeroed() },
            tilemap_node: self.tilemap_node.clone(),
        }
    }
}
// Helper implementation for the orchestration layer, allowing easy default initialization.
impl Default for ChunkPresenter {
    fn default() -> Self {
        // FIX 1: Use attribute to suppress invalid_value warning for zero-initialized Base<Node>
        #[allow(invalid_value)]
        Self {
            // Use unsafe zero-initialization for the Base<Node> field, as it is only a temporary internal state.
            base: unsafe { mem::zeroed() },
            tilemap_node: None,
        }
    }
}