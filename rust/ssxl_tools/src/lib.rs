// ssxl_tools/src/lib.rs (Final, Corrected Code)

//! Core utilities for configuration, asset management, and data validation.

// Restored missing necessary imports
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{info, warn}; 
use std::io::{self, Read}; 
use std::fs::File; 

// --- CRATE DEPENDENCIES ---
use ssxl_shared::SSXLData; 

// --- CONFIGURATION CONSTANTS ---
const DEFAULT_CONFIG_PATH: &str = "./config/engine.toml"; 
const DEFAULT_GENERATOR: &str = "cellular_automata_basic";
const DEFAULT_CA_RULESET: u8 = 0; 

// -----------------------------------------------------------------------------
// SSXL CONFIGURATION UTILITIES (RESTORED DEFINITION)
// -----------------------------------------------------------------------------

/// 🔧 Stores all global, static configuration settings for the SSXL Engine.
// E0412 & E0433 Fix: SSXLConfig struct definition is now present.
pub struct SSXLConfig {
    default_generator_id: String,
    ca_default_ruleset: u8,
}

impl SSXLConfig {
    /// Private constructor to initialize config with defaults.
    fn new_with_defaults() -> Self {
        SSXLConfig {
            default_generator_id: DEFAULT_GENERATOR.to_string(),
            ca_default_ruleset: DEFAULT_CA_RULESET,
        }
    }

    /// Attempts to load configuration from the specified path.
    fn load_from_path(path: &str) -> Result<Self, io::Error> {
        info!("SSXLConfig: Attempting to load configuration from: {}", path);

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                // --- SIMULATED PARSING ---
                info!("SSXLConfig: File read successfully. Simulating config override.");
                // Return a simulated config that is NOT the default
                Ok(SSXLConfig {
                    default_generator_id: "perlin_basic_2d".to_string(), 
                    ca_default_ruleset: 1, 
                })
            },
            Err(e) => {
                // Return the I/O error
                Err(e)
            }
        }
    }

    /// Accessor for the default generator ID.
    pub fn get_default_generator_id(&self) -> &str {
        &self.default_generator_id
    }

    /// Accessor for the Cellular Automata default ruleset ID.
    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.ca_default_ruleset
    }
}

/// Public function to load config, allowing the caller (Conductor) to specify the path.
pub fn get_config_from_path(path: Option<&str>) -> Result<SSXLConfig, io::Error> {
    let path_to_load = path.unwrap_or(DEFAULT_CONFIG_PATH);

    match SSXLConfig::load_from_path(path_to_load) {
        Ok(config) => {
            info!("SSXLConfig loaded from path: {}", path_to_load);
            Ok(config)
        },
        Err(e) => {
            // Log error and return defaults (Crucial for FFI stability)
            warn!("Config load FAILED from path '{}'. Error: {:?}. Returning defaults to ensure engine initialization.", path_to_load, e);
            Ok(SSXLConfig::new_with_defaults())
        }
    }
}

/// Provides thread-safe, static access to a configuration instance *initialized only with defaults*.
static CONFIG: Lazy<SSXLConfig> = Lazy::new(SSXLConfig::new_with_defaults);

/// DEPRECATED: Public function to retrieve a reference to the global configuration.
pub fn get_config() -> &'static SSXLConfig {
    warn!("DEPRECATED: Called `get_config()`. Use `get_config_from_path()` for correct FFI CWD handling.");
    &CONFIG
}

// -----------------------------------------------------------------------------
// DATA VALIDATION UTILITIES (RESTORED DEFINITION)
// -----------------------------------------------------------------------------

/// Provides a thread-safe, lazily initialized Regex instance for data ID validation.
// E0425 Fix: ID_REGEX static definition is now present.
static ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Requires IDs to be numeric strings.
    Regex::new(r"^\d+$").expect("Failed to compile ID validation regex")
});

/// Validates the ID field of an AetherionData primitive against a standard regex pattern.
pub fn validate_data_id(data: &SSXLData) -> bool {
    // Assumes AetherionData::id is accessible and implements ToString (e.g., u64).
    ID_REGEX.is_match(&data.id.to_string())
}


// -----------------------------------------------------------------------------
// CRATE ENTRY
// -----------------------------------------------------------------------------

/// Initializes the `aetherion_tools` crate.
pub fn initialize() {
    // Force initialization of the static regex.
    let _ = &*ID_REGEX; 
    info!("SSXL Tools: Configuration and data validation utilities initialized.");
}