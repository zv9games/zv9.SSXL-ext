// rust/SSXL-ext/src/generate_runtime.rs

use crate::shared_job::{GenerationJob, JobStep};
use crate::shared_config::GenerationConfig;
use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::generate_batch_processor;
use crate::tools::Profiler;

// --- FIX: Import logging macros from the crate root ---
use crate::{ssxl_info, ssxl_error}; 

/// The primary entry point for a worker thread to execute a multi-step generation job.
/// This function manages the entire runtime lifecycle for a single chunk's generation.
pub fn run_generation_job(
    mut job: GenerationJob,
    config: &GenerationConfig,
) -> Result<Chunk, SSXLCoreError> {
    
    // --- 1. Runtime Profiling Context ---
    // Start a profiler for the entire chunk's execution time (Perlin, CA, Post-process combined).
    let profiler_name = format!("Chunk_Gen_Runtime ({}, {})", job.id.0, job.id.1);
    // Note: Box::leak is used to convert the dynamically created string into a 'static str for Profiler.
    let _p = Profiler::start(Box::leak(profiler_name.into_boxed_str()));
    
    // Set the initial step correctly
    if job.current_step == JobStep::Queued {
        job.advance_step(); // Move to NoiseGeneration
    }
    
    // --- 2. The Multi-Step Execution Loop ---
    
    // Continue processing the job until it reaches a terminal state (Finished or Failed).
    while job.current_step != JobStep::Finished && job.current_step != JobStep::Failed {
        
        // FIX: Capture the necessary fields *before* moving the `job` into the processor.
        let job_id_for_logging = job.id;
        let job_step_for_logging = job.current_step; // Since JobStep is likely Copy or Clone, this works.

        let result = generate_batch_processor::process_generation_job(
            job, 
            config
        );

        match result {
            Ok(next_job) => {
                // Step succeeded and job was advanced by the processor. Continue the loop.
                job = next_job;
            }
            Err(e) => {
                // Step failed. Log the error, transition to Failed, and break the loop.
                // FIX: Use the captured variables for logging.
                ssxl_error!("Job {:?} failed at step {:?}: {:?}", job_id_for_logging, job_step_for_logging, e);
                // NOTE: The compiler still complains about using `job` here, so we must rely on
                // the `process_generation_job` function to have updated the job state inside
                // the thread pool before returning an error, or the `job` struct must implement `Copy`.
                
                // Since the original code attempted to access the moved value, and the intent was to update it,
                // we *must* rely on the captured step for logging and return immediately.
                // If `job` implemented Copy, we could just copy the job. We'll proceed with the assumption
                // that `job.id` and `job.current_step` are Copy/Clone.
                
                // We cannot use `job.current_step = JobStep::Failed;` here because `job` is moved.
                // The responsibility to mark the job as failed must fall to the `process_generation_job`
                // function before it returns the error, or the worker manager needs to handle it.
                // Given the existing structure, we remove the access to the moved value.
                // We remove the line `job.current_step = JobStep::Failed;` because it is unreachable.
                
                return Err(e);
            }
        }
    }
    
    // --- 3. Terminal State Handling ---
    
    match job.current_step {
        JobStep::Finished => {
            ssxl_info!("Job {:?} successfully completed runtime.", job.id);
            // The job is complete, return the final chunk data payload.
            // NOTE: Assuming the final chunk data is correctly stored in job.chunk_data by the processor.
            Ok(job.chunk_data)
        }
        JobStep::Failed => {
            // Should be handled above, but included for complete error coverage.
            ssxl_error!("Runtime ended in failed state for job {:?}", job.id);
            Err(SSXLCoreError::InvalidConductorState("Job loop finished but state is Failed.".to_string()))
        }
        _ => {
            // Should be unreachable if the batch processor is correct.
            Err(SSXLCoreError::InvalidConductorState(format!("Runtime finished in non-terminal state: {:?}", job.current_step)))
        }
    }
}