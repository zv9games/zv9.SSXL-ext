use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::{Base, Gd, WithBaseField};
use godot::builtin::GString;

// Standard library dependencies
use std::sync::{Arc, Mutex};
use tracing::info;

// Internal Crate Dependencies for State
use ssxl_generate::Conductor;
use ssxl_generate::conductor_state::ConductorState;
use ssxl_sync::AnimationConductorHandle;
use ssxl_sync::primitives::AnimationState;

// --- NEW DELEGATE MODULES ---
use crate::async_poll::AsyncPoller;
use crate::chunk_presenter::ChunkPresenter;
use crate::channel_handler::ChannelHandler;
use crate::api_initializers::EngineInitializer;
use crate::generation_api::GenerationAPI;
use crate::animation_api::AnimationAPI; // This module must contain the command sending logic

// -------------------------------------------------------------------------------------------------
// SSXL ENGINE GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// The Godot-facing wrapper class for the SSXL Engine.
// ... (omitted struct definition - no change)
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLEngine {
    // --- CORE STATE (Shared with worker threads) ---
    conductor: Option<Arc<Mutex<Conductor>>>,
    animation_conductor: Option<AnimationConductorHandle>,

    // Store the thread-safe state objects to access the workers' status.
    conductor_state: Option<ConductorState>,
    animation_state: Option<AnimationState>,

    // --- GODOT NODE REFERENCES ---
    signals_node: Option<Gd<Node>>,
    tilemap_node: Option<Gd<TileMap>>,

    // --- DELEGATE OBJECTS (Perform all work) ---
    initializer: EngineInitializer,
    poller: AsyncPoller,
    presenter: ChunkPresenter,
    handler: ChannelHandler,

    #[base]
    base: Base<Node>,
}


// ------------------------------------------------------------------------------------
// Constructor & Lifecycle
// ------------------------------------------------------------------------------------
// ... (omitted impl SSXLEngine - no change)
impl SSXLEngine {
    pub fn init(base: Base<Node>) -> Self {
        Self {
            conductor: None,
            animation_conductor: None,
            conductor_state: None,
            animation_state: None,
            signals_node: None,
            tilemap_node: None,
            
            initializer: EngineInitializer::new(),
            poller: AsyncPoller::new(),
            presenter: ChunkPresenter::new(),
            handler: ChannelHandler::new(),
            
            base,
        }
    }

    // Private helper for safe signal emission (required by GDExtension).
    fn emit_status_updated(&mut self, status: GString) {
        self.base_mut().emit_signal("status_updated", &[status.to_variant()]);
    }

    /// Internal helper called by gde_api_defs::_ready.
    pub fn on_ready(&mut self) {
        // Initialize workers and get their channel handles AND the state objects.
        let (conductor_arc, gen_rx, gen_state) = self.initializer.ensure_conductor();
        let (anim_handle, anim_rx, anim_state) = self.initializer.ensure_animation_conductor();

        self.conductor = conductor_arc;
        self.conductor_state = gen_state;
        self.animation_conductor = anim_handle;
        self.animation_state = anim_state;
        
        // Configure the Poller and Handler with the communication channels and presenter.
        self.poller.set_generation_receiver(gen_rx);
        self.poller.set_animation_receiver(anim_rx);
        self.handler.set_presenter(self.presenter.clone());
        self.handler.set_signals_node(self.signals_node.clone());
    }

    // Handle generation polling and processing with mutable borrow
    fn poll_generation(&mut self) {
        // Poll for completed generation chunks and messages
        let gen_messages = self.poller.poll_generation_messages();
        
        // Process generation messages (applies chunk data, emits signals)
        if let Some(status_update) = self.handler.process_generation_messages(
            gen_messages,
            self.conductor.as_ref().map(|arc| arc.clone()),
            self.tilemap_node.as_mut(),
        ) {
            self.emit_status_updated(status_update);
        }
    }
}


