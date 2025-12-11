// ============================================================================
// ⚙️ SSXL Engine Configuration (`ssxl_tools::config`)
// ----------------------------------------------------------------------------
// This module defines the configuration management system for the SSXL engine.
// It provides a lightweight way to load, validate, and fall back to defaults
// when configuration files are missing or invalid.
//
// Key Concepts:
//   • SSXLConfig struct:
//       - Holds core engine settings such as the default world generator ID
//         and the default Cellular Automata ruleset.
//       - Designed to be clonable and debuggable for easy inspection.
//   • Default constants:
//       - DEFAULT_CONFIG_PATH: canonical location of the engine’s config file.
//       - DEFAULT_GENERATOR: fallback generator ID if none is specified.
//       - DEFAULT_CA_RULESET: fallback ruleset ID for Cellular Automata.
//   • Fallback strategy:
//       - If configuration loading fails, the engine logs a warning and
//         initializes with hardcoded defaults to guarantee safe startup.
//       - This ensures the engine can always run, even in environments where
//         configuration files are missing or corrupted.
//
// Design Choices:
//   • File I/O is simulated (no TOML parsing yet) to minimize complexity.
//     - The system checks for file existence, then applies a simulated override.
//     - This allows developers to test configuration flow without full parsing.
//   • Logging via `tracing`:
//     - `info!` logs successful load attempts.
//     - `warn!` logs failures and fallback usage.
//   • Accessor methods (`get_default_generator_id`, `get_ca_default_ruleset`)
//     provide O(1) retrieval of settings, keeping runtime overhead negligible.
//
// Workflow:
//   1. Engine calls `get_config_from_path()` with an optional path.
//   2. If the file exists, a simulated override config is returned.
//   3. If the file does not exist, defaults are returned with a warning.
//   4. Downstream systems (generation, simulation) query the config via
//      accessor methods to determine which generator and ruleset to use.
//
// Educational Note:
//   • This module demonstrates a common pattern in engine design:
//       - Centralized configuration management.
//       - Safe fallbacks to ensure robustness.
//       - Clear logging for observability.
//   • Even though parsing is simulated here, the structure is ready to be
//     extended with real TOML/JSON/YAML parsing in the future.
// ============================================================================


use tracing::{info, warn};
use std::io;
use std::fs::File;

const DEFAULT_CONFIG_PATH: &str = "./config/engine.toml";
const DEFAULT_GENERATOR: &str = "cellular_automata_basic";
const DEFAULT_CA_RULESET: u8 = 0;

#[derive(Debug, Clone)]
pub struct SSXLConfig {
    default_generator_id: String,
    ca_default_ruleset: u8,
}

impl SSXLConfig {
    fn new_with_defaults() -> Self {
        SSXLConfig {
            default_generator_id: DEFAULT_GENERATOR.to_string(),
            ca_default_ruleset: DEFAULT_CA_RULESET,
        }
    }

    fn load_from_path(path: &str) -> Result<Self, io::Error> {
        info!("SSXLConfig: Attempting to load configuration from: {}", path);

        match File::open(path) {
            Ok(_file) => {
                info!("SSXLConfig: Config file found. Simulating config override (No TOML parsing).");
                Ok(SSXLConfig {
                    default_generator_id: "perlin_basic_2d".to_string(),
                    ca_default_ruleset: 1,
                })
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_default_generator_id(&self) -> &str {
        &self.default_generator_id
    }

    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.ca_default_ruleset
    }
}

pub fn get_config_from_path(path: Option<&str>) -> SSXLConfig {
    let path_to_load = path.unwrap_or(DEFAULT_CONFIG_PATH);

    match SSXLConfig::load_from_path(path_to_load) {
        Ok(config) => {
            info!("SSXLConfig loaded from path: {}", path_to_load);
            config
        },
        Err(e) => {
            warn!("Config load FAILED from path '{}'. Error: {:?}. Returning defaults to ensure engine initialization.", path_to_load, e);
            SSXLConfig::new_with_defaults()
        }
    }
}
