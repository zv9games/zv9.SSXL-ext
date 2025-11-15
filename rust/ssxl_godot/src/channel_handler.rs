// rust/ssxl_godot/src/channel_handler.rs (Cleaned)
//! # ChannelHandler
//!
//! This module implements the `ChannelHandler`. It acts as the central **dispatcher**
//! on the Godot main thread, receiving raw messages from the `AsyncPoller` and delegating:
//! 1. Generation messages to the `ChunkPresenter` for deferred rendering.
//! 2. Animation updates directly into Godot signals for real-time responsiveness.

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Gd;
use godot::builtin::GString;

// --- SSXL-ext Internal Crates Imports ---
use ssxl_generate::Conductor;
// Added AnimationPayload import for the match statement
use ssxl_shared::messages::{AnimationUpdate, AnimationPayload};
use ssxl_shared::messages::ChunkMessage; 
// FIX: Removed unused import `ssxl_generate::task_queue::GenerationMessage;`

// --- Local Crate Imports ---
use crate::chunk_presenter::ChunkPresenter;

// --- Standard Library Imports ---
use std::sync::{Arc, Mutex};
use tracing::{info, error};


// -----------------------------------------------------------------------------
// ChannelHandler Struct
// -----------------------------------------------------------------------------

/// Manages the context required to process messages and apply them to the Godot scene tree.
#[derive(Debug, Default, Clone)]
pub struct ChannelHandler {
    presenter: Option<ChunkPresenter>,
    signals_node: Option<Gd<Node>>,
}

impl ChannelHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_presenter_handle(&mut self, presenter: ChunkPresenter) {
        self.presenter = Some(presenter);
    }

    pub fn set_signals_node(&mut self, signals_node: Option<Gd<Node>>) {
        self.signals_node = signals_node;
    }

    // -------------------------------------------------------------------------
    // Processing Logic: Generation (Deferred Rendering)
    // -------------------------------------------------------------------------

    /// Processes a batch of newly generated chunk messages.
    pub fn process_generation_messages_deferred(
        &mut self,
        messages: Vec<ChunkMessage>,
        conductor: Option<Arc<Mutex<Conductor>>>,
    ) -> Option<GString> {
        if messages.is_empty() {
            return None;
        }

        let mut is_complete = false;

        if let Some(ref presenter) = self.presenter {
            for msg in messages {
                if let ChunkMessage::Generated(data) = &msg {
                    if data.dimension_tag == "complete" {
                        is_complete = true;
                        continue;
                    }
                }

                if let Some(deferred_call) = presenter.create_deferred_present_call(msg) {
                    deferred_call.call_deferred(&[]);
                } else {
                    error!("Failed to create deferred call for chunk message. Is TileMap set on Presenter?");
                }
            }
        }

        // --- Post-Processing: Handle Completion Status ---
        if is_complete {
            if let Some(arc_mutex_conductor) = conductor {
                match arc_mutex_conductor.lock() {
                    Ok(conductor_lock) => {
                        conductor_lock.signal_generation_complete();
                        info!("GenerationComplete received. Conductor status set to Running (Idle).");
                    },
                    Err(e) => {
                        error!("Failed to acquire Conductor lock to set status: {}", e);
                        return Some(GString::from("ERR_CONDUCTOR_MUTEX_POISONED"));
                    }
                }
            } else {
                error!("GenerationComplete received, but Conductor reference is None.");
            }
        }

        None
    }

    // -------------------------------------------------------------------------
    // Processing Logic: Animation (Real-time Signal Emission)
    // -------------------------------------------------------------------------

    /// Processes a batch of real-time animation updates and emits Godot signals.
    pub fn process_animation_messages(&mut self, updates: Vec<AnimationUpdate>) {
        if updates.is_empty() {
            return;
        }

        if let Some(mut node) = self.signals_node.clone() {
            for update in updates {
                
                // Use a match statement to safely extract the frame ID from the payload enum.
                let new_frame_id = match update.payload {
                    AnimationPayload::FrameUpdate { new_frame } => new_frame,
                    
                    // Handle other variants gracefully
                    AnimationPayload::TweenValue { .. } => {
                        error!("Received unhandled TweenValue payload. Expected FrameUpdate.");
                        0
                    },
                };
                
                // Emitting the signal that the TileMap code should be listening to.
                node.emit_signal(
                    "tile_flip_updated",
                    &[
                        // 1. Tile ID (from the coord field, using x component)
                        (update.coord.x as i32).to_variant(), 
                        // 2. New Frame ID (the value extracted from the payload match)
                        (new_frame_id as i32).to_variant(), 
                    ],
                );
            }
        }
    }
}