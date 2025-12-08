// ============================================================================
// 📊 SSXL CLI: Line of Code (LOC) Analysis (`ssxl_cli::cli_util_loc`)
// ----------------------------------------------------------------------------
// This module provides developer utilities for scanning the SSXL-ext workspace,
// calculating Lines of Code (LOC) for both Rust (`.rs`) and GDScript (`.gd`)
// files, and generating reports consumed by the Godot engine.
//
// Purpose:
//   • Track codebase growth and complexity across Rust and Godot components.
//   • Provide a single-number LOC file (`RUST_LOC_TOTAL.txt`) for fast parsing
//     by Godot at bootup.
//   • Generate detailed reports with per-file LOC counts and full file contents
//     for auditing and review.
//
// Key Components:
//   • OUTPUT_FINAL_LOC_FILE
//       - Fixed-name file (`RUST_LOC_TOTAL.txt`) written to the project root.
//       - Contains a single integer: the total Rust LOC count.
//       - Used by Godot for quick boot-time validation.
//
//   • LOC_REPORTS_DIR
//       - Directory (`../loc_reports`) where full LOC reports are stored.
//       - Each report is timestamped with epoch seconds for uniqueness.
//
//   • count_loc_from_content
//       - Helper function that counts non-empty, non-comment lines.
//       - Ignores lines starting with `//` (Rust) or `#` (GDScript).
//       - Provides a simplified but effective LOC metric.
//
//   • write_final_loc_total
//       - Writes the total Rust LOC count to the fixed-name file.
//       - Ensures Godot can quickly parse LOC without scanning the workspace.
//
//   • scan_and_report_loc
//       - Main entry point for LOC analysis.
//       - Scans all Rust crate directories under `ssxl-ext/rust/`.
//       - Scans GDScript files under `../ssxl_engine_tester`.
//       - Aggregates LOC counts, builds a detailed report, and writes both:
//           1. A full report with per-file LOC and file contents.
//           2. A single-number LOC file for Godot boot parsing.
//       - Uses `WalkDir` for recursive traversal and `fs::read_to_string`
//         for file content analysis.
//       - Sleeps briefly at the end to ensure logs and writes are flushed.
//
// Workflow:
//   1. Traverse Rust and GDScript source directories.
//   2. Count LOC for each file using `count_loc_from_content`.
//   3. Build a detailed report with LOC counts and file contents.
//   4. Write the report to `../loc_reports/ssxl_loc_report_live_<timestamp>.txt`.
//   5. Write the total Rust LOC to `../RUST_LOC_TOTAL.txt`.
//   6. Log results for developer visibility.
//
// Design Choices:
//   • Simplified LOC counting avoids parsing complexity while still providing
//     meaningful metrics.
//   • Reports include full file contents for transparency and auditing.
//   • Timestamped filenames prevent overwriting and allow historical tracking.
//   • Fixed-name LOC file ensures fast integration with Godot boot logic.
//
// Educational Note:
//   • This module demonstrates how Rust can be used to build developer tooling
//     that integrates with external engines (Godot).
//   • By automating LOC analysis, developers gain visibility into codebase
//     growth and maintainability, while Godot gains a quick boot-time metric.
// ============================================================================


use walkdir::WalkDir;
use std::path::PathBuf;
use std::fs;
use tracing::{info, error};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const OUTPUT_FINAL_LOC_FILE: &str = "RUST_LOC_TOTAL.txt";
const LOC_REPORTS_DIR: &str = "../loc_reports";

fn count_loc_from_content(content: &str) -> u64 {
    let mut count = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
            count += 1;
        }
    }
    count
}

fn write_final_loc_total(loc_count: u64) {
    let root_dir = PathBuf::from("../");
    let output_path = root_dir.join(OUTPUT_FINAL_LOC_FILE);

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

pub fn scan_and_report_loc() {
    let mut total_rs_loc: u64 = 0;
    let mut total_gd_loc: u64 = 0;
    let mut report_lines: Vec<String> = Vec::new();

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
                        report_lines.push(format!("{:>10} LOC | {}", loc, path_str));
                        report_lines.push(format!("// --- START: {} ---", path_str));
                        report_lines.push(content);
                        report_lines.push(format!("// --- END: {} ---", path_str));
                    }
                }
            }
        }
    }

    let gd_dirs = ["../ssxl_engine_tester"];
    for dir in &gd_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "gd") {
                if let Ok(content) = fs::read_to_string(path) {
                    let loc = count_loc_from_content(&content);
                    total_gd_loc += loc;
                    
                    if loc > 0 {
                        let path_str = path.display().to_string();
                        report_lines.push(format!("{:>10} LOC | {}", loc, path_str));
                        report_lines.push(format!("# --- START: {} ---", path_str));
                        report_lines.push(content);
                        report_lines.push(format!("# --- END: {} ---", path_str));
                    }
                }
            }
        }
    }

    let reports_dir = PathBuf::from(LOC_REPORTS_DIR);

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

    match fs::write(&final_report_path, full_report_content) {
        Ok(_) => {
            info!("✅ Full LOC report written to: {}", final_report_path.display());
        }
        Err(e) => {
            error!("❌ Failed to write full LOC report to {:?}. Error: {}", final_report_path, e);
        }
    }

    write_final_loc_total(total_rs_loc);

    thread::sleep(Duration::from_millis(100));
}
