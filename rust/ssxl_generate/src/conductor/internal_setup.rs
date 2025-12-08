// ============================================================================
// 🎼 Conductor Internal Setup (`crate::conductor::internal_setup`)
// ----------------------------------------------------------------------------
// This module defines the `ConductorInternalSetup` struct, a bundle of all
// internal components required to initialize and run the Conductor. It acts
// as a "setup package" that can be passed between modules to ensure consistent
// initialization of channels, state, and managers.
//
// Purpose:
//   • Encapsulate all dependencies needed to start the Conductor runtime.
//   • Provide a clean, modular way to prepare async channels and state.
//   • Simplify spawning of the Conductor by bundling setup into one struct.
//
// Key Components:
//   • request_receiver
//       - UnboundedReceiver for incoming chunk generation requests.
//       - Consumed by the async request loop to process tasks.
//
//   • progress_sender
//       - Bounded Sender for progress updates.
//       - Ensures backpressure when too many updates are queued.
//
//   • request_sender_api
//       - UnboundedSender exposed to external modules.
//       - Allows API or other systems to submit chunk generation requests.
//
//   • progress_receiver
//       - Bounded Receiver for progress updates.
//       - Enables monitoring or reporting of generation progress.
//
//   • initial_state
//       - ConductorState tracking lifecycle status, queue depth,
//         active generator ID, and tile counters.
//       - Provides thread-safe, atomic access for concurrent tasks.
//
//   • generator_manager
//       - Manages available generator instances and their configurations.
//       - Provides access to generator logic during request loop execution.
//
// Workflow:
//   1. `setup_channels_and_state` constructs a `ConductorInternalSetup` bundle.
//   2. The bundle is passed into `spawn`, which consumes it to start the async
//      request loop and build a Conductor instance.
//   3. Channels handle communication between tasks and progress reporting.
//   4. State and managers ensure consistent orchestration of generation logic.
//
// Design Choices:
//   • Bundling setup into a single struct improves clarity and reduces boilerplate.
//   • Separation of bounded vs. unbounded channels balances flexibility with safety.
//   • Explicit inclusion of state and managers ensures all dependencies are tracked.
//
// Educational Note:
//   • This struct demonstrates how Rust can encapsulate complex initialization
//     logic into a single, reusable package.
//   • By centralizing setup, the Conductor remains modular, testable, and easy
//     to extend with new components or channels.
// ============================================================================


use crate::conductor::conductor_state::ConductorState;
use crate::manager::generator_manager::GeneratorManager;
use crate::task::task_queue::{GenerationTask as ChunkRequest, GenerationMessage};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, Receiver, Sender};

pub(crate) struct ConductorInternalSetup {
    pub request_receiver: UnboundedReceiver<ChunkRequest>,
    pub progress_sender: Sender<GenerationMessage>,
    pub request_sender_api: UnboundedSender<ChunkRequest>,
    pub progress_receiver: Receiver<GenerationMessage>,
    pub initial_state: ConductorState,
    pub generator_manager: GeneratorManager,
}
