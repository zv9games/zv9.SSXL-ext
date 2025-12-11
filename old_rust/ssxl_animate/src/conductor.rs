// ============================================================================
// 🎼 Animation Conductor (`ssxl_animate::conductor`)
// ----------------------------------------------------------------------------
// This module defines the `AnimationConductor`, the central manager for
// animation commands in the SSXL engine. It acts as the runtime "orchestra
// conductor," listening for incoming commands, updating internal state, and
// delegating heavy work to parallel worker threads.
//
// Key Concepts:
//   • AnimationConductor struct:
//       - Holds the command receiver (from Godot/FFI).
//       - Holds the update sender (to Godot/FFI).
//       - Maintains internal animation state (enabled flag, time scale).
//   • ConductorBehavior trait:
//       - Defines the lifecycle of a conductor: start loop, process commands,
//         and expose current state.
//   • process_command_parallel:
//       - Offloads CPU-intensive animation work to worker threads, keeping
//         the conductor’s main loop responsive.
//   • godot_print:
//       - Provides visibility into state changes directly in Godot’s console.
//
// Workflow:
//   1. Construction (`new`):
//      - Initializes the conductor with a command channel, update channel,
//        and initial state.
//   2. Event loop (`start_loop`):
//      - Asynchronously listens for incoming `AnimationCommand` messages.
//      - For each command, delegates to `process_command`.
//      - Gracefully exits when the channel is closed (Shutdown).
//   3. Command processing (`process_command`):
//      - AnimateChunkSet / StartTestAnimation → heavy work delegated to workers.
//      - SetTimeScale → updates local time scale.
//      - SetEnabled → toggles enabled flag and logs to Godot.
//      - Shutdown → closes channel, ending the loop.
//   4. State inspection (`get_state`):
//      - Returns a clone of the current animation state for external systems.
//
// Design Choices:
//   • Async trait (`async_trait`) allows clean async definitions inside traits.
//   • Tokio unbounded channels provide non-blocking communication between
//     Godot and Rust workers.
//   • Separation of concerns: conductor manages orchestration, workers handle
//     heavy computation.
//   • Logging ensures transparency for debugging and runtime observability.
//
// Educational Note:
//   • This module demonstrates a common concurrency pattern in game engines:
//     - A lightweight event loop listens for commands.
//     - Heavy work is delegated to parallel workers.
//     - Internal state is updated locally and exposed externally.
//   • By structuring the conductor this way, the engine achieves both
//     responsiveness (non-blocking loop) and throughput (parallel workers).
// ============================================================================


use crate::{ConductorBehavior, AnimationCommand, CommandResult, AnimationState, UpdateSender};
use crate::worker::process_command_parallel; 
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use godot::prelude::godot_print; 

pub struct AnimationConductor {
    command_rx: UnboundedReceiver<AnimationCommand>,
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
        while let Some(command) = self.command_rx.recv().await {
            let _ = self.process_command(command);
        }
    }

    fn process_command(&mut self, command: AnimationCommand) -> CommandResult {
        match command {
            AnimationCommand::AnimateChunkSet { .. } | AnimationCommand::StartTestAnimation => {
                process_command_parallel(command, self.update_tx.clone());
                Ok(())
            }
            AnimationCommand::SetTimeScale(scale) => {
                self.state.set_time_scale(scale); 
                Ok(())
            }
            AnimationCommand::SetEnabled(enabled) => {
                self.state.set_enabled(enabled);
                godot_print!("Animation Conductor: is_enabled set to {}", enabled);
                Ok(())
            }
            AnimationCommand::Shutdown => {
                self.command_rx.close();
                Ok(())
            }
        }
    }

    fn get_state(&self) -> AnimationState {
        self.state.clone()
    }
}
