// rust/ssxl_animate/src/conductor.rs (Optimized: Zero Entropy Command Flow)

use crate::{ConductorBehavior, AnimationCommand, CommandResult, AnimationState, UpdateSender};
use crate::worker::process_command_parallel; 
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use godot::prelude::godot_print; 

/// The core, single-threaded struct responsible for managing all animation workers.
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

    // DELETED: fn stop_animation(&mut self) -> CommandResult 
    // RATIONALE: Redundant state modification API. All state changes MUST flow through 
    // the AnimationCommand::SetEnabled(false) in the process_command match arm 
    // for guaranteed sequential processing (Zero Entropy).
}

#[async_trait]
impl ConductorBehavior for AnimationConductor {
    async fn start_loop(&mut self) {
        // The main event loop for the Conductor. This loop manages the **tempo**.
        while let Some(command) = self.command_rx.recv().await {
            let _ = self.process_command(command);
        }
    }

    /// Processes a command, delegating heavy computation to the worker pool.
    /// This function MUST return quickly to keep the Conductor's tempo fast.
    fn process_command(&mut self, command: AnimationCommand) -> CommandResult {
        match command {
            // ----------------------------------------------------
            // 1. Delegate High-Performance Work
            // ----------------------------------------------------
            AnimationCommand::AnimateChunkSet { .. } | AnimationCommand::StartTestAnimation => {
                // CRITICAL OPTIMIZATION: Delegate work and clone the sender for the worker
                process_command_parallel(command, self.update_tx.clone());
                Ok(())
            }
            // ----------------------------------------------------
            // 2. Local State Management (Zero-Entropy Control)
            // ----------------------------------------------------
            AnimationCommand::SetTimeScale(scale) => {
                self.state.set_time_scale(scale); 
                Ok(())
            }
            AnimationCommand::SetEnabled(enabled) => {
                // SINGLE ENTRY POINT for state control.
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