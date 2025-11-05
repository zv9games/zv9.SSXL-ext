// ssxl_godot/src/api_initializers.rs

use std::sync::{Arc, Mutex};
use tracing::{info, error};

// Internal Crate Dependencies
use ssxl_generate::{
    Conductor, 
    ConductorProgressReceiver, 
    ConductorRequestSender
};
use ssxl_sync::{
    AnimationConductor, 
    AnimationConductorHandle, 
    AnimationReceiver,
    // We import AnimationCommand to use it in the shutdown logic
    AnimationCommand 
};


/// Responsible for initializing and managing the lifetime of the core Rust
/// worker threads (`Conductor` and `AnimationConductor`).
#[derive(Debug, Default)]
pub struct EngineInitializer {}

impl EngineInitializer {
    pub fn new() -> Self {
        EngineInitializer {}
    }

    /// Ensures the Conductor runtime is initialized and returns the thread-safe
    /// handle and the generation channel receiver.
    /// 
    /// Returns: (Arc<Mutex<Conductor>>, ConductorProgressReceiver)
    pub fn ensure_conductor(
        &self
    ) -> (Option<Arc<Mutex<Conductor>>>, Option<ConductorProgressReceiver>) {
        info!("EngineInitializer: Attempting to initialize Conductor...");
        
        // Conductor::new returns an Ok((Conductor, ConductorState, ProgressReceiver, RequestSender)) tuple.
        match Conductor::new(None) {
            // ✅ FIX 1: The ProgressReceiver (gen_rx) is the third element.
            // The argument order here matches the Conductor::new return order.
            Ok((conductor, _state, gen_rx, _gen_tx)) => {
                info!("Conductor initialized and background thread started successfully.");
                (Some(Arc::new(Mutex::new(conductor))), Some(gen_rx))
            }
            Err(e) => {
                error!("Failed to initialize Conductor: {}", e);
                (None, None)
            }
        }
    }

    /// Ensures the AnimationConductor runtime is initialized and returns the 
    /// thread-safe handle and the animation channel receiver.
    /// 
    /// Returns: (AnimationConductorHandle, AnimationReceiver)
    pub fn ensure_animation_conductor(
        &self
    ) -> (Option<AnimationConductorHandle>, Option<AnimationReceiver>) {
        info!("EngineInitializer: Attempting to initialize AnimationConductor...");
        
        // AnimationConductor::new() returns: Result<(AnimationConductor (Handle), AnimationReceiver), String>
        // The destructuring must match the return order.
        let Ok((handle_found, anim_rx_found)) = AnimationConductor::new() else {
            error!("Failed to initialize AnimationConductor.");
            return (None, None);
        };
        
        info!("AnimationConductor initialized and thread started successfully.");
        
        // ✅ FIX 2 & 3: Return the values in the order of the function signature: (Handle, Receiver).
        (Some(handle_found), Some(anim_rx_found))
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
                    // This method name is confirmed by conductor.rs
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
            // ✅ FIX 4: Send the shutdown command using the correct method.
            match handle.send_command(AnimationCommand::Complete) {
                Ok(_) => info!("AnimationConductor shutdown command sent successfully."),
                Err(e) => error!("Failed to send shutdown command to AnimationConductor: {}", e),
            }
            info!("AnimationConductor shutdown complete.");
        }
        
        info!("EngineInitializer: All background runtimes terminated.");
    }
}