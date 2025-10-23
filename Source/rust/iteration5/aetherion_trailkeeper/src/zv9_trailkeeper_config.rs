
#[allow(unused_imports)]
use crate::zv9_prelude::*;
use crate::zv9_trailkeeper_collector::Trailkeeper;
use crate::zv9_trailkeeper_entry::{LogEntry, EventType, LogStatus};

use chrono::Utc;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

static CONFIG_PATH: &str = "config.toml";
static HASH_STORE: &str = ".trailkeeper_config_hash";

pub fn check_config_change() {
    if !Path::new(CONFIG_PATH).exists() {
        return;
    }

    let config_data = fs::read(CONFIG_PATH).expect("Failed to read config");
    let mut hasher = Sha256::new();
    hasher.update(&config_data);
    let current_hash = format!("{:x}", hasher.finalize());

    let previous_hash = fs::read_to_string(HASH_STORE).unwrap_or_default();

    if current_hash != previous_hash {
        Trailkeeper::record(LogEntry {
            event_type: EventType::ConfigChange,
            timestamp: Utc::now(),
            actor: "system".to_string(),
            description: "Config file changed".to_string(),
            affected_components: vec![CONFIG_PATH.to_string()],
            status: LogStatus::Detected,
        });

        fs::write(HASH_STORE, current_hash).expect("Failed to write hash");
    }
}
