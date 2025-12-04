// ssxl_config/src/config.rs (Final O(1) State)

use serde::Deserialize;
use std::sync::OnceLock;
// Assumes 'info!' and 'warn!' are globally available macros or imports.

// --- CONFIG STRUCTS (Zero Entropy Mapping) ---

#[derive(Debug, Deserialize, Clone)]
pub struct SSXLConfig {
    pub generation: GenerationConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GenerationConfig {
    pub default_generator_id: String,
    pub ca_default_ruleset: u8, // Maps to 1
}

// --- O(1) STATE INJECTION ---

static SSXL_CONFIG_STATE: OnceLock<SSXLConfig> = OnceLock::new();

/// O(1) Accessor: Returns the global config.
#[inline(always)]
pub fn config() -> &'static SSXLConfig {
    // Guarantees O(1) read time after initialization.
    SSXL_CONFIG_STATE.get().expect("SSXLConfig uninitialized: Call init_config() first.")
}

impl SSXLConfig {
    /// O(1) Accessor: Returns the configured default Cellular Automata ruleset ID.
    #[inline(always)]
    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.generation.ca_default_ruleset
    }

    // ... (Implement load_from_path and new_with_defaults here)
    // ... (The previous load logic should be moved into an associated function)
}

// --- INITIALIZATION (O(N) One-Time Cost) ---
// (Move the rest of the 'init_config' function logic here)