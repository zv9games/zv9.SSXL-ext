// ssxl_cli/src/cli_util_inspect.rs (Final, Cleaned Code)

//! Inspection utilities for diagnostics and developer introspection.

use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use tracing::{info, warn, error};
use std::thread;
use std::time::Duration; 

// --- Module Tree Inspection ---

/// 📦 Prints a tree of Rust modules across all workspace crates.
/// NOTE: This assumes the CLI is executed from the 'rust/' directory.
pub fn print_module_tree() {
	println!("\n=========================================================================");
	println!("| 🌲 RUST WORKSPACE MODULE TREE (Scanning...)                           |");
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
        let crate_path = PathBuf::from(crate_dir);
		println!("\n🔍 Crate: {}", crate_dir);

		if crate_path.exists() && crate_path.is_dir() {
			for entry in WalkDir::new(&crate_path).min_depth(1) {
				match entry {
					Ok(e) => {
                        let path = e.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                            
                            let file_name = path.file_name().map_or("", |name| name.to_str().unwrap_or(""));

                            let prefix = if file_name == "lib.rs" || file_name == "main.rs" {
                                // Core files are highlighted
                                "├── [CORE] "
                            } else {
                                "│   └── "
                            };
                            
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
    // FIX: Define an array of all known Godot API files.
	let godot_api_files: [&str; 3] = [
        "ssxl_godot/src/ssxl_engine.rs",
        "ssxl_godot/src/ssxl_oracle.rs", // Assuming this is where oracle functions live
        "ssxl_godot/src/ssxl_signals.rs",
    ];

    println!("🧪 API scan triggered (targeting {} files in ssxl_godot/src/)...", godot_api_files.len());
    
    // Regex to find the function signature line that follows #[func].
    // Captures: 1=method_name, 2=arguments, 3=return_type
    let fn_signature_regex = Regex::new(
        r"^\s*pub\s+fn\s+(\w+)\s*(\([^\{]*)\s*(?:->\s*([^\{]*))?\s*\{"
    ).unwrap();

    // Regex to find the #[func] marker line.
    let func_marker_regex = Regex::new(r"^\s*#\[func\]\s*$").unwrap();

    // Store API methods with source file name
    let mut api_methods: Vec<(String, String, String, String)> = Vec::new();

    // Iterate over all target files
    for file_path_str in godot_api_files.iter() {
        // FIX: The following variables are correctly scoped to the loop iteration.
        let godot_lib_path: &Path = Path::new(file_path_str);
        let file_name = Path::new(file_path_str).file_name().unwrap().to_str().unwrap();
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
                            
                            // Store the method along with its source file name
                            api_methods.push((method_name, args, return_type, file_name.to_string()));
                            
                            func_line_pending = false;
                        } else if !line.trim().is_empty() {
                            func_line_pending = false;
                        }
                    }
                }
            },
            Err(e) => {
                // FIX: Use `file_path_str` here to report the error correctly.
                error!("Failed to read file {}: {}", file_path_str, e);
            }
        }
    }

	println!("\n--- 🎮 SSXL Engine Godot API Surface ---");
    if api_methods.is_empty() {
        warn!("No #[func] methods found in the targeted ssxl_godot API files. Is the Godot binding active?");
    } else {
        println!("Registered {} callable methods:", api_methods.len());
        for (name, args, return_type, source_file) in &api_methods {
            // Display as: MethodName(arguments) -> ReturnType [SourceFile]
            println!("  ✅ func {}({}) -> {} [{}]", name, args, return_type, source_file);
        }
    }
	println!("-------------------------------------------\n");

    info!("API scan complete: {} methods detected.", api_methods.len());
    thread::sleep(Duration::from_secs(2));
}