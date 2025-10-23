// c:/zv9/zv9.aetherion/rust/src/zv9_trailkeeper_entry.rs
#[allow(unused_imports)]
use aetherion_core::zv9_prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Build,
    Deploy,
    FileChange,
    ConfigChange,
    GitDiff,
    ArtifactDetected,
	System,
	StructurePlacement,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum LogStatus {
    Success,
    Failure,
    Detected,
    Ignored,
    Info,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub description: String,
    pub affected_components: Vec<String>,
    pub status: LogStatus,
}

//the end