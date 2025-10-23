


use std::sync::Mutex;
use lazy_static::lazy_static;
#[allow(unused_imports)]
use aetherion_core::zv9_prelude::*;
#[allow(unused_imports)]
use crate::zv9_trailkeeper_entry::{LogEntry, EventType, LogStatus};

lazy_static! {
    static ref LOG_REGISTRY: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}


pub struct Trailkeeper;

impl Trailkeeper {
    pub fn record(entry: LogEntry) {
        let mut registry = LOG_REGISTRY.lock().unwrap();
        registry.push(entry);
    }

    pub fn query<F>(filter: F) -> Vec<LogEntry>
    where
        F: Fn(&LogEntry) -> bool,
    {
        let registry = LOG_REGISTRY.lock().unwrap();
        registry.iter().filter(|entry| filter(entry)).cloned().collect()
    }

    pub fn all() -> Vec<LogEntry> {
        let registry = LOG_REGISTRY.lock().unwrap();
        registry.clone()
    }
}
