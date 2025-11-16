//! # Godot Engine Initializers (`ssxl_godot::api_initializers`)
//!
//! Manages the initialization and graceful shutdown of the two core background
//! asynchronous systems: the **Generation Conductor** and the **Animation Conductor**.
//! This is the FFI layer's entry point for starting the Rust engine runtime.

use std::sync::{Arc, Mutex};
use tracing::{info, error};

// --- Standard Library Imports for Concurrency ---
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

// --- Imports from ssxl_generate ---
use ssxl_generate::{
    Conductor,
    ConductorProgressReceiver,
};
use ssxl_generate::conductor_state::ConductorState;

// --- Imports from ssxl_sync ---
use ssxl_sync::{
    AnimationConductor,
    AnimationConductorHandle as CoreAnimationConductorHandle, // Use alias for clarity
};

// --- Imports from ssxl_shared ---
use ssxl_shared::{
    messages::AnimationCommand,
    AnimationState,
    messages::AnimationUpdate as CoreAnimationUpdate,
};

// -----------------------------------------------------------------------------
// Type Aliases
// -----------------------------------------------------------------------------

/// Type alias for the `AnimationConductor` command sender handle.
// FIX: Removed generic argument <AnimationCommand> as the original type alias is already concrete.
pub type AnimationConductorHandle = CoreAnimationConductorHandle; 

/// Type alias for the `AnimationConductor` update receiver.
pub type AnimationUpdateReceiver = UnboundedReceiver<CoreAnimationUpdate>;


// -----------------------------------------------------------------------------
// EngineInitializer Struct and Implementation
// -----------------------------------------------------------------------------

/// Responsible for initializing and managing the lifecycle of the entire SSXL Engine runtime.
#[derive(Debug, Default)]
pub struct EngineInitializer {}

impl EngineInitializer {
    /// Creates a new, default instance of the initializer.
    pub fn new() -> Self {
        Self {}
    }

    // -------------------------------------------------------------------------
    // 1. Generation Conductor Initialization
    // -------------------------------------------------------------------------

    /// Initializes the main world generation system (`Conductor`).
    pub fn ensure_conductor(
        &self
    ) -> (
        // Thread-safe handle to the Conductor struct itself.
        Option<Arc<Mutex<Conductor>>>,
        // Receiver for generated chunk data and progress messages.
        Option<ConductorProgressReceiver>,
        // The thread-safe state tracker for the generation pipeline.
        Option<ConductorState>
    ) {
        info!("EngineInitializer: Attempting to initialize Conductor...");

        // Conductor::new starts the Tokio runtime and the main request loop.
        match Conductor::new(None) {
            Ok((conductor, state, gen_rx, _request_tx)) => {
                info!("Conductor initialized and background thread started successfully.");
                // We return the Conductor in a Mutex/Arc for thread-safe access from Godot.
                (Some(Arc::new(Mutex::new(conductor))), Some(gen_rx), Some(state))
            }
            Err(e) => {
                error!("Failed to initialize Conductor: {}", e);
                (None, None, None)
            }
        }
    }

    // -------------------------------------------------------------------------
    // 2. Animation Conductor Initialization
    // -------------------------------------------------------------------------

    /// Initializes the dedicated animation processing system (`AnimationConductor`).
    pub fn ensure_animation_conductor(
        &self
    ) -> (
        // Sender for control commands (e.g., Start, Stop, UpdateFramerate).
        Option<AnimationConductorHandle>,
        // Receiver for real-time animation updates.
        Option<AnimationUpdateReceiver>,
        // The thread-safe state tracker for the animation pipeline.
        Option<AnimationState>
    ) {
        info!("EngineInitializer: Attempting to initialize AnimationConductor...");

        // Create the communication channels.
        let (anim_tx, anim_rx) = mpsc::unbounded_channel::<AnimationCommand>();
        let (update_tx, update_rx) = mpsc::unbounded_channel::<CoreAnimationUpdate>();
        let anim_state = AnimationState::default();

        // Clone the AnimationState to pass to the background thread while retaining the original.
        let state_to_pass = anim_state.clone();

        // Start the background AnimationConductor thread.
        let _conductor = AnimationConductor::new(anim_rx, update_tx, state_to_pass);

        info!("AnimationConductor initialized and thread started successfully.");
        (
            // Return the sender handle for commands
            Some(anim_tx),
            // Return the receiver for tile updates, using the new alias
            Some(update_rx),
            // Return the original state tracker
            Some(anim_state)
        )
    }

    // -------------------------------------------------------------------------
    // 3. Core Setup Orchestration
    // -------------------------------------------------------------------------

    /// Orchestrates the setup for both the Generation and Animation cores.
    pub fn execute_core_setup(
        &self
    ) -> (
        // Conductor Handles
        Option<Arc<Mutex<Conductor>>>,
        Option<ConductorProgressReceiver>,
        Option<ConductorState>,
        // Animation Handles
        Option<AnimationConductorHandle>,
        Option<AnimationUpdateReceiver>,
        Option<AnimationState>,
    ) {
        let (c, grx, gs) = self.ensure_conductor();
        let (ah, arx, as_) = self.ensure_animation_conductor();

        (c, grx, gs, ah, arx, as_)
    }

    // -------------------------------------------------------------------------
    // 4. Graceful Shutdown
    // -------------------------------------------------------------------------

    /// Performs a **graceful shutdown** of both background conductors.
    pub fn shutdown(
        &self,
        mut anim_handle: Option<AnimationConductorHandle>,
        mut conductor_arc: Option<Arc<Mutex<Conductor>>>,
    ) {
        info!("EngineInitializer: Starting graceful shutdown process...");

        // 1. Shut down Generation Conductor (requires unique ownership)
        if let Some(arc) = conductor_arc.take() {
            // Attempt to unwrap the Arc to ensure we have the *only* reference.
            match Arc::try_unwrap(arc) {
                Ok(mutex) => {
                    info!("Shutting down Conductor...");
                    match mutex.into_inner() {
                        Ok(conductor) => {
                            conductor.graceful_teardown();
                            info!("Conductor shutdown complete.");
                        }
                        Err(e) => {
                            error!("Failed to unwrap Conductor Mutex (poisoned): {:?}", e);
                        }
                    }
                }
                Err(_) => {
                    error!("Could not unwrap Conductor Arc; other references may exist. Conductor may leak resources.");
                }
            }
        }

        // 2. Shut down Animation Conductor (by sending a command)
        if let Some(handle) = anim_handle.take() {
            // Send a Shutdown command to the animation thread's receiver.
            match handle.send(AnimationCommand::Shutdown) {
                Ok(_) => info!("AnimationConductor shutdown command sent successfully."),
                Err(e) => error!("Failed to send shutdown command to AnimationConductor: {}", e),
            }
        } else {
            info!("AnimationConductor handle was already consumed or not initialized.");
        }

        info!("EngineInitializer: All background runtimes terminated.");
    }
}