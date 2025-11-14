// ssxl_generate/src/runtime_manager.rs

//! Manages the Tokio asynchronous runtime for the SSXL generation engine.
//!
//! The RuntimeManager is responsible for initializing the multi-threaded
//! runtime, providing handles to worker threads, and coordinating a graceful shutdown,
//! ensuring the highest possible **tempo** for asynchronous operations.

use tokio::runtime::{Runtime, Handle};
use tracing::info;
use num_cpus;
use std::io;

/// Manages the lifecycle of the underlying Tokio asynchronous runtime.
pub struct RuntimeManager {
    /// The core Tokio multi-threaded runtime instance.
    runtime: Runtime,
}

impl RuntimeManager {
    /// Initializes a new multi-threaded Tokio runtime.
    ///
    /// The runtime is configured to use the same number of worker threads as the
    /// machine's logical CPU cores, maximizing parallelism for generation tasks.
    pub fn new() -> Result<Self, io::Error> {
        let num_cores = num_cpus::get();
        info!("Tokio Runtime initializing with {} worker threads (all logical cores).", num_cores);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            // Set worker count to the number of logical cores.
            .worker_threads(num_cores)
            // Enable all features (timers, I/O) for general utility.
            .enable_all()
            .build()?;
            
        Ok(RuntimeManager { runtime })
    }

    /// Returns a clone of the runtime's handle.
    ///
    /// This handle is used by the **Conductor** to spawn new asynchronous tasks
    /// and to execute CPU-heavy tasks on the dedicated blocking thread pool.
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// Signals the Tokio runtime to begin a graceful shutdown in the background.
    ///
    /// This allows all remaining tasks to complete their current work before the
    /// threads are finally torn down.
    pub fn shutdown_graceful(self) {
        info!("Tokio Runtime signaling graceful shutdown.");
        // This method instructs the runtime to shut down without blocking the current thread.
        self.runtime.shutdown_background();
        info!("Tokio Runtime full teardown complete.");
    }
}