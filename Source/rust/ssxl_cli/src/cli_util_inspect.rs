// aetherion_cli/src/cli_util_inspect.rs
//! Inspection utilities for diagnostics and developer introspection.

use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use tracing::{info, warn, error};
use std::thread;
use std::time::Duration; // Added for placeholders

// --- Module Tree Inspection ---

/// 📦 Prints a tree of Rust modules across all workspace crates.
/// NOTE: This assumes the CLI is executed from the 'rust/' directory.
pub fn print_module_tree() {
	println!("\n=========================================================================");
	println!("| 🌲 RUST WORKSPACE MODULE TREE (Scanning...)                           |");
	println!("=========================================================================");
	
	// Assuming execution from the workspace root (e.g., 'rust/')
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
        // Use PathBuf for cleaner path manipulation
        let crate_path = PathBuf::from(crate_dir);
		println!("\n🔍 Crate: {}", crate_dir);

		if crate_path.exists() && crate_path.is_dir() {
            // Use WalkDir to traverse the directory
			for entry in WalkDir::new(&crate_path).min_depth(1) { // min_depth(1) to avoid re-listing the src/ directory itself
				match entry {
					Ok(e) => {
                        let path = e.path();
                        // Only process files that end with `.rs`
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                            
                            let file_name = path.file_name().map_or("", |name| name.to_str().unwrap_or(""));

                            let prefix = if file_name == "lib.rs" || file_name == "main.rs" {
                                // Core files are highlighted
                                "├── [CORE] "
                            } else {
                                "│   └── "
                            };
                            
                            // Get the path relative to the crate's src folder
                            if let Ok(relative_path) = path.strip_prefix(&crate_path) {
                                println!("{} {}", prefix, relative_path.display());
                            } else {
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


// --- API Inspection ---

/// 🧪 Scans for GDScript-callable Rust methods exposed via #[func]
/// NOTE: This assumes the CLI is executed from the 'rust/' directory.
pub fn print_godot_api_surface() {
	println!("🧪 API scan triggered (targeting ssxl_godot/src/lib.rs)...");

    // Path is relative to the workspace root, assuming the CLI runs from there.
    let godot_lib_path: &Path = Path::new("ssxl_godot/src/lib.rs");
    
    // Regex to find the function signature line that follows #[func].
    // Captures: 1=method_name, 2=arguments, 3=return_type
    let fn_signature_regex = Regex::new(
        r"^\s*pub\s+fn\s+(\w+)\s*(\([^\{]*)\s*(?:->\s*([^\{]*))?\s*\{"
    ).unwrap();

    // Regex to find the #[func] marker line.
    let func_marker_regex = Regex::new(r"^\s*#\[func\]\s*$").unwrap();

    let mut api_methods: Vec<(String, String, String)> = Vec::new();
    let mut func_line_pending = false;

    match fs::read_to_string(godot_lib_path) {
        Ok(contents) => {
            info!("Successfully read {}", godot_lib_path.display());
            
            for line in contents.lines() {
                // 1. Check for the #[func] marker
                if func_marker_regex.is_match(line) {
                    func_line_pending = true;
                    continue;
                }

                // 2. If marker was found, check for the function signature
                if func_line_pending {
                    if let Some(captures) = fn_signature_regex.captures(line) {
                        
                        let method_name = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown_method").to_string();
                        
                        // Argument capture: remove surrounding parentheses and trim whitespace
                        let args = captures.get(2)
                            .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                            .unwrap_or_default();
                        
                        // Return type capture: trim whitespace, default to "()"
                        let return_type = captures.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                        
                        api_methods.push((method_name, args, return_type));
                        
                        // Reset the flag after processing the function signature
                        func_line_pending = false;
                    } else if !line.trim().is_empty() {
                        // If a non-blank line was found that didn't match the signature, reset.
                        func_line_pending = false;
                    }
                }
            }
        },
        Err(e) => {
            error!("Failed to read file {}: {}", godot_lib_path.display(), e);
            println!("\n❌ API Scan Failed. Could not read source file.");
            return;
        }
    }

	println!("\n--- 🎮 SSXL Engine Godot API Surface ---");
    if api_methods.is_empty() {
        warn!("No #[func] methods found in aetherion_godot/src/lib.rs. Is the Godot binding active?");
    } else {
        println!("Registered {} callable methods:", api_methods.len());
        for (name, args, return_type) in &api_methods {
            // Display as: MethodName(arguments) -> ReturnType
            println!("  ✅ func {}({}) -> {}", name, args, return_type);
        }
    }
	println!("-------------------------------------------\n");

    info!("API scan complete: {} methods detected.", api_methods.len());
    thread::sleep(Duration::from_secs(2));
}