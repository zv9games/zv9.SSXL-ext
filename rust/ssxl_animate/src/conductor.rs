// rust/ssxl_animate/src/conductor.rs

use crate::{ConductorBehavior, AnimationCommand, CommandResult, AnimationState, UpdateSender};
use crate::worker::process_command_parallel; // CRITICAL: Import the delegation function
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use godot::prelude::godot_print; // Assuming godot_print is available for logging

/// The core, single-threaded struct responsible for managing all animation workers.
/// It holds the Receiver for commands and the Sender for updates to the Godot main thread.
pub struct AnimationConductor {
    // The Receiver side of the command channel
    command_rx: UnboundedReceiver<AnimationCommand>,
    // This Sender is USED to pass to the worker.
    update_tx: UpdateSender, 
    state: AnimationState,
}

impl AnimationConductor {
    pub fn new(
        command_rx: UnboundedReceiver<AnimationCommand>,
        update_tx: UpdateSender,
        initial_state: AnimationState,
    ) -> Self {
        AnimationConductor {
            command_rx,
            update_tx,
            state: initial_state,
        }
    }

    /// Synchronously stops the animation by immediately updating the internal state.
    ///
    /// This method is designed to be called when the Conductor is locked via a Mutex.
    /// It functions as the **animation equivalent** of the `stop_generation` command.
    /// It relies on the external `AnimationState` struct having a `set_enabled(bool)` method.
    pub fn stop_animation(&mut self) -> CommandResult {
        self.state.set_enabled(false);
        godot_print!("Animation Conductor: Synchronous stop command received. State set to disabled.");
        Ok(())
    }
}

#[async_trait]
impl ConductorBehavior for AnimationConductor {
    async fn start_loop(&mut self) {
        // The main event loop for the Conductor. This loop manages the **tempo**.
        // It awaits a command, then immediately processes it (delegates) or acts on it (state change).
        while let Some(command) = self.command_rx.recv().await {
            let _ = self.process_command(command);
        }
    }

    /// Processes a command, delegating heavy computation to the worker pool.
    /// This function MUST return quickly to keep the Conductor's tempo fast.
    fn process_command(&mut self, command: AnimationCommand) -> CommandResult {
        match command {
            // ----------------------------------------------------
            // 1. Delegate High-Performance Work (Tile Updates)
            // ----------------------------------------------------
            AnimationCommand::AnimateChunkSet { .. } | AnimationCommand::StartTestAnimation => {
                // CRITICAL OPTIMIZATION: Delegate work and clone the sender for the worker
                process_command_parallel(command, self.update_tx.clone());
                Ok(())
            }
            // ----------------------------------------------------
            // 2. Local State Management (FIX E0004)
            // ----------------------------------------------------
            AnimationCommand::SetTimeScale(scale) => {
                // NOTE: This state update is safe because it only runs on the async Conductor thread.
                self.state.set_time_scale(scale); 
                Ok(())
            }
            // FIX E0004: Handle the new SetEnabled command
            AnimationCommand::SetEnabled(enabled) => {
                // NOTE: Requires `set_enabled(bool)` to be implemented on `AnimationState`.
                // This updates the local state which can be queried by `get_state()`.
                self.state.set_enabled(enabled);
                godot_print!("Animation Conductor: is_enabled set to {}", enabled);
                Ok(())
            }
            // ----------------------------------------------------
            // 3. System Commands
            // ----------------------------------------------------
            AnimationCommand::Shutdown => {
                // Close the receiver, which will gracefully exit the start_loop
                self.command_rx.close();
                Ok(())
            }
        }
    }

    fn get_state(&self) -> AnimationState {
        self.state.clone()
    }
}