//! # ssxl_animate
//!
//! This crate contains the **Animation Conductor** and associated logic responsible for
//! managing all real-time visual updates, tweens, and complex tile animations for the
//! SSXL-ext engine. It operates asynchronously, feeding updates back to the Godot
//! main thread via non-blocking communication channels.

// --- Standard Library & External Crates ---
use tracing::info;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use tokio::sync::mpsc; // CRITICAL: Used for the asynchronous, non-blocking channel

// --- SSXL Internal Crates (Contracts and Types) ---
// FIX: All contracts, including the FFI handles, are now imported from ssxl_shared.
use ssxl_shared::{
    AnimationCommand,
    UpdateSender,
    // Removed unused import: AnimationType,
    AnimationConductorHandle, // Now resolved from ssxl_shared/src/lib.rs re-export
    AnimationState,           // Now resolved from ssxl_shared/src/lib.rs re-export
    CommandResult,            // Now resolved from ssxl_shared/src/lib.rs re-export
};

// --- Internal Modules ---
pub mod conductor;
mod worker;
mod animation_logic;


// -----------------------------------------------------------------------------
// Core Public API
// -----------------------------------------------------------------------------

pub use conductor::AnimationConductor;

/// Defines the primary trait for the Animation Conductor.
/// This trait enforces the necessary async structure for the runtime.
#[async_trait]
pub trait ConductorBehavior: Send + Sync + 'static {
    async fn start_loop(&mut self);

    fn process_command(&mut self, command: AnimationCommand) -> CommandResult;

    fn get_state(&self) -> AnimationState;
}

/// Initializes and returns the necessary components for the Animation Conductor.
///
/// Returns: (Command Handle, Arc<Mutex<Conductor>>)
/// The Handle is sent to the Godot FFI layer (`ssxl_sync`) to send commands.
pub fn initialize_animation_conductor(
    update_tx: UpdateSender,
    initial_state: AnimationState,
) -> (AnimationConductorHandle, Arc<Mutex<AnimationConductor>>) {
    info!("ssxl_animate: Initializing Animation Conductor circuit.");
    
    // Wire the command channel: The primary control line
    let (command_tx, command_rx) = mpsc::unbounded_channel();
    
    let handle: AnimationConductorHandle = command_tx; 

    // Create the core conductor structure which owns the Receiver (command_rx).
    let conductor = AnimationConductor::new(
        command_rx, 
        update_tx,
        initial_state,
    );

    // Conductor is wrapped in Arc<Mutex> for safe shutdown/runtime management.
    (handle, Arc::new(Mutex::new(conductor)))
}