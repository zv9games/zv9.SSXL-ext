// rust/SSXL-ext/src/generate_runtime.rs

use std::time::Instant;

use crate::shared_job::{GenerationJob, JobStep};
use crate::shared_config::GenerationConfig;
use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::generate_batch_processor;

/// Worker-thread entry point.
/// MUST NOT TOUCH GODOT BINDINGS OR GODOT LOGGING.
/// Only pure Rust + eprintln! is allowed here.
pub fn run_generation_job(
    mut job: GenerationJob,
    config: &GenerationConfig,
) -> Result<Chunk, SSXLCoreError> {
    let start = Instant::now();

    eprintln!(
        "[runtime] ENTER job {:?} at initial step {:?}",
        job.id,
        job.current_step
    );

    // Step correction
    if job.current_step == JobStep::Queued {
        eprintln!(
            "[runtime] job {:?} Queued → advancing to NoiseGeneration",
            job.id
        );
        job.advance_step();
    }

    // --- Multi-step execution loop ---
    while job.current_step != JobStep::Finished && job.current_step != JobStep::Failed {
        eprintln!(
            "[runtime] loop start job {:?}, current_step = {:?}",
            job.id,
            job.current_step
        );

        let job_id = job.id;
        let step = job.current_step;

        eprintln!(
            "[runtime] calling process_generation_job for job {:?} at step {:?}",
            job_id,
            step
        );

        let result = generate_batch_processor::process_generation_job(job, config);

        match result {
            Ok(next_job) => {
                eprintln!(
                    "[runtime] process_generation_job OK for job {:?} at step {:?} → next_step = {:?}",
                    job_id,
                    step,
                    next_job.current_step
                );
                job = next_job;
            }
            Err(e) => {
                eprintln!(
                    "[runtime] job {:?} FAILED at step {:?}: {:?}",
                    job_id,
                    step,
                    e
                );
                let elapsed = start.elapsed();
                eprintln!(
                    "[runtime] EXIT job {:?} with ERROR after {:.3}ms",
                    job_id,
                    elapsed.as_secs_f64() * 1000.0
                );
                return Err(e);
            }
        }
    }

    // --- Terminal state ---
    let elapsed = start.elapsed();
    eprintln!(
        "[runtime] job {:?} EXIT loop with terminal step {:?} after {:.3}ms",
        job.id,
        job.current_step,
        elapsed.as_secs_f64() * 1000.0
    );

    match job.current_step {
        JobStep::Finished => {
            eprintln!(
                "[runtime] job {:?} successfully completed; returning chunk.",
                job.id
            );
            Ok(job.chunk_data)
        }
        JobStep::Failed => {
            eprintln!("[runtime] job {:?} ended in FAILED state", job.id);
            Err(SSXLCoreError::InvalidConductorState(
                "Job loop finished but state is Failed.".to_string(),
            ))
        }
        other => {
            eprintln!(
                "[runtime] job {:?} ended in NON-TERMINAL state {:?}",
                job.id,
                other
            );
            Err(SSXLCoreError::InvalidConductorState(format!(
                "Runtime finished in non-terminal state: {:?}",
                other
            )))
        }
    }
}
