//! # SSXLEngine (The Core Orchestrator)
//!
//! This minimal module defines the `SSXLEngine` Godot class. Its sole focus is **orchestration**
//! and **lifecycle management**. All complex logic for thread initialization, API access,
//! and scene manipulation is strictly delegated to specialized components, resulting in
//! a clean, fast main-thread component.

// --- Godot GDExtension Imports (Minimized) ---
use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::Base;
use godot::builtin::GString;
// use godot::log::godot_print; // <-- REMOVED: godot_print is a macro and is available via prelude

// --- Standard Library Imports (Synchronization) ---
use std::sync::{Arc, Mutex};

// --- Core Handles & State ---
use ssxl_generate::Conductor;
use ssxl_generate::conductor_state::ConductorState;
use ssxl_sync::AnimationConductorHandle;
use ssxl_shared::AnimationState;

// --- Local Crate Imports (Components & Delegates) ---
use crate::async_poll::AsyncPoller;
use crate::chunk_presenter::ChunkPresenter;
use crate::channel_handler::ChannelHandler;
use crate::api_initializers::EngineInitializer;
use crate::status_reporter::StatusReporter;
use crate::engine_api_extension::EngineApiExtension; // Trait providing delegated API methods
use crate::ssxl_tilemap::SSXLTileMap; // <-- NEW: Import custom TileMap struct

// -----------------------------------------------------------------------------
// SSXLEngine Struct Definition
// -----------------------------------------------------------------------------

/// The central Godot Node that holds the state and handles the orchestration.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLEngine {
    // --- Threaded Core Handles ---
    conductor: Option<Arc<Mutex<Conductor>>>,
    animation_conductor: Option<AnimationConductorHandle>,

    // --- State Accessors ---
    conductor_state: Option<ConductorState>,
    animation_state: Option<AnimationState>,

    // --- Godot Node References ---
    signals_node: Option<Gd<Node>>,
    tilemap_node: Option<Gd<TileMap>>,

    // --- Main-Thread System Components ---
    initializer: EngineInitializer,
    poller: AsyncPoller,
    presenter: ChunkPresenter,
    handler: ChannelHandler,

    #[base]
    base: Base<Node>,
}


// -----------------------------------------------------------------------------
// Internal Logic (Minimal Lifecycle Hooks)
// -----------------------------------------------------------------------------

impl SSXLEngine {
    pub fn init(base: Base<Node>) -> Self {
        // Initializes all internal components to their default/empty state.
        Self {
            conductor: None, animation_conductor: None,
            conductor_state: None, animation_state: None,
            signals_node: None, tilemap_node: None,
            initializer: EngineInitializer::new(),
            poller: AsyncPoller::new(),
            presenter: ChunkPresenter::new(),
            handler: ChannelHandler::new(),
            base,
        }
    }

    /// Centralized Core Initialization Logic (Delegates complexity to EngineInitializer)
    fn initialize_core(&mut self) -> bool {
        // Single call to retrieve all handles and states from a dedicated module.
        let (c, grx, gs, ah, arx, as_) = self.initializer.execute_core_setup(); 

        if c.is_none() || ah.is_none() { 
            godot_print!("CRITICAL ERROR: Failed to spawn Rust conductors.");
            return false;
        }

        // Store handles and configure main-thread pipeline.
        self.conductor = c; self.conductor_state = gs;
        self.animation_conductor = ah; self.animation_state = as_;
        self.poller.set_generation_receiver(grx);
        self.poller.set_animation_receiver(arx);
        self.handler.set_presenter_handle(self.presenter.clone());
        self.handler.set_signals_node(self.signals_node.clone());
        true
    }

    /// Drains the generation channel and updates the scene if new data is available.
    fn poll_generation(&mut self) {
        let gen_messages = self.poller.poll_generation_messages();

        if let Some(status_update) = self.handler.process_generation_messages_deferred(
            gen_messages,
            self.conductor.as_ref().map(|arc| arc.clone()),
        ) {
            // Inlined helper: Emit status signal on self.
            self.base_mut().emit_signal("status_updated", &[status_update.to_variant()]);
        }
    }
}


// -----------------------------------------------------------------------------
// Exposed Godot API (Minimal)
// -----------------------------------------------------------------------------

#[godot_api]
impl SSXLEngine {
    /// Godot `_ready` Hook: Called when the node enters the scene tree.
    #[func]
    pub fn _ready(&mut self) { self.initialize_core(); }

    // --- State and Status (Delegated) ---
    #[func]
    pub fn get_current_tile_count(&self) -> u64 {
        StatusReporter::get_current_tile_count_value(self.conductor_state.as_ref())
    }

    #[func]
    pub fn get_status(&self) -> GString {
        StatusReporter::get_status_report(
            self.conductor_state.as_ref(),
            self.animation_state.as_ref(),
        )
    }

    // --- Signals (Events exposed to Godot) (Unchanged) ---
    #[signal]
    fn status_updated(status_message: godot::prelude::GString);

    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);


    // --- Core Game Loop (CLEANED) ---
    /// Godot `_process` mapped function: Called every frame for updating.
    #[func]
    pub fn tick(&mut self, _current_tick: u64) {
        if self.conductor.is_none() && !self.initialize_core() {
            return;
        }
        
        // 1. Check for and render newly generated chunks.
        self.poll_generation();
        
        // 2. Check for, drain, and process real-time animation updates.
        // The Poller only drains messages; the Handler emits the signal to prevent main-thread lag.
        
        if let Some(signals_node_gd) = self.signals_node.as_mut() {
            let anim_messages = self.poller.poll_animations(signals_node_gd);
            // This line is where the error occurs. It will be fixed by correcting the target method's signature.
            self.handler.process_animation_messages(anim_messages);
        } else {
            // This is a safety check; signals_node should be set in set_signals_node.
            godot_print!("Warning: Signals node is required for animation polling but is None.");
        }
    }

    // --- API Delegation (Provided by EngineApiExtension trait implementation below) ---
    
    // --- Engine Configuration (Minimal Delegation) ---

    /// Sets the signals node. Delegation is to ChannelHandler.
    #[func]
    pub fn set_signals_node(&mut self, signals_node: Gd<Node>) {
        self.signals_node = Some(signals_node.clone());
        self.handler.set_signals_node(Some(signals_node));
    }

    /// Sets the target TileMap node. Delegation is to ChunkPresenter.
    #[func]
    pub fn set_tilemap(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node.clone());
        
        // FIX E0308 (Final): Use .ok() to convert the Result returned by try_cast into an Option.
        if let Some(ssxl_tilemap_node) = tilemap_node.try_cast::<SSXLTileMap>().ok() {
            self.presenter.set_tilemap_node(ssxl_tilemap_node);
        } else {
            godot_print!("Warning: set_tilemap called with a TileMap node that is not an SSXLTileMap instance. Presenter will not be configured.");
        }
    }

    /// Safely shuts down the multi-threaded conductors and clears resources.
    #[func]
    pub fn shutdown_engine(&mut self) {
        self.initializer.shutdown(self.animation_conductor.take(), self.conductor.take());
        self.poller.clear_receivers();
    }
}

// -----------------------------------------------------------------------------
// Trait Implementation: Brings back the public API functionality without the bulk.
// -----------------------------------------------------------------------------
impl EngineApiExtension for SSXLEngine {}