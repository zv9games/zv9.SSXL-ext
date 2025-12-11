// ============================================================================
// 🎼 SSXL Engine Godot API (`crate::engine::ssxl_engine`)
// ----------------------------------------------------------------------------
// This module exposes the SSXL engine’s functionality to the Godot game engine
// via the `#[godot_api]` and `#[func]` attributes. It bridges Rust’s procedural
// generation system with Godot’s scripting layer, enabling game developers to
// control and query the engine directly from GDScript.
//
// Purpose:
//   • Provide a public-facing API for initializing, controlling, and querying
//     the SSXL engine from Godot.
//   • Wire conductor, animation, and generation subsystems into Godot’s runtime.
//   • Ensure safe lifecycle management with graceful startup and shutdown.
//   • Expose ergonomic methods for terrain generation, animation toggling,
//     and engine state queries.
//
// Key Components:
//   • SSXLEngine (struct)
//       - Core engine state exposed to Godot.
//       - Holds conductor, animation handles, and runtime state.
//   • initialize_runtime_shell
//       - Public entry point for initializing the engine runtime.
//       - Wires channels, conductor, and animation subsystems.
//       - Returns success/failure as a boolean to Godot.
//   • initialize_runtime_shell_logic (private)
//       - Internal setup logic separated from public API.
//       - Creates channels, spawns conductor threads, and stores handles.
//   • shutdown
//       - Gracefully tears down engine resources via `cleanup::shutdown_logic`.
//   • build_map
//       - Stubbed method for starting a new generation run.
//   • stop_generation
//       - Halts current generation tasks via `commands::stop_generation_logic`.
//   • set_animation_enabled
//       - Toggles animation conductor (flow fields, particles) on/off.
//   • set_generator
//       - Switches active generator mid-session (biome switching).
//   • get_current_tile_count
//       - Returns total number of tiles generated so far.
//   • get_status
//       - Returns human-readable engine status string.
//   • get_active_generator_id
//       - Returns the ID of the currently active generator.
//   • fetch_chunk_data
//       - Stubbed method returning an empty `Dictionary` for chunk data.
//   • process_engine_tick
//       - Must be called every frame to update engine state.
//       - Increments tick counter and processes async messages.
//
// Design Choices:
//   • Separation of public API (`#[func]`) from internal logic ensures clarity.
//   • Defensive error handling prevents runtime crashes in Godot scripts.
//   • Logging (`info`, `error`) provides visibility into lifecycle events.
//   • Godot-compatible types (`GString`, `Dictionary`, `Node`) ensure seamless FFI.
//   • Stubbed methods (`build_map`, `fetch_chunk_data`) provide placeholders
//     for future expansion.
//
// Educational Note:
//   • This module demonstrates how Rust can integrate deeply with Godot via
//     FFI. By exposing structured APIs with safe concurrency and lifecycle
//     management, SSXL enables developers to harness procedural generation
//     directly in their game scripts while maintaining reliability and clarity.
// ============================================================================


use godot::prelude::*;
use godot::classes::{Node, TileMap}; 
use godot::builtin::Dictionary;
use std::error::Error;
use tracing::{info, error};
use crate::engine::GenerationAPI;
use crate::engine::__godot_SSXLEngine_Funcs;
use super::state::SSXLEngine;
use crate::engine::tick::process_engine_tick;
use crate::engine::commands::{
    stop_generation_logic,
    set_animation_enabled_logic,
    set_generator_logic,
};
use crate::engine::query::{
    get_current_tile_count_logic,
    get_status_logic,
    get_active_generator_id_logic,
};
use crate::engine::cleanup::shutdown_logic;
use ssxl_shared::AnimationConductorHandle;
use ssxl_shared::message::GenerationCommand; 
use ssxl_shared::{CHUNK_SIZE, TileCoord}; 
use ssxl_math::prelude::Vec2i; 

#[godot_api]
impl SSXLEngine {
    #[func]
    pub fn initialize_runtime_shell(&mut self, signals_node: Gd<Node>) -> bool {
        match self.initialize_runtime_shell_logic(signals_node) {
            Ok(_) => {
                info!("SSXL Engine initialized successfully.");
                true
            }
            Err(e) => {
                error!("CRITICAL FAILURE: SSXL Engine failed to initialize: {}", e);
                self.shutdown();
                false
            }
        }
    }

    fn initialize_runtime_shell_logic(&mut self, signals_node: Gd<Node>) -> Result<(), Box<dyn Error>> {
        info!("SSXL Engine: Starting initialization.");
        let internal_state = unsafe { &mut *self._internal_state.get() };

        let handles = internal_state.initializer.execute_channel_and_state_setup(None)?;

        // Clone the necessary senders before moving 'handles'.
        let anim_command_tx_for_internal_state: AnimationConductorHandle = handles.anim_command_tx.clone();
        let gen_command_tx_clone = handles.gen_command_tx.clone();

        // 'handles' is moved into this function
        let (conductor_arc, _redundant_gen_rx) = internal_state.initializer.execute_conductor_setup_and_spawn(handles);

        internal_state.conductor = conductor_arc;
        internal_state.signals_node = Some(signals_node);
        internal_state.animation_conductor = Some(anim_command_tx_for_internal_state);
        
        internal_state.request_sender = gen_command_tx_clone; 

        info!("SSXL Channels successfully wired to AsyncPoller.");

        Ok(())
    }

