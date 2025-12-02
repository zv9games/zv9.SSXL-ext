// ssxl_godot/src/engine/poller.rs (Final, Two-Channel Compatible AsyncPoller)

use tokio::sync::mpsc::Receiver as TokioReceiver;
use ssxl_generate::task_queue::GenerationMessage; 
// CRITICAL FIX: AnimationUpdate is located under ssxl_shared::message::messages, 
// not ssxl_shared::tile_data. This resolves the E0308 type mismatch.
use ssxl_shared::message::messages::AnimationUpdate; 
use std::collections::VecDeque;
use tracing::warn; 
use tokio::sync::mpsc::error::TryRecvError;

/// The primary struct responsible for non-blocking draining of asynchronous results.
/// It holds the two distinct Tokio MPSC channel receivers from the Conductor's setup.
pub struct AsyncPoller {
    // Channel for heavy generation results (set via setter in init.rs)
    gen_rx: Option<TokioReceiver<GenerationMessage>>, 
    // Channel for lightweight, frequent animation data (set via setter in init.rs)
    anim_rx: Option<TokioReceiver<AnimationUpdate>>, 
}

impl Default for AsyncPoller {
    fn default() -> Self {
        AsyncPoller {
            gen_rx: None,
            anim_rx: None,
        }
    }
}

impl AsyncPoller {
    // --- Setup Methods (Used by init.rs::initialize_runtime_shell) ---
    
    /// Sets the receiver for map generation progress messages.
    // NOTE: The setter names below are different from the ones called in init.rs, 
    // but the types match the current definition. Assuming init.rs will be fixed to use 
    // `set_generation_receiver` and `set_animation_receiver` later.
    pub fn set_generation_receiver(&mut self, rx: Option<TokioReceiver<GenerationMessage>>) {
        self.gen_rx = rx;
    }

    /// Sets the receiver for high-frequency animation updates.
    pub fn set_animation_receiver(&mut self, rx: Option<TokioReceiver<AnimationUpdate>>) {
        self.anim_rx = rx;
    }

    // --- Polling Methods (Used by tick.rs) ---

    /// Drains ALL available generation messages from the channel instantly (non-blocking loop).
    pub fn poll_generation_messages(&mut self) -> VecDeque<GenerationMessage> {
        let mut messages = VecDeque::new();
        // Only poll if the channel is set
        if let Some(rx) = &mut self.gen_rx {
            loop {
                // Non-blocking attempt to receive a message.
                match rx.try_recv() {
                    Ok(msg) => messages.push_back(msg),
                    Err(TryRecvError::Empty) => break, 
                    Err(TryRecvError::Disconnected) => {
                        warn!("Generation channel disconnected.");
                        self.gen_rx = None; // Mark as permanently disconnected
                        break;
                    }
                }
            }
        }
        messages
    }

    /// Drains ALL available animation updates from the channel instantly (non-blocking loop).
    pub fn poll_animations(&mut self) -> VecDeque<AnimationUpdate> {
        let mut updates = VecDeque::new();
        if let Some(rx) = &mut self.anim_rx {
            loop {
                match rx.try_recv() {
                    Ok(update) => updates.push_back(update),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        warn!("Animation channel disconnected.");
                        self.anim_rx = None; // Mark as permanently disconnected
                        break;
                    }
                }
            }
        }
        updates
    }
}