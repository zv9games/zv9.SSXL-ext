// rust/SSXL-ext/src/config.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::default::Default; // Required for Default implementation

// --- FIX: Remove Godot-dependent macro imports ---
// We remove the imports for ssxl_info, ssxl_warn, ssxl_error
// and replace them with eprintln! in the logic below.

use crate::shared_error::SSXLCoreError;

// Import the configuration structs from the designated shared location
use crate::shared_config::{
    ThreadingConfig, MapSettingsConfig, GenerationConfig, AnimationConfig // Assuming AnimationConfig is here too
};

// --------------------------------------------------------------------------
// --- GlobalConfig Structure (The Root) ---
// --------------------------------------------------------------------------

/// The main configuration loaded at runtime by host_init.rs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    #[serde(default)]
    pub threading: ThreadingConfig,
    #[serde(default)]
    pub map_settings: MapSettingsConfig,
    #[serde(default)]
    pub generation: GenerationConfig,
    // Include AnimationConfig for completeness based on the manifest
    #[serde(default)]
    pub animation: AnimationConfig,
}

impl GlobalConfig {
    const CONFIG_FILE_PATH: &'static str = "ssxl_config.toml";
    
    /// Attempts to load the configuration from a file, falling back to defaults 
    /// and saving the default if the file is missing.
    pub fn load_or_default() -> Result<Self, SSXLCoreError> {
        // FIX: Use standard console output (eprintln!) instead of ssxl_info!
        eprintln!("INFO: Attempting to load configuration from: {}.", Self::CONFIG_FILE_PATH);

        match fs::read_to_string(Self::CONFIG_FILE_PATH) {
            Ok(content) => {
                // Deserialize the TOML content into the GlobalConfig struct
                match toml::from_str(&content) {
                    Ok(config) => {
                        // FIX: Use standard console output
                        eprintln!("INFO: Successfully loaded SSXL configuration.");
                        Ok(config)
                    }
                    Err(e) => {
                        // FIX: Use standard console output
                        eprintln!("ERROR: Failed to parse TOML configuration: {}. Using default settings.", e);
                        // Use SSXLCoreError to wrap the TOML parsing failure
                        Err(SSXLCoreError::InvalidConfig(format!("TOML deserialization failed: {}", e)))
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // File not found, use default settings
                let default_config = Self::default();
                // FIX: Use standard console output
                eprintln!("WARN: Configuration file not found ({}). Using default settings.", Self::CONFIG_FILE_PATH);

                // Optionally write the default config back to disk for easy editing
                if default_config.save_to_disk().is_err() {
                    // FIX: Use standard console output
                    eprintln!("ERROR: Could not save default configuration file.");
                }
                
                Ok(default_config)
            }
            Err(e) => {
                // FIX: Use standard console output
                eprintln!("ERROR: I/O Error reading config file: {}", e);
                Err(SSXLCoreError::FFIWriteError(format!("Config file I/O failure: {}", e)))
            }
        }
    }
    
    /// Saves the current configuration structure back to the TOML file.
    pub fn save_to_disk(&self) -> Result<(), SSXLCoreError> {
        match toml::to_string_pretty(self) {
            Ok(toml_string) => {
                match fs::write(Self::CONFIG_FILE_PATH, toml_string) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        Err(SSXLCoreError::FFIWriteError(format!("Failed to write config file: {}", e)))
                    }
                }
            },
            Err(e) => {
                Err(SSXLCoreError::InvalidConfig(format!("Failed to serialize TOML: {}", e)))
            }
        }
    }
}

// Provide sensible defaults for easy startup
impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            // Note: These must rely on the Default impls being in `shared_config.rs`
            threading: ThreadingConfig::default(),
            map_settings: MapSettingsConfig::default(),
            generation: GenerationConfig::default(),
            animation: AnimationConfig::default(),
        }
    }
}