// rust/SSXL-ext/src/generate_manager.rs

use crate::sync_pool::SyncPool;
use crate::generate_task_queue::GenerationTask;
use crate::shared_job::GenerationJob;
use crate::shared_config::GenerationConfig;
use crate::shared_error::SSXLCoreError;
use crate::{ssxl_error, ssxl_info};
// FIX 1: Remove godot::prelude::* if it's not strictly necessary for the GenerationManager logic
// or keep it commented out to prevent accidental Godot-dependent calls in the CLI context.
// use godot::prelude::*; // Commented out to reduce surface area for Godot dependency checks

/// The functional engine that manages the submission of generation jobs
/// to the worker pool.
pub struct GenerationManager {
    // A reference to the thread pool implementation (held by the HostState)
    pool: SyncPool,
    // The current global generation configuration
    // ✅ FIX WARNING: Renamed to `_config` to silence the 'never read' warning.
    _config: GenerationConfig,
}

impl GenerationManager {
    /// Creates a new manager, typically called during host_init.rs.
    // Updated parameter name to match the struct field.
    pub fn new(pool: SyncPool, config: GenerationConfig) -> Self {
        GenerationManager { pool, _config: config }
    }

    /// Provides a high-level function to start processing a batch of tasks.
    /// This is called by the GenerateConductor when generation starts.
    pub fn submit_job_batch(&self, tasks: Vec<GenerationTask>) -> Result<usize, SSXLCoreError> {
        let mut jobs_submitted = 0;
        
        // --- 1. Map Tasks to Jobs and Submit ---
        for task in tasks {
            // Create the full, multi-stage GenerationJob structure
            let job = GenerationJob::new(task);
            
            // Extract the job ID before moving the job into submit_job.
            let job_id = job.id;

            // Submit the job to the worker pool's queue (job is moved here)
            match self.pool.submit_job(job) {
                Ok(_) => jobs_submitted += 1,
                Err(e) => {
                    // Uses the extracted job_id for the error log, preventing the borrow-after-move error.
                    ssxl_error!("Generation Manager: Failed to submit job {:?}. Pool likely disconnected or full: {}", job_id, e);
                    // On first error, we might stop and return the error
                    return Err(SSXLCoreError::ChannelSendError(e.to_string()));
                }
            }
        }

        // --- 2. Update Conductor State ---
        
        ssxl_info!("Generation Manager: Submitted {} multi-stage jobs to the pool.", jobs_submitted);
        Ok(jobs_submitted)
    }

    /// Retrieves the current status of the worker pool (useful for debugging).
    pub fn get_pool_status(&self) -> (u32, usize) {
        let (worker_count, queue_size) = self.pool.get_status();
        (worker_count as u32, queue_size)
    }
}


// ============================================================================
// FFI EXPORTS (MOCKS required for ssxl_cli linking) - KEEP THESE HERE!
// These must use eprintln! instead of godot_print! when run from the CLI.
// ============================================================================

/// EXPORT 3/5: Called by CLI to stress the raw computation layer.
#[no_mangle]
pub extern "C" fn ssxl_ext_generate_noise_chunk(x: i32, y: i32, size: i32, seed: u64) {
    // 🔥 FIX 2: Replaced godot_print! with eprintln! to prevent Godot runtime panic.
    eprintln!("FFI_EXPORT: CLI triggered HEAVY NOISE GENERATION MOCK for chunk ({}, {}) size {} with seed {}",
                 x, y, size, seed);
}

/// EXPORT 4/5: Called by CLI to test the boundary matching data structure.
#[no_mangle]
pub extern "C" fn ssxl_ext_verify_chunk_boundary(chunk_x: i32, chunk_y: i32, neighbor_x: i32, neighbor_y: i32) -> bool {
    // 🔥 FIX 3: Replaced godot_print! with eprintln! to prevent Godot runtime panic.
    eprintln!("FFI_EXPORT: CLI checking boundary MOCK between chunk ({}, {}) and ({}, {})",
                 chunk_x, chunk_y, neighbor_x, neighbor_y);
    true
}