// ssxl_source_scan.rs
use walkdir::WalkDir;
use std::path::PathBuf;
use std::fs;
use tracing::{info, error};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// --- CONSTANTS ---
const OUTPUT_FINAL_LOC_FILE: &str = "RUST_LOC_TOTAL.txt";
const LOC_REPORTS_DIR: &str = "../loc_reports";
// Note: Relative paths are assumed to be run from the 'rust' directory.

// --- LOC COUNTING LOGIC ---

/// Counts non-empty, non-comment lines of code.
fn count_loc_from_content(content: &str) -> u64 {
    let mut count = 0;
    for line in content.lines() {
        // Ignores lines starting with '//' (Rust) or '#' (GDScript).
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
            count += 1;
        }
    }
    count
}

/// Writes the total Rust LOC count to the fixed-name file for Godot's boot parser.
fn write_final_loc_total(loc_count: u64) {
    // Navigate up one level from 'rust' to the project root for the output file.
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

// --- MAIN SCAN FUNCTION ---

pub fn scan_and_report_loc() {
    let mut total_rs_loc: u64 = 0;
    let mut total_gd_loc: u64 = 0;
    let mut report_lines: Vec<String> = Vec::new();

    // UPDATED DIRECTORIES for the consolidated ssxl-ext structure (as per manifest).
    // Scans the main GDExtension library and the CLI tool itself.
    let rust_dirs = [
        "ssxl_ext/src", // Contains all core logic (host, generate, shared).
        "ssxl_cli/src", // Contains CLI tool logic (like this file).
    ];

    // --- SCAN RUST FILES ---
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

    // --- SCAN GDSCRIPT FILES ---
    // Assuming the GDScript test directory remains the same relative path.
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

    // --- WRITE REPORTS ---
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
        "SSXL-ext Live LOC Report\nGenerated (Epoch Seconds): {}\nRoot Directories: ssxl_ext, ssxl_cli, ssxl_engine_tester\n\n------------------------------------------------------\n FILE LOC | Relative File Path\n------------------------------------------------------\n{}\n------------------------------------------------------\n        {} LOC | *.rs (Rust Total)\n        {} LOC | *.gd (GDScript Total)\n        {} LOC | TOTALS\n",
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

    // --- CRITICAL STEP FOR GODOT ---
    write_final_loc_total(total_rs_loc);

    // Final sleep to ensure writes/logs are flushed
    thread::sleep(Duration::from_millis(100));
}