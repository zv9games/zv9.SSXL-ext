//! # CLI Utilities: Line of Code (LOC) Analysis (`ssxl_cli::cli_util_loc`)
//!
//! This module provides functions for scanning the SSXL-ext workspace, calculating
//! Lines of Code (LOC) for both Rust (`.rs`) and GDScript (`.gd`) files, and
//! generating the LOC reports required by the Godot engine.

use walkdir::WalkDir;
use std::path::{PathBuf};
use std::fs;
use tracing::{info, error};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// Constants for the fixed-name output file, which is read by Godot.
const OUTPUT_FINAL_LOC_FILE: &str = "RUST_LOC_TOTAL.txt";
// Directory for full LOC reports, relative to the project root (../)
const LOC_REPORTS_DIR: &str = "../loc_reports";

/// Helper to count lines of code in file content, ignoring empty lines and comments (simplified).
fn count_loc_from_content(content: &str) -> u64 {
    let mut count = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        // Simple check: ignore empty lines and lines starting with comment markers.
        if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
            count += 1;
        }
    }
    count
}

/// Helper function to write the final Rust LOC total to a fixed-name file
/// for fast parsing by Godot at bootup.
fn write_final_loc_total(loc_count: u64) {
    // Navigate up one level to the project root (ssxl-ext/)
    let root_dir = PathBuf::from("../");
    let output_path = root_dir.join(OUTPUT_FINAL_LOC_FILE); // Final Path: ../RUST_LOC_TOTAL.txt

    let content = format!("{}\n", loc_count);

    match fs::write(&output_path, content.as_bytes()) {
        Ok(_) => {
            eprintln!("🔥 SSXL-ext CLI: Wrote total Rust LOC **{}** to {}.",
                      loc_count, output_path.display());
        }
        Err(e) => {
            error!("❌ CRITICAL FAIL: Could not write LOC file {:?}. Error: {}",
                      output_path, e);
        }
    }
}

/// Scans the Rust workspace, calculates lines of code (LOC), and generates
/// the full report and the final single-number LOC file.
///
/// Assumes CWD is inside the `ssxl-ext/rust/` directory.
pub fn scan_and_report_loc() {
    let mut total_rs_loc: u64 = 0;
    let mut total_gd_loc: u64 = 0;
    let mut report_lines: Vec<String> = Vec::new();

    // 1. Scan Rust Code
    // These paths are correct relative to the ssxl-ext/rust/ directory.
    let rust_dirs = ["ssxl_cache/src", "ssxl_engine_ffi/src", "ssxl_generate/src",
                     "ssxl_godot/src", "ssxl_math/src", "ssxl_shared/src",
                     "ssxl_sync/src", "ssxl_tools/src", "ssxl_cli/src"];

    for dir in &rust_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(path) {
                    let loc = count_loc_from_content(&content);
                    total_rs_loc += loc;
                    
                    if loc > 0 {
                        let path_str = format!("rust/{}", path.display());
                        // 1. Add LOC and path line
                        report_lines.push(format!("{:>10} LOC | {}", loc, path_str));
                        
                        // 2. Add file content wrapped in delimiters
                        report_lines.push(format!("// --- START: {} ---", path_str));
                        report_lines.push(content);
                        report_lines.push(format!("// --- END: {} ---", path_str));
                    }
                }
            }
        }
    }

    // 2. Scan GDScript Code
    // Path traversal is correct: `ssxl-ext/rust/` -> `../ssxl_engine_tester`
    let gd_dirs = ["../ssxl_engine_tester"];
    for dir in &gd_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "gd") {
                if let Ok(content) = fs::read_to_string(path) {
                    let loc = count_loc_from_content(&content);
                    total_gd_loc += loc;
                    
                    if loc > 0 {
                        // Note: path.display() will include the '..' but this is acceptable for internal reports.
                        let path_str = path.display().to_string();
                        // 1. Add LOC and path line
                        report_lines.push(format!("{:>10} LOC | {}", loc, path_str));
                        
                        // 2. Add file content wrapped in delimiters (using '#' for GDScript comments)
                        report_lines.push(format!("# --- START: {} ---", path_str));
                        report_lines.push(content);
                        report_lines.push(format!("# --- END: {} ---", path_str));
                    }
                }
            }
        }
    }

    // 3. Generate the full, dynamically-named report in the `../loc_reports/` directory
    let reports_dir = PathBuf::from(LOC_REPORTS_DIR);

    // Create the directory if it doesn't exist.
    if let Err(e) = fs::create_dir_all(&reports_dir) {
        error!("❌ Failed to create LOC reports directory {:?}. Error: {}", reports_dir, e);
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    let filename = format!("ssxl_loc_report_live_{}.txt", timestamp);
    let final_report_path = reports_dir.join(filename); 

    let full_report_content = format!(
        "SSXL-ext Live LOC Report\nGenerated (Epoch Seconds): {}\nRoot Directories: rust, ssxl_engine_tester\n\n------------------------------------------------------\n FILE LOC | Relative File Path\n------------------------------------------------------\n{}\n------------------------------------------------------\n       {} LOC | *.rs (Rust Total)\n       {} LOC | *.gd (GDScript Total)\n       {} LOC | TOTALS\n",
        timestamp,
        report_lines.join("\n"),
        total_rs_loc,
        total_gd_loc,
        total_rs_loc + total_gd_loc
    );

    // Write the full report to the corrected dynamic path.
    match fs::write(&final_report_path, full_report_content) {
        Ok(_) => {
            info!("✅ Full LOC report written to: {}", final_report_path.display());
        }
        Err(e) => {
            error!("❌ Failed to write full LOC report to {:?}. Error: {}", final_report_path, e);
        }
    }

    // 4. Write the single line count to the fixed-name file (RUST_LOC_TOTAL.txt)
    write_final_loc_total(total_rs_loc);

    thread::sleep(Duration::from_millis(100));
}