// ------------------------------------------------------------------------------------
// GODOT API DELEGATION
// ------------------------------------------------------------------------------------
#[godot_api]
impl SSXLEngine {
    // ... (omitted get_status, signals, and tick methods - no change)
    #[func]
    pub fn get_status(&self) -> GString {
        // Reads the actual, dynamically updated status from the thread-safe state objects.
        let gen_status = self.conductor_state.as_ref()
            .map(|state| format!("{:?}", state.get_status()))
            .unwrap_or_else(|| String::from("Uninitialized"));
        
        let anim_status = self.animation_state.as_ref()
            .map(|state| format!("{:?}", state.get_status()))
            .unwrap_or_else(|| String::from("Uninitialized"));

        let mut status = String::from("STATUS: ");
        status.push_str("Generation: ");
        status.push_str(&gen_status);
        status.push_str(" | Animation: ");
        status.push_str(&anim_status);

        GString::from(status.as_str())
    }

    #[signal]
    fn status_updated(status_message: godot::prelude::GString);

    // Signal definition for the animation bridge
    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);

    /// Main engine tick method, delegates polling and processing.
    #[func]
    // In ssxl_godot\src\ssxl_engine.rs, inside #[godot_api] impl SSXLEngine
	pub fn tick(&mut self, _current_tick: u64) {
		// 1. Handle all mutable operations
		self.poll_generation();

		// 2. Poll for animation updates with mutable base to emit signals
		{
			// Clone the base's Gd<Node> reference so it can be passed as a mutable,
			// independent emitter, avoiding conflicts with the mutable borrow of self.poller.
			let mut emitter_gd = self.base().clone().upcast::<Node>();
			self.poller.poll_animations(&mut emitter_gd); // Pass the mutable reference
		} // emitter_gd is dropped here
	}
    
    // --- CONDUCTOR API DELEGATION ---
    // ... (omitted GenerationAPI functions - no change)
    #[func]
    pub fn build_map(&mut self, width: i32, height: i32, seed: GString, generator_name: GString) {
        GenerationAPI::new(self.conductor.as_ref()).build_map(width, height, seed, generator_name, self.signals_node.as_ref());
    }
    
    #[func]
    pub fn set_generator(&mut self, id: GString) -> bool {
        GenerationAPI::new(self.conductor.as_ref()).set_generator(id)
    }

    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        GenerationAPI::new(self.conductor.as_ref()).get_active_generator_id()
    }

    // --- ANIMATION API DELEGATION ---

    #[func]
    pub fn start_loading_animation(&mut self, framerate: f32) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).start_loading_animation(framerate, self.signals_node.as_ref());
    }
    
    #[func]
    pub fn register_chunk_for_animation(&mut self, chunk_x: i32, chunk_y: i32) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).register_chunk_for_animation(chunk_x, chunk_y);
    }

    #[func]
    pub fn stop_loading_animation(&mut self) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).stop_loading_animation(self.signals_node.as_ref());
    }

    // 🌟 REQUIRED FIX: Expose the method the GDScript is trying to call 🌟
    #[func]
    pub fn send_animation_command(&mut self, command_name: GString) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .send_command_by_name(command_name.to_string());
    }
    
    // --- NEW TEST ANIMATION API DELEGATION (FIXED LOCATION) ---
    // ... (omitted start_test_animation and stop_test_animation - no change)
    /// Starts the dedicated 30x30 self-clocked test animation.
    /// This runs in the Rust core and uses TileMap::force_update() for rendering.
    #[func]
    pub fn start_test_animation(&mut self) {
        // Pass both conductors and the TileMap reference to the AnimationAPI
        // The TileMap is critical because the Rust core is now responsible for rendering (force_update).
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .start_test_animation(self.tilemap_node.as_ref());
    }

    /// Stops the dedicated 30x30 self-clocked test animation.
    #[func]
    pub fn stop_test_animation(&mut self) {
        // Pass only the animation conductor as the stop command doesn't need the TileMap
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .stop_test_animation(self.signals_node.as_ref());
    }
    
    // --- NODE SETTERS ---
    // ... (omitted setters and shutdown_engine - no change)
    #[func]
    pub fn set_signals_node(&mut self, signals_node: Gd<Node>) {
        self.signals_node = Some(signals_node.clone());
        self.handler.set_signals_node(Some(signals_node));
        info!("SSXLEngine: Signals Node reference set.");
    }
    
    #[func]
    pub fn set_tilemap(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node);
        info!("SSXLEngine: TileMap Node reference set.");
    }
    
    /// Expose a function for Godot to call on cleanup.
    #[func]
    pub fn shutdown_engine(&mut self) {
        info!("SSXLEngine: Shutting down.");
        self.initializer.shutdown(self.animation_conductor.take(), self.conductor.take());
        self.poller.clear_receivers();
    }
}
