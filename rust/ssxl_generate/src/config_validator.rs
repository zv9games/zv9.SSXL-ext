//! Contains data structures and logic for validating map generation parameters
//! against engine constraints.

use tracing::{error, info};
use std::fmt;

// --- CONSTANTS (Copied from Conductor for self-contained validation logic) ---
/// The size of a chunk in tiles.
const CHUNK_SIZE: i64 = 64;

/// A safety measure to prevent the Conductor from trying to generate or track
/// an excessive number of chunks that could lead to memory exhaustion.
const MAX_ACTIVE_CHUNKS: i64 = 100_000_000;

// -----------------------------------------------------------------------------
// GENERATOR CONFIGURATION
// -----------------------------------------------------------------------------

/// Configuration data passed from the Godot API to the Conductor to start a
/// full map generation run. (Moved from conductor.rs)
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub width: usize,
    pub height: usize,
    pub seed: String,
    pub generator_name: String,
}

impl fmt::Display for GeneratorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ Width: {}, Height: {}, Seed: '{}', Generator: '{}' }}",
            self.width, self.height, self.seed, self.generator_name
        )
    }
}

// -----------------------------------------------------------------------------
// VALIDATION LOGIC
// -----------------------------------------------------------------------------

/// Responsible for checking configuration limits before starting a batch generation job.
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validates the map dimensions against the maximum allowed chunk count.
    ///
    /// # Returns
    /// A Result indicating success or a detailed error message if validation fails.
    pub fn validate_map_dimensions(config: &GeneratorConfig) -> Result<(), String> {
        info!("Validating batch generation command with config: {}", config);

        let chunk_size_i64 = CHUNK_SIZE;
        // Calculate chunks needed (ceiling division for chunk coverage)
        let width_in_chunks = (config.width as i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let height_in_chunks = (config.height as i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let total_chunks = width_in_chunks * height_in_chunks;

        if total_chunks == 0 {
            let error_msg = format!(
                "Validation Failed: Calculated chunk count is zero for size {}x{}. Dimensions are too small.",
                config.width, config.height
            );
            error!("{}", error_msg);
            return Err(error_msg);
        }

        if total_chunks > MAX_ACTIVE_CHUNKS {
            let error_msg = format!(
                "Validation Failed: Max chunks limit exceeded. Requested {} chunks ({}x{}) but the limit is {}. Adjust MAX_ACTIVE_CHUNKS or reduce map size.",
                total_chunks, width_in_chunks, height_in_chunks, MAX_ACTIVE_CHUNKS
            );
            error!("{}", error_msg);
            return Err(error_msg);
        }

        info!(
            "Validation Passed: Map size is {}x{} chunks (Total: {}).",
            width_in_chunks, height_in_chunks, total_chunks
        );
        Ok(())
    }
}