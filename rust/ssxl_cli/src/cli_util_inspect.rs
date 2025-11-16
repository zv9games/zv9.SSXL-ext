//! # CLI Utilities: Inspection and Codebase Analysis (`ssxl_cli::cli_util_inspect`)
//!
//! This module provides utility functions for inspecting the structure of the SSXL-ext
//! Rust workspace and scanning the exposed Godot API surface. These tools are used
//! for development, debugging, and verification of the engine's public interface.

use walkdir::WalkDir; // Crate for iterating over directory trees
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex; // Crate for regular expressions
use tracing::{info, warn, error};
use std::thread;
use std::time::Duration;


/// Scans the SSXL-ext workspace and prints a structured, visual tree of all `.rs` files.
///
/// This provides a quick overview of the codebase structure, highlighting core files (`lib.rs`, `main.rs`).
pub fn print_module_tree() {
	println!("\n=========================================================================");
	println!("| 🌲 RUST WORKSPACE MODULE TREE (Scanning...)                           |");
	println!("=========================================================================");
	
	// List of all crate source directories in the workspace.
	// NOTE: These are relative paths assuming execution from ssxl-ext/rust/ or a subdirectory.
	let crate_dirs = [
		"ssxl_cache/src",
		"ssxl_engine_ffi/src",
		"ssxl_generate/src",
		"ssxl_godot/src",
		"ssxl_math/src",
		"ssxl_shared/src",
		"ssxl_sync/src",
		"ssxl_tools/src",
		"ssxl_cli/src",
	];

	for crate_dir in crate_dirs {
        let crate_path = PathBuf::from(crate_dir);
		println!("\n🔍 Crate: {}", crate_dir);

		if crate_path.exists() && crate_path.is_dir() {
            // Walk the directory starting at a depth of 1 (excluding the root `src` folder).
			for entry in WalkDir::new(&crate_path).min_depth(1) {
				match entry {
					Ok(e) => {
                        let path = e.path();
                        // Check if the path is a file and ends with the .rs extension.
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                            
                            let file_name = path.file_name().map_or("", |name| name.to_str().unwrap_or(""));

                            // Differentiate between core files and nested module files for visualization.
                            let prefix = if file_name == "lib.rs" || file_name == "main.rs" {
                                "├── [CORE] "
                            } else {
                                "│   └── "
                            };
                            
                            // Print the path relative to the crate's `src/` directory.
                            if let Ok(relative_path) = path.strip_prefix(&crate_path) {
                                println!("{} {}", prefix, relative_path.display());
                            } else {
                                // Fallback if strip_prefix fails (shouldn't happen here).
                                println!("{} {}", prefix, path.display());
                            }
                        }
					},
					Err(e) => warn!("Error walking directory {}: {}", crate_dir, e),
				}
			}
		} else {
			warn!("Path does not exist or is not a directory: {}", crate_dir);
		}
	}
	println!("=========================================================================\n");
}


/// Scans specific `ssxl_godot` files to extract and print function signatures
/// marked with the `#[func]` attribute, exposing the engine's public API surface.
pub fn print_godot_api_surface() {
    // Files where the Godot API methods are typically defined.
	let godot_api_files: [&str; 3] = [
        "ssxl_godot/src/ssxl_engine.rs",
        "ssxl_godot/src/ssxl_oracle.rs", // Assumed file for oracle/query functions
        "ssxl_godot/src/ssxl_signals.rs",
    ];

    println!("🧪 API scan triggered (targeting {} files in ssxl_godot/src/)...", godot_api_files.len());
    
    // Regex to capture function signature: `pub fn method_name(args) -> return_type {`
    let fn_signature_regex = Regex::new(
        r"^\s*pub\s+fn\s+(\w+)\s*(\([^\{]*)\s*(?:->\s*([^\{]*))?\s*\{"
    ).unwrap();

    // Regex to find the Godot `#[func]` marker line.
    let func_marker_regex = Regex::new(r"^\s*#\[func\]\s*$").unwrap();

    // Vector to store (method_name, args, return_type, source_file) tuples.
    let mut api_methods: Vec<(String, String, String, String)> = Vec::new();

    for file_path_str in godot_api_files.iter() {
        let godot_lib_path: &Path = Path::new(file_path_str);
        // Extract the filename for reporting.
        let file_name = Path::new(file_path_str).file_name().unwrap().to_str().unwrap();
        let mut func_line_pending = false;

        match fs::read_to_string(godot_lib_path) {
            Ok(contents) => {
                info!("Successfully read {}", godot_lib_path.display());
                
                for line in contents.lines() {
                    // Step 1: Detect the #[func] marker.
                    if func_marker_regex.is_match(line) {
                        func_line_pending = true;
                        continue;
                    }

                    // Step 2: If the marker was found, check the next non-empty line for the function signature.
                    if func_line_pending {
                        if let Some(captures) = fn_signature_regex.captures(line) {
                            
                            // Capture Group 1: Method name
                            let method_name = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown_method").to_string();
                            
                            // Capture Group 2: Arguments (including parentheses)
                            let args = captures.get(2)
                                // Clean up the argument string: remove surrounding parentheses and trim whitespace.
                                .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                                .unwrap_or_default();
                            
                            // Capture Group 3: Return type (optional). Defaults to `()` if not present.
                            let return_type = captures.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                            
                            // Store the captured method details.
                            api_methods.push((method_name, args, return_type, file_name.to_string()));
                            
                            func_line_pending = false; // Reset state after successful capture.
                        } else if !line.trim().is_empty() {
                            // If we hit a non-#[func], non-function line, reset the pending flag.
                            func_line_pending = false;
                        }
                    }
                }
            },
            Err(e) => {
                error!("Failed to read file {}: {}", file_path_str, e);
            }
        }
    }

	println!("\n--- 🎮 SSXL Engine Godot API Surface ---");
    if api_methods.is_empty() {
        warn!("No #[func] methods found in the targeted ssxl_godot API files. Is the Godot binding active?");
    } else {
        println!("Registered {} callable methods:", api_methods.len());
        // Print all detected API methods in a standardized format.
        for (name, args, return_type, source_file) in &api_methods {
            println!("  ✅ func {}({}) -> {} [{}]", name, args, return_type, source_file);
        }
    }
	println!("-------------------------------------------\n");

    info!("API scan complete: {} methods detected.", api_methods.len());
    // Pause briefly for dramatic effect and to ensure logging is flushed.
    thread::sleep(Duration::from_secs(2));
}
