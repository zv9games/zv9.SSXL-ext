//! # File Walker and LOC Scanner (`ssxl_cli::actions::file_walker`)
//!
//! Provides utilities for recursively traversing the filesystem. Primarily used
//! to perform a Lines of Code (LOC) count on source files, excluding common
//! build and obsolete directories.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tracing::{warn, error};

/// The default root directory for the LOC scan, assumed to be relative to the CLI execution path.
pub const RUST_DIR: &str = "../rust/";
pub const GODOT_TESTER_DIR: &str = "../SSXL_engine_tester/"; // Renamed for consistency

/// An array of all default directories that should be scanned.
/// The CLI command should be able to extend this list with arguments.
pub const DEFAULT_SCAN_ROOTS: &[&str] = &[
    RUST_DIR,
    GODOT_TESTER_DIR,
];


/// Represents the data collected for a single file during a scan.
#[derive(Debug, Clone)]
pub struct FileLoc {
    /// The absolute or relative path to the file.
    pub path: PathBuf,
    /// The count of lines in the file (Lines of Code).
    pub loc: usize,
}

// --------------------------------------------------------------------------------------------------
// NEW LOGIC: Multi-Root Orchestration
// --------------------------------------------------------------------------------------------------

/// Initiates a scan across multiple root directories and aggregates the results.
///
/// This function should be called by the main CLI action.
///
/// # Arguments
/// * `roots`: A slice of directory paths to scan.
/// * `extensions`: A slice of file extensions to target (e.g., &["rs", "gd"]).
///
/// # Returns
/// A `Result` containing a vector of all `FileLoc` structs collected, or an `io::Error` on failure.
pub fn scan_multiple_roots(roots: &[&str], extensions: &[&str]) -> io::Result<Vec<FileLoc>> {
    let mut total_results = Vec::new();

    for root_str in roots {
        let root_path = PathBuf::from(root_str);
        if !root_path.exists() {
            warn!("Skipping non-existent scan root: {}", root_path.display());
            continue;
        }

        for extension in extensions.iter() {
            match scan_single_root_loc(&root_path, extension) {
                Ok(mut file_locs) => total_results.append(&mut file_locs),
                Err(e) => {
                    // Log the error but keep going! A failure in one path shouldn't halt the whole scan.
                    error!("Failed to scan root '{}' for extension '{}': {}", root_path.display(), extension, e);
                }
            }
        }
    }
    // We return Ok even if some individual scans failed, as long as we gathered some data.
    Ok(total_results)
}


// --------------------------------------------------------------------------------------------------
// RENAMED HELPER: Single-Root, Single-Extension Scan
// --------------------------------------------------------------------------------------------------

/// Initiates a recursive scan starting from a single given root path and a single extension.
///
/// It delegates the actual traversal and processing to the `scan_dir` helper function.
///
/// # Arguments
/// * `root_path`: The starting directory for the scan.
/// * `extension`: The file extension to target (e.g., "rs" for Rust files).
///
/// # Returns
/// A `Result` containing a vector of `FileLoc` structs on success, or an `io::Error` on failure.
pub fn scan_single_root_loc(root_path: &Path, extension: &str) -> io::Result<Vec<FileLoc>> {
    let mut results = Vec::new();
    // PathBuf changed to &Path for better ergonomics
    scan_dir(root_path, extension, &mut results)?;
    Ok(results)
}


// --------------------------------------------------------------------------------------------------
// TRAVERSAL HELPER: Internal Recursive Logic
// --------------------------------------------------------------------------------------------------

/// Recursively traverses a directory, counting LOC for files matching the given extension.
fn scan_dir(path: &Path, extension: &str, results: &mut Vec<FileLoc>) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip common build output directories.
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "target") {
                    warn!("LOC Scanner skipping build output directory: {}", path.display());
                    continue;
                }
                
                // Skip obsolete/archived directories.
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "iteration5") {
                    warn!("LOC Scanner skipping obsolete directory: {}", path.display());
                    continue;
                }

                // Recurse into valid subdirectories.
                scan_dir(&path, extension, results)?;
            } else if path.extension().map_or(false, |ext| ext == extension) {
                // Process file if its extension matches the target.
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        // Count lines by counting line endings.
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