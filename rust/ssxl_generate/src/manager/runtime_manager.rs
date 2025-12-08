// ============================================================================
// 🎼 Runtime Manager (`crate::manager::runtime_manager`)
// ----------------------------------------------------------------------------
// This module defines the `RuntimeManager`, a wrapper around the Tokio runtime
// that manages its lifecycle. It ensures that asynchronous tasks in the SSXL
// engine are executed efficiently and that the runtime is properly initialized
// and shut down.
//
// Purpose:
//   • Provide a centralized manager for the Tokio runtime.
//   • Configure runtime with optimal worker threads based on available CPU cores.
//   • Expose a safe handle for spawning tasks without owning the runtime directly.
//   • Ensure graceful shutdown when the manager is dropped.
//
// Key Components:
//   • RuntimeManager (struct)
//       - Wraps an `Option<Runtime>` to allow safe ownership transfer during Drop.
//       - Encapsulates initialization, access, and shutdown logic.
//
// Implementation Methods:
//   • new
//       - Creates a new multi-threaded Tokio runtime.
//       - Configures worker threads equal to the number of CPU cores.
//       - Enables all Tokio features (I/O, time, etc.).
//       - Logs initialization details.
//       - Returns `Ok(RuntimeManager)` or `Err(io::Error)` if runtime fails to build.
//
//   • get_handle
//       - Provides a clone of the runtime handle.
//       - Allows spawning tasks without owning the runtime itself.
//       - Panics if runtime is unexpectedly None.
//
//   • shutdown_graceful
//       - Logs a graceful shutdown request.
//       - Actual shutdown is deferred to the Drop implementation.
//
// Drop Implementation:
//   • Ensures runtime is properly shut down when `RuntimeManager` goes out of scope.
//   • Steps:
//       1. Take ownership of runtime from Option (leaving None behind).
//       2. Log shutdown message.
//       3. Call `shutdown_background()` to stop runtime without blocking current thread.
//
// Workflow:
//   1. RuntimeManager is created via `new`, initializing a multi-threaded runtime.
//   2. Tasks are spawned using `get_handle`.
//   3. When shutdown is requested, `shutdown_graceful` logs the intent.
//   4. On drop, runtime is safely shut down in the background.
//
// Design Choices:
//   • Wrapping runtime in `Option` allows safe transfer of ownership during Drop.
//   • Using `num_cpus` ensures runtime scales with hardware capabilities.
//   • Logging provides visibility into initialization and shutdown events.
//   • Background shutdown prevents blocking the main thread during cleanup.
//
// Educational Note:
//   • This module demonstrates how Rust can manage async runtimes safely,
//     ensuring deterministic lifecycle control.
//   • By combining smart pointers, logging, and Drop semantics, it provides
//     a robust foundation for concurrent task execution in the SSXL engine.
// ============================================================================


use tokio::runtime::{Runtime, Handle};
use tracing::info;
use num_cpus;
use std::io;

pub struct RuntimeManager {
    runtime: Option<Runtime>,
}

impl RuntimeManager {
    pub fn new() -> Result<Self, io::Error> {
        let num_cores = num_cpus::get();
        info!("Tokio Runtime initializing with {} worker threads.", num_cores);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cores)
            .enable_all()
            .build()?;

        Ok(RuntimeManager {
            runtime: Some(runtime),
        })
    }

    pub fn get_handle(&self) -> Handle {
        self.runtime.as_ref().unwrap().handle().clone()
    }

    pub fn shutdown_graceful(&self) {
        info!("Tokio Runtime graceful shutdown requested.");
    }
}

impl Drop for RuntimeManager {
    fn drop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            info!("RuntimeManager dropped — shutting down Tokio runtime.");
            runtime.shutdown_background();
        }
    }
}
