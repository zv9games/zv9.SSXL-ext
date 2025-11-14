// ssxl_generate/config_validator.rs

//! Contains the structural definition for batch generation requests and the
//! logic for validating user-defined map dimensions against engine limits.
//!
//! This ensures stability and prevents excessive memory usage during large-scale
//! **Bulldozer** generation tasks.

use tracing::{error, info};
use std::fmt;

/// The canonical size of a chunk in tiles, used for dimension conversion.
/// (Local definition for validation, typically sourced from ssxl_shared).
const CHUNK_SIZE: i64 = 64;

/// The hard safety limit on the total number of chunks allowed to be generated
/// or held in active memory to prevent system overload.
const MAX_ACTIVE_CHUNKS: i64 = 100_000_000;

// --- 1. Configuration Data Structure ---

/// Configuration parameters defining a single batch world generation request.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Desired world width in tiles.
    pub width: usize,
    /// Desired world height in tiles.
    pub height: usize,
    /// The deterministic seed string for the generation process (**Crypto-coded memory**).
    pub seed: String,
    /// The ID of the generator to execute (e.g., 'cellular_automata_basic').
    pub generator_name: String,
}

impl fmt::Display for GeneratorConfig {
    /// Implements display for easy logging and error reporting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ Width: {}, Height: {}, Seed: '{}', Generator: '{}' }}",
            self.width, self.height, self.seed, self.generator_name
        )
    }
}

// --- 2. Validation Logic ---

/// Utility struct containing static methods for validating generator configurations.
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validates that the requested map dimensions (width, height) are sane and
    /// do not exceed the `MAX_ACTIVE_CHUNKS` limit.
    ///
    /// This step is crucial for maintaining system **balance** and ensuring project **completion**.
    ///
    /// # Arguments
    /// * `config`: The generation request parameters.
    ///
    /// # Returns
    /// `Ok(())` if validation passes, or `Err(String)` with an error message otherwise.
    pub fn validate_map_dimensions(config: &GeneratorConfig) -> Result<(), String> {
        info!("Validating batch generation command with config: {}", config);

        let chunk_size_i64 = CHUNK_SIZE;

        // Calculate world size in chunks (uses integer division ceiling trick for partial chunks).
        let width_in_chunks = (config.width as i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let height_in_chunks = (config.height as i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let total_chunks = width_in_chunks * height_in_chunks;

        // Validation Check 1: Must generate at least one chunk.
        if total_chunks <= 0 {
            let error_msg = format!(
                "Validation Failed: Calculated chunk count is zero for size {}x{}. Dimensions are too small.",
                config.width, config.height
            );
            error!("{}", error_msg);
            return Err(error_msg);
        }

        // Validation Check 2: Must not exceed the safety limit.
        if total_chunks > MAX_ACTIVE_CHUNKS {
            let error_msg = format!(
                "Validation Failed: Max chunks limit exceeded. Requested {} chunks ({}x{}) but the limit is {}. Adjust MAX_ACTIVE_CHUNKS or reduce map size.",
                total_chunks, width_in_chunks, height_in_chunks, MAX_ACTIVE_CHUNKS
            );
            error!("{}", error_msg);
            return Err(error_msg);
        }

        info!(
            "Validation Passed: Map size is {}x{} chunks (Total: {}). Starting **Bulldozer** operation.",
            width_in_chunks, height_in_chunks, total_chunks
        );
        Ok(())
    }
}