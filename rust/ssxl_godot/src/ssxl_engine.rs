use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::Base;
use godot::builtin::GString;
use std::sync::{Arc, Mutex};
use std::convert::TryInto;
use ssxl_generate::{Conductor, GeneratorConfig};
use ssxl_generate::conductor_state::ConductorState;
use ssxl_shared::AnimationState;
use ssxl_shared::AnimationCommand;
use crate::async_poll::AsyncPoller;
use crate::chunk_presenter::ChunkPresenter;
use crate::channel_handler::ChannelHandler;
use crate::api_initializers::{
    EngineInitializer,
    AnimationConductorHandle,
};
use crate::status_reporter::StatusReporter;
use crate::ssxl_tilemap::SSXLTileMap;
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLEngine {
    conductor: Option<Arc<Mutex<Conductor>>>,
    animation_conductor: Option<AnimationConductorHandle>,
    conductor_state: Option<ConductorState>,
    animation_state: Option<AnimationState>,
    signals_node: Option<Gd<Node>>,
    tilemap_node: Option<Gd<TileMap>>,
    initializer: EngineInitializer,
    poller: AsyncPoller,
    presenter: Option<Arc<Mutex<ChunkPresenter>>>,
    handler: ChannelHandler,
    #[base]
    base: Base<Node>,
}
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
            presenter: Some(Arc::new(Mutex::new(ChunkPresenter::new_internal()))),
            handler: ChannelHandler::default(),
            base,
        }
    }
    fn initialize_core(&mut self) -> bool {
        if self.conductor.is_some() {
            return true;
        }
        godot_print!("SSXLEngine: Initializing Rust core runtime...");
        let (c, grx, gs, ah, arx, as_) = self.initializer.execute_core_setup();
        if c.is_none() || ah.is_none() {
            godot_print!("CRITICAL ERROR: Failed to spawn Rust conductors.");
            return false;
        }
        self.conductor = c;
        self.conductor_state = gs;
        self.animation_conductor = ah;
        self.animation_state = as_;
        self.poller.set_generation_receiver(grx);
        self.poller.set_animation_receiver(arx);
        if let Some(presenter_handle) = &self.presenter {
             self.handler.set_presenter_handle(presenter_handle.clone());
        }
        self.handler.set_signals_node(self.signals_node.clone());
        godot_print!("SSXLEngine: Core runtime initialized successfully.");
        true
    }
    fn poll_generation(&mut self) {
        let gen_messages = self.poller.poll_generation_messages();
        if let Some(status_update) = self.handler.process_generation_messages_deferred(
            gen_messages,
            self.conductor.as_ref().map(|arc| arc.clone()),
        ) {
            self.base_mut().emit_signal("status_updated", &[status_update.to_variant()]);
        }
    }
}
#[godot_api]
impl SSXLEngine {
    #[func]
    pub fn _ready(&mut self) {
        self.initialize_core();
    }
    #[func]
    pub fn build_map(&mut self, width: u32, height: u32, seed_str: GString, generator_name: GString) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(mut conductor) => {
                    self.handler.emit_build_map_start();
                    let seed = seed_str.to_string().parse::<u64>().unwrap_or_else(|_| {
                        godot_print!("Warning: Invalid seed input: {}. Using 0.", seed_str);
                        0
                    });
                    let config = GeneratorConfig {
                        width: width.try_into().unwrap(),
                        height: height.try_into().unwrap(),
                        seed: seed.to_string(),
                        generator_name: generator_name.to_string(),
                    };
                    let result = conductor.start_generation(config);
                    match result {
                        Ok(_) => godot_print!("SSXLEngine: Map build task SENT to Conductor ({}x{}).", width, height),
                        Err(e) => {
                            godot_print!("Error starting generation task: {:?}", e);
                            let error_msg = format!("Failed to start: {:?}", e);
                            self.handler.emit_generation_error(GString::from(error_msg.as_str()));
                        },
                    }
                },
                Err(e) => godot_print!("Error locking Conductor for build_map: {:?}", e),
            }
        } else {
            godot_print!("Error: Conductor not initialized. Cannot start generation.");
        }
    }
    #[func]
    pub fn stop_generation(&mut self) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(conductor) => {
                    let result = conductor.stop_generation();
                    match result {
                        Ok(_) => {
                            godot_print!("SSXLEngine: Generation STOP command SENT to Conductor.");
                            self.handler.emit_build_map_stopped();
                        },
                        Err(e) => godot_print!("Error sending stop command: {:?}", e),
                    }
                },
                Err(e) => godot_print!("Error locking Conductor for stop_generation: {:?}", e),
            }
        } else {
            godot_print!("Warning: Conductor not initialized. Cannot send stop command.");
        }
    }
    #[func]
    pub fn set_generator(&mut self, tile_type_name: GString) {
        if let Some(conductor_arc) = self.conductor.as_ref() {
            match conductor_arc.lock() {
                Ok(_conductor) => {
                    godot_print!("SSXLEngine: Placeholder: Attempted to set generator '{}'. Conductor API is missing.", tile_type_name);
                },
                Err(e) => godot_print!("Error locking Conductor for set_generator: {:?}", e),
            }
        }
    }
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
    #[func]
    pub fn tick(&mut self, current_tick: u64) {
        if self.conductor.is_none() {
            return;
        }
        self.poll_generation();
        if self.signals_node.is_some() {
            let anim_messages = self.poller.poll_animations();
            self.handler.process_animation_messages(anim_messages);
        }
        self.handler.emit_tick_complete(current_tick);
    }
    #[func]
    pub fn stop_animation(&mut self) {
        if let Some(ref conductor_sender) = self.animation_conductor {
            let command = AnimationCommand::SetEnabled(false);
            match conductor_sender.send(command) {
                Ok(_) => {
                    godot_print!("SSXLEngine: Animation STOP command SENT to Conductor.");
                    self.handler.emit_animation_state_changed(false);
                },
                Err(e) => godot_print!("Error sending animation stop command: {}", e),
            }
        } else {
            godot_print!("Warning: Animation Conductor is not initialized. Cannot send stop command.");
        }
    }
    #[func]
    pub fn set_animation_enabled(&mut self, enabled: bool) {
        if let Some(ref conductor_sender) = self.animation_conductor {
            let command = AnimationCommand::SetEnabled(enabled);
            match conductor_sender.send(command) {
                Ok(_) => {
                    godot_print!("SSXLEngine: Animation conductor command SENT: {}", enabled);
                    self.handler.emit_animation_state_changed(enabled);
                },
                Err(e) => godot_print!("Error sending animation command: {}", e),
            }
        } else {
            godot_print!("Warning: Animation Conductor is not initialized.");
        }
    }
    #[func]
    pub fn set_signals_node(&mut self, signals_node: Gd<Node>) {
        self.signals_node = Some(signals_node.clone());
        self.handler.set_signals_node(Some(signals_node));
    }
    #[func]
    pub fn set_tilemap(&mut self, tilemap_node: Gd<TileMap>) {
        self.tilemap_node = Some(tilemap_node.clone());
        if let Ok(ssxl_tilemap_node) = tilemap_node.try_cast::<SSXLTileMap>() {
            if let Some(presenter_arc) = &self.presenter {
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
    #[func]
    pub fn shutdown_engine(&mut self) {
        self.initializer.shutdown(self.animation_conductor.take(), self.conductor.take());
        self.poller.clear_receivers();
        godot_print!("SSXLEngine: Engine shutdown complete.");
    }
}