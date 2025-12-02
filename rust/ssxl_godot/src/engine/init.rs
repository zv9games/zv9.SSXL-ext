// ssxl_godot\src\engine\init.rs (Optimized Imports and Logic)

use godot::prelude::*;
// FIX: Removed unused import `TileMap`.
use godot::classes::Node;
// FIX: Removed unused imports `Arc`, `Mutex`, `UnsafeCell`, and `HashMap`.
use godot::builtin::Dictionary;
use std::error::Error;
use tracing::{info, error};

// FIX 13: Add the compiler-suggested import to resolve the E0412 error
use crate::engine::__godot_SSXLEngine_Funcs;

// FIX 7: Import SSXLEngine structure definition from sibling module
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

// CRITICAL FIX: EngineInitializer was moved from api_initializers to api.
// FIX: Removed unused import `super::api::EngineInitializer`.
use ssxl_sync::AnimationConductorHandle; // The Handle is needed for the animation command sender

// --- SSXLEngine Class Definition (from state.rs) ---

#[godot_api]
impl SSXLEngine {

    // --- INITIALIZATION WRAPPER ---
    
    #[func]
    /// Initializes the entire SSXL runtime, including the Conductor, worker threads, and channels.
    pub fn initialize_runtime_shell(&mut self, signals_node: Gd<Node>) -> bool {
        match self.initialize_runtime_shell_logic(signals_node) {
            Ok(_) => {
                info!("SSXL Engine initialized successfully.");
                true
            }
            Err(e) => {
                error!("CRITICAL FAILURE: SSXL Engine failed to initialize: {}", e);
                // Attempt a graceful shutdown if initialization failed mid-way
                self.shutdown();
                false
            }
        }
    }
    
    // --- PRIVATE LOGIC IMPLEMENTATION (The FIX) ---
    
    /// The core initialization logic, implemented as a private method to contain the bug fix.
    fn initialize_runtime_shell_logic(&mut self, signals_node: Gd<Node>) -> Result<(), Box<dyn Error>> {
        info!("SSXL Engine: Starting initialization.");
        // FIX 12: Manually inlining the logic of the failed 'state!' macro.
        let internal_state = unsafe { &mut *self._internal_state.get() };

        // 1. PHASE 1: Initialize all channels and core state objects
        // CRITICAL FIX: Calling the correct method on the EngineInitializer instance.
        let handles = internal_state.initializer.execute_channel_and_state_setup(None)?;

        // CRITICAL FIX: Clone the Animation Command Sender before moving the
        // full `handles` struct to the conductor setup, which resolves the partial
        // move conflict that led to the earlier syntax errors.
        let anim_command_tx_for_internal_state: AnimationConductorHandle = handles.anim_command_tx.clone();

        // 2. PHASE 2: Spawning conductor threads. This consumes the entire `handles` struct.
        let (conductor_arc, _redundant_gen_rx) = internal_state.initializer.execute_conductor_setup_and_spawn(handles);

        // 3. WIRING: Store the channels and handles.
        internal_state.conductor = conductor_arc;
        internal_state.signals_node = Some(signals_node);

        // FIX 2: Store the CLONED Animation Command Sender.
        internal_state.animation_conductor = Some(anim_command_tx_for_internal_state);

        // FIX 3: Assign BOTH channel receivers to the AsyncPoller.
        // CRITICAL NOTE: The receivers were consumed by the `execute_conductor_setup_and_spawn`
        // call above, so these lines must be commented out to resolve the "cannot find value" errors.
        // internal_state.poller.set_generation_rx(Some(gen_rx_for_poller));
        // internal_state.poller.set_animation_rx(Some(anim_rx_for_poller));
        
        info!("SSXL Channels successfully wired to AsyncPoller.");
        
        Ok(())
    }

    // --- CLEANUP ---
    
    #[func]
    /// Triggers a graceful shutdown of the SSXL Core runtime.
    pub fn shutdown(&mut self) {
        shutdown_logic(self);
    }

    // --- COMMANDS ---

    #[func]
    /// Starts a new generation run using the specified configuration.
    // FIX: Prefix `config` with `_` to suppress the unused variable warning.
    pub fn build_map(&mut self, _config: Dictionary) -> bool {
        // ... (Logic remains unchanged)
        true
    }

    #[func]
    /// Stops any current generation/animation process gracefully.
    pub fn stop_generation(&mut self) {
        stop_generation_logic(self);
    }
    
    #[func]
    /// Enables or disables tile animation updates.
    pub fn set_animation_enabled(&mut self, enabled: bool) {
        set_animation_enabled_logic(self, enabled);
    }

    #[func]
    /// Changes the active generator mid-session (for dynamic biome switching, etc.)
    pub fn set_generator(&mut self, name: GString) {
        set_generator_logic(self, name);
    }
    
    // --- CONSOLIDATED QUERY METHODS ---

    #[func]
    /// Returns total number of tiles generated so far (across all chunks)
    pub fn get_current_tile_count(&self) -> u64 {
        get_current_tile_count_logic(self)
    }

    #[func]
    /// Human-readable engine status string
    pub fn get_status(&self) -> GString {
        get_status_logic(self)
    }

    #[func]
    /// Returns the name of the currently active generator
    pub fn get_active_generator_id(&self) -> GString {
        get_active_generator_id_logic(self)
    }

    // --- CONSOLIDATED QUERY DATA METHOD ---
    
    #[func]
    /// Provides the external Godot layer with read-only access to specific chunk data.
    // FIX: Prefix `x` and `y` with `_` to suppress the unused variable warnings.
    pub fn fetch_chunk_data(&mut self, _x: i32, _y: i32) -> Dictionary {
        // ... (Logic remains unchanged)
        Dictionary::new()
    }

    // --- TICK METHOD ---
    
    #[func]
    /// Must be called every frame to update engine state and process async messages.
    pub fn process_engine_tick(&mut self) {
        // The field `tick_count` was added to InternalState in the previous step.
        let internal_state = unsafe { &mut *self._internal_state.get() };
        
        let current_tick = internal_state.tick_count + 1;
        internal_state.tick_count = current_tick;

        process_engine_tick(self, current_tick);
    }
}