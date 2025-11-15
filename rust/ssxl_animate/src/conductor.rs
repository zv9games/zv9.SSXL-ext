// rust/ssxl_animate/src/conductor.rs (Updated)
use crate::{ConductorBehavior, AnimationCommand, CommandResult, AnimationState, UpdateSender};
use crate::worker::process_command_parallel; // CRITICAL: Import the delegation function
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

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
            // FIX: Renamed the command variant from AnimateChunks to AnimateChunkSet.
            AnimationCommand::AnimateChunkSet { .. } | AnimationCommand::StartTestAnimation => {
                // CRITICAL OPTIMIZATION: Delegate work and clone the sender for the worker
                // This call SUBMITS the job to a parallel thread/task and returns immediately.
                process_command_parallel(command, self.update_tx.clone());
                Ok(())
            }
            // ----------------------------------------------------
            // 2. Local State Management
            // ----------------------------------------------------
            AnimationCommand::SetTimeScale(scale) => {
                // NOTE: This state update is safe because it only runs on the async Conductor thread.
                // The `set_time_scale` method is now defined on AnimationState in ssxl_shared/messages.rs.
                self.state.set_time_scale(scale); 
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
            // The catch-all is now removed because the match is exhaustive.
        }
    }

    fn get_state(&self) -> AnimationState {
        self.state.clone()
    }
}