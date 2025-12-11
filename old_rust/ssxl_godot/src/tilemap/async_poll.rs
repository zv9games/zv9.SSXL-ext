// ============================================================================
// ⚡ AsyncPoller (`crate::tilemap::async_poll`)
// ----------------------------------------------------------------------------
// This module defines the `AsyncPoller` struct, which acts as the bridge
// between Tokio asynchronous channels and Godot’s main thread. It provides
// non-blocking polling methods to safely drain generation and animation
// message channels each frame.
//
// Purpose:
//   • Integrate Tokio async channels with Godot’s synchronous game loop.
//   • Provide safe, non-blocking polling of generation and animation updates.
//   • Prevent runaway loops by enforcing maximum message limits.
//   • Centralize channel management for clarity and maintainability.
//
// Key Components:
//   • Constants
//       - MAX_GEN_MSGS (64): maximum generation messages processed per poll.
//         Prevents runaway loops if channel is flooded.
//       - MAX_ANIM_MSGS (2048): maximum animation messages processed per poll.
//         Higher limit since animation updates are more frequent.
//
//   • Type Aliases
//       - AnimationReceiver: unbounded channel receiver for `AnimationUpdate`.
//         Animation updates are frequent, so unbounded channels are used.
//       - GenerationReceiver: bounded channel receiver for `GenerationMessage`.
//         Generation messages are controlled by the conductor, so bounded channels are used.
//
//   • AsyncPoller (struct)
//       - Fields:
//           • generation_rx: optional bounded receiver for generation messages.
//           • animation_rx: optional unbounded receiver for animation updates.
//       - Implements `Default` for easy initialization.
//
//   • Methods
//       - new()
//           • Creates a new AsyncPoller with no receivers.
//       - set_animation_rx(rx)
//           • Assigns an animation receiver.
//       - set_generation_rx(rx)
//           • Assigns a generation receiver.
//       - clear_receivers()
//           • Clears both receivers, useful for resetting or shutting down.
//       - poll_generation()
//           • Non-blocking poll of generation messages.
//           • Drains up to MAX_GEN_MSGS messages.
//           • Handles Empty (no messages) and Disconnected (channel closed).
//           • Returns a vector of `GenerationMessage`.
//       - poll_animations()
//           • Non-blocking poll of animation updates.
//           • Drains up to MAX_ANIM_MSGS messages.
//           • Uses smaller initial capacity (min(256)) for efficiency.
//           • Returns a vector of `AnimationUpdate`.
//
// Design Choices:
//   • Non-blocking polling ensures responsiveness in Godot’s frame loop.
//   • Bounded vs. unbounded channels reflect expected message frequency.
//   • Temporary ownership of receivers avoids borrow checker conflicts,
//     then receivers are restored for subsequent polls.
//   • Logging errors (e.g., channel disconnection) aids debugging.
//
// Educational Note:
//   • This module demonstrates how Rust can integrate asynchronous systems
//     (Tokio channels) into synchronous environments like Godot. By enforcing
//     message limits and restoring receivers after polling, `AsyncPoller`
//     ensures safe, efficient communication between concurrent generation/
//     animation tasks and the main game loop.
// ============================================================================


use tokio::sync::mpsc::{
    Receiver as TokioBoundedReceiver,
    UnboundedReceiver as TokioUnboundedReceiver,
    error::TryRecvError,
};

use ssxl_shared::AnimationUpdate; 
use ssxl_shared::message::generation_message::GenerationMessage; 

const MAX_GEN_MSGS: usize = 64;
const MAX_ANIM_MSGS: usize = 2048;

pub type AnimationReceiver = TokioUnboundedReceiver<ssxl_shared::AnimationUpdate>;
pub type GenerationReceiver = TokioBoundedReceiver<GenerationMessage>;

#[derive(Default)]
pub struct AsyncPoller {
    generation_rx: Option<GenerationReceiver>,
    animation_rx: Option<AnimationReceiver>,
}

impl AsyncPoller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_animation_rx(&mut self, rx: Option<AnimationReceiver>) {
        self.animation_rx = rx;
    }

    pub fn set_generation_rx(&mut self, rx: Option<GenerationReceiver>) {
        self.generation_rx = rx;
    }

    pub fn clear_receivers(&mut self) {
        self.generation_rx = None;
        self.animation_rx = None;
    }

    pub fn poll_generation(&mut self) -> Vec<GenerationMessage> {
        let Some(mut rx) = self.generation_rx.take() else {
            return Vec::new();
        };

        let mut updates = Vec::with_capacity(MAX_GEN_MSGS);

        loop {
            match rx.try_recv() {
                Ok(update) => updates.push(update),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    tracing::error!("Generation channel disconnected!");
                    break;
                }
            }

            if updates.len() >= MAX_GEN_MSGS {
                break;
            }
        }

        self.generation_rx = Some(rx);
        updates
    }

    pub fn poll_animations(&mut self) -> Vec<AnimationUpdate> {
        let Some(mut rx) = self.animation_rx.take() else {
            return Vec::new();
        };

        let mut updates = Vec::with_capacity(MAX_ANIM_MSGS.min(256));

        loop {
            match rx.try_recv() {
                Ok(update) => updates.push(update),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    tracing::error!("Animation channel disconnected!");
                    break;
                }
            }

            if updates.len() >= MAX_ANIM_MSGS {
                break;
            }
        }

        self.animation_rx = Some(rx);
        updates
    }
}
