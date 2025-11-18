use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Gd;
use godot::builtin::GString;
use ssxl_generate::Conductor;
use ssxl_shared::messages::{AnimationUpdate, AnimationPayload};
use ssxl_shared::messages::ChunkMessage;
use crate::chunk_presenter::ChunkPresenter;
use std::sync::{Arc, Mutex};
use tracing::{info, error};

type PresenterHandle = Arc<Mutex<ChunkPresenter>>;

#[derive(Debug, Default, Clone)]
pub struct ChannelHandler {
    presenter: Option<PresenterHandle>,
    signals_node: Option<Gd<Node>>,
}

impl ChannelHandler {
    pub fn set_presenter_handle(&mut self, presenter: PresenterHandle) {
        self.presenter = Some(presenter);
    }

    pub fn set_signals_node(&mut self, signals_node: Option<Gd<Node>>) {
        self.signals_node = signals_node;
    }

    pub fn emit_build_map_start(&self) {
        if let Some(mut node) = self.signals_node.clone() {
            node.emit_signal("build_map_start", &[]);
        }
    }
    
    // NEW: Emits the signal when generation command is stopped
    pub fn emit_build_map_stopped(&self) {
        if let Some(mut node) = self.signals_node.clone() {
            node.emit_signal("build_map_stopped", &[]);
        }
    }
    
    // NEW: Emits the signal when the engine finishes a tick
    pub fn emit_tick_complete(&self, current_tick: u64) {
        if let Some(mut node) = self.signals_node.clone() {
            node.emit_signal("tick_complete", &[current_tick.to_variant()]);
        }
    }
    
    // NEW: Emits the signal when the animation thread state changes
    pub fn emit_animation_state_changed(&self, enabled: bool) {
        if let Some(mut node) = self.signals_node.clone() {
            node.emit_signal("animation_state_changed", &[enabled.to_variant()]);
        }
    }

    pub fn emit_generation_error(&self, error_message: GString) {
        if let Some(mut node) = self.signals_node.clone() {
            node.emit_signal("generation_error", &[error_message.to_variant()]);
        }
    }

    pub fn process_generation_messages_deferred(
        &mut self,
        messages: Vec<ChunkMessage>,
        conductor: Option<Arc<Mutex<Conductor>>>,
    ) -> Option<GString> {
        if messages.is_empty() {
            return None;
        }

        let mut is_complete = false;

        if let Some(ref presenter_handle) = self.presenter {
            match presenter_handle.lock() {
                Ok(presenter_lock) => {
                    for msg in messages {
                        if let ChunkMessage::Generated(data) = &msg {
                            if data.dimension_tag == "complete" {
                                is_complete = true;
                                continue;
                            }
                        }

                        if let Some(deferred_call) = presenter_lock.create_deferred_present_call(msg) {
                            deferred_call.call_deferred(&[]);
                        } else {
                        }
                    }
                },
                Err(_e) => {
                    return Some(GString::from("ERR_PRESENTER_MUTEX_POISONED"));
                }
            }
        } else {
        }

        if is_complete {
            if let Some(arc_mutex_conductor) = conductor {
                match arc_mutex_conductor.lock() {
                    Ok(conductor_lock) => {
                        conductor_lock.signal_generation_complete();
                    },
                    Err(_e) => {
                        return Some(GString::from("ERR_CONDUCTOR_MUTEX_POISONED"));
                    }
                }
            } else {
            }
        }

        None
    }

    pub fn process_animation_messages(&mut self, updates: Vec<AnimationUpdate>) {
        if updates.is_empty() {
            return;
        }

        if let Some(mut node) = self.signals_node.clone() {
            for update in updates {
                
                let new_frame_id = match update.payload {
                    AnimationPayload::FrameUpdate { new_frame } => new_frame,
                    
                    AnimationPayload::TweenValue { .. } => {
                        0
                    },
                };
                
                node.emit_signal(
                    "tile_flip_updated",
                    &[
                        (update.coord.x as i32).to_variant(),
                        (new_frame_id as i32).to_variant(),
                    ],
                );
            }
        }
    }
}