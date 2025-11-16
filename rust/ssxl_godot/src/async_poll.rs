//! # AsyncPoller
//!
//! This module implements the `AsyncPoller` struct, which acts as a crucial **thread-safe bridge**
//! between the SSXL-ext engine's **Rust worker threads** (Generation and Animation) and the
//! **Godot main thread**. It uses non-blocking channel polling (`try_recv`) to safely drain
//! messages without stalling the game engine loop.

// --- Godot GDExtension Imports ---
// NOTE: Removed unused godot imports (Node, Gd)
// --- Standard Library Imports ---
use std::sync::Arc;
// --- External Asynchronous Runtime Imports (Tokio) ---
use tokio::sync::mpsc::{
    Receiver as TokioReceiver,
    UnboundedReceiver as TokioUnboundedReceiver,
    error::TryRecvError as TokioTryRecvError
};
// --- SSXL-ext Shared & Generation Crates Imports ---
use ssxl_generate::task_queue::GenerationMessage;
use ssxl_shared::chunk_data::ChunkData;
use ssxl_shared::grid_bounds::GridBounds;
use ssxl_shared::messages::{ChunkMessage, AnimationUpdate};

// -----------------------------------------------------------------------------
// Constants and Type Aliases
// -----------------------------------------------------------------------------

/// **CRITICAL THROTTLE:** Max number of chunk messages to process in a single Godot frame.
/// This prevents the main thread from stalling when the Rust core finishes generating too quickly.
const MAX_GENERATION_MESSAGES_PER_POLL: usize = 64;

/// **REAL-TIME THROTTLE:** Max number of animation updates to process in a single Godot frame.
/// This ensures the main thread doesn't stutter under high animation load.
const MAX_ANIMATION_MESSAGES_PER_POLL: usize = 2048;

/// Type alias for the bounded channel receiver used by the **Generation** conductor.
pub type GenerationReceiver = TokioReceiver<GenerationMessage>;

/// Type alias for the unbounded channel receiver used by the **Animation** conductor.
pub type AnimationReceiver = TokioUnboundedReceiver<AnimationUpdate>;


// -----------------------------------------------------------------------------
// AsyncPoller Struct and Implementation
// -----------------------------------------------------------------------------

/// # AsyncPoller
///
/// Manages the receiver ends of the inter-thread message passing system.
/// It is responsible for draining these channels during the Godot frame loop.
#[derive(Default)]
pub struct AsyncPoller {
    /// Receiver for messages from the SSXL-ext **Generation** pipeline.
    generation_receiver: Option<GenerationReceiver>,
    /// Receiver for messages from the SSXL-ext **Animation** pipeline.
    animation_receiver: Option<AnimationReceiver>,
}

impl AsyncPoller {
    /// Creates a new, default instance of the `AsyncPoller`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the receiver for generated chunk data messages.
    pub fn set_generation_receiver(&mut self, receiver: Option<GenerationReceiver>) {
        self.generation_receiver = receiver;
    }

    /// Sets the receiver for real-time animation update messages.
    pub fn set_animation_receiver(&mut self, receiver: Option<AnimationReceiver>) {
        self.animation_receiver = receiver;
    }

    /// Clears both receivers, typically called during engine shutdown.
    pub fn clear_receivers(&mut self) {
        self.generation_receiver = None;
        self.animation_receiver = None;
    }

    // -------------------------------------------------------------------------
    // Polling Logic: Generation (THROTTLED)
    // -------------------------------------------------------------------------

    /// Polls the generation channel for available messages, **throttling** the
    /// maximum number of messages processed per Godot frame.
    pub fn poll_generation_messages(&mut self) -> Vec<ChunkMessage> {
        // Use `Option::take` to temporarily own the receiver.
        let mut receiver = match self.generation_receiver.take() {
            Some(r) => r,
            None => return Vec::new(),
        };

        // Pre-allocate vector capacity to the throttle limit.
        let mut messages = Vec::with_capacity(MAX_GENERATION_MESSAGES_PER_POLL);

        for _ in 0..MAX_GENERATION_MESSAGES_PER_POLL {
            match receiver.try_recv() {
                Ok(message) => {
                    let chunk_message = match message {
                        // Performance Optimization (Zero-Copy) using Arc::try_unwrap
                        GenerationMessage::ChunkGenerated(_coords, chunk_data_arc) => {
                            let chunk_data = Arc::try_unwrap(chunk_data_arc)
                                .unwrap_or_else(|arc| (*arc).clone());
                            ChunkMessage::Generated(chunk_data)
                        }
                        // Sentinel message for overall batch completion.
                        GenerationMessage::GenerationComplete => {
                            ChunkMessage::Generated(ChunkData::new(
                                0,
                                GridBounds::default(),
                                "complete".to_string(), // Sentinel string
                            ))
                        }
                    };
                    messages.push(chunk_message);
                }
                // Non-blocking exit: Channel is currently empty.
                Err(TokioTryRecvError::Empty) => break,
                // Critical Error: Generation channel disconnected.
                Err(TokioTryRecvError::Disconnected) => {
                    eprintln!("[SSXL-SYNC CRITICAL] Generation channel disconnected.");
                    // Do not restore the receiver; it's permanently dead.
                    return messages;
                }
            }
        }

        // Restore the receiver to the poller for the next Godot frame's poll.
        self.generation_receiver = Some(receiver);

        messages
    }

    // -------------------------------------------------------------------------
    // Polling Logic: Animation (THROTTLED)
    // -------------------------------------------------------------------------

    /// Polls the animation channel for available updates, **throttling** the
    /// maximum number of messages processed per Godot frame.
    pub fn poll_animations(&mut self) -> Vec<AnimationUpdate> {
        // Use `Option::take` to temporarily own the receiver.
        let mut receiver = match self.animation_receiver.take() {
            Some(r) => r,
            None => return Vec::new(),
        };

        // Pre-allocate vector capacity to the throttle limit.
        let mut updates = Vec::with_capacity(MAX_ANIMATION_MESSAGES_PER_POLL);

        for _ in 0..MAX_ANIMATION_MESSAGES_PER_POLL {
            match receiver.try_recv() {
                Ok(update) => {
                    updates.push(update);
                }
                // Non-blocking exit: Channel is currently empty.
                Err(TokioTryRecvError::Empty) => break,
                // Critical Error: Animation channel disconnected.
                Err(TokioTryRecvError::Disconnected) => {
                    eprintln!("[SSXL-SYNC CRITICAL] Animation channel disconnected.");
                    // Do not restore the receiver; it's permanently dead.
                    return updates;
                }
            }
        }

        // Restore the receiver to the poller for the next frame's poll.
        self.animation_receiver = Some(receiver);

        updates
    }
}