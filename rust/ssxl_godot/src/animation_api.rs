// ssxl_godot/src/animation_api.rs

//!
//! Provides the public API interface for Godot to interact with the
//! asynchronous **Animation Conductor**.
//!
//! This adapter is responsible for:
//! 1. Translating Godot calls into structured `AnimationCommand` messages.
//! 2. Synchronously retrieving generated `ChunkData` from the main **Conductor**.
//! 3. Sending chunk data and commands to the animation worker thread for visual updates.

use godot::classes::{Node, TileMap};
use godot::obj::Gd;

use std::sync::{Arc, Mutex};
use tracing::{info, error, warn};

use ssxl_generate::Conductor;
use ssxl_math::Vec2i;
use ssxl_sync::AnimationConductorHandle;
// The AnimationCommand definition has changed to use unit variants and SetTimeScale(f32).
use ssxl_shared::messages::AnimationCommand;

// -----------------------------------------------------------------------------
// 1. API Structure
// -----------------------------------------------------------------------------

/// The Godot API layer for controlling the background animation processing thread.
///
/// It holds non-owning, bounded references (`'a`) to the essential Conductor handles.
#[derive(Default)]
pub struct AnimationAPI<'a> {
    /// Handle used to send commands to the Animation Conductor (responsible for frame updates).
    animation_conductor: Option<&'a AnimationConductorHandle>,
    /// Thread-safe reference to the main Generation Conductor (used to retrieve completed chunk data).
    _conductor: Option<&'a Arc<Mutex<Conductor>>>,
}

impl<'a> AnimationAPI<'a> {
    /// Constructs a new AnimationAPI, setting the internal references upon initialization.
    pub fn new(
        animation_conductor: Option<&'a AnimationConductorHandle>,
        conductor: Option<&'a Arc<Mutex<Conductor>>>,
    ) -> Self {
        AnimationAPI {
            animation_conductor,
            _conductor: conductor,
        }
    }


    // -------------------------------------------------------------------------
    // 2. High-Level Command Mapping
    // -------------------------------------------------------------------------

    /// Receives a string command from Godot and maps it to the appropriate `AnimationCommand` enum.
    /// This provides a flexible entry point for custom scripting.
    pub fn send_command_by_name(&self, command_name: String) {
        if let Some(handle) = self.animation_conductor {
            let command = match command_name.as_str() {
                // FIX: Map to the unit variant
                "StartTestAnimation" => {
                    info!("AnimationAPI: Mapping to StartTestAnimation command.");
                    AnimationCommand::StartTestAnimation
                },
                // FIX: Map stop/disable to SetTimeScale(0.0)
                "StopTestAnimation" => {
                    info!("AnimationAPI: Mapping stop test animation to SetTimeScale(0.0).");
                    AnimationCommand::SetTimeScale(0.0)
                },
                // FIX: Map enable/start to SetTimeScale(1.0)
                "ANIMATION_ENABLE" => {
                    info!("AnimationAPI: Mapping animation enable to SetTimeScale(1.0).");
                    AnimationCommand::SetTimeScale(1.0)
                },
                // FIX: Map disable/stop to SetTimeScale(0.0)
                "ANIMATION_DISABLE" => {
                    info!("AnimationAPI: Mapping animation disable to SetTimeScale(0.0).");
                    AnimationCommand::SetTimeScale(0.0)
                },
                _ => {
                    warn!("AnimationAPI: Received unrecognized command: {}", command_name);
                    return;
                }
            };

            if let Err(e) = handle.send(command) {
                error!("AnimationAPI: Failed to send command to worker: {}", e);
            }
        } else {
            warn!("AnimationAPI: Command '{}' received, but animation conductor is not initialized.", command_name);
        }
    }

    // -------------------------------------------------------------------------
    // 3. Specific Control Methods
    // -------------------------------------------------------------------------

