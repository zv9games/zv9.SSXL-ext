// rust/SSXL-ext/src/config.rs

use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;

use crate::shared_error::SSXLCoreError;
use crate::shared_config::{
    ThreadingConfig,
    MapSettingsConfig,
    GenerationConfig,
    AnimationConfig,
};

// --------------------------------------------------------------------------
// Global debug toggles (engine-wide)
// --------------------------------------------------------------------------
//
// These flags control ALL debug output across the Rust backend.
// Set to `true` to enable verbose logging for that subsystem.
// --------------------------------------------------------------------------

// --- Renderer subsystem ---
pub const DEBUG_RENDERER: bool = false;            // renderer_node.rs (apply_chunk, transforms)
pub const DEBUG_CHUNK_MESH: bool = false;          // chunk_mesh.rs
pub const DEBUG_MESH_BUILDER: bool = false;        // mesh_builder.rs
pub const DEBUG_MESH_UPLOAD: bool = false;         // mesh_upload.rs
pub const DEBUG_ATLAS: bool = false;               // atlas.rs
pub const DEBUG_RENDER_PACING: bool = false;

// --- Conductor / Host subsystem ---
pub const DEBUG_CONDUCTOR: bool = false;           // host_conductor.rs
pub const DEBUG_POLLER: bool = false;              // host_poller.rs
pub const DEBUG_HOST_INIT: bool = false;           // host_init.rs
pub const DEBUG_HOST_RENDER: bool = false;         // host_render.rs
pub const DEBUG_HOST_TILEMAP: bool = false;        // host_tilemap.rs
pub const DEBUG_HOST_STATE: bool = false;          // host_state.rs

// --- Generation pipeline ---
pub const DEBUG_BATCH_PROCESSOR: bool = false;     // generate_batch_processor.rs
pub const DEBUG_GENERATION_RUNTIME: bool = false;  // generate_runtime.rs
pub const DEBUG_GENERATION_PERLIN: bool = false;   // generate_perlin.rs
pub const DEBUG_GENERATION_CA: bool = false;       // generate_ca.rs
pub const DEBUG_GENERATION_MANAGER: bool = false;  // generate_manager.rs

// --- Chunk buffer subsystem ---
pub const DEBUG_CHUNK_BUFFER: bool = false;        // ssxl_chunk_buffer.rs

// --- Controller / Pipeline ---
pub const DEBUG_CONTROLLER: bool = false;          // SSXLController (Rust side)
pub const DEBUG_PIPELINE: bool = false;            // high-level pipeline logs


// --------------------------------------------------------------------------
// GlobalConfig structure (the root)
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    #[serde(default)]
    pub threading: ThreadingConfig,
    #[serde(default)]
    pub map_settings: MapSettingsConfig,
    #[serde(default)]
    pub generation: GenerationConfig,
    #[serde(default)]
    pub animation: AnimationConfig,
}

impl GlobalConfig {
    const CONFIG_FILE_PATH: &'static str = "ssxl_config.toml";

    pub fn load_or_default() -> Result<Self, SSXLCoreError> {
        eprintln!(
            "INFO: Attempting to load configuration from: {}.",
            Self::CONFIG_FILE_PATH
        );

        match fs::read_to_string(Self::CONFIG_FILE_PATH) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => {
                    eprintln!("INFO: Successfully loaded SSXL configuration.");
                    Ok(config)
                }
                Err(e) => {
                    eprintln!(
                        "ERROR: Failed to parse TOML configuration: {}. Using default settings.",
                        e
                    );
                    Err(SSXLCoreError::InvalidConfig(format!(
                        "TOML deserialization failed: {}",
                        e
                    )))
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let default_config = Self::default();
                eprintln!(
                    "WARN: Configuration file not found ({}). Using default settings.",
                    Self::CONFIG_FILE_PATH
                );

                if default_config.save_to_disk().is_err() {
                    eprintln!("ERROR: Could not save default configuration file.");
                }

                Ok(default_config)
            }
            Err(e) => {
                eprintln!("ERROR: I/O Error reading config file: {}", e);
                Err(SSXLCoreError::FFIWriteError(format!(
                    "Config file I/O failure: {}",
                    e
                )))
            }
        }
    }

    pub fn save_to_disk(&self) -> Result<(), SSXLCoreError> {
        match toml::to_string_pretty(self) {
            Ok(toml_string) => match fs::write(Self::CONFIG_FILE_PATH, toml_string) {
                Ok(_) => Ok(()),
                Err(e) => Err(SSXLCoreError::FFIWriteError(format!(
                    "Failed to write config file: {}",
                    e
                ))),
            },
            Err(e) => Err(SSXLCoreError::InvalidConfig(format!(
                "Failed to serialize TOML: {}",
                e
            ))),
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            threading: ThreadingConfig::default(),
            map_settings: MapSettingsConfig::default(),
            generation: GenerationConfig::default(),
            animation: AnimationConfig::default(),
        }
    }
}
