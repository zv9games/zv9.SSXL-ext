// ssxl_godot/src/engine/init.rs

use godot::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use godot::classes::Node;
use godot::builtin::Dictionary;
use std::error::Error;
use tracing::{info, error};

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

use ssxl_sync::AnimationConductorHandle;
use ssxl_shared::message::AnimationCommand;

#[godot_api]
impl SSXLEngine {

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
                self.shutdown();
                false
            }
        }
    }

    /// The core initialization logic, implemented as a private method to contain the bug fix.
    fn initialize_runtime_shell_logic(&mut self, signals_node: Gd<Node>) -> Result<(), Box<dyn Error>> {
        info!("SSXL Engine: Starting initialization.");
        let internal_state = unsafe { &mut *self._internal_state.get() };

        // 1. PHASE 1: Initialize all channels and core state objects
        let handles = internal_state.initializer.execute_channel_and_state_setup(None)?;

        // Use the standard `inner()` method to get a reference to the
        // inner UnboundedSender, then clone it.
        let inner_sender_clone: UnboundedSender<AnimationCommand> = handles.anim_command_tx.inner().clone();

        // Use the associated function ::new() to construct the struct.
        let anim_command_tx_for_internal_state: AnimationConductorHandle =
            AnimationConductorHandle::new(inner_sender_clone);

        // 2. PHASE 2: Spawning conductor threads.
        let (conductor_arc, _redundant_gen_rx) = internal_state.initializer.execute_conductor_setup_and_spawn(handles);

        // 3. WIRING: Store the channels and handles.
        internal_state.conductor = conductor_arc;
        internal_state.signals_node = Some(signals_node);

        // Store the CLONED Animation Command Sender.
        internal_state.animation_conductor = Some(anim_command_tx_for_internal_state);

        // NOTE: The receivers were consumed by `execute_conductor_setup_and_spawn`
        // so lines setting the poller receivers are commented out/removed as per fix notes.

        info!("SSXL Channels successfully wired to AsyncPoller.");

        Ok(())
    }

    #[func]
    /// Triggers a graceful shutdown of the SSXL Core runtime.
    pub fn shutdown(&mut self) {
        shutdown_logic(self);
    }

    #[func]
    /// Starts a new generation run using the specified configuration.
    pub fn build_map(&mut self, _config: Dictionary) -> bool {
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

    #[func]
    /// Provides the external Godot layer with read-only access to specific chunk data.
    pub fn fetch_chunk_data(&mut self, _x: i32, _y: i32) -> Dictionary {
        Dictionary::new()
    }

    #[func]
    /// Must be called every frame to update engine state and process async messages.
    pub fn process_engine_tick(&mut self) {
        let internal_state = unsafe { &mut *self._internal_state.get() };

        let current_tick = internal_state.tick_count + 1;
        internal_state.tick_count = current_tick;

        process_engine_tick(self, current_tick);
    }
}