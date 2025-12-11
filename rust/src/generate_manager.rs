// rust/SSXL-ext/src/generate_manager.rs

use crate::sync_pool::SyncPool;
use crate::generate_task_queue::GenerationTask;
use crate::shared_job::GenerationJob;
use crate::shared_config::{GenerationConfig, ThreadingConfig};
use crate::shared_error::SSXLCoreError;

/// The functional engine that manages the submission of generation jobs 
/// to the worker pool.
pub struct GenerationManager {
    // A reference to the thread pool implementation (held by the HostState)
    pool: SyncPool,
    // The current global generation configuration
    config: GenerationConfig,
}

impl GenerationManager {
    /// Creates a new manager, typically called during host_init.rs.
    pub fn new(pool: SyncPool, config: GenerationConfig) -> Self {
        GenerationManager { pool, config }
    }

    /// Provides a high-level function to start processing a batch of tasks.
    /// This is called by the GenerateConductor when generation starts.
    pub fn submit_job_batch(&self, tasks: Vec<GenerationTask>) -> Result<usize, SSXLCoreError> {
        let mut jobs_submitted = 0;
        
        // --- 1. Map Tasks to Jobs and Submit ---
        for task in tasks {
            // Create the full, multi-stage GenerationJob structure
            let job = GenerationJob::new(task);

            // Submit the job to the worker pool's queue
            match self.pool.submit_job(job) {
                Ok(_) => jobs_submitted += 1,
                Err(e) => {
                    ssxl_error!("Generation Manager: Failed to submit job {:?}. Pool likely disconnected or full: {}", job.id, e);
                    // On first error, we might stop and return the error
                    return Err(SSXLCoreError::ChannelSendError(e.to_string()));
                }
            }
        }

        // --- 2. Update Conductor State ---
        // The conductor (which owns this manager) should be responsible for setting the total count.
        
        ssxl_info!("Generation Manager: Submitted {} multi-stage jobs to the pool.", jobs_submitted);
        Ok(jobs_submitted)
    }

    /// Retrieves the current status of the worker pool (useful for debugging).
    pub fn get_pool_status(&self) -> (u32, usize) {
        let (worker_count, queue_size) = self.pool.get_status();
        (worker_count as u32, queue_size)
    }

    /// Cleanly shuts down the worker pool.
    pub fn shutdown(&self) {
        self.pool.shutdown();
    }
    
    // Additional methods like `pause_generation()` or `update_config()` would live here.
}