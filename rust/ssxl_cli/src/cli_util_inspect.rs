// ============================================================================
// 🌲 SSXL CLI: Developer Utilities (`ssxl_cli::cli_util_dev_scan`)
// ----------------------------------------------------------------------------
// This module provides developer-facing utilities for scanning and visualizing
// the SSXL engine’s Rust workspace and Godot API surface. It is designed to
// help contributors quickly understand project structure and validate exposed
// FFI/GDExtension methods and signals.
//
// Key Functions:
//   • print_module_tree
//       - Recursively scans all crate source directories in the SSXL workspace.
//       - Prints a tree view of `.rs` files, highlighting core entry points
//         (`lib.rs`, `main.rs`) for each crate.
//       - Helps developers visualize module organization and identify where
//         architectural logic resides.
//       - Uses `walkdir` for recursive traversal and structured output.
//
//   • print_godot_api_surface
//       - Scans targeted Godot and FFI source files for callable methods and
//         registered signals.
//       - Uses regex to detect:
//           - `#[func]` annotated methods (GDExtension entry points).
//           - `#[no_mangle] pub extern "C"` functions (FFI core).
//           - `#[signal]` annotated broadcasts (Godot signal definitions).
//       - Collects method signatures (name, args, return type) and signal
//         signatures for reporting.
//       - Prints a consolidated “MASTER API Surface” showing all callable
//         methods and signals, grouped by source file.
//       - Differentiates between `[FFI CORE]` and `[GDExt]` methods for clarity.
//
// Workflow:
//   1. `print_module_tree` scans crate directories and prints a visual tree.
//   2. `print_godot_api_surface` reads targeted files, applies regex, and
//      extracts callable methods and signals.
//   3. Results are printed to stdout with structured formatting and markers.
//   4. Developers can use this output to validate API exposure and ensure
//      consistency across Rust and Godot integration layers.
//
// Design Choices:
//   • `walkdir` provides robust recursive directory traversal.
//   • `regex` enables flexible detection of annotated methods and signals.
//   • `tracing` macros (`info`, `warn`, `error`) provide structured logging for
//     success, warnings, and errors.
//   • Output formatting emphasizes clarity, highlighting core files and FFI
//     entry points.
//   • A short sleep (`thread::sleep`) ensures logs are flushed before returning.
//
// Educational Note:
//   • This module demonstrates how Rust tooling can be extended to provide
//     developer ergonomics beyond compilation—offering visibility into project
//     structure and API contracts.
//   • By centralizing scans here, contributors gain confidence that workspace
//     modules are organized correctly and that Godot-facing APIs are properly
//     exposed for integration.
// ============================================================================


use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use tracing::{info, warn, error};
use std::thread;
use std::time::Duration;

pub fn print_module_tree() {
    println!("\n=========================================================================");
    println!("| 🌲 RUST WORKSPACE MODULE TREE (Scanning...)                            |");
    println!("=========================================================================");
    
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
                                "├── [CORE] "
                            } else {
                                "│   └── "
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

pub fn print_godot_api_surface() {
    let godot_api_files: [&str; 4] = [ 
        "ssxl_godot/src/engine/init.rs",
        "ssxl_godot/src/ffi/oracle.rs",      
        "ssxl_godot/src/ffi/signals.rs",
        "ssxl_engine_ffi/src/lib.rs", 
    ];

    println!("🧪 API scan triggered (targeting {} files in ssxl_godot/src/ and FFI core)...", godot_api_files.len());
    
    // FIX: Replaced literal newlines in annotations with \s* to match any whitespace.
    let method_regex = Regex::new(
        r#"(?s)(?:\s*#\s*\[func\]\s*.*?|#\s*\[no_mangle\]\s*.*?pub\s+extern\s+"C"\s*)\s*(?:pub\s+fn|fn)\s+(\w+)\s*(\([^\{;]*)\s*(?:->\s*([^\{]*))?"#
    ).unwrap();

    let signal_signature_regex = Regex::new(
        r"^\s*fn\s+(\w+)\s*(\([^;]*)\s*;\s*$"
    ).unwrap();
    // FIX: Replaced literal newlines in annotation with \s*.
    let signal_marker_regex = Regex::new(r"^\s*#\s*\[signal\]\s*$").unwrap();
    
    let mut api_methods: Vec<(String, String, String, String)> = Vec::new();
    let mut api_signals: Vec<(String, String, String)> = Vec::new();

    for file_path_str in godot_api_files.iter() {
        let path: &Path = Path::new(file_path_str);
        let file_name = Path::new(file_path_str).file_name().unwrap().to_str().unwrap();
        
        let mut signal_line_pending = false;

        match fs::read_to_string(path) {
            Ok(contents) => {
                info!("Successfully read {}", path.display());
                
                for cap in method_regex.captures_iter(&contents) {
                    let method_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown_method").to_string();
                    let args = cap.get(2)
                        .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                        .unwrap_or_default();
                    let return_type = cap.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                    
                    let clean_args = args.replace("self, ", "self,").trim().trim_start_matches("self").trim_start_matches(",").trim().to_string();

                    api_methods.push((method_name, clean_args, return_type, file_name.to_string()));
                }

                for line in contents.lines() {
                    let trimmed_line = line.trim();

                    if signal_marker_regex.is_match(trimmed_line) {
                        signal_line_pending = true;
                        continue;
                    }
                    
                    if signal_line_pending {
                        if let Some(captures) = signal_signature_regex.captures(trimmed_line) {
                            let signal_name = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown_signal").to_string();
                            let args = captures.get(2)
                                .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                                .unwrap_or_default();
                            
                            api_signals.push((signal_name, args, file_name.to_string()));
                            signal_line_pending = false; 
                        } else if !trimmed_line.is_empty() {
                            signal_line_pending = false;
                        }
                    }
                }
            },
            Err(e) => {
                error!("Failed to read file {}: {}", file_path_str, e);
            }
        }
    }

    println!("\n--- 🎮 SSXL Engine Developer MASTER API Surface ---");
    
    println!("\n✅ Callable Methods ({} total):", api_methods.len());
    if api_methods.is_empty() {
        warn!(" No callable methods found in targeted files (check FFI core!).");
    } else {
        for (name, args, return_type, source_file) in &api_methods {
            let marker = if source_file == "lib.rs" && api_methods.iter().any(|(n,_,_,s)| n == name && s == source_file) {
                "[FFI CORE]"
            } else {
                "[GDExt]"
            };
            println!(" > func {}({}) -> {} {} [{}]", name, args, return_type, marker, source_file);
        }
    }

    println!("\n⭐ Registered Signal Broadcasts ({} total):", api_signals.len());
    if api_signals.is_empty() {
        warn!(" No #[signal] broadcasts found in targeted files.");
    } else {
        for (name, args, source_file) in &api_signals {
            println!(" > signal {}({}) [{}]", name, args, source_file);
        }
    }
    println!("--------------------------------------------------");

    info!("API scan complete: {} methods and {} signals detected.", api_methods.len(), api_signals.len());
    thread::sleep(Duration::from_secs(2));
}