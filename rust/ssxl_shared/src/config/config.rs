// -----------------------------------------------------------------------------
// Global Configuration Module Overview
// -----------------------------------------------------------------------------
// This module defines the configuration structure and constants that govern
// the SSXL engine’s behavior. It ensures consistent values across crates
// (math, generate, cache, godot) and provides safe defaults when loading fails.
//
// Key Components:
// - SSXLConfig: Struct holding runtime configuration settings.
// - CHUNK_SIZE / TILE_ARRAY_SIZE: Constants defining chunk geometry.
// - DEFAULT_CONFIG_PATH: Default path for configuration file.
// - new_with_defaults: Provides safe fallback values.
// - load_from_path: Attempts to load configuration (placeholder implementation).
// - default_generator_id: Accessor for generator ID.
// - get_config_from_path: Public function to load configuration safely.
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------
// tracing::{info, warn}
//   - Logging macros for runtime diagnostics (info-level success, warn-level failure).
// std::error::Error
//   - Trait object used for error handling in load_from_path.
// serde::{Deserialize, Serialize}
//   - Enables serialization/deserialization of SSXLConfig for persistence and loading.
use tracing::{info, warn};
use std::error::Error;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Struct: SSXLConfig
// -----------------------------------------------------------------------------
// Purpose:
//   - Holds runtime configuration settings for the SSXL engine.
// Fields:
//   - ca_default_ruleset: Default ruleset ID for cellular automata generation.
//   - default_generator_id: Identifier for the default generator used in world creation.
// Derives:
//   - Debug, Clone: For inspection and duplication.
//   - Serialize, Deserialize: For persistence and loading from config files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSXLConfig {
    pub ca_default_ruleset: u8,
    pub default_generator_id: String,
}

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------
// CHUNK_SIZE
//   - Canonical side length of a chunk in tiles (32).
// TILE_ARRAY_SIZE
//   - Total number of tiles in a chunk (32 * 32 = 1024).
// DEFAULT_CONFIG_PATH
//   - Default file path for configuration JSON.
pub const CHUNK_SIZE: u32 = 32;
pub const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;
const DEFAULT_CONFIG_PATH: &str = "res://ssxl_config.json";

// -----------------------------------------------------------------------------
// Implementation: SSXLConfig
// -----------------------------------------------------------------------------
impl SSXLConfig {
    // -------------------------------------------------------------------------
    // Method: new_with_defaults
    // -------------------------------------------------------------------------
    // Provides a safe, hardcoded default configuration.
    // Ensures engine can initialize even if config file is missing or invalid.
    pub fn new_with_defaults() -> Self {
        SSXLConfig {
            ca_default_ruleset: 1,
            default_generator_id: "default_noise_gen".to_string(),
        }
    }

    // -------------------------------------------------------------------------
    // Method: load_from_path
    // -------------------------------------------------------------------------
    // Attempts to load configuration from a file path.
    // Currently a placeholder: always returns defaults.
    // Returns:
    //   - Ok(Self) on success
    //   - Err(Box<dyn Error>) on failure
    pub fn load_from_path(_path: &str) -> Result<Self, Box<dyn Error>> {
        Ok(SSXLConfig::new_with_defaults())
    }
    
    // -------------------------------------------------------------------------
    // Method: default_generator_id
    // -------------------------------------------------------------------------
    // Accessor for the default generator ID.
    // Returns a clone of the string to avoid ownership issues.
    pub fn default_generator_id(&self) -> String {
        self.default_generator_id.clone()
    }
}

// -----------------------------------------------------------------------------
// Function: get_config_from_path
// -----------------------------------------------------------------------------
// Purpose:
//   - Public entry point for loading configuration.
//   - Attempts to load from provided path or falls back to DEFAULT_CONFIG_PATH.
// Behavior:
//   - On success: logs info and returns loaded config.
//   - On failure: logs warning and returns safe defaults.
// Ensures engine always initializes with valid configuration.
pub fn get_config_from_path(path: Option<&str>) -> SSXLConfig {
    let path_to_load = path.unwrap_or(DEFAULT_CONFIG_PATH);

    match SSXLConfig::load_from_path(path_to_load) {
        Ok(config) => {
            info!("SSXLConfig loaded from path: {}", path_to_load);
            config
        },
        Err(e) => {
            warn!(
                "Config load FAILED from path '{}'. Error: {:?}. Returning defaults to ensure engine initialization.",
                path_to_load,
                e
            );
            SSXLConfig::new_with_defaults()
        }
    }
}
