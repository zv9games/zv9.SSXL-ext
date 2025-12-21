use crate::shared_job::{GenerationJob, JobStep};
use crate::generate_perlin;
use crate::generate_ca;
use crate::shared_error::SSXLCoreError;
use crate::shared_config::GenerationConfig;
use std::mem;

/// Executes the next required step for the given GenerationJob.
///
/// This version is CA‑ready: the CA call is isolated so we can later
/// replace it with a neighbor‑aware / halo‑based CA without touching
/// the rest of the pipeline.
///
/// IMPORTANT: This function runs on worker threads and MUST NOT touch
/// Godot bindings or use ssxl_* logging. Only pure Rust + `eprintln!`.
pub fn process_generation_job(
    mut job: GenerationJob,
    config: &GenerationConfig,
) -> Result<GenerationJob, SSXLCoreError> {
    eprintln!(
        "[batch] ENTER job {:?} at step {:?}",
        job.id,
        job.current_step
    );

    let result = match job.current_step {
        // ------------------------------------------------------------
        // STEP 1: PERLIN NOISE GENERATION
        // ------------------------------------------------------------
        JobStep::NoiseGeneration => {
            eprintln!("[batch] job {:?} → NoiseGeneration", job.id);

            let generator = generate_perlin::NoiseGenerator::new(
                config.perlin,
                config.world_seed,
            );

            // Move out chunk data for processing.
            let chunk_to_process = mem::take(&mut job.chunk_data);
            eprintln!(
                "[batch] job {:?} NoiseGeneration starting on chunk at {:?}",
                job.id,
                chunk_to_process.position
            );

            let perlin_result =
                generate_perlin::generate_noise_map(chunk_to_process, &generator);
            eprintln!(
                "[batch] job {:?} NoiseGeneration result = {:?}",
                job.id,
                perlin_result.as_ref().map(|c| c.position)
            );

            match perlin_result {
                Ok(chunk) => {
                    job.chunk_data = chunk;
                    Ok(())
                }
                Err(e) => Err(SSXLCoreError::GenerationDataError(format!(
                    "Perlin failed: {}",
                    e
                ))),
            }
        }

        // ------------------------------------------------------------
        // STEP 2: CELLULAR AUTOMATA REFINEMENT
        // ------------------------------------------------------------
        JobStep::CARefinement => {
            eprintln!("[batch] job {:?} → CARefinement", job.id);

            let chunk_to_process = mem::take(&mut job.chunk_data);
            eprintln!(
                "[batch] job {:?} CARefinement starting on chunk at {:?}",
                job.id,
                chunk_to_process.position
            );

            let ca_result = generate_ca::simulate_ca(chunk_to_process, config.ca.into());
            eprintln!(
                "[batch] job {:?} CARefinement result = {:?}",
                job.id,
                ca_result.as_ref().map(|c| c.position)
            );

            match ca_result {
                Ok(chunk) => {
                    job.chunk_data = chunk;
                    Ok(())
                }
                Err(e) => Err(SSXLCoreError::GenerationDataError(format!(
                    "CA failed: {}",
                    e
                ))),
            }
        }

        // ------------------------------------------------------------
        // STEP 3: POST‑PROCESSING
        // ------------------------------------------------------------
        JobStep::PostProcessing => {
            eprintln!(
                "[batch] job {:?} → PostProcessing (no-op placeholder)",
                job.id
            );
            Ok(())
        }

        // ------------------------------------------------------------
        // TERMINAL STATES
        // ------------------------------------------------------------
        JobStep::Queued | JobStep::Finished | JobStep::Failed => {
            eprintln!(
                "[batch] job {:?} received in terminal step {:?} (ERROR)",
                job.id,
                job.current_step
            );
            Err(SSXLCoreError::InvalidConductorState(format!(
                "Job received in terminal step: {:?}",
                job.current_step
            )))
        }
    };

    match result {
        Ok(_) => {
            let prev = job.current_step;
            job.advance_step();
            eprintln!(
                "[batch] job {:?} step advanced {:?} -> {:?}",
                job.id,
                prev,
                job.current_step
            );
            eprintln!(
                "[batch] EXIT job {:?} with success at new step {:?}",
                job.id,
                job.current_step
            );
            Ok(job)
        }
        Err(e) => {
            eprintln!(
                "[batch] job {:?} failed at step {:?}, marking Failed",
                job.id,
                job.current_step
            );
            job.current_step = JobStep::Failed;
            eprintln!(
                "[batch] EXIT job {:?} with error, state = Failed",
                job.id
            );
            Err(e)
        }
    }
}
