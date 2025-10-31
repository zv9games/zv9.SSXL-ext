use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use godot::builtin::{GString, Vector2i};
use godot::classes::TileMap;

use crate::ssxl_signals::SSXLSignals;

// Required for the asynchronous receiver
use tokio::sync::mpsc;

// Internal Crate Dependencies
use ssxl_generate::{Conductor, GeneratorConfig};
// NOTE: GenerationMessage is assumed to be updated in conductor.rs to use Arc<ChunkData>
use ssxl_generate::conductor::GenerationMessage;
use ssxl_shared::chunk_data; 
use ssxl_shared::chunk_data::ChunkData;

// Standard library and utilities
use std::sync::{Arc, Mutex};
use tracing::info;
use godot::prelude::godot_error;
use godot::prelude::godot_print;


// -------------------------------------------------------------------------------------------------
// SSXL ENGINE GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// The Godot-facing wrapper class for the SSXL Engine.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLEngine {
    /// The core Rust logic manager, safely wrapped in an Arc/Mutex for shared access.
    conductor: Option<Arc<Mutex<Conductor>>>,
    
    /// Reference to the SSXLSignals node for signal emission.
    signals_node: Option<Gd<Node>>,
    
    /// Reference to the main expansive TileMap for drawing/manipulation.
    tilemap_node: Option<Gd<TileMap>>,

    /// The receiver side of the MPSC channel for progress updates.
    /// This is stored on the main thread and polled every tick.
    generation_receiver: Option<mpsc::Receiver<GenerationMessage>>,

    #[base]
    base: Base<Node>,
}

// ------------------------------------------------------------------------------------
// Constructor
// ------------------------------------------------------------------------------------
impl SSXLEngine {
    pub fn init(base: Base<Node>) -> Self {
        Self {
            conductor: None,
            signals_node: None,
            tilemap_node: None,
            generation_receiver: None,
            base,
        }
    }
}

// ------------------------------------------------------------------------------------
// API methods remain in the #[godot_api] block.
// ------------------------------------------------------------------------------------
#[godot_api]
impl SSXLEngine {
    #[func]
    pub fn get_status(&self) -> GString {
        if self.conductor.is_some() {
            GString::from("STATUS: Conductor Ready")
        } else {
            GString::from("STATUS: Waiting for Init")
        }
    }

    /// Helper to emit the status signal.
    pub fn emit_status_updated(&mut self, status: GString) {
        self.base_mut().emit_signal("status_updated", &[status.to_variant()]);
    }
    
    #[signal]
    fn status_updated(status_message: godot::prelude::GString);

    // ------------------------- CHUNK DATA APPLICATION ----------------------------

    /// Synchronously applies the generated ChunkData to the Godot TileMap on the main thread.
    /// 🚀 **TEMPO BOOST:** This now accepts an Arc<ChunkData>, reducing data transfer time.
    /// ⚠️ **TODO: BATCHING REFACTOR:** The core loop here is the *biggest* bottleneck.
    /// It must be replaced with a single TileMap batch operation (e.g., `tilemap.set_cells_i`)
    /// which will be 5-10x faster than this per-tile loop.
    fn apply_chunk_data(&mut self, chunk_data_arc: Arc<ChunkData>) {
        // Take ownership of the tilemap for mutation during the operation
        if let Some(mut tilemap) = self.tilemap_node.take() {
            // Dereference the Arc once for efficient access
            let chunk_data = chunk_data_arc.as_ref();

            // Use i64 for all internal geometry calculations
            let chunk_size = chunk_data::ChunkData::SIZE as i64;
            
            // Chunk coordinates are now pulled as i64 from the ChunkData struct (Bulldozer)
            let chunk_world_x: i64 = chunk_data.bounds.min.x;
            let chunk_world_y: i64 = chunk_data.bounds.min.y;
            
            let layer = 0;
            let source_id = 0;
            let alternative_tile = 0;

            // ⚠️ THIS LOOP MUST BE REPLACED WITH A BATCH CALL
            for y_i64 in 0..chunk_size {
                for x_i64 in 0..chunk_size {
                    // Index calculation uses i64 to prevent usize overflow on large chunks
                    let index = (y_i64 * chunk_size + x_i64) as usize; 
                    
                    if let Some(tile) = chunk_data.tiles.get(index) {
                        
                        let tile_type_id = tile.tile_type as i32;
                        
                        // Calculate absolute world coordinates using i64
                        let world_x_i64 = x_i64 + chunk_world_x;
                        let world_y_i64 = y_i64 + chunk_world_y;
                        
                        // 🚨 CRITICAL CHECK: Acknowledge the Godot i32 limitation (Bulldozer Guard)
                        if world_x_i64 > i32::MAX as i64 || world_x_i64 < i32::MIN as i64 || 
                            world_y_i64 > i32::MAX as i64 || world_y_i64 < i32::MIN as i64 {
                            
                            godot_error!("Bulldozer Warning: Tile ({}, {}) exceeds Godot TileMap i32 coordinate range. World is too big for current TileMap rendering strategy.", world_x_i64, world_y_i64);
                            continue; 
                        }
                        
                        // Simplified Atlas Mapping (Assumes a 1D strip of 16 tiles per row)
                        let atlas_x = tile_type_id % 16;
                        let atlas_y = tile_type_id / 16;
                        
                        let atlas_coords = Vector2i::new(atlas_x, atlas_y);
                        // Final unavoidable downcast to Godot's 32-bit Vector2i
                        let tile_coords = Vector2i::new(world_x_i64 as i32, world_y_i64 as i32); 

                        tilemap
                            .set_cell_ex(layer, tile_coords)
                            .source_id(source_id)
                            .atlas_coords(atlas_coords)
                            .alternative_tile(alternative_tile)
                            .done();
                    }
                }
            }
            
            // Put the tilemap back
            self.tilemap_node = Some(tilemap);
            
            info!("Chunk data for ({}, {}) applied to TileMap.",
                chunk_world_x / chunk_size,
                chunk_world_y / chunk_size);
        } else {
            godot_error!("Cannot apply chunk data: TileMap node is not set.");
        }
    }


