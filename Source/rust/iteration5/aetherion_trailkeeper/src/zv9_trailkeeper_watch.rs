

#[allow(unused_imports)]
use crate::zv9_prelude::*;
use ::notify::{RecommendedWatcher, Error, RecursiveMode}; // Disambiguated crate root
use chrono::Utc;
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};
use crate::zv9_trailkeeper_collector::Trailkeeper;
use crate::zv9_trailkeeper_entry::{LogEntry, EventType, LogStatus};


use std::sync::Mutex;
use std::time::Duration;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref FILE_WATCHER: Mutex<Option<Debouncer<RecommendedWatcher>>> = Mutex::new(None);
}

/// Starts watching one or more file paths with debounce.
pub fn start_file_watch(paths: &[&str]) {
    let watched_paths: Vec<PathBuf> = paths.iter().map(|p| PathBuf::from(p)).collect();
    let debounce_duration = Duration::from_secs(2);

    let mut debouncer = new_debouncer(debounce_duration, move |result: Result<Vec<DebouncedEvent>, Error>| {
        match result {
            Ok(events) => {
                for event in events {
                    let path_str = event.path.to_string_lossy().to_string();

                    // Optional: filter by extension
                    if !path_str.ends_with(".cfg") && !path_str.ends_with(".json") {
                        continue;
                    }

                    Trailkeeper::record(LogEntry {
                        event_type: EventType::FileChange,
                        timestamp: Utc::now(),
                        actor: "system".to_string(),
                        description: format!("File {} changed ({:?})", path_str, event.kind),
                        affected_components: vec![path_str],
                        status: LogStatus::Detected,
                    });
                }
            }
            Err(err) => {
                Trailkeeper::record(LogEntry {
                    event_type: EventType::FileChange,
                    timestamp: Utc::now(),
                    actor: "system".to_string(),
                    description: format!("Watcher error: {:?}", err),
                    affected_components: vec!["<unknown>".into()],
                    status: LogStatus::Failure,
                });
            }
        }
    }).expect("Failed to create debouncer");

    // Register paths
    for path in &watched_paths {
        debouncer.watcher().watch(path, RecursiveMode::NonRecursive)

            .expect("Failed to watch path");
    }

    let mut watcher = FILE_WATCHER.lock().unwrap();
    *watcher = Some(debouncer);
}

/// Stops the active file watcher.
pub fn stop_file_watch() {
    let mut watcher = FILE_WATCHER.lock().unwrap();
    *watcher = None;
}

/// Returns whether a watcher is currently active.
pub fn is_watching() -> bool {
    FILE_WATCHER.lock().unwrap().is_some()
}


