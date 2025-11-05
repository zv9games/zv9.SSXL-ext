//! Manages the lifecycle and configuration of the Tokio asynchronous runtime.
//! This ensures the Conductor remains decoupled from low-level thread pool setup.

use tokio::runtime::{Runtime, Handle};
use tracing::info;
use num_cpus;
use std::io;

/// Manages the Tokio Runtime, which provides the thread pool necessary
/// for executing asynchronous and blocking generation tasks.
pub struct RuntimeManager {
    runtime: Runtime,
}

impl RuntimeManager {
    /// Initializes a new multi-threaded Tokio Runtime, configured to use
    /// all available logical CPU cores for maximum concurrency.
    pub fn new() -> Result<Self, io::Error> {
        let num_cores = num_cpus::get();
        info!("Tokio Runtime initializing with {} worker threads (all logical cores).", num_cores);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cores)
            .enable_all()
            .build()?;
            
        Ok(RuntimeManager { runtime })
    }

    /// Provides a thread-safe handle to the Runtime for spawning tasks
    /// from various parts of the Conductor.
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// Performs a graceful shutdown of the underlying Tokio runtime.
    /// This should be called during the `Conductor`'s teardown process.
    pub fn shutdown_graceful(self) {
        info!("Tokio Runtime signaling graceful shutdown.");
        // This stops the Tokio runtime pool, freeing its resources.
        self.runtime.shutdown_background();
        info!("Tokio Runtime full teardown complete.");
    }
}