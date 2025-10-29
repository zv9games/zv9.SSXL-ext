// ssxl_cli/src/cli_util_loc_scan.rs

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error};

// --- CONFIGURATION ---
// USER-SPECIFIED PATH: Retained as per your last request.
const ROOT_DIR: &str = "../rust/";
const OUTPUT_FILENAME_PREFIX: &str = "ssxl_loc_report_";
// FIXED: Output directory is now consistent with ROOT_DIR (one level up from CWD).
const OUTPUT_DIR: &str = "../loc_reports/";

/// 📐 Represents the LOC result for a single file.
struct FileLoc {
    path: PathBuf,
    loc: usize,
}

/// Helper function to generate a simple epoch seconds string for the filename.
fn get_timestamp_string() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{}", since_the_epoch.as_secs())
}


/// Recursively scans the target directory for all files matching the given extension
/// and calculates their Lines of Code (LOC).
fn recursive_loc_scan(path: &PathBuf, extension: &str, results: &mut Vec<FileLoc>) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Fixed: Direct usage of path.file_name().map_or to avoid E0382 move error.
                
                // Skip the build output directory (target/)
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "target") {
                    warn!("LOC Scanner skipping build output directory: {}", path.display());
                    continue; // This stops recursion into the 'target' directory
                }
                
                // Skip the temporary/obsolete directory 'iteration5'
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "iteration5") {
                    warn!("LOC Scanner skipping obsolete directory: {}", path.display());
                    continue;
                }

                recursive_loc_scan(&path, extension, results)?;
            } else if path.extension().map_or(false, |ext| ext == extension) {
                // Only process files with the specified extension (e.g., "rs")
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let loc = content.lines().count();
                        results.push(FileLoc { path, loc });
                    }
                    Err(e) => {
                        error!("Failed to read file {}: {}", path.display(), e);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Executes the full LOC scan, prints the summary, and writes the detailed report.
pub fn execute_loc_scan() {
    info!("LOC Scanner: Starting recursive scan for Rust files in {}", ROOT_DIR);
    
    let root_path = PathBuf::from(ROOT_DIR);
    if !root_path.exists() {
        error!("LOC Scanner failed: Root directory not found at {}", root_path.display());
        return;
    }

    let mut loc_results = Vec::new();
    let scan_start = SystemTime::now();

    match recursive_loc_scan(&root_path, "rs", &mut loc_results) {
        Ok(_) => {
            let scan_duration = scan_start.elapsed()
                                          .map_or("N/A".to_string(), |d| format!("{:.2}ms", d.as_millis() as f32));

            let total_loc: usize = loc_results.iter().map(|f| f.loc).sum();
            let file_count = loc_results.len();

            // --- Generate Output File ---
            let timestamp_str = get_timestamp_string();
            let output_filename = format!("{}{}.txt", OUTPUT_FILENAME_PREFIX, timestamp_str);
            let output_path = PathBuf::from(OUTPUT_DIR).join(&output_filename);

            // Ensure output directory exists
            if let Err(e) = fs::create_dir_all(PathBuf::from(OUTPUT_DIR)) {
                error!("Failed to create output directory {}: {}", OUTPUT_DIR, e);
                return;
            }

            match fs::File::create(&output_path) {
                Ok(mut file) => {
                    // Write File Details (sorted by LOC descending) - this is done before header 
                    // only to ensure the results are sorted for both the table and the dump.
                    loc_results.sort_unstable_by(|a, b| b.loc.cmp(&a.loc));
                    
                    // 1. Write Header
                    let header = format!(
                        "SSXL-ext Codebase LOC Report\n\
                        Generated (Epoch Seconds): {}\n\
                        Root Directory: {}\n\
                        Scan Time: {}\n\
                        Total Files Scanned: {}\n\
                        Total Lines of Code (LOC): {}\n\n\
                        ------------------------------------------------------\n\
                        {:>5} LOC | Relative File Path\n\
                        ------------------------------------------------------\n",
                        timestamp_str,
                        ROOT_DIR,
                        scan_duration,
                        file_count,
                        total_loc,
                        "FILE"
                    );
                    if file.write_all(header.as_bytes()).is_err() {
                        error!("Failed to write header to LOC report.");
                        return;
                    }

                    // 2. Write Summary Table
                    for result in &loc_results {
                        // Attempt to strip the root prefix for cleaner output path
                        let path_str = result.path.strip_prefix(&root_path)
                                                   .unwrap_or(&result.path)
                                                   .display()
                                                   .to_string();
                        let line = format!("{:>5} LOC | {}\n", result.loc, path_str);
                        if file.write_all(line.as_bytes()).is_err() {
                            error!("Failed to write line for {} to LOC report.", path_str);
                            // Continue to next file on failure
                        }
                    }

                    // 3. Write Detailed Content Dump (The requested feature)
                    let content_header = format!(
                        "\n\n\n\n======================================================\n\
                         SSXL-ext Codebase DETAILED CONTENT DUMP\n\
                         ======================================================\n"
                    );
                    if file.write_all(content_header.as_bytes()).is_err() {
                        error!("Failed to write content dump header.");
                        return;
                    }

                    for result in &loc_results {
                        let path_str = result.path.strip_prefix(&root_path)
                                                   .unwrap_or(&result.path)
                                                   .display()
                                                   .to_string();
                        
                        let file_separator = format!(
                            "\n\n\n//////////////////////////////////////////////////////\n\
                             // FILE: {} ({} LOC)\n\
                             //////////////////////////////////////////////////////\n\n",
                            path_str,
                            result.loc
                        );

                        // Write file separator header
                        if file.write_all(file_separator.as_bytes()).is_err() {
                            error!("Failed to write content separator for {}.", path_str);
                            continue;
                        }

                        // Read and write the entire file content
                        match fs::read_to_string(&result.path) {
                            Ok(content) => {
                                if file.write_all(content.as_bytes()).is_err() {
                                    error!("Failed to write content for {}.", path_str);
                                }
                            }
                            Err(e) => {
                                let error_message = format!("\n[ ERROR: FAILED TO READ FILE CONTENT: {} ]\n", e);
                                if file.write_all(error_message.as_bytes()).is_err() {
                                    error!("Failed to write error message for {}.", path_str);
                                }
                            }
                        }
                    }
                    
                    // Final Footer
                    if file.write_all(b"\n\n======================================================\nEND OF REPORT\n======================================================\n").is_err() {
                        error!("Failed to write report footer.");
                    }

                    info!("LOC Report: Successfully created report file: {}", output_path.display());

                    // Print summary to console
                    println!("\n[ LOC Scan Complete ]");
                    println!("Total Rust Files: {}", file_count);
                    println!("Total Lines of Code: {}", total_loc);
                    println!("Report saved to: {}", output_path.display());
                }
                Err(e) => {
                    error!("Failed to create LOC report file {}: {}", output_path.display(), e);
                }
            }
        }
        Err(e) => {
            error!("LOC Scanner: Recursive scan failed: {}", e);
        }
    }
}