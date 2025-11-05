// ssxl_godot/src/animation_api.rs

use godot::prelude::*;
use godot::classes::{Node, TileMap}; 
use godot::obj::Gd;

use std::sync::{Arc, Mutex};
use tracing::{info, error, warn};

// Internal Crate Dependencies
use ssxl_generate::Conductor;
use ssxl_shared::chunk_data::ChunkData; 
use ssxl_math::Vec2i; 
use ssxl_sync::{AnimationConductorHandle, AnimationCommand}; 


/// Delegate struct responsible for handling all calls from Godot related to 
/// animation control and sending commands to the background AnimationConductor thread.
#[derive(Default)] 
pub struct AnimationAPI<'a> {
    animation_conductor: Option<&'a AnimationConductorHandle>,
    _conductor: Option<&'a Arc<Mutex<Conductor>>>,
}

impl<'a> AnimationAPI<'a> {
    pub fn new(
        animation_conductor: Option<&'a AnimationConductorHandle>,
        conductor: Option<&'a Arc<Mutex<Conductor>>>,
    ) -> Self {
        AnimationAPI {
            animation_conductor,
            _conductor: conductor,
        }
    }

    // --------------------------------------------------------------------------
    // API IMPLEMENTATION
    // --------------------------------------------------------------------------

    /// Maps a Godot String command name to an AnimationCommand enum and sends it.
    /// This resolves the GDScript call `ssxl_engine.send_animation_command("...")`.
    pub fn send_command_by_name(&self, command_name: String) {
        if let Some(handle) = self.animation_conductor {
            let command = match command_name.as_str() {
                "StartTestAnimation" => {
                    info!("AnimationAPI: Mapping to StartTestAnimation command.");
                    AnimationCommand::StartTestAnimation 
                },
                "StopTestAnimation" => {
                    info!("AnimationAPI: Mapping to StopTestAnimation command.");
                    AnimationCommand::StopTestAnimation
                },
                _ => {
                    warn!("AnimationAPI: Received unrecognized command: {}", command_name);
                    return; // Exit if command is unrecognized
                }
            };
            
            // Send the command via the conductor handle
            if let Err(e) = handle.send(command) {
                error!("AnimationAPI: Failed to send command to worker: {}", e);
            }
        } else {
            warn!("AnimationAPI: Command '{}' received, but animation conductor is not initialized.", command_name);
        }
    }

    /// Starts the dedicated 30x30 self-clocked test animation.
    /// This only sends the start command to the worker, keeping the TileMap handle 
    /// on the main thread (Godot side).
    pub fn start_test_animation(&self, _tilemap_node: Option<&Gd<TileMap>>) {
        if let Some(handle) = self.animation_conductor {
            // ✅ UPDATED: Assuming AnimationCommand::StartTestAnimation is a unit variant.
            let command = AnimationCommand::StartTestAnimation;

            match handle.send(command) {
                Ok(_) => info!("AnimationAPI: Sent StartTestAnimation command."),
                Err(e) => error!("Failed to send StartTestAnimation command: {}", e),
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot start test animation.");
        }
    }

    /// Stops the dedicated 30x30 self-clocked test animation.
    pub fn stop_test_animation(&self, _signals_node: Option<&Gd<Node>>) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Stopping 30x30 test animation.");
            
            // Assuming AnimationCommand::StopTestAnimation is defined
            if let Err(e) = handle.send(AnimationCommand::StopTestAnimation) {
                 error!("Failed to send StopTestAnimation command: {}", e);
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot stop test animation.");
        }
    }

    /// Starts the animation worker thread's update loop.
    pub fn start_loading_animation(
        &self,
        framerate: f32,
        _signals_node: Option<&Gd<Node>>,
    ) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Starting loading animation at {:.2} FPS.", framerate);
            
            let update_result = handle.send(AnimationCommand::UpdateFramerate(framerate));
            let start_result = handle.send(AnimationCommand::Start);
            
            if let Err(e) = update_result {
                error!("Failed to send UpdateFramerate command: {}", e);
            }
            if let Err(e) = start_result {
                 error!("Failed to send Start command: {}", e);
            }
            
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot start animation.");
        }
    }

    /// Registers a specific chunk's coordinates to be included in the animation update cycle.
    pub fn register_chunk_for_animation(&self, chunk_x: i32, chunk_y: i32) {
        if let Some(handle) = self.animation_conductor {
            
            // Cast i32 to i64 to satisfy Vec2i::new argument types.
            let coords = Vec2i::new(chunk_x as i64, chunk_y as i64); 
            
            if let Some(conductor_arc) = self._conductor {
                let result = conductor_arc.lock().map(|conductor| {
                    
                    let chunk_data = conductor.get_chunk_data(&coords);
                    let chunk_arc = Arc::new(chunk_data);
                    
                    handle.send(AnimationCommand::RegisterChunk(chunk_arc))
                        .map_err(|e| error!("Failed to send RegisterChunk command: {}", e))
                        .ok();
                    info!("AnimationAPI: Registered chunk ({}, {}) for animation.", chunk_x, chunk_y);
                });

                if let Err(e) = result {
                    error!("Failed to lock Conductor mutex for chunk registration: {}", e);
                }
            } else {
                warn!("Generation Conductor is not initialized. Cannot register chunk data.");
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot register chunk.");
        }
    }

    /// Stops the animation worker thread's update loop.
    pub fn stop_loading_animation(&self, _signals_node: Option<&Gd<Node>>) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Stopping loading animation.");
            
            if let Err(e) = handle.send(AnimationCommand::Stop) {
                 error!("Failed to send Stop command: {}", e);
            }
            
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot stop animation.");
        }
    }
}