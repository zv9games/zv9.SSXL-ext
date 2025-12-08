// ============================================================================
// 🎼 AsyncPoller (`crate::engine::async_poller`)
// ----------------------------------------------------------------------------
// This module defines the `AsyncPoller` struct, which is responsible for
// non-blocking draining of asynchronous channels in the SSXL engine. It
// provides a unified interface for collecting both generation results and
// animation updates without stalling the runtime.
//
// Purpose:
//   • Manage asynchronous communication between engine subsystems.
//   • Provide non-blocking polling for generation messages and animation updates.
//   • Handle channel disconnections gracefully by disabling receivers.
//   • Collect messages into `VecDeque` for batch processing.
//
// Key Components:
//   • AsyncPoller (struct)
//       - Holds two optional channel receivers:
//           • gen_rx: receives heavy generation results (chunks, progress).
//           • anim_rx: receives lightweight animation updates (flow fields, particles).
//       - Both receivers are wrapped in `Option<T>` to allow flexible initialization
//         and safe disabling when channels disconnect.
//
//   • Default Implementation
//       - Provides an empty `AsyncPoller` with no channels set.
//       - Useful for initializing before wiring channels during runtime setup.
//
//   • Methods
//       - set_generation_receiver
//           • Assigns the receiver for generation messages.
//           • Called during initialization when channels are created.
//       - set_animation_receiver
//           • Assigns the receiver for animation updates.
//           • Called during initialization when channels are created.
//       - poll_generation_messages
//           • Drains all available generation messages using `try_recv()`.
//           • Collects messages into a `VecDeque` for batch processing.
//           • If channel disconnects, logs a warning and disables receiver.
//       - poll_animations
//           • Drains all available animation updates using `try_recv()`.
//           • Collects updates into a `VecDeque` for batch processing.
//           • If channel disconnects, logs a warning and disables receiver.
//
// Design Choices:
//   • Non-blocking polling ensures the engine remains responsive.
//   • `VecDeque` provides efficient push/pop operations for batch collection.
//   • Logging (`warn!`) provides visibility into unexpected channel disconnections.
//   • Optional receivers (`Option<T>`) allow safe handling of missing or disconnected channels.
//
// Educational Note:
//   • This module demonstrates how Rust’s async channels can be integrated into
//     a polling system that avoids blocking the runtime. By combining `try_recv()`,
//     structured logging, and safe handling of disconnections, `AsyncPoller` provides
//     a robust foundation for managing asynchronous communication in game engines.
// ============================================================================


use tokio::sync::mpsc::Receiver as TokioReceiver;
use ssxl_generate::task_queue::GenerationMessage; 
use ssxl_shared::message::messages::AnimationUpdate; 
use std::collections::VecDeque;
use tracing::warn; 
use tokio::sync::mpsc::error::TryRecvError;

pub struct AsyncPoller {
    gen_rx: Option<TokioReceiver<GenerationMessage>>, 
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
    pub fn set_generation_receiver(&mut self, rx: Option<TokioReceiver<GenerationMessage>>) {
        self.gen_rx = rx;
    }

    pub fn set_animation_receiver(&mut self, rx: Option<TokioReceiver<AnimationUpdate>>) {
        self.anim_rx = rx;
    }

    pub fn poll_generation_messages(&mut self) -> VecDeque<GenerationMessage> {
        let mut messages = VecDeque::new();
        if let Some(rx) = &mut self.gen_rx {
            loop {
                match rx.try_recv() {
                    Ok(msg) => messages.push_back(msg),
                    Err(TryRecvError::Empty) => break, 
                    Err(TryRecvError::Disconnected) => {
                        warn!("Generation channel disconnected.");
                        self.gen_rx = None;
                        break;
                    }
                }
            }
        }
        messages
    }

    pub fn poll_animations(&mut self) -> VecDeque<AnimationUpdate> {
        let mut updates = VecDeque::new();
        if let Some(rx) = &mut self.anim_rx {
            loop {
                match rx.try_recv() {
                    Ok(update) => updates.push_back(update),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        warn!("Animation channel disconnected.");
                        self.anim_rx = None;
                        break;
                    }
                }
            }
        }
        updates
    }
}
