// ssxl_generate/src/runtime_manager.rs
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