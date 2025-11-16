//! # SSXLEngine (The Core Orchestrator)
//!
//! This minimal module defines the `SSXLEngine` Godot class. Its sole focus is **orchestration**
//! and **lifecycle management**. All complex logic for thread initialization, API access,
//! and scene manipulation is strictly delegated to specialized components, resulting in
//! a clean, fast main-thread component.

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::Base;
use godot::builtin::GString;

// --- Standard Library Imports (Synchronization) ---
use std::sync::{Arc, Mutex};
// FIX E0308: Need `try_into` for safe u32 -> usize conversion.
use std::convert::TryInto; 

// --- Core Handles & State ---
use ssxl_generate::{Conductor, GeneratorConfig}; 
use ssxl_generate::conductor_state::ConductorState;
use ssxl_shared::AnimationState;
use ssxl_shared::AnimationCommand; // Used for inter-thread communication.

// --- Local Crate Imports (Components & Delegates) ---
use crate::async_poll::AsyncPoller;
use crate::chunk_presenter::ChunkPresenter;
use crate::channel_handler::ChannelHandler;
use crate::api_initializers::{
    EngineInitializer,
    AnimationConductorHandle,
};
use crate::status_reporter::StatusReporter;
use crate::ssxl_tilemap::SSXLTileMap; // Import custom TileMap struct

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
    // Arc<Mutex<ChunkPresenter>> is wrapped in Option for proper initialization
    presenter: Option<Arc<Mutex<ChunkPresenter>>>,
    handler: ChannelHandler,

    #[base]
    base: Base<Node>,
}


// -----------------------------------------------------------------------------
// Internal Logic (Minimal Lifecycle Hooks)
// -----------------------------------------------------------------------------

impl SSXLEngine {
    /// The required constructor for a class marked with `#[class(init)]`.
    fn init(base: Base<Node>) -> Self {
        Self {
            conductor: None,
            animation_conductor: None,
            conductor_state: None,
            animation_state: None,
            signals_node: None,
            tilemap_node: None,
            // Initialize all components explicitly.
            initializer: EngineInitializer::new(),
            poller: AsyncPoller::new(),
            // Initialize the Option field with Some(ChunkPresenter::new_internal())
            presenter: Some(Arc::new(Mutex::new(ChunkPresenter::new_internal()))),
            handler: ChannelHandler::default(),
            base,
        }
    }

    /// Centralized Core Initialization Logic (Delegates complexity to EngineInitializer)
    fn initialize_core(&mut self) -> bool {
        // Prevent re-initialization if systems are already running
        if self.conductor.is_some() {
            return true;
        }

        godot_print!("SSXLEngine: Initializing Rust core runtime...");

        // Single call to retrieve all handles and states from a dedicated module.
        let (c, grx, gs, ah, arx, as_) = self.initializer.execute_core_setup();

        if c.is_none() || ah.is_none() {
            godot_print!("CRITICAL ERROR: Failed to spawn Rust conductors.");
            return false;
        }

        // Store handles and configure main-thread pipeline.
        self.conductor = c;
        self.conductor_state = gs;
        self.animation_conductor = ah;
        self.animation_state = as_;

        // Configure Poller with receivers
        self.poller.set_generation_receiver(grx);
        self.poller.set_animation_receiver(arx);

        // Configure ChannelHandler by safely cloning the presenter handle from the Option.
        if let Some(presenter_handle) = &self.presenter {
             self.handler.set_presenter_handle(presenter_handle.clone());
        }

        self.handler.set_signals_node(self.signals_node.clone());

        godot_print!("SSXLEngine: Core runtime initialized successfully.");
        true
    }

    /// Drains the generation channel and updates the scene if new data is available.
    fn poll_generation(&mut self) {
        let gen_messages = self.poller.poll_generation_messages();

        if let Some(status_update) = self.handler.process_generation_messages_deferred(
            gen_messages,
            // Pass a cloned Arc<Mutex<Conductor>> only if it exists
            self.conductor.as_ref().map(|arc| arc.clone()),
        ) {
            // Emit status signal on the SSXLEngine node itself.
            self.base_mut().emit_signal("status_updated", &[status_update.to_variant()]);
        }
    }
}


// -----------------------------------------------------------------------------
// Exposed Godot API
// -----------------------------------------------------------------------------

#[godot_api]
impl SSXLEngine {
    /// Godot `_ready` Hook: Called when the node enters the scene tree.
    #[func]
    pub fn _ready(&mut self) {
        // Attempt to initialize core systems right away.
        self.initialize_core();
    }

    // --- FFI COMMANDS FOR GENERATION CONTROL ---

