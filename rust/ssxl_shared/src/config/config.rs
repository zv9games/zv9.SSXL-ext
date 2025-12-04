// FILE: ssxl_shared/src/config.rs

//! # Global Configuration Constants (`ssxl_shared::config`)
//!
//! This module defines fundamental, immutable constants that govern the structure
//! and scale of the SSXL procedural world. These values must be consistent
//! across all SSXL-ext crates (math, generate, cache, godot) to ensure data
//! integrity and system entropy is controlled.

use tracing::{info, warn};
use std::error::Error;
use serde::{Deserialize, Serialize}; // Assume Serde is used for configuration loading

// --- Config Struct Definition ---

/// Structure holding all runtime configuration settings for the SSXL Engine.
/// **NOTE:** This must be public to be used by other crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSXLConfig {
    // Example field required by other code snippets (from new_with_defaults)
    pub ca_default_ruleset: u8,
    
    // FIX 1: Add the required field for the default generator ID.
    pub default_generator_id: String,
}

// --- World Geometry Constants ---

/// The canonical side length of a procedural chunk in tiles.
/// **Value:** 32 (meaning chunks are 32x32 tiles).
pub const CHUNK_SIZE: u32 = 32;

/// The total number of tiles contained within a single `ChunkData` structure.
/// **Calculation:** CHUNK_SIZE * CHUNK_SIZE (32 * 32 = 1024).
pub const TILE_ARRAY_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;

// --- Loading Constants ---
const DEFAULT_CONFIG_PATH: &str = "res://ssxl_config.json";

impl SSXLConfig {
    /// Creates a safe, hardcoded default configuration.
    pub fn new_with_defaults() -> Self {
        SSXLConfig {
            ca_default_ruleset: 1, // Default CA ruleset ID
            
            // FIX 2: Initialize the new field.
            default_generator_id: "default_noise_gen".to_string(), 
        }
    }

    /// Internal method to attempt loading config from a file path.
    /// Returns `Ok(Self)` or an `Err` on failure. (Placeholder implementation).
    pub fn load_from_path(_path: &str) -> Result<Self, Box<dyn Error>> {
        // In a real application, this would handle file I/O and deserialization.
        // For now, we simulate success with defaults.
        Ok(SSXLConfig::new_with_defaults())
    }
    
    // FIX 3: Add the missing accessor method to resolve E0599.
    /// Returns the configured default Generator ID.
    pub fn default_generator_id(&self) -> String {
        self.default_generator_id.clone()
    }
}


/// Attempts to load the configuration from the specified path.
///
/// **FIX:** This function is defined publicly here, resolving `E0425` in `api_initializers.rs`.
pub fn get_config_from_path(path: Option<&str>) -> SSXLConfig {
    let path_to_load = path.unwrap_or(DEFAULT_CONFIG_PATH);

    match SSXLConfig::load_from_path(path_to_load) {
        Ok(config) => {
            info!("SSXLConfig loaded from path: {}", path_to_load);
            config
        },
        Err(e) => {
            // Safe fallback: Logs failure but ensures the engine initializes with defaults.
            warn!("Config load FAILED from path '{}'. Error: {:?}. Returning defaults to ensure engine initialization.", path_to_load, e);
            SSXLConfig::new_with_defaults()
        }
    }
}