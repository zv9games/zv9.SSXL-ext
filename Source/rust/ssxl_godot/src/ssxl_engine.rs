use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use godot::builtin::{GString, Dictionary, Array, Variant};
use godot::classes::TileMap;
use crate::ssxl_signals::SSXLSignals;

// PHASE 8.4: Required for the asynchronous receiver
use tokio::sync::mpsc; 

// Internal Crate Dependencies
use ssxl_generate::{Conductor, GeneratorConfig}; 
use ssxl_generate::conductor::GenerationMessage;
use ssxl_math::Vec2i;

// Standard library and utilities
use std::sync::{Arc, Mutex};
use tracing::info;
use godot::prelude::godot_error;
use godot::prelude::godot_print;


// -------------------------------------------------------------------------------------------------
// SSXL ENGINE GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// The Godot-facing wrapper class for the Aetherion Engine.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLEngine {
    /// The core Rust logic manager, safely wrapped in an Arc/Mutex for shared access.
    conductor: Option<Arc<Mutex<Conductor>>>,
    
    /// Reference to the AetherionSignals node for signal emission.
    signals_node: Option<Gd<Node>>, 
    
    /// Reference to the main expansive TileMap for drawing/manipulation.
    tilemap_node: Option<Gd<TileMap>>, 

    /// PHASE 8.4: The receiver side of the MPSC channel for progress updates.
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

    /// Helper to emit the status signal (optional, but good practice for internal changes)
    pub fn emit_status_updated(&mut self, status: GString) {
        self.base_mut().emit_signal("status_updated", &[status.to_variant()]);
    }
    
    
    
    #[signal] 
    fn status_updated(status_message: godot::prelude::GString);

    // --- PHASE 8.4: MPSC POLLING AND HANDLING ---

    /// Private helper to process a single `GenerationMessage` and emit the corresponding Godot signals.
    fn handle_generation_update(&mut self, message: GenerationMessage) {
        // Collect the status message first, so we can call self.emit_status_updated 
        // after the immutable borrow of self.signals_node ends. (FIXES E0502)
        let mut status_update: Option<GString> = None;
        
        if let Some(signals) = &self.signals_node {
            match message {
                GenerationMessage::StatusUpdate(msg) => {
                    info!("Generation Status: {}", msg);
                    status_update = Some(GString::from(format!("GENERATING: {}", msg).as_str()));
                }
                GenerationMessage::ChunkGenerated(coords) => {
                    // Convert Vec2i to i32 for Godot signal
                    info!("Chunk Generated: ({}, {})", coords.x, coords.y);
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        ssxl_signals_instance.bind_mut().emit_chunk_generated(coords.x as i32, coords.y as i32);
                    }
                }
                GenerationMessage::GenerationComplete => {
                    info!("Generation Complete.");
                    status_update = Some(GString::from("IDLE: Ready for next command."));
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        ssxl_signals_instance.bind_mut().emit_build_map_complete();
                    }
                }
                GenerationMessage::Error(e) => {
                    godot_error!("Generation Task Error: {}", e);
                    status_update = Some(GString::from(format!("ERROR: {}", e).as_str()));
                }
            }
        }

        // FIX E0502: Mutably borrow self here, after the immutable borrow of self.signals_node has ended.
        if let Some(status) = status_update {
            self.emit_status_updated(status);
        }
    }

    /// Main engine tick method, called by SSXLOracle.
    #[func]
    pub fn tick(&mut self, current_tick: u64) {
        godot_print!("SSXLEngine: Received tick #{}", current_tick);

        // FIX E0499: Take ownership of the receiver to avoid a mutable borrow of self 
        // that conflicts with the mutable borrow in self.handle_generation_update().
        if let Some(mut receiver) = self.generation_receiver.take() {
            let mut messages = Vec::new();
            let mut disconnected = false;

            // Use a non-blocking loop to drain the channel quickly
            loop {
                match receiver.try_recv() {
                    Ok(message) => {
                        // Store the message for processing later
                        messages.push(message); 
                    }
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // Channel is empty, done for this tick.
                        break;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        // All senders have dropped (task finished or crashed).
                        godot_error!("Generation channel disconnected.");
                        disconnected = true;
                        break;
                    }
                }
            }

            // Process messages here, now that the mutable borrow from the receiver is released.
            for message in messages {
                self.handle_generation_update(message);
            }
            
            // Put the receiver back ONLY if it wasn't disconnected.
            if !disconnected {
                self.generation_receiver = Some(receiver);
            }
        }
    }
    
    // Helper to initialize conductor lazily with logging
    fn ensure_conductor(&mut self) -> bool {
        if self.conductor.is_some() {
            return true;
        }

        // --- FIX APPLIED HERE ---
        // Changed godot_error! to godot_print! to stop this message from appearing as an error (E) in Godot.
        godot_print!("--- Initializing Conductor lazily ---");
        
        // PHASE 8.4: Update to handle the new return tuple: (Conductor, State, Receiver)
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

    // --- NODE SETTERS (Required by control_panel.gd) ---

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
    
    // --- CORE GENERATION & PIPELINE (Phase 8.3 Implementation) ---

    /// Implements Phase 8.3: Triggers the core map generation pipeline in the Conductor.
    #[func]
    pub fn build_map(&mut self, width: i32, height: i32, seed: GString, generator_name: GString) {
        // --- 1. Validation and Initialization ---
        if !self.ensure_conductor() {
            return;
        }

        // Clone the Arc before locking to satisfy the borrow checker (E0502 fix from prior step).
        let conductor_arc = self.conductor.as_ref().unwrap().clone();

        // Prepare the configuration object
        let config = GeneratorConfig {
            width: width as usize,
            height: height as usize,
            seed: seed.to_string(),
            generator_name: generator_name.to_string(),
        };
        
        // --- 2. Conductor Command and Error Handling ---
        let result = match conductor_arc.lock() {
            Ok(mut conductor) => {
                // Call the conductor's start generation method.
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
            // Gracefully handle Mutex Poisoning
            Err(e) => {
                godot_error!("CRITICAL: Mutex lock failed during build_map command: {:?}", e);
                Err("ERROR: Engine Lock Failure")
            }
        };

        // --- 3. Status & Signals (Moved outside the lock block to avoid E0502) ---
        match result {
            Ok(_) => {
                self.emit_status_updated(GString::from("GENERATING"));

                if let Some(signals) = &self.signals_node {
                    if let Ok(mut ssxl_signals_instance) = signals.clone().try_cast::<SSXLSignals>() {
                        // Emit the build_map_start signal to trigger Godot-side UI/flow logic.
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


    /// Returns the generated chunk data as a Godot Dictionary,
	/// satisfying the GDScript validation expectations.
	#[func]
    // FIX: Changed to `&mut self` to allow lazy initialization via `ensure_conductor`.
	pub fn generate_chunk(&mut self, x: i32, y: i32, key_z: i32) -> Dictionary {
		let chunk_coords = Vec2i::new(x, y);
		let mut result_dict = Dictionary::new();

		// 1. FIX: Ensure Conductor is initialized, solving the FFI test failure.
		if !self.ensure_conductor() { 
			godot_error!("Cannot generate chunk: Conductor not initialized.");
			return result_dict;
		}

		let conductor_arc = self.conductor.as_ref().unwrap();

		// 2. Lock the Mutex to access the Conductor's data
		match conductor_arc.lock() {
			Ok(conductor) => {
				// Log for debugging purposes
				godot_print!("Godot: Calling core generate_single_chunk for {:?}", chunk_coords);
        
				// NOTE: This is a synchronous call to retrieve data.
				let chunk_data = conductor.generate_single_chunk(chunk_coords);

				// --- DATA CONVERSION: Rust `ChunkData` to Godot `Dictionary` ---
				let mut tile_array = Array::new();

				for tile in chunk_data.tiles {
					let mut tile_dict = Dictionary::new();

					// Assumed fields for TileData struct
					tile_dict.set("id", Variant::from(tile.tile_type as i32));
					tile_dict.set("level", Variant::from(tile.noise_value));

					// Safely push complex data
					tile_array.push(&tile_dict.to_variant());
				}

				// Populate the result dictionary
				result_dict.set("key_x", Variant::from(x));
				result_dict.set("key_y", Variant::from(y));
				result_dict.set("key_z", Variant::from(key_z));
				result_dict.set("tile_count", Variant::from(tile_array.len() as i32));
				result_dict.set("tiles", tile_array.to_variant());
			},
			Err(e) => {
				godot_error!("Mutex lock failed during chunk generation: {:?}", e);
			}
		}

		result_dict	
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
            Ok(mut conductor) => {
                conductor.set_active_generator(&id_str).is_ok()
            },
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
            godot_error!("Cannot get active generator: Conductor not initialized.");
            return GString::from("ERROR: CONDUCTOR NOT INITIALIZED");
        }

        let conductor_arc = self.conductor.as_ref().unwrap();

        match conductor_arc.lock() {
            Ok(conductor) => GString::from(conductor.get_active_generator_id().as_str()),
            Err(_) => GString::from("ERROR: MUTEX POISONED"),
        }
    }

    // --- Destructor Replacement ---
    /// Expose a function for Godot to call on cleanup.
    #[func]
    pub fn shutdown_engine(&mut self) {
        info!("SSXLEngine: Shutting down.");

        // Clear the receiver, forcing a disconnection if the task is still running
        self.generation_receiver = None; 

        if let Some(conductor_arc) = self.conductor.take() {
            if let Ok(c) = Arc::try_unwrap(conductor_arc) {
                if let Ok(conductor) = c.into_inner() {
                    conductor.graceful_teardown();
                }
            } else {
                godot_error!("SSXLEngine: Cannot fully shutdown Conductor; other references still exist. This is usually okay if other systems hold a shared reference, but it might indicate a leak if unintended.");
            }
        }
    }
}