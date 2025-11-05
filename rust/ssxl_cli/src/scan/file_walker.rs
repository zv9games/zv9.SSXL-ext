// ssxl_cli/src/actions/file_walker.rs

use std::fs;
use std::io;
use std::path::PathBuf;
use tracing::{warn, error};

// --- CONFIGURATION ---
/// The root directory to start the LOC scan from.
pub const ROOT_DIR: &str = "../rust/";

/// 📐 Represents the LOC result for a single file.
#[derive(Debug, Clone)]
pub struct FileLoc {
    pub path: PathBuf,
    pub loc: usize,
}

/// Public entry point for the file walking process.
/// Recursively scans the target directory for all files matching the given extension
/// and calculates their Lines of Code (LOC).
pub fn recursive_loc_scan(root_path: &PathBuf, extension: &str) -> io::Result<Vec<FileLoc>> {
    let mut results = Vec::new();
    // Start the recursive traversal from the root path
    scan_dir(root_path, extension, &mut results)?;
    Ok(results)
}

/// Internal function to perform the recursive directory scan.
fn scan_dir(path: &PathBuf, extension: &str, results: &mut Vec<FileLoc>) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip the build output directory (target/) and obsolete directories
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "target") {
                    warn!("LOC Scanner skipping build output directory: {}", path.display());
                    continue;
                }
                
                // Skip the temporary/obsolete directory 'iteration5'
                if path.file_name().map_or(false, |n| n.to_string_lossy() == "iteration5") {
                    warn!("LOC Scanner skipping obsolete directory: {}", path.display());
                    continue;
                }

                scan_dir(&path, extension, results)?;
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