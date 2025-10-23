

#[allow(unused_imports)]
use crate::zv9_prelude::*;
#[allow(unused_imports)]
use crate::zv9_trailkeeper_entry::{LogEntry, EventType, LogStatus};use crate::zv9_trailkeeper_collector::Trailkeeper;use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct SerializableLogEntry {
    title: String,
    timestamp: String,
    actor: String,
    description: String,
    affected_components: Vec<String>,
    status: String,
}

pub fn export_json(path: &str) {
    let logs: Vec<LogEntry> = Trailkeeper::all();


    let serialized: Vec<SerializableLogEntry> = logs.iter().map(|entry| SerializableLogEntry {
        title: format!("{:?}", entry.event_type),
        timestamp: entry.timestamp.to_rfc3339(),
        actor: entry.actor.clone(),
        description: entry.description.clone(),
        affected_components: entry.affected_components.clone(),
        status: format!("{:?}", entry.status),
    }).collect();

    let json = serde_json::to_string_pretty(&serialized)
        .expect("Failed to serialize logs");

    let mut file = File::create(path)
        .expect("Failed to create export file");

    file.write_all(json.as_bytes())
        .expect("Failed to write logs");
}
