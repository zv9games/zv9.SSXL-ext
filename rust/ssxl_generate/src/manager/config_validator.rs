use tracing::{error, info};
use std::fmt;
use std::collections::HashMap;

// FIX: Import CHUNK_SIZE directly from the ssxl_shared crate root.
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

        // The ceiling division to calculate the number of chunks: (dividend + divisor - 1) / divisor
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