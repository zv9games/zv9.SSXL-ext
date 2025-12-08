// ============================================================================
// ⚙️ SSXL Engine Configuration (`ssxl_shared::config`)
// ----------------------------------------------------------------------------
// This module defines the global configuration system for the SSXL engine.
// It provides a strongly typed, thread-safe way to store and access engine
// parameters such as generation defaults and rulesets.
//
// Purpose:
//   • Centralize engine configuration in a single global object.
//   • Allow deserialization from external formats (JSON, TOML, YAML) for
//     flexible configuration management.
//   • Ensure thread-safe, one-time initialization of configuration state.
//
// Key Components:
//   • SSXLConfig
//       - Root configuration struct for the engine.
//       - Currently contains `GenerationConfig` as a nested sub-configuration.
//       - Derives `Debug`, `Deserialize`, and `Clone` for debugging, parsing,
//         and duplication.
//
//   • GenerationConfig
//       - Defines parameters for the generation subsystem.
//       - Fields:
//           • default_generator_id: string identifier for the default generator.
//           • ca_default_ruleset: numeric ID for the default Cellular Automata ruleset.
//
//   • SSXL_CONFIG_STATE
//       - A static `OnceLock<SSXLConfig>` used to hold the global configuration.
//       - Guarantees one-time initialization and thread-safe access.
//       - Prevents accidental overwrites after initialization.
//
//   • config()
//       - Global accessor function returning a reference to the initialized
//         `SSXLConfig`.
//       - Panics if called before initialization, enforcing proper setup.
//
//   • SSXLConfig Implementation
//       - Provides helper methods for convenient access to specific values.
//       - Example: `get_ca_default_ruleset()` returns the default CA ruleset ID.
//
// Workflow:
//   1. Configuration is deserialized from a file or provided at runtime.
//   2. `SSXL_CONFIG_STATE` is initialized once with the parsed `SSXLConfig`.
//   3. The `config()` function provides read-only global access.
//   4. Helper methods allow subsystems to query configuration values directly.
//
// Design Choices:
//   • `serde::Deserialize` enables flexible parsing from multiple file formats.
//   • `OnceLock` ensures safe, one-time initialization without global mutability.
//   • Inline accessors (`#[inline(always)]`) optimize performance for frequent
//     configuration lookups.
//   • Modular design allows future expansion (e.g., additional subsystems).
//
// Educational Note:
//   • This module demonstrates how Rust can enforce safe global state management
//     using `OnceLock` instead of unsafe statics.
//   • By centralizing configuration here, the SSXL engine gains consistency,
//     safety, and flexibility in managing runtime parameters.
// ============================================================================


use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct SSXLConfig {
    pub generation: GenerationConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GenerationConfig {
    pub default_generator_id: String,
    pub ca_default_ruleset: u8,
}

static SSXL_CONFIG_STATE: OnceLock<SSXLConfig> = OnceLock::new();

#[inline(always)]
pub fn config() -> &'static SSXLConfig {
    SSXL_CONFIG_STATE.get().expect("SSXLConfig uninitialized: Call init_config() first.")
}

impl SSXLConfig {
    #[inline(always)]
    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.generation.ca_default_ruleset
    }
}
