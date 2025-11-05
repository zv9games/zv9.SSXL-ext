use godot::prelude::*; // Added to ensure all Godot traits are in scope
use godot::classes::Node;
use godot::obj::{GodotClass, Inherits, Gd, WithBaseField}; // For emit_signal
use godot::meta::ToGodot; // For to_variant
use std::sync::Arc;

// Import Tokio components for the generation receiver as the channel is Tokio-based
use tokio::sync::mpsc::{Receiver as TokioReceiver, UnboundedReceiver as TokioUnboundedReceiver, error::TryRecvError as TokioTryRecvError};

// Internal Crate Dependencies
use ssxl_shared::messages::ChunkMessage;
use ssxl_generate::task_queue::GenerationMessage;
use ssxl_shared::tile_data::AnimationUpdate;
use ssxl_shared::grid_bounds::GridBounds;
use ssxl_shared::chunk_data::ChunkData;

// Type aliases for clarity
pub type GenerationReceiver = TokioReceiver<GenerationMessage>;
pub type AnimationReceiver = TokioUnboundedReceiver<AnimationUpdate>;

/// The **AsyncPoller** is responsible for **non-blocking retrieval** of generated
/// chunks and animation updates from the asynchronous worker threads. It is called
/// every frame by the Godot main loop (`_process` / `tick`).
#[derive(Default)]
pub struct AsyncPoller {
    generation_receiver: Option<GenerationReceiver>,
    animation_receiver: Option<AnimationReceiver>,
}

impl AsyncPoller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_generation_receiver(&mut self, receiver: Option<GenerationReceiver>) {
        self.generation_receiver = receiver;
    }

    pub fn set_animation_receiver(&mut self, receiver: Option<AnimationReceiver>) {
        self.animation_receiver = receiver;
    }

    pub fn clear_receivers(&mut self) {
        self.generation_receiver = None;
        self.animation_receiver = None;
    }

    pub fn poll_generation_messages(&mut self) -> Vec<ChunkMessage> {
        let mut messages = Vec::new();

        if let Some(mut receiver) = self.generation_receiver.take() {
            loop {
                match receiver.try_recv() {
                    Ok(message) => {
                        let chunk_message = match message {
                            GenerationMessage::ChunkGenerated(_coords, chunk_data_arc) => {
                                let chunk_data = Arc::try_unwrap(chunk_data_arc)
                                    .unwrap_or_else(|arc| (*arc).clone());
                                ChunkMessage::Generated(chunk_data)
                            }
                            GenerationMessage::GenerationComplete => {
                                // This is a sentinel message; Godot side should handle completion
                                ChunkMessage::Generated(ChunkData::new(
                                    0,
                                    GridBounds::default(),
                                    "complete".to_string(), // Sentinel string
                                ))
                            }
                            // ✅ FIX: Removed Error and StatusUpdate, as they are not defined
                            // on GenerationMessage from ssxl_generate::task_queue.
                            // The Conductor's state now tracks status and errors directly.
                        };
                        messages.push(chunk_message);
                    }
                    Err(TokioTryRecvError::Empty) => break,
                    Err(TokioTryRecvError::Disconnected) => {
                        eprintln!("[SSXL-SYNC ERROR] Generation channel disconnected.");
                        // Put the receiver back if it disconnected, though this is usually fatal
                        self.generation_receiver = Some(receiver); 
                        return messages;
                    }
                }
            }
            self.generation_receiver = Some(receiver);
        }

        messages
    }

    pub fn poll_animation_updates<T: GodotClass + Inherits<Node>>(
        &mut self,
        emitter: &mut Gd<T>,
    ) {
        if let Some(mut receiver) = self.animation_receiver.take() {
            loop {
                match receiver.try_recv() {
                    Ok(update) => {
                        let mut node = emitter.clone().upcast::<Node>();
                        node.emit_signal(
                            "tile_flip_updated",
                            &[
                                (update.tile_coords.x as i32).to_variant(),
                                (update.new_atlas_coords.x as i32).to_variant(),
                            ],
                        );

                    }
                    Err(TokioTryRecvError::Empty) => break,
                    Err(TokioTryRecvError::Disconnected) => {
                        eprintln!("[SSXL-SYNC ERROR] Animation channel disconnected.");
                        return;
                    }
                }
            }
            self.animation_receiver = Some(receiver);
        }
    }

    pub fn poll_animations(&mut self, emitter: &mut Gd<Node>) {
        self.poll_animation_updates(emitter);
    }
}