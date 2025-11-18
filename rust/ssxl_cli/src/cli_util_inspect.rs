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
	let godot_api_files: [&str; 3] = [
        "ssxl_godot/src/ssxl_engine.rs",
        "ssxl_godot/src/ssxl_oracle.rs", 
        "ssxl_godot/src/ssxl_signals.rs",
    ];

    println!("🧪 API scan triggered (targeting {} files in ssxl_godot/src/)...", godot_api_files.len());
    
    let fn_signature_regex = Regex::new(
        r"^\s*pub\s+fn\s+(\w+)\s*(\([^\{]*)\s*(?:->\s*([^\{]*))?\s*\{"
    ).unwrap();
    let func_marker_regex = Regex::new(r"^\s*#\[func\]\s*$").unwrap();

    let signal_signature_regex = Regex::new(
        r"^\s*fn\s+(\w+)\s*(\([^;]*)\s*;\s*$"
    ).unwrap();
    let signal_marker_regex = Regex::new(r"^\s*#\[signal\]\s*$").unwrap();
    
    let mut api_methods: Vec<(String, String, String, String)> = Vec::new();
    let mut api_signals: Vec<(String, String, String)> = Vec::new();

    for file_path_str in godot_api_files.iter() {
        let godot_lib_path: &Path = Path::new(file_path_str);
        let file_name = Path::new(file_path_str).file_name().unwrap().to_str().unwrap();
        
        let mut func_line_pending = false;
        let mut signal_line_pending = false;

        match fs::read_to_string(godot_lib_path) {
            Ok(contents) => {
                info!("Successfully read {}", godot_lib_path.display());
                
                for line in contents.lines() {
                    let trimmed_line = line.trim();

                    if func_marker_regex.is_match(trimmed_line) {
                        func_line_pending = true;
                        signal_line_pending = false;
                        continue;
                    }
                    if signal_marker_regex.is_match(trimmed_line) {
                        signal_line_pending = true;
                        func_line_pending = false;
                        continue;
                    }

                    if func_line_pending {
                        if let Some(captures) = fn_signature_regex.captures(trimmed_line) {
                            let method_name = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown_method").to_string();
                            let args = captures.get(2)
                                .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                                .unwrap_or_default();
                            let return_type = captures.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                            
                            api_methods.push((method_name, args, return_type, file_name.to_string()));
                            func_line_pending = false; 
                        } else if !trimmed_line.is_empty() {
                            func_line_pending = false;
                        }
                    } 
                    
                    else if signal_line_pending {
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

	println!("\n--- 🎮 SSXL Engine Developer API Surface ---");
    
    println!("\n✅ Callable Methods ({} total):", api_methods.len());
    if api_methods.is_empty() {
        warn!("  No #[func] methods found in targeted files.");
    } else {
        for (name, args, return_type, source_file) in &api_methods {
            println!("  > func {}({}) -> {} [{}]", name, args, return_type, source_file);
        }
    }

    println!("\n⭐ Registered Signal Broadcasts ({} total):", api_signals.len());
    if api_signals.is_empty() {
        warn!("  No #[signal] broadcasts found in targeted files.");
    } else {
        for (name, args, source_file) in &api_signals {
            println!("  > signal {}({}) [{}]", name, args, source_file);
        }
    }
	println!("-------------------------------------------\n");

    info!("API scan complete: {} methods and {} signals detected.", api_methods.len(), api_signals.len());
    thread::sleep(Duration::from_secs(2));
}