    #[func]
    pub fn shutdown(&mut self) {
        shutdown_logic(self);
    }

    #[func]
	pub fn build_map(&mut self, config: Dictionary) -> bool {
		let internal_state = unsafe { &mut *self._internal_state.get() };

		if internal_state.request_sender.is_closed() {
			if let Some(node) = internal_state.signals_node.as_mut() {
				node.emit_signal("generation_error", &[GString::from("Conductor shut down").to_variant()]);
			}
			return false;
		}

		let width: i32 = config.get("width").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0);
		let height: i32 = config.get("height").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0);
		let generator_id: GString = config.get("generator_id").and_then(|v| v.try_to::<GString>().ok()).unwrap_or_else(|| GString::from("perlin"));

		if width <= 0 || height <= 0 {
			if let Some(node) = internal_state.signals_node.as_mut() {
				node.emit_signal("generation_error", &[GString::from("Invalid dimensions").to_variant()]);
			}
			return false;
		}

		// 🔥 Flip engine state to Active
		internal_state.generation_api = GenerationAPI::Active; // or generation_active = true

		// 🔧 Apply generator ID before chunk dispatch
		set_generator_logic(self, generator_id);

		// 🔔 Emit start signal
		if let Some(node) = internal_state.signals_node.as_mut() {
			node.emit_signal("build_map_start", &[]);
		}

		let chunks_x = width.div_ceil(CHUNK_SIZE as i32);
		let chunks_y = height.div_ceil(CHUNK_SIZE as i32);

		for x in 0..chunks_x {
			for y in 0..chunks_y {
				let command = GenerationCommand::GenerateChunk {
					coords: TileCoord::from(Vec2i::new(x as i64, y as i64)),
				};
				if let Err(e) = internal_state.request_sender.send(command) {
					let msg = format!("Command send failed: {}", e);
					if let Some(node) = internal_state.signals_node.as_mut() {
						node.emit_signal("generation_error", &[GString::from(msg.as_str()).to_variant()]);
					}
					return false;
				}
			}
		}

		true
	}

    
    // -------------------------------------------------
    // PUBLIC API FUNCTIONS
    // -------------------------------------------------

    /// Exposes the TileMap reference setter. (API: set_tilemap)
    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        let internal_state = unsafe { &mut *self._internal_state.get() };
        internal_state.tilemap_node = Some(tilemap);
        info!("TileMap reference set in GDExtension.");
    }
    
    /// Exposes the FFI-style map build based on size. (API: build_map)
    #[func]
    pub fn build_map_by_size(&mut self, width: i32, height: i32, _generator_id: GString) { // W4: Added _ prefix
        let internal_state = unsafe { &mut *self._internal_state.get() };

        if internal_state.request_sender.is_closed() {
            error!("Conductor is shut down. Cannot request map.");
            return;
        }

        let chunks_x = width.div_ceil(CHUNK_SIZE as i32);
        let chunks_y = height.div_ceil(CHUNK_SIZE as i32);
        
        // NOTE: The generator_id parameter is ignored because the GenerationCommand
        // only supports GenerateChunk and SetGenerator, and we are sending chunks here.

        for x in 0..chunks_x {
            for y in 0..chunks_y {
                let command = GenerationCommand::GenerateChunk { 
                    coords: TileCoord::from(Vec2i::new(x as i64, y as i64)),
                };
                
                let _ = internal_state.request_sender.send(command);
            }
        }

        info!("Sent {} generation tasks.", chunks_x * chunks_y);
    }
    
    // -------------------------------------------------

    #[func]
    pub fn stop_generation(&mut self) {
        stop_generation_logic(self);
    }

    #[func]
    pub fn set_animation_enabled(&mut self, enabled: bool) {
        set_animation_enabled_logic(self, enabled);
    }

    #[func]
    pub fn set_generator(&mut self, name: GString) {
        set_generator_logic(self, name);
    }

    #[func]
    pub fn get_current_tile_count(&self) -> u64 {
        get_current_tile_count_logic(self)
    }

    #[func]
    pub fn get_status(&self) -> GString {
        get_status_logic(self)
    }

    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        get_active_generator_id_logic(self)
    }

    #[func]
    pub fn fetch_chunk_data(&mut self, _x: i32, _y: i32) -> Dictionary {
        Dictionary::new()
    }

    #[func]
    pub fn process_engine_tick(&mut self) {
        let internal_state = unsafe { &mut *self._internal_state.get() };

        let current_tick = internal_state.tick_count + 1;
        internal_state.tick_count = current_tick;

        process_engine_tick(self, current_tick);
    }
}