    // ------------------------- MPSC POLLING AND HANDLING -------------------------

    /// Private helper to process a single `GenerationMessage` and emit the corresponding Godot signals.
    fn handle_generation_update(&mut self, message: GenerationMessage) {
        let mut status_update: Option<GString> = None;
        
        match message {
            GenerationMessage::StatusUpdate(msg) => {
                info!("Generation Status: {}", msg);
                status_update = Some(GString::from(format!("GENERATING: {}", msg).as_str()));
            }
            // 🚀 TEMPO BOOST: Now receives Arc<ChunkData> for zero-copy transfer
            GenerationMessage::ChunkGenerated(coords, chunk_data_arc) => {
                // coords.x and coords.y are now i64 (from Conductor refactor)
                info!("Chunk Generated: ({}, {}). Applying chunk data...", coords.x, coords.y);
                
                // CRITICAL: Call the synchronous drawing function on the main thread
                self.apply_chunk_data(chunk_data_arc);
                
                // Emit signal (mostly for UI/metrics)
                if let Some(signals) = &self.signals_node {
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        // The signal arguments must be i32 for Godot, clipping the coordinates if the world is too large.
                        ssxl_signals_instance.bind_mut().emit_chunk_generated(coords.x as i32, coords.y as i32);
                    }
                }
            }
            GenerationMessage::GenerationComplete => {
                info!("Generation Complete.");
                status_update = Some(GString::from("IDLE: Ready for next command."));
                if let Some(signals) = &self.signals_node {
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        ssxl_signals_instance.bind_mut().emit_build_map_complete();
                    }
                }
            }
            GenerationMessage::Error(e) => {
                godot_error!("Generation Task Error: {}", e);
                status_update = Some(GString::from(format!("ERROR: {}", e).as_str()));
            }
        }

        // Re-borrow self to emit status signal after signal node use.
        if let Some(status) = status_update {
            self.emit_status_updated(status);
        }
    }

    /// Main engine tick method, called by SSXLOracle to poll for updates.
    /// (Tempo strategy: non-blocking channel drain)
    #[func]
    pub fn tick(&mut self, _current_tick: u64) {
        // godot_print! is removed for cleaner logs and performance focus.
        
        // Take ownership of the receiver to avoid mutable borrow conflicts
        if let Some(mut receiver) = self.generation_receiver.take() {
            let mut messages = Vec::new();
            let mut disconnected = false;

            // Drain the channel quickly
            loop {
                match receiver.try_recv() {
                    Ok(message) => messages.push(message),
                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        godot_error!("Generation channel disconnected.");
                        disconnected = true;
                        break;
                    }
                }
            }

            // Process messages
            for message in messages {
                self.handle_generation_update(message);
            }
            
            // Put the receiver back ONLY if it wasn't disconnected.
            if !disconnected {
                self.generation_receiver = Some(receiver);
            }
        }
    }
    
    // Helper to initialize conductor lazily
    fn ensure_conductor(&mut self) -> bool {
        if self.conductor.is_some() {
            return true;
        }

        godot_print!("--- Initializing Conductor lazily ---");
        
        // Configuration path is None by default, relying on environment or defaults
        match Conductor::new(None) {
            Ok((conductor_instance, _state, progress_receiver)) => {
                info!("SSXL Conductor initialized successfully.");
                self.conductor = Some(Arc::new(Mutex::new(conductor_instance)));
                self.generation_receiver = Some(progress_receiver); // Store the receiver
                true
            }
            Err(e) => {
                godot_error!("SSXL Conductor failed to initialize: REASON: {:?}", e);
                false
            }
        }
    }

    // ------------------------- NODE SETTERS -------------------------

    /// Sets the reference to the SSXLSignals node for emitting core events.
    #[func]
    pub fn set_signals_node(&mut self, signals_node: Gd<Node>) {
        self.signals_node = Some(signals_node);
        info!("SSXLEngine: Signals Node reference set.");
    }
    
    /// Sets the reference to the main expansive TileMap for visual updates.
    #[func]
    pub fn set_tilemap(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node);
        info!("SSXLEngine: TileMap Node reference set.");
    }
    
    // ------------------------- CORE GENERATION -------------------------

    /// Triggers the core map generation pipeline in the Conductor.
    #[func]
    pub fn build_map(&mut self, width: i32, height: i32, seed: GString, generator_name: GString) {
        // 1. Validation and Initialization
        if !self.ensure_conductor() {
            return;
        }

        let conductor_arc = self.conductor.as_ref().unwrap().clone();

        let config = GeneratorConfig {
            width: width as usize,
            height: height as usize,
            seed: seed.to_string(),
            generator_name: generator_name.to_string(),
        };
        
        // 2. Conductor Command and Error Handling
        let result = match conductor_arc.lock() {
            Ok(mut conductor) => {
                match conductor.start_generation(config) {
                    Ok(_) => {
                        info!("SSXLEngine: Conductor command accepted: START GENERATION.");
                        Ok(())
                    }
                    Err(e) => {
                        godot_error!("Conductor failed to start generation: REASON: {:?}", e);
                        Err("ERROR: Generation Command Failed")
                    }
                }
            }
            Err(e) => {
                godot_error!("CRITICAL: Mutex lock failed during build_map command: {:?}", e);
                Err("ERROR: Engine Lock Failure")
            }
        };

        // 3. Status & Signals
        match result {
            Ok(_) => {
                self.emit_status_updated(GString::from("GENERATING"));

                if let Some(signals) = &self.signals_node {
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        ssxl_signals_instance.bind_mut().emit_build_map_start();
                    } else {
                        godot_error!("Signal Emission Failed: Signals Node reference is not of type SSXLSignals.");
                    }
                } else {
                    godot_error!("Signal Emission Failed: SSXLSignals node reference not set.");
                }
            }
            Err(status_msg) => {
                self.emit_status_updated(GString::from(status_msg));
            }
        }
    }
    
    /// Sets the active generator algorithm by its string ID (e.g., "perlin_basic_2d").
    #[func]
    pub fn set_generator(&mut self, id: GString) -> bool {
        let id_str = id.to_string();

        if !self.ensure_conductor() {
            godot_error!("Cannot set generator: Conductor not initialized.");
            return false;
        }

        let conductor_arc = self.conductor.as_ref().unwrap();

        match conductor_arc.lock() {
            Ok(mut conductor) => conductor.set_active_generator(&id_str).is_ok(),
            Err(e) => {
                godot_error!("Mutex lock failed during set_generator: {:?}", e);
                false
            }
        }
    }

    /// Returns the ID of the currently active generator.
    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        if self.conductor.is_none() {
            return GString::from("ERROR: CONDUCTOR NOT INITIALIZED");
        }

        let conductor_arc = self.conductor.as_ref().unwrap();

        match conductor_arc.lock() {
            Ok(conductor) => GString::from(conductor.get_active_generator_id().as_str()),
            Err(_) => GString::from("ERROR: MUTEX POISONED"),
        }
    }

    // ------------------------- CLEANUP -------------------------

    /// Expose a function for Godot to call on cleanup.
    #[func]
    pub fn shutdown_engine(&mut self) {
        info!("SSXLEngine: Shutting down.");

        // Clear the receiver, forcing a disconnection if the task is still running
        self.generation_receiver = None; 

        if let Some(conductor_arc) = self.conductor.take() {
            // Attempt to unwrap and teardown the conductor gracefully if no other references exist
            if let Ok(c) = Arc::try_unwrap(conductor_arc) {
                if let Ok(conductor) = c.into_inner() {
                    conductor.graceful_teardown();
                }
            } else {
                godot_error!("SSXLEngine: Cannot fully shutdown Conductor; other references still exist.");
            }
        }
    }
}