// ssxl_godot/src/chunk_presenter.rs (Final Cleaned and Optimized)
//! # ChunkPresenter
//!
//! This component is responsible for translating multi-threaded ChunkData messages into
//! a single, efficient Godot method call on the SSXLTileMap, executing the actual rendering
//! logic on the main thread via a deferred Callable.
// --- Godot GDExtension Imports (godot-rust) 🧩 ---
use godot::prelude::*;
use godot::classes::Node;
// Use our custom class for typed batch call
use crate::ssxl_tilemap::SSXLTileMap;
use godot::builtin::{Variant, Vector2i, Callable, Array, Dictionary};
use godot::obj::{Gd, Base};
use godot::meta::{
    GodotConvert, ToGodot, FromGodot, ByValue // CLEANUP: Removed unused ArgPassing, AsArg
};
use godot::meta::error::ConvertError;
// --- SSXL-ext Shared Crates Imports ---
use ssxl_shared::{ChunkData, CHUNK_SIZE};
use ssxl_shared::messages::ChunkMessage;
use ssxl_shared::tile_type::TileType;
use serde_json;
// --- Standard Library Imports ---
use std::io::Error as GodotError;
use tracing::{error, info};

// -----------------------------------------------------------------------------
// Godot Conversion Wrapper for ChunkMessage
// -----------------------------------------------------------------------------

/// Wrapper struct required to implement `GodotConvert` for `ChunkMessage`.
///
/// FIX & OPTIMIZATION: Uses fast binary serialization (`serde` bytes) into a
/// Godot `PackedByteArray` (via `Variant`) for safe, high-performance,
/// multi-threaded transfer across the FFI boundary.
struct ChunkMessageWrapper(ChunkMessage);

// Declares that the struct should be converted using `Variant` as the intermediate type.
impl GodotConvert for ChunkMessageWrapper {
    type Via = Variant;
}

// Implements conversion from the Rust struct into a PackedByteArray Variant.
impl ToGodot for ChunkMessageWrapper {
    // DEFINITIVE FIX (E0107): ByValue does not take a generic argument.
    // This allows the owned Variant to be transferred across the FFI boundary safely.
    type Pass = ByValue; 

    fn to_godot(&self) -> Variant { // The return type must match Self::Via (Variant)
        // OPTIMIZATION: Serialize ChunkMessage to a binary Vec<u8>.
        let bytes = match serde_json::to_vec(&self.0) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to serialize ChunkMessage to binary: {:?}", e);
                // Return the owned nil Variant directly.
                return godot::prelude::Variant::nil();
            }
        };

        // LIFETIME FIX (E0515): Wrap the binary data in a native Godot PackedByteArray
        // and return the owned Variant derived from it.
        let packed_array = godot::builtin::PackedByteArray::from(bytes.as_slice());
        
        packed_array.to_variant() // Returns the owned Variant, safely transferring ownership.
    }
}

// Implements conversion from a Godot Variant (containing PackedByteArray) back into the Rust struct.
impl FromGodot for ChunkMessageWrapper {
    fn try_from_godot(value: Self::Via) -> Result<Self, ConvertError> {
        // 1. Extract the PackedByteArray from the Variant.
        let packed_array = value.try_to::<godot::builtin::PackedByteArray>()
             .map_err(|e| ConvertError::new(format!("Variant conversion failed while extracting PackedByteArray: {}", e)))?;
        
        // 2. Convert the PackedByteArray to a Rust Vec<u8> slice.
        let bytes = packed_array.to_vec();
        
        // 3. Deserialize the binary data back into the ChunkMessage struct.
        let msg: ChunkMessage = serde_json::from_slice(&bytes)
             .map_err(|e| ConvertError::new(format!("Failed to deserialize ChunkMessage from binary: {}", e)))?;
             
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
    // Use the specific typed Godot class SSXLTileMap
    tilemap_node: Option<Gd<SSXLTileMap>>, // Optional reference to the target TileMap.
}

// -----------------------------------------------------------------------------
// Safe Constructor for Internal Rust Use
// -----------------------------------------------------------------------------

impl ChunkPresenter {
    /// Helper constructor for creating a new, inert instance outside of the Godot engine's flow.
    /// This is used by the orchestrator (`SSXLEngine`) to hold state before the `_ready` hook.
    #[allow(invalid_value)] // Suppresses warning about zero-initialization of Base<Node>
    pub fn new_internal() -> Self {
        // NOTE: This unsafe zero-initialization is currently required when a GodotClass-derived struct
        // is used as a plain Rust component outside of the engine's initialization flow.
        ChunkPresenter {
            base: unsafe { std::mem::zeroed() },
            tilemap_node: None,
        }
    }
}

// -----------------------------------------------------------------------------
// Exposed Godot API
// -----------------------------------------------------------------------------

#[godot_api]
impl ChunkPresenter {
    /// Godot `init` constructor.
    #[allow(dead_code)]
    pub fn init(base: Base<Node>) -> Self {
        ChunkPresenter {
            base,
            tilemap_node: None,
        }
    }

    /// Public method to set the TileMap reference.
    /// FIX: Update signature to accept the new typed class
    pub fn set_tilemap_node(&mut self, tilemap_node: Gd<SSXLTileMap>) {
        self.tilemap_node = Some(tilemap_node);
    }

    // -------------------------------------------------------------------------
    // Deferred Rendering API
    // -------------------------------------------------------------------------

    /// Creates a Godot `Callable` to safely queue the rendering work to run on the main thread.
    pub fn create_deferred_present_call(&self, msg: ChunkMessage) -> Option<Callable> {
        let presenter_node = self.base().clone();
        // Convert the ChunkMessage into a Variant argument using the custom wrapper.
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
                // Call the revised batch function.
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
    fn grunt_apply_chunk_data(tile_map: &mut Gd<SSXLTileMap>, chunk_data: ChunkData) -> Result<(), GodotError> {
        let chunk_size_i32 = CHUNK_SIZE as i32;
        
        // Calculate the starting grid coordinate (top-left) for this chunk.
        let chunk_pos_grid = Vector2i::new(
            (chunk_data.bounds.min.x / CHUNK_SIZE as i64) as i32 * chunk_size_i32,
            (chunk_data.bounds.min.y / CHUNK_SIZE as i64) as i32 * chunk_size_i32,
        );

        // --- FFI BATCH OPTIMIZATION: Collect data into Godot typed arrays ---
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

                // Push the position regardless of content, as the C++ side handles clearing/setting.
                pos_array.push(cell_pos);

                if TileType::is_empty(tile_type) {
                    // To CLEAR a cell, set source_id to -1.
                    source_id_array.push(-1);
                    atlas_array.push(Vector2i::new(-1, -1));
                    alt_tile_array.push(0); // Default alternative tile ID
                } else {
                    // Place a tile.
                    let (_tile_id_render, (atlas_x, atlas_y)) =
                        (TileType::get_default_tile_id(tile_type), TileType::get_default_atlas_coords(tile_type));

                    let atlas_coords = Vector2i::new(atlas_x as i32, atlas_y as i32);

                    source_id_array.push(ATLAS_SOURCE_ID);
                    atlas_array.push(atlas_coords);
                    alt_tile_array.push(0); // Default alternative tile ID
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
        tile_map.bind_mut().batch_set_tiles_v4(batch_data);

        Ok(())
    }
}