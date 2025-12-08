// ============================================================================
// 🎼 Generator Configuration and Validation (`crate::manager::generator_config`)
// ----------------------------------------------------------------------------
// This module defines the configuration parameters for a generator run and
// provides validation logic to ensure map dimensions are safe and reasonable.
// It is a critical part of the SSXL engine’s setup pipeline, ensuring that
// generation requests do not exceed system limits.
//
// Purpose:
//   • Encapsulate generator metadata (map size, seed, generator type, overrides).
//   • Provide human-readable logging for configuration state.
//   • Validate map dimensions in terms of chunk counts before generation begins.
//
// Key Components:
//   • GeneratorConfig (struct)
//       - Holds configuration parameters for a generator run:
//           • width: map width in tiles.
//           • height: map height in tiles.
//           • seed: string used for deterministic random generation.
//           • generator_name: identifier for which generator to use.
//           • tile_overrides: optional overrides for specific tiles.
//       - Implements `Display` for human-readable logging.
//
//   • ConfigValidator (struct)
//       - Provides validation logic for `GeneratorConfig`.
//       - Ensures map dimensions are within safe bounds.
//       - Prevents runaway generation requests that could exhaust resources.
//
// Constants:
//   • MAX_ACTIVE_CHUNKS
//       - Defines the maximum number of active chunks allowed in memory.
//       - Protects against excessive generation requests.
//
// Workflow:
//   1. A `GeneratorConfig` is created with map dimensions, seed, generator name,
//      and optional tile overrides.
//   2. `ConfigValidator::validate_map_dimensions` is called before generation.
//   3. Validation steps:
//        • Convert dimensions and chunk size to i64 for safe arithmetic.
//        • Compute width and height in chunks using ceiling division.
//        • Calculate total number of chunks.
//        • Ensure total chunks > 0.
//        • Ensure total chunks ≤ MAX_ACTIVE_CHUNKS.
//   4. Log success or failure with structured messages.
//   5. Return `Ok(())` if valid, or `Err(String)` with error message if invalid.
//
// Design Choices:
//   • Ceiling division ensures partial chunks are counted as full.
//   • Logging provides traceability for both success and failure cases.
//   • Separation of configuration and validation improves modularity.
//   • Using `Display` for `GeneratorConfig` makes logs concise and readable.
//
// Educational Note:
//   • This module demonstrates how Rust can enforce safety at the configuration
//     level, preventing invalid or excessive workloads before they reach runtime.
//   • By combining strong typing, validation, and logging, it ensures reliability
//     and transparency in procedural generation workflows.
// ============================================================================


use tracing::{error, info};
use std::fmt;
use std::collections::HashMap;
use ssxl_shared::{CHUNK_SIZE, TileCoord, TileType};

const MAX_ACTIVE_CHUNKS: i64 = 100_000_000;

#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub width: usize,
    pub height: usize,
    pub seed: String,
    pub generator_name: String,
    pub tile_overrides: HashMap<TileCoord, TileType>,
}

impl fmt::Display for GeneratorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ W: {}, H: {}, Seed: '{}', Gen: '{}', Overrides: {} }}",
            self.width,
            self.height,
            self.seed,
            self.generator_name,
            self.tile_overrides.len()
        )
    }
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_map_dimensions(config: &GeneratorConfig) -> Result<(), String> {
        info!("Validating batch generation command with config: {}", config);

        let chunk_size_i64 = CHUNK_SIZE as i64;
        let map_width_i64 = config.width as i64;
        let map_height_i64 = config.height as i64;

        let width_in_chunks = (map_width_i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let height_in_chunks = (map_height_i64 + chunk_size_i64 - 1) / chunk_size_i64;
        let total_chunks = width_in_chunks * height_in_chunks;

        if total_chunks <= 0 {
            let error_msg = format!(
                "Validation Failed: Calculated chunk count is zero for size {}x{}.",
                config.width, config.height
            );
            error!("{}", error_msg);
            return Err(error_msg);
        }

        if total_chunks > MAX_ACTIVE_CHUNKS {
            let error_msg = format!(
                "Validation Failed: Max chunks limit exceeded. Requested {} chunks ({}x{}) but the limit is {}.",
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
