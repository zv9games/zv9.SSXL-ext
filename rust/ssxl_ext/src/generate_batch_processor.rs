// rust/SSXL-ext/src/generate_batch_processor.rs

use crate::shared_job::{GenerationJob, JobStep};
use crate::generate_perlin;
use crate::generate_ca;
use crate::shared_error::SSXLCoreError;
use crate::shared_config::GenerationConfig;
use crate::ssxl_info;
use std::mem; // Required for mem::take

/// Executes the next required step for the given GenerationJob.
/// 
/// Returns: The modified job, advanced to the next step, or an error.
pub fn process_generation_job(
    mut job: GenerationJob,
    config: &GenerationConfig,
) -> Result<GenerationJob, SSXLCoreError> {
    
    ssxl_info!("Processing job {:?} at step: {:?}", job.id, job.current_step);

    let result = match job.current_step {
        
        // --- STEP 1: PERLIN NOISE GENERATION ---
        JobStep::NoiseGeneration => {
            let generator = generate_perlin::NoiseGenerator::new(
                config.perlin, 
                config.world_seed
            );
            
            // FIX: Use mem::take() to safely extract the Chunk value for the move.
            let chunk_to_process = mem::take(&mut job.chunk_data); 

            // Call the Perlin noise module, passing the chunk by value (moving it).
            match generate_perlin::generate_noise_map(chunk_to_process, &generator) {
                Ok(chunk) => {
                    job.chunk_data = chunk; // Reassign the returned chunk.
                    Ok(())
                }
                // If it fails, `job.chunk_data` is left with the dummy value from `mem::take`, 
                // but the overall `job` struct remains valid to be returned/used.
                Err(e) => Err(SSXLCoreError::GenerationDataError(format!("Perlin failed: {}", e))),
            }
        },
        
        // --- STEP 2: CELLULAR AUTOMATA REFINEMENT ---
        JobStep::CARefinement => {
            // FIX: Use mem::take() to safely extract the Chunk value for the move.
            let chunk_to_process = mem::take(&mut job.chunk_data); 

            // Call the CA module, passing the chunk by value (moving it).
            match generate_ca::simulate_ca(chunk_to_process, config.ca.into()) {
                Ok(chunk) => {
                    job.chunk_data = chunk; // Reassign the returned chunk.
                    Ok(())
                }
                Err(e) => Err(SSXLCoreError::GenerationDataError(format!("CA failed: {}", e))),
            }
        },
        
        // --- STEP 3: POST-PROCESSING (E.g., Border Blending, Entity Placement) ---
        JobStep::PostProcessing => {
            // Placeholder logic would also take the chunk by value or reference.
            // If it takes by value, it would need the same mem::take/reassign pattern.
            // For now, assume it's successful and the chunk remains.
            
            Ok(())
        },
        
        // --- TERMINAL STATES ---
        JobStep::Queued | JobStep::Finished | JobStep::Failed => {
            Err(SSXLCoreError::InvalidConductorState(format!("Job received in terminal step: {:?}", job.current_step)))
        }
    };

    // If successful, advance the job to the next stage and return it.
    match result {
        Ok(_) => {
            job.advance_step(); 
            Ok(job)
        },
        Err(e) => {
            job.current_step = JobStep::Failed;
            Err(e)
        }
    }
}