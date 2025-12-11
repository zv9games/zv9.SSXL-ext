// ============================================================================
// 🎬 Animation Conductor (FFI Worker Wrapper)
// File: ssxl_sync/src/animation_conductor.rs
// ----------------------------------------------------------------------------
// This module defines the SSXL Animation Conductor, which is responsible for
// coordinating animation logic in a background thread. It acts as the bridge
// between the Godot runtime (via FFI) and the Rust engine’s animation system.
//
// Key responsibilities:
//   • Set up non-blocking channels for communication between Godot and Rust.
//   • Maintain initial animation state for the worker thread.
//   • Spawn the heavy background worker (`CoreAnimationWorker`) defined in
//     the `ssxl_animate` crate.
//   • Resolve type mismatches by aliasing the exact nested types required.
//
// Educational notes:
//   • Rust’s `tokio::sync::mpsc` channels are used for async, multi-producer,
//     single-consumer communication. Here, they connect Godot commands to the
//     worker and worker updates back to Godot.
//   • Aliasing `AnimationUpdate` ensures type signatures match exactly across
//     crate boundaries, fixing compiler errors like E0308.
//   • Splitting into two stages (FAST setup vs HEAVY spawn) separates lightweight
//     channel creation from expensive thread spawning.
// ============================================================================

use ssxl_shared::{
    AnimationConductorHandle, // Public handle for sending commands to the conductor
    AnimationState,           // Global animation state struct
    AnimationCommand,         // Enum of animation commands (start, stop, etc.)
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender}; // Async channels
use tracing::info; // Structured logging

// FIX: Import the exact nested type required by CoreAnimationWorker.
// Aliased as `AnimationUpdate` to resolve type mismatch errors (E0308).
use ssxl_shared::message::messages::AnimationUpdate as AnimationUpdate; 

// NOTE: The core worker implementation lives in the `ssxl_animate` crate.
// We alias it here for clarity and to emphasize the separation of concerns.
use ssxl_animate::conductor::AnimationConductor as CoreAnimationWorker;

// -----------------------------------------------------------------------------
// 1. Internal Setup Struct
// -----------------------------------------------------------------------------
// This struct holds the internal channels and initial state required to spawn
// the background worker. It is not exposed directly to FFI consumers; instead,
// it is used internally during setup.
// -----------------------------------------------------------------------------
pub struct AnimationConductorInternalSetup {
    pub initial_state: AnimationState,                  // Starting animation state
    pub command_receiver: UnboundedReceiver<AnimationCommand>, // Commands from Godot
    pub update_sender: UnboundedSender<AnimationUpdate>,       // Updates back to Godot
}

// -----------------------------------------------------------------------------
// 2. Public Conductor Struct
// -----------------------------------------------------------------------------
// This is the FFI-facing wrapper. It exposes safe methods for setting up
// channels and spawning the background worker thread.
// -----------------------------------------------------------------------------
pub struct AnimationConductor {}

impl AnimationConductor {
    // -------------------------------------------------------------------------
    // Stage 1: FAST Channel and State Setup
    // -------------------------------------------------------------------------
    // Creates the necessary channels and initializes the animation state.
    // This stage is lightweight and non-blocking.
    //
    // Returns:
    //   1. AnimationConductorInternalSetup: internal handles for spawning.
    //   2. AnimationConductorHandle: public command sender for Godot.
    //   3. UnboundedReceiver<AnimationUpdate>: public update receiver for Godot.
    // -------------------------------------------------------------------------
    pub fn setup_channels_and_state() -> (
        AnimationConductorInternalSetup,
        AnimationConductorHandle,
        UnboundedReceiver<AnimationUpdate>,
    ) {
        info!("Animation Conductor: Starting FAST Channel and State Setup.");

        // Channel for commands: Godot → worker
        let (command_tx, command_rx) = mpsc::unbounded_channel::<AnimationCommand>();
        
        // Channel for updates: worker → Godot
        let (update_tx, update_rx) = mpsc::unbounded_channel::<AnimationUpdate>();
        
        // Initialize animation state to defaults
        let initial_state = AnimationState::default();

        // Bundle internal setup handles
        let internal_setup = AnimationConductorInternalSetup {
            initial_state,
            command_receiver: command_rx,
            update_sender: update_tx,
        };
        
        // Public handle for sending commands into the conductor
        let public_command_handle: AnimationConductorHandle = command_tx;

        (internal_setup, public_command_handle, update_rx)
    }

    // -------------------------------------------------------------------------
    // Stage 2: HEAVY Thread Spawn
    // -------------------------------------------------------------------------
    // Consumes the setup handles and spawns the background worker thread.
    // This stage is heavier because it involves thread creation and worker
    // initialization.
    //
    // Arguments:
    //   • setup: internal channels and initial state from Stage 1.
    //
    // Returns:
    //   • AnimationConductor: opaque public handle representing the running worker.
    // -------------------------------------------------------------------------
    pub fn new(setup: AnimationConductorInternalSetup) -> Self {
        info!("Animation Conductor: Spawning background worker thread.");

        // Spawn the core worker with the correct channel types and initial state.
        let _core_worker = CoreAnimationWorker::new(
            setup.command_receiver,
            setup.update_sender,
            setup.initial_state,
        );
        
        info!("Animation Conductor: Worker thread started successfully.");

        // Return the public wrapper
        AnimationConductor {} 
    }
}
