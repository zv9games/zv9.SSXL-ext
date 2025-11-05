use std::sync::{Arc, Mutex};
use tracing::{info, error};

// Internal Crate Dependencies
use ssxl_generate::{
    Conductor,
    ConductorProgressReceiver,
    ConductorRequestSender,
};
use ssxl_generate::conductor_state::ConductorState; // Corrected import path

use ssxl_sync::{
    AnimationConductor,
    AnimationConductorHandle,
    AnimationReceiver,
    AnimationCommand,
};
use ssxl_sync::primitives::AnimationState; // Corrected import path (assuming primitives)


/// Responsible for initializing and managing the lifetime of the core Rust
/// worker threads (`Conductor` and `AnimationConductor`).
#[derive(Debug, Default)]
pub struct EngineInitializer {}

impl EngineInitializer {
    pub fn new() -> Self {
        EngineInitializer {}
    }

    /// Ensures the Conductor runtime is initialized and returns the thread-safe
    /// handle, the generation channel receiver, and the **ConductorState**.
    ///
    /// Returns: (Arc<Mutex<Conductor>>, ConductorProgressReceiver, ConductorState)
    pub fn ensure_conductor(
        &self
    ) -> (
        Option<Arc<Mutex<Conductor>>>,
        Option<ConductorProgressReceiver>,
        Option<ConductorState>
    ) {
        info!("EngineInitializer: Attempting to initialize Conductor...");
        
        // Conductor::new returns: (Conductor, ConductorState, ProgressReceiver, RequestSender).
        match Conductor::new(None) {
            Ok((conductor, state, gen_rx, _request_tx)) => {
                info!("Conductor initialized and background thread started successfully.");
                (Some(Arc::new(Mutex::new(conductor))), Some(gen_rx), Some(state)) 
            }
            Err(e) => {
                error!("Failed to initialize Conductor: {}", e);
                (None, None, None)
            }
        }
    }

    /// Ensures the AnimationConductor runtime is initialized and returns the
    /// thread-safe handle, the animation channel receiver, and the **AnimationState**.
    ///
    /// Returns: (AnimationConductorHandle, AnimationReceiver, AnimationState)
    pub fn ensure_animation_conductor(
        &self
    ) -> (
        Option<AnimationConductorHandle>,
        Option<AnimationReceiver>,
        Option<AnimationState>
    ) {
        info!("EngineInitializer: Attempting to initialize AnimationConductor...");
        
        // AnimationConductor::new() returns: (ConductorStruct, AnimationReceiver, AnimationState).
        let Ok((conductor_struct, anim_rx_found, anim_state_found)) = AnimationConductor::new() else {
            error!("Failed to initialize AnimationConductor.");
            return (None, None, None);
        };
        
        info!("AnimationConductor initialized and thread started successfully.");
        
        (
            Some(conductor_struct.get_command_sender()),
            Some(anim_rx_found),
            Some(anim_state_found)
        )
    }

    /// Gracefully shuts down both conductors and ensures worker threads join.
    pub fn shutdown(
        &self,
        mut anim_handle: Option<AnimationConductorHandle>,
        mut conductor_arc: Option<Arc<Mutex<Conductor>>>,
    ) {
        info!("EngineInitializer: Starting graceful shutdown process...");
        
        // 1. Shutdown the Generation Conductor
        if let Some(arc) = conductor_arc.take() {
            match Arc::try_unwrap(arc) {
                Ok(mutex) => {
                    info!("Shutting down Conductor...");
                    mutex.into_inner().unwrap().graceful_teardown();
                    info!("Conductor shutdown complete.");
                }
                Err(_) => {
                    error!("Could not unwrap Conductor Arc; other references may exist.");
                }
            }
        }

        // 2. Shutdown the Animation Conductor
        if let Some(handle) = anim_handle.take() {
            // ✅ FIX: Using AnimationCommand::Stop for clearer graceful shutdown.
            match handle.send(AnimationCommand::Stop) { 
                Ok(_) => info!("AnimationConductor shutdown command sent successfully."),
                Err(e) => error!("Failed to send shutdown command to AnimationConductor: {}", e),
            }
            info!("AnimationConductor shutdown complete.");
        }
        
        info!("EngineInitializer: All background runtimes terminated.");
    }
}