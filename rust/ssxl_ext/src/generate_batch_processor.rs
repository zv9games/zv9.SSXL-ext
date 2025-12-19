use crate::shared_job::{GenerationJob, JobStep};
use crate::generate_perlin;
use crate::generate_ca;
use crate::shared_error::SSXLCoreError;
use crate::shared_config::GenerationConfig;
use crate::ssxl_info;
use std::mem;

/// Executes the next required step for the given GenerationJob.
///
/// This version is CA‑ready: the CA call is isolated so we can later
/// replace it with a neighbor‑aware / halo‑based CA without touching
/// the rest of the pipeline.
pub fn process_generation_job(
    mut job: GenerationJob,
    config: &GenerationConfig,
) -> Result<GenerationJob, SSXLCoreError> {

    ssxl_info!("Processing job {:?} at step: {:?}", job.id, job.current_step);

    let result = match job.current_step {

        // ------------------------------------------------------------
        // STEP 1: PERLIN NOISE GENERATION
        // ------------------------------------------------------------
        JobStep::NoiseGeneration => {
            let generator = generate_perlin::NoiseGenerator::new(
                config.perlin,
                config.world_seed,
            );

            let chunk_to_process = mem::take(&mut job.chunk_data);

            match generate_perlin::generate_noise_map(chunk_to_process, &generator) {
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
            let chunk_to_process = mem::take(&mut job.chunk_data);

            // ✅ This is the seam where halo‑based CA will plug in.
            // For now, we call the chunk‑local CA.
            match generate_ca::simulate_ca(chunk_to_process, config.ca.into()) {
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
            // Placeholder — remains unchanged.
            Ok(())
        }

        // ------------------------------------------------------------
        // TERMINAL STATES
        // ------------------------------------------------------------
        JobStep::Queued | JobStep::Finished | JobStep::Failed => Err(
            SSXLCoreError::InvalidConductorState(format!(
                "Job received in terminal step: {:?}",
                job.current_step
            )),
        ),
    };

    match result {
        Ok(_) => {
            job.advance_step();
            Ok(job)
        }
        Err(e) => {
            job.current_step = JobStep::Failed;
            Err(e)
        }
    }
}
