// ============================================================================
// 🎼 Conductor Channel Types (`crate::conductor::sync_get`)
// ----------------------------------------------------------------------------
// This module defines domain-specific wrappers and aliases for the async
// communication channels used by the Conductor system. By introducing clear,
// descriptive names, it improves readability and makes the intent of each
// channel explicit.
//
// Purpose:
//   • Provide a type alias for request-sending channels to simplify usage.
//   • Wrap progress-receiving channels in a dedicated struct for clarity.
//   • Ensure that communication between tasks and the Conductor is both
//     ergonomic and domain-specific.
//
// Key Components:
//   • ConductorRequestSender (type alias)
//       - Alias for `UnboundedSender<GenerationTask>`.
//       - Represents the channel through which new chunk generation requests
//         are submitted into the Conductor system.
//       - Improves readability by avoiding repetitive generic type signatures.
//
//   • ConductorProgressReceiver (struct)
//       - Wraps a bounded `Receiver<GenerationMessage>`.
//       - Explicitly represents the channel for receiving progress updates
//         from generation tasks.
//       - Provides a constructor (`new`) for ergonomic initialization.
//
// Workflow:
//   1. External modules submit chunk generation requests via `ConductorRequestSender`.
//   2. The Conductor processes tasks asynchronously.
//   3. Progress updates are sent back through bounded channels.
//   4. `ConductorProgressReceiver` wraps the receiver, clarifying its purpose
//      as a conduit for progress messages.
//
// Design Choices:
//   • Separation of request and progress channels improves clarity.
//   • Use of type alias reduces boilerplate and emphasizes domain semantics.
//   • Wrapping the progress receiver in a struct makes its role explicit,
//     aiding maintainability and readability.
//   • Bounded vs. unbounded channels balance flexibility (requests) with
//     safety (progress updates).
//
// Educational Note:
//   • This module demonstrates how Rust’s type system can be leveraged to
//     create domain-specific abstractions over generic async channels.
//   • By naming and wrapping channels, developers can more easily understand
//     their purpose and avoid misuse in complex async systems.
// ============================================================================


use tokio::sync::mpsc::{Receiver, UnboundedSender};

use crate::task::task_queue::GenerationMessage;
use crate::task::task_queue::GenerationTask;

pub type ConductorRequestSender = UnboundedSender<GenerationTask>;

pub struct ConductorProgressReceiver {
    pub rx: Receiver<GenerationMessage>,
}

impl ConductorProgressReceiver {
    pub fn new(rx: Receiver<GenerationMessage>) -> Self {
        ConductorProgressReceiver { rx }
    }
}