    /// Starts a predefined test animation sequence.
    pub fn start_test_animation(&self, _tilemap_node: Option<&Gd<TileMap>>) {
        if let Some(handle) = self.animation_conductor {
            // FIX: Using the unit variant StartTestAnimation
            let command = AnimationCommand::StartTestAnimation;

            match handle.send(command) {
                Ok(_) => info!("AnimationAPI: Sent StartTestAnimation command."),
                Err(e) => error!("Failed to send StartTestAnimation command: {}", e),
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot start test animation.");
        }
    }

    /// Stops the running test animation sequence.
    pub fn stop_test_animation(&self, _signals_node: Option<&Gd<Node>>) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Stopping 30x30 test animation by setting time scale to zero.");

            // FIX: Mapping stop to SetTimeScale(0.0)
            if let Err(e) = handle.send(AnimationCommand::SetTimeScale(0.0)) {
                error!("Failed to send SetTimeScale(0.0) (stop) command: {}", e);
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot stop test animation.");
        }
    }

    /// Configures the animation framerate and sends the global start command.
    pub fn start_loading_animation(
        &self,
        framerate: f32,
        _signals_node: Option<&Gd<Node>>,
    ) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Starting loading animation at {:.2} FPS by setting time scale.", framerate);

            // FIX: The framerate is used as the f32 argument for SetTimeScale
            let update_result = handle.send(AnimationCommand::SetTimeScale(framerate));
            
            // The Start command is removed, as SetTimeScale(non-zero) implies running.
            // If the framerate is 0, the animation will not start/run.

            if let Err(e) = update_result {
                error!("Failed to send SetTimeScale command: {}", e);
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot start animation.");
        }
    }

    /// Stops the generic loading animation sequence.
    pub fn stop_loading_animation(&self, _signals_node: Option<&Gd<Node>>) {
        if let Some(handle) = self.animation_conductor {
            info!("AnimationAPI: Stopping loading animation by setting time scale to zero.");

            // FIX: Mapping stop to SetTimeScale(0.0)
            if let Err(e) = handle.send(AnimationCommand::SetTimeScale(0.0)) {
                error!("Failed to send SetTimeScale(0.0) (stop) command: {}", e);
            }

        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot stop animation.");
        }
    }

    // -------------------------------------------------------------------------
    // 4. Data Transfer Method
    // -------------------------------------------------------------------------

    /// Retrieves pre-generated chunk data from the main Conductor and registers it
    /// with the animation system for display/interpolation.
    pub fn register_chunk_for_animation(&self, chunk_x: i32, chunk_y: i32) {
        if let Some(_handle) = self.animation_conductor {

            let coords = Vec2i::new(chunk_x as i64, chunk_y as i64);

            if let Some(conductor_arc) = self._conductor {
                // Lock the Conductor Mutex to call the synchronous data retrieval method.
                let result = conductor_arc.lock().map(|conductor| {
                    // Synchronously get the data (this will perform generation if not cached).
                    // The actual chunk data retrieval is fine.
                    let _chunk_data = conductor.get_chunk_data(&coords);
                    
                    // FIX: The AnimationCommand enum no longer has a 'RegisterChunk' or 'LoadChunk'
                    // variant for sending raw data. The intended functionality is currently missing
                    // from the communication contract (ssxl_shared::messages::AnimationCommand).
                    warn!("AnimationAPI: Cannot register chunk ({}, {}). The command for data registration (e.g., RegisterChunk) is not available in the current AnimationCommand definition. Skipping command send.", chunk_x, chunk_y);
                    
                    // Old, now incompatible command: 
                    // handle.send(AnimationCommand::RegisterChunk(Arc::new(chunk_data))) 

                });

                if let Err(e) = result {
                    error!("Failed to lock Conductor mutex for chunk data retrieval: {}", e);
                }
            } else {
                warn!("Generation Conductor is not initialized. Cannot register chunk data.");
            }
        } else {
            warn!("AnimationAPI: Animation Conductor is not initialized. Cannot register chunk.");
        }
    }
}