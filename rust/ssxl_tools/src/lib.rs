// ssxl_tools/src/lib.rs

//! # SSXL Engine Tools (`ssxl_tools`)
//!
//! Provides utility functions for **configuration management**, **data validation**,
//! and other engine-wide tooling not specific to generation or synchronization.

use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{info, warn};
use std::io::{self, Read};
use std::fs::File;

use ssxl_shared::SSXLData;

// --------------------------------------------------------------------------------
// --- Configuration Constants ---
// --------------------------------------------------------------------------------

/// The default file path to check for engine configuration.
const DEFAULT_CONFIG_PATH: &str = "./config/engine.toml";
/// The ID of the generator used when configuration loading fails or is not specified.
const DEFAULT_GENERATOR: &str = "cellular_automata_basic";
/// The default Cellular Automata ruleset ID to use.
const DEFAULT_CA_RULESET: u8 = 0;

// --------------------------------------------------------------------------------
// --- SSXL Configuration Management ---
// --------------------------------------------------------------------------------

/// Configuration structure holding key engine settings, primarily for generation defaults.
pub struct SSXLConfig {
    /// The ID of the world generator to use when no specific one is requested.
    default_generator_id: String,
    /// The default ruleset ID for the Cellular Automata generator.
    ca_default_ruleset: u8,
}

impl SSXLConfig {
    /// Creates a new `SSXLConfig` instance populated with hardcoded default values.
    fn new_with_defaults() -> Self {
        SSXLConfig {
            default_generator_id: DEFAULT_GENERATOR.to_string(),
            ca_default_ruleset: DEFAULT_CA_RULESET,
        }
    }

    /// Attempts to read and simulate loading engine configuration from a file path.
    ///
    /// The actual configuration parsing logic is currently simulated:
    /// it reads the file and then unconditionally returns hardcoded override values.
    fn load_from_path(path: &str) -> Result<Self, io::Error> {
        info!("SSXLConfig: Attempting to load configuration from: {}", path);

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                // Read the entire file content into a string.
                file.read_to_string(&mut contents)?;

                info!("SSXLConfig: File read successfully. Simulating config override.");
                // Placeholder: In a real implementation, 'contents' would be parsed (e.g., via TOML).
                Ok(SSXLConfig {
                    default_generator_id: "perlin_basic_2d".to_string(), // Simulated override
                    ca_default_ruleset: 1,                               // Simulated override
                })
            },
            // If file opening fails, propagate the standard I/O error.
            Err(e) => Err(e),
        }
    }

    /// Returns the configured default generator ID string.
    pub fn get_default_generator_id(&self) -> &str {
        &self.default_generator_id
    }

    /// Returns the configured default Cellular Automata ruleset ID.
    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.ca_default_ruleset
    }
}

/// Attempts to load the configuration from the specified path.
///
/// If `path` is `None`, it defaults to `DEFAULT_CONFIG_PATH`.
/// If the file loading fails for any reason, it logs a warning and **returns
/// a new `SSXLConfig` instance populated with hardcoded defaults** (safe fallback).
pub fn get_config_from_path(path: Option<&str>) -> Result<SSXLConfig, io::Error> {
    let path_to_load = path.unwrap_or(DEFAULT_CONFIG_PATH);

    match SSXLConfig::load_from_path(path_to_load) {
        Ok(config) => {
            info!("SSXLConfig loaded from path: {}", path_to_load);
            Ok(config)
        },
        Err(e) => {
            // Log the failure but ensure the engine has a runnable configuration.
            warn!("Config load FAILED from path '{}'. Error: {:?}. Returning defaults to ensure engine initialization.", path_to_load, e);
            // Return the defaults wrapped in an Ok() to ensure initialization succeeds.
            Ok(SSXLConfig::new_with_defaults())
        }
    }
}

// --------------------------------------------------------------------------------
// --- Static Configuration (DEPRECATED) ---
// --------------------------------------------------------------------------------

/// Lazy-initialized static instance of the default configuration.
///
/// Used by the deprecated `get_config()` function.
static CONFIG: Lazy<SSXLConfig> = Lazy::new(SSXLConfig::new_with_defaults);

/// Retrieves the global static configuration instance.
///
/// **WARNING:** This function is **DEPRECATED**. It does not allow for specifying
/// a configuration path and relies on a hardcoded static default.
/// Use [`get_config_from_path`] instead.
pub fn get_config() -> &'static SSXLConfig {
    warn!("DEPRECATED: Called `get_config()`. Use `get_config_from_path()` for correct FFI CWD handling and dynamic loading.");
    &CONFIG
}

// --------------------------------------------------------------------------------
// --- Utility Functions ---
// --------------------------------------------------------------------------------

/// Lazy-initialized regular expression for validating SSXL data IDs (must be composed only of digits).
static ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Pattern matches one or more digits from start to end of the string.
    Regex::new(r"^\d+$").expect("Failed to compile ID validation regex")
});

/// Validates that an `SSXLData` ID is composed solely of digits.
pub fn validate_data_id(data: &SSXLData) -> bool {
    // Checks if the string representation of the data ID matches the digit-only regex.
    ID_REGEX.is_match(&data.id.to_string())
}

/// Initializes engine tool utilities.
///
/// This function triggers the lazy initialization of necessary static resources,
/// such as the ID validation regex, ensuring they are ready before first use.
pub fn initialize() {
    // Force initialization of the static regex.
    let _ = &*ID_REGEX;
    info!("SSXL Tools: Configuration and data validation utilities initialized.");
}