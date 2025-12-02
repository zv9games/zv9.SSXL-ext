// ssxl_godot/src/tilemap/async_poll.rs
//
// The sacred bridge between Tokio and Godot.
// Polls background channels on the main thread.
// Zero blocking. Zero panic. Eternal.

use tokio::sync::mpsc::{
    // FIX 3: Import the Bounded Receiver for Generation Messages
    Receiver as TokioBoundedReceiver,
    UnboundedReceiver as TokioUnboundedReceiver,
    error::TryRecvError,
};
// CRITICAL FIX: The imported type for AnimationUpdate is now at the crate root
use ssxl_shared::AnimationUpdate; 
// FIX 1: The correct GenerationMessage location (from prior fix)
use ssxl_shared::message::generation_message::GenerationMessage; 

const MAX_GEN_MSGS: usize = 64;
const MAX_ANIM_MSGS: usize = 2048;

// CRITICAL FIX 4: Update the type alias to use the crate root path for AnimationUpdate
pub type AnimationReceiver = TokioUnboundedReceiver<ssxl_shared::AnimationUpdate>;
// CRITICAL FIX 5: Define the type alias for the Bounded Receiver to expect GenerationMessage
pub type GenerationReceiver = TokioBoundedReceiver<GenerationMessage>;

#[derive(Default)]
pub struct AsyncPoller {
    // Only animation uses unbounded — generation is bounded and handled via Conductor
    // FIX 6: Add state for the Generation Receiver, using GenerationMessage
    generation_rx: Option<GenerationReceiver>,
    // FIX 7: Rename animation_rx for consistency with the new setter in init.rs
    animation_rx: Option<AnimationReceiver>,
}

impl AsyncPoller {
    pub fn new() -> Self {
        Self::default()
    }

    // --- Animation Methods (Renamed for consistency with init.rs fix) ---

    /// Setter matching the logic in init.rs
    pub fn set_animation_rx(&mut self, rx: Option<AnimationReceiver>) {
        // FIX 8: The 'take()' in init.rs already pulls the receiver out of an Option,
        // so the setter must accept an Option<Receiver> for cases where the channel is uninitialized (None).
        self.animation_rx = rx;
    }

    // --- Generation Methods (New) ---

    /// Setter for the generation channel, matching the logic in init.rs
    pub fn set_generation_rx(&mut self, rx: Option<GenerationReceiver>) {
        // FIX 9: Set the generation receiver
        self.generation_rx = rx;
    }

    pub fn clear_receivers(&mut self) {
        // FIX 10: Clear both receivers
        self.generation_rx = None;
        self.animation_rx = None;
    }

    /// Poll all pending generation updates — safe to call every frame
    pub fn poll_generation(&mut self) -> Vec<GenerationMessage> {
        // CRITICAL FIX 11: Change return type and message type to GenerationMessage
        let Some(mut rx) = self.generation_rx.take() else {
            return Vec::new();
        };

        let mut updates = Vec::with_capacity(MAX_GEN_MSGS);

        loop {
            // NOTE: The tokio Bounded Receiver uses a different try_recv signature (TryRecvError)
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


    /// Poll all pending animation updates — safe to call every frame
    pub fn poll_animations(&mut self) -> Vec<AnimationUpdate> {
        // FIX 12: Use the consistent name
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

        // FIX 13: Use the consistent name
        self.animation_rx = Some(rx);
        updates
    }
}