    /// FFI Command: Initiates the map generation task in the Rust Conductor.
    #[func]
    pub fn build_map(&mut self, width: u32, height: u32, seed_str: GString, generator_name: GString) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(mut conductor) => {
                    let seed = seed_str.to_string().parse::<u64>().unwrap_or_else(|_| {
                        godot_print!("Warning: Invalid seed input: {}. Using 0.", seed_str);
                        0
                    });
                    
                    // FIX E0308 (width/height types), E0308 (seed type), E0560 (field name)
                    let config = GeneratorConfig {
                        // FIX E0308: Convert u32 to usize. `try_into().unwrap()` is used based on the compiler's suggestion.
                        width: width.try_into().unwrap(),
                        height: height.try_into().unwrap(),
                        // FIX E0308: Convert u64 to String.
                        seed: seed.to_string(),
                        // FIX E0560: Change non-existent `generator_id` to the correct `generator_name`.
                        generator_name: generator_name.to_string(),
                    };
                    let result = conductor.start_generation(config);
                    
                    match result {
                        Ok(_) => godot_print!("SSXLEngine: Map build task SENT to Conductor ({}x{}).", width, height),
                        Err(e) => godot_print!("Error starting generation task: {:?}", e),
                    }
                },
                Err(e) => godot_print!("Error locking Conductor for build_map: {:?}", e),
            }
        } else {
            godot_print!("Error: Conductor not initialized. Cannot start generation.");
        }
    }

    /// FFI Command: Immediately stops all pending and active generation tasks.
    #[func]
    pub fn stop_generation(&mut self) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(mut conductor) => {
                    // FIX E0599: Assuming `stop_generation` is the correct method name in the external `Conductor` API.
                    let result = conductor.stop_generation();
                    
                    match result {
                        Ok(_) => godot_print!("SSXLEngine: Generation STOP command SENT to Conductor."),
                        Err(e) => godot_print!("Error sending stop command: {:?}", e),
                    }
                },
                Err(e) => godot_print!("Error locking Conductor for stop_generation: {:?}", e),
            }
        } else {
            godot_print!("Warning: Conductor not initialized. Cannot send stop command.");
        }
    }

    /// FFI Command: Sets the tile type or generator ID to be used for the next generation.
    #[func]
    pub fn set_generator(&mut self, tile_type_name: GString) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(mut _conductor) => {
                    // NOTE: The `Conductor::set_generator_id` method is currently unimplemented
                    // in the external API, so this is a placeholder.
                    
                    // let result = _conductor.set_generator_id(tile_type_name.to_string());
                    
                    godot_print!("SSXLEngine: Placeholder: Attempted to set generator '{}'. Conductor API is missing.", tile_type_name);
                },
                Err(e) => godot_print!("Error locking Conductor for set_generator: {:?}", e),
            }
        }
    }

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

    // --- Signals (Events exposed to Godot) ---
    #[signal]
    fn status_updated(status_message: godot::prelude::GString);

    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);


    // --- Core Game Loop ---
    /// Godot `_process` mapped function: Called every frame for updating.
    #[func]
    pub fn tick(&mut self, _current_tick: u64) {
        // Only run if the core systems were successfully initialized.
        if self.conductor.is_none() {
            return;
        }

        // 1. Check for and render newly generated chunks (Deferred).
        self.poll_generation();

        // 2. Check for, drain, and process real-time animation updates (Immediate).
        if self.signals_node.is_some() {
            let anim_messages = self.poller.poll_animations();
            self.handler.process_animation_messages(anim_messages);
        }
    }

    // --- Animation Control ---
    
    /// FFI Command: Immediately stops the Animation Conductor by sending a command to set its state to disabled.
    #[func]
    pub fn stop_animation(&mut self) {
        if let Some(ref conductor_sender) = self.animation_conductor {
            let command = AnimationCommand::SetEnabled(false);
            
            match conductor_sender.send(command) {
                Ok(_) => godot_print!("SSXLEngine: Animation STOP command SENT to Conductor."),
                Err(e) => godot_print!("Error sending animation stop command: {}", e),
            }
        } else {
            godot_print!("Warning: Animation Conductor is not initialized. Cannot send stop command.");
        }
    }
    
    /// Controls the state of the separate animation conductor.
    /// Sends an `AnimationCommand::SetEnabled` message to the async thread.
    #[func]
    pub fn set_animation_enabled(&mut self, enabled: bool) {
        if let Some(ref conductor_sender) = self.animation_conductor {
            // Use the channel's send method with the AnimationCommand::SetEnabled variant.
            let command = AnimationCommand::SetEnabled(enabled);
            
            match conductor_sender.send(command) {
                Ok(_) => godot_print!("SSXLEngine: Animation conductor command SENT: {}", enabled),
                Err(e) => godot_print!("Error sending animation command: {}", e),
            }
        } else {
            godot_print!("Warning: Animation Conductor is not initialized.");
        }
    }

    // --- Engine Configuration ---

    /// Sets the signals node and delegates the reference to the ChannelHandler.
    #[func]
    pub fn set_signals_node(&mut self, signals_node: Gd<Node>) {
        self.signals_node = Some(signals_node.clone());
        self.handler.set_signals_node(Some(signals_node));
    }

    /// Sets the target TileMap node and configures the ChunkPresenter.
    /// Requires the node to be an SSXLTileMap instance.
    #[func]
    pub fn set_tilemap(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node.clone());
        
        // Attempt to cast the generic TileMap node to our custom SSXLTileMap type.
        if let Ok(ssxl_tilemap_node) = tilemap_node.try_cast::<SSXLTileMap>() {
            // Access the presenter through the Option.
            if let Some(presenter_arc) = &self.presenter {
                // Lock the presenter's Mutex to safely update the inner state.
                if let Ok(mut presenter) = presenter_arc.lock() {
                    presenter.set_tilemap_node(ssxl_tilemap_node);
                    godot_print!("SSXLEngine: SSXLTileMap successfully set on ChunkPresenter.");
                } else {
                    godot_print!("Warning: Failed to acquire lock on ChunkPresenter during set_tilemap.");
                }
            } else {
                godot_print!("Warning: ChunkPresenter not initialized in SSXLEngine.");
            }
        } else {
            godot_print!("Warning: set_tilemap called with a TileMap node that is not an SSXLTileMap instance. Presenter will not be configured.");
        }
    }

    /// Safely shuts down the multi-threaded conductors and clears resources.
    #[func]
    pub fn shutdown_engine(&mut self) {
        // Delegate shutdown logic, clearing local handles in the process.
        self.initializer.shutdown(self.animation_conductor.take(), self.conductor.take());
        self.poller.clear_receivers();
        godot_print!("SSXLEngine: Engine shutdown complete.");
    }
}