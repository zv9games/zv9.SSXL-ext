use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct SsxlConfig {
    pub godot: GodotConfig,
}

#[derive(Debug, Deserialize)]
pub struct GodotConfig {
    pub binary: String,
}

impl SsxlConfig {
    pub fn load() -> Self {
        let raw = fs::read_to_string("ssxl_config.toml")
            .expect("Failed to read ssxl_config.toml");

        toml::from_str(&raw)
            .expect("Failed to parse ssxl_config.toml")
    }
}
