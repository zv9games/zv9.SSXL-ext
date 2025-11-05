// ssxl_cli/src/actions/report_formatter.rs

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, error};

// Import necessary items from the file_walker sibling module.
use super::file_walker::{recursive_loc_scan, FileLoc, ROOT_DIR};


// --- CONFIGURATION ---
const OUTPUT_FILENAME_PREFIX: &str = "ssxl_loc_report_";
const OUTPUT_DIR: &str = "../loc_reports/";


/// Helper function to generate a simple epoch seconds string for the filename.
fn get_timestamp_string() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{}", since_the_epoch.as_secs())
}

/// Executes the full LOC scan, prints the summary, and writes the detailed report.
pub fn execute_loc_scan() {
    info!("LOC Scanner: Starting recursive scan for Rust files in {}", ROOT_DIR);
    
    let root_path = PathBuf::from(ROOT_DIR);
    if !root_path.exists() {
        error!("LOC Scanner failed: Root directory not found at {}", root_path.display());
        return;
    }

    let scan_start = SystemTime::now();

    // 1. Execute the scan using file_walker
    let mut loc_results = match recursive_loc_scan(&root_path, "rs") {
        Ok(results) => results,
        Err(e) => {
            error!("LOC Scanner: Recursive scan failed: {}", e);
            return;
        }
    };
    
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
            // Sort results by LOC descending for the report
            loc_results.sort_unstable_by(|a, b| b.loc.cmp(&a.loc));
            
            // 2. Write Header, Summary Table, and Content Dump
            if let Err(e) = write_report_header(&mut file, &root_path, &scan_duration, total_loc, file_count, &timestamp_str) {
                error!("Failed to write header to LOC report: {}", e);
                return;
            }

            if let Err(e) = write_summary_table(&mut file, &loc_results, &root_path) {
                error!("Failed to write summary table to LOC report: {}", e);
            }

            if let Err(e) = write_content_dump(&mut file, &loc_results, &root_path) {
                 error!("Failed to write content dump to LOC report: {}", e);
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

// --- Internal Report Writing Helpers ---

fn write_report_header(
    file: &mut fs::File,
    root_path: &PathBuf,
    scan_duration: &str,
    total_loc: usize,
    file_count: usize,
    timestamp_str: &str,
) -> io::Result<()> {
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
        root_path.display(),
        scan_duration,
        file_count,
        total_loc,
        "FILE"
    );
    file.write_all(header.as_bytes())
}

fn write_summary_table(
    file: &mut fs::File,
    loc_results: &[FileLoc],
    root_path: &PathBuf,
) -> io::Result<()> {
    for result in loc_results {
        // Attempt to strip the root prefix for cleaner output path
        let path_str = result.path.strip_prefix(root_path)
                                 .unwrap_or(&result.path)
                                 .display()
                                 .to_string();
        let line = format!("{:>5} LOC | {}\n", result.loc, path_str);
        // Using write_all ensures all bytes are written or an error is returned.
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn write_content_dump(
    file: &mut fs::File,
    loc_results: &[FileLoc],
    root_path: &PathBuf,
) -> io::Result<()> {
    let content_header = format!(
        "\n\n\n\n======================================================\n\
        SSXL-ext Codebase DETAILED CONTENT DUMP\n\
        ======================================================\n"
    );
    file.write_all(content_header.as_bytes())?;

    for result in loc_results {
        let path_str = result.path.strip_prefix(root_path)
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

        file.write_all(file_separator.as_bytes())?;

        // Read and write the entire file content
        match fs::read_to_string(&result.path) {
            Ok(content) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    error!("Failed to write content for {}: {}", path_str, e);
                }
            }
            Err(e) => {
                let error_message = format!("\n[ ERROR: FAILED TO READ FILE CONTENT: {} ]\n", e);
                if let Err(e) = file.write_all(error_message.as_bytes()) {
                    error!("Failed to write error message for {}: {}", path_str, e);
                }
            }
        }
    }
    Ok(())
}