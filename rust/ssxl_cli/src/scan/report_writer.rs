//!
//! This module contains the low-level logic for writing the LOC report
//! to disk, separated from the scanning orchestration logic.

use std::fs;
use std::io::{self, Write};
use std::path::{PathBuf};
use tracing::error;

/// NOTE: This struct is duplicated here for demonstration purposes, but in a real project
/// it would be imported from a data definition module like `super::file_walker::FileLoc`.
#[derive(Debug)]
pub struct FileLoc {
    pub path: PathBuf,
    pub loc: usize,
}

/// Define all target extensions for the multi-lingual project.
pub const TARGET_EXTENSIONS: &[&str] = &["rs", "gd", "gdc", "toml", "md"];


/// Writes the metadata and header section to the report file.
pub fn write_report_header(
    file: &mut fs::File,
    root_desc: &str,
    scan_duration: &str,
    total_loc: usize,
    file_count: usize,
    timestamp_str: &str,
) -> io::Result<()> {
    let header = format!(
        "SSXL-ext Codebase LOC Report\n\
        Generated (Epoch Seconds): {}\n\
        Root Directories: {}\n\
        Target Extensions: {}\n\
        Scan Time: {}\n\
        Total Files Scanned: {}\n\
        Total Lines of Code (LOC): {}\n\n\
        ------------------------------------------------------\n\
        {:>5} LOC | Relative File Path\n\
        ------------------------------------------------------\n",
        timestamp_str,
        root_desc,
        TARGET_EXTENSIONS.join(", "),
        scan_duration,
        file_count,
        total_loc,
        "FILE" // Column header for LOC count
    );
    file.write_all(header.as_bytes())
}

/// Writes a table listing each file, its LOC count, and its relative path.
pub fn write_summary_table(
    file: &mut fs::File,
    loc_results: &[FileLoc],
) -> io::Result<()> {
    for result in loc_results {
        let path_str = result.path.display().to_string();
        let line = format!("{:>5} LOC | {}\n", result.loc, path_str);
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

/// Appends the full contents of every scanned file to the report, separated by clear headers.
pub fn write_content_dump(
    file: &mut fs::File,
    loc_results: &[FileLoc],
) -> io::Result<()> {
    let content_header = format!(
        "\n\n\n\n======================================================\n\
        SSXL-ext Codebase DETAILED CONTENT DUMP\n\
        ======================================================\n"
    );
    file.write_all(content_header.as_bytes())?;

    for result in loc_results {
        let path_str = result.path.display().to_string();
        
        let file_separator = format!(
            "\n\n\n//////////////////////////////////////////////////////\n\
            // FILE: {} ({} LOC)\n\
            //////////////////////////////////////////////////////\n\n",
            path_str,
            result.loc
        );

        file.write_all(file_separator.as_bytes())?;

        // Read and write the full content of the file.
        match fs::read_to_string(&result.path) {
            Ok(content) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    // Use a tracing error for write failures.
                    error!("Failed to write content for {}: {}", path_str, e);
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
    Ok(())
}