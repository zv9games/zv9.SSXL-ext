
#[allow(unused_imports)]
use crate::zv9_prelude::*;
use crate::zv9_trailkeeper_collector::Trailkeeper;
use crate::zv9_trailkeeper_entry::{LogEntry, EventType, LogStatus};


use chrono::Utc;
use std::process::Command;

pub fn scan_git_diff() {
    let output = Command::new("git")
        .args(&["diff", "--unified=0", "HEAD~1"])
        .output()
        .expect("Failed to run git diff");

    let diff = String::from_utf8_lossy(&output.stdout);
    let mut current_file: Option<String> = None;

    for line in diff.lines() {
        if line.starts_with("diff --git") {
            // Extract file path from the diff header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let file_path = parts[2].trim_start_matches("b/");
                current_file = Some(file_path.to_string());
            }
        } else if line.starts_with("@@") {
            // Extract line number from hunk header
            if let Some(file) = &current_file {
                let hunk_parts: Vec<&str> = line.split_whitespace().collect();
                let line_info = hunk_parts.get(2).unwrap_or(&"");
                let line_number = line_info
                    .trim_start_matches('+')
                    .split(',')
                    .next()
                    .unwrap_or("?");

                Trailkeeper::record(LogEntry {
                    event_type: EventType::GitDiff,
                    timestamp: Utc::now(),
                    actor: "system".to_string(),
                    description: format!("Modified {} at line {}", file, line_number),
                    affected_components: vec![file.clone()],
                    status: LogStatus::Detected,
                });
            }
        }
    }
}
