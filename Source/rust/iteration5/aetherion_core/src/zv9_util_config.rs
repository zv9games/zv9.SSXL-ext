#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use serde::{Deserialize, Serialize};

/// ⚙️ Configuration for the Aetherion engine runtime.
/// Can be loaded from external files or constructed programmatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Target tick rate (frames per second).
    pub tick_rate: u32,

    /// Maximum number of threads for procedural generation.
    pub max_threads: usize,

    /// Enables verbose logging and diagnostics.
    pub enable_logging: bool,

    /// Delivery pacing interval in milliseconds.
    pub interval_ms: u64,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            tick_rate: 60,
            max_threads: 4,
            enable_logging: true,
            interval_ms: 16, // ~60 FPS pacing
        }
    }
}

impl EngineConfig {
    /// Returns the tick duration in seconds.
    pub fn tick_interval(&self) -> f64 {
        1.0 / self.tick_rate as f64
    }

    /// Returns true if multithreading is enabled.
    pub fn is_multithreaded(&self) -> bool {
        self.max_threads > 1
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_default_config_values() {
        let config = EngineConfig::default();
        assert_eq!(config.tick_rate, 60);
        assert_eq!(config.max_threads, 4);
        assert!(config.enable_logging);
        assert_eq!(config.interval_ms, 16);
    }

    #[test]
    fn stress_tick_interval_calculation() {
        let config = EngineConfig { tick_rate: 120, ..Default::default() };
        let interval = config.tick_interval();
        assert!((interval - 0.0083).abs() < 0.001); // ~1/120
    }

    #[test]
    fn stress_multithreaded_flag() {
        let single_thread = EngineConfig { max_threads: 1, ..Default::default() };
        let multi_thread = EngineConfig { max_threads: 8, ..Default::default() };

        assert!(!single_thread.is_multithreaded());
        assert!(multi_thread.is_multithreaded());
    }

    #[test]
    fn stress_custom_config_variants() {
        let config = EngineConfig {
            tick_rate: 30,
            max_threads: 16,
            enable_logging: false,
            interval_ms: 33,
        };

        assert_eq!(config.tick_rate, 30);
        assert_eq!(config.max_threads, 16);
        assert!(!config.enable_logging);
        assert_eq!(config.interval_ms, 33);
    }
}
