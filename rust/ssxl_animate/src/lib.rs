// ============================================================================
// 🎼 SSXL Animation Conductor Initialization (`ssxl_animate`)
// ----------------------------------------------------------------------------
// This module wires up the animation conductor system, which orchestrates
// animation commands and updates across the engine. It defines the conductor’s
// behavior contract, initializes communication channels, and exposes the
// conductor for safe, concurrent use.
//
// Key Concepts:
//   • AnimationConductor:
//       - The central manager that listens for incoming animation commands,
//         updates internal state, and delegates heavy work to worker threads.
//   • ConductorBehavior trait:
//       - Defines the contract for any conductor implementation.
//       - Requires three core behaviors:
//           1. start_loop: async event loop that continuously processes commands.
//           2. process_command: handles a single command (delegates work or updates state).
//           3. get_state: returns the current conductor state for inspection.
//   • initialize_animation_conductor function:
//       - Sets up the conductor and returns two handles:
//           1. AnimationConductorHandle: external handle used by Godot/FFI to send commands.
//           2. Arc<Mutex<AnimationConductor>>: thread-safe wrapper around the conductor itself.
//       - Flow:
//           a. Log initialization.
//           b. Create an unbounded command channel (tx/rx).
//           c. Wrap the sender in the alias handle.
//           d. Construct the conductor with receiver, update sender, and initial state.
//           e. Wrap the conductor in Arc<Mutex> for safe sharing across threads.
//
// Design Choices:
//   • Tokio’s unbounded channels provide non-blocking communication between
//     Godot and Rust workers.
//   • Arc<Mutex> ensures safe concurrent access to the conductor without
//     sacrificing simplicity.
//   • async_trait allows async functions inside traits, making the conductor’s
//     event loop ergonomic and idiomatic.
//   • Logging via `tracing::info` provides visibility into initialization and
//     runtime behavior.
//
// Educational Note:
//   • This module demonstrates a common orchestration pattern in game engines:
//       - A lightweight conductor manages state and communication.
//       - Heavy computation is delegated to worker threads.
//       - Safe concurrency primitives (Arc, Mutex, channels) ensure correctness.
//   • By centralizing initialization here, the engine guarantees that all
//     animation subsystems share a consistent, thread-safe conductor instance.
// ============================================================================


use tracing::info;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use tokio::sync::mpsc;

use ssxl_shared::{
    AnimationCommand,
    UpdateSender,
    AnimationConductorHandle,
    AnimationState,
    CommandResult,
};

pub mod conductor;
mod worker;
mod animation_logic;

pub use conductor::AnimationConductor;

#[async_trait]
pub trait ConductorBehavior: Send + Sync + 'static {
    async fn start_loop(&mut self);

    fn process_command(&mut self, command: AnimationCommand) -> CommandResult;

    fn get_state(&self) -> AnimationState;
}

pub fn initialize_animation_conductor(
    update_tx: UpdateSender,
    initial_state: AnimationState,
) -> (AnimationConductorHandle, Arc<Mutex<AnimationConductor>>) {
    info!("ssxl_animate: Initializing Animation Conductor circuit.");
    
    let (command_tx, command_rx) = mpsc::unbounded_channel();
    
    let handle: AnimationConductorHandle = command_tx; 

    let conductor = AnimationConductor::new(
        command_rx, 
        update_tx,
        initial_state,
    );

    (handle, Arc::new(Mutex::new(conductor)))
}
