// ssxl_sync/src/animation_conductor.rs (Type Resolution Fix)

use ssxl_shared::{
    AnimationConductorHandle,
    AnimationState,
    AnimationCommand, 
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::info;

// FIX: Import the exact nested type required by the CoreAnimationWorker's signature 
// and alias it as `AnimationUpdate`. This resolves the E0308 type mismatch.
use ssxl_shared::message::messages::AnimationUpdate as AnimationUpdate; 

// NOTE: We rely on the core worker being defined as `conductor::AnimationConductor` within ssxl_animate.
use ssxl_animate::conductor::AnimationConductor as CoreAnimationWorker;

// -----------------------------------------------------------------------------
// 1. Internal Setup Struct (for passing state between FFI stages)
// -----------------------------------------------------------------------------

/// Holds all the necessary internal channels and initial state required to spawn the
/// heavy, background Animation Conductor thread.
pub struct AnimationConductorInternalSetup {
    pub initial_state: AnimationState,
    /// The receiver for Godot commands, which the worker thread will consume.
    pub command_receiver: UnboundedReceiver<AnimationCommand>,
    
    /// The sender for updates, which the worker thread will use.
    // This field now holds the correctly nested type due to the aliased import above.
    pub update_sender: UnboundedSender<AnimationUpdate>,
}

// -----------------------------------------------------------------------------
// 2. The Public Conductor Struct (The FFI-facing worker wrapper)
// -----------------------------------------------------------------------------

/// The SSXL Animation Conductor. This struct is responsible for executing the
/// animation logic in a background thread.
pub struct AnimationConductor {}

impl AnimationConductor {
    // -------------------------------------------------------------------------
    // Stage 1: FAST Channel and State Setup (Non-blocking)
    // -------------------------------------------------------------------------
    
    /// Creates all the necessary MPSC channels and the initial `AnimationState`.
    /// 
    /// # Returns
    /// A tuple containing:
    /// 1. `AnimationConductorInternalSetup`: The struct holding internal handles for spawning.
    /// 2. `AnimationConductorHandle`: The public command sender handle exposed to Godot's FFI layer.
    /// 3. `UnboundedReceiver<AnimationUpdate>`: The public update receiver handle exposed to Godot's Poller.
    pub fn setup_channels_and_state() -> (AnimationConductorInternalSetup, AnimationConductorHandle, UnboundedReceiver<AnimationUpdate>) {
        info!("Animation Conductor: Starting FAST Channel and State Setup.");

        // Channels for commands from Godot to the worker.
        let (command_tx, command_rx) = mpsc::unbounded_channel::<AnimationCommand>();
        
        // Channels for updates from the worker to Godot (Poller).
        // This channel uses the correctly aliased type.
        let (update_tx, update_rx) = mpsc::unbounded_channel::<AnimationUpdate>();
        
        let initial_state = AnimationState::default();

        let internal_setup = AnimationConductorInternalSetup {
            initial_state,
            command_receiver: command_rx,
            update_sender: update_tx,
        };
        
        let public_command_handle = command_tx;

        (internal_setup, public_command_handle, update_rx)
    }

    // -------------------------------------------------------------------------
    // Stage 2: HEAVY Thread Spawn
    // -------------------------------------------------------------------------

    /// Consumes the setup handles and spawns the core animation worker thread.
    /// 
    /// # Arguments
    /// * `setup`: The internal channels and initial state from the setup stage.
    /// 
    /// # Returns
    /// A new `AnimationConductor` instance representing the running worker.
    pub fn new(setup: AnimationConductorInternalSetup) -> Self {
        info!("Animation Conductor: Spawning background worker thread.");

        // This call is now correct because `setup.update_sender` holds the 
        // type the `CoreAnimationWorker::new` function requires.
        let _core_worker = CoreAnimationWorker::new(
            setup.command_receiver,
            setup.update_sender,
            setup.initial_state,
        );
        
        info!("Animation Conductor: Worker thread started successfully.");

        // Return the opaque public handle struct
        AnimationConductor {} 
    }
}