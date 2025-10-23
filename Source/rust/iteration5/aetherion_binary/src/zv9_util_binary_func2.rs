use walkdir::WalkDir;
use std::path::Path;
#[allow(unused_imports)] // hidden post compiler
//use aetherion_core::trailkeeper::{LogEntry, EventType};
use aetherion_shared::zv9_util_logging::log_info;

/// 📦 Prints a tree of Rust modules across all workspace crates
pub fn print_module_tree() {
    println!("\n📦 Scanning for Rust modules across workspace...\n");

    let crate_dirs = [
        "../aetherion_core/src",
        "../aetherion_engine/src",
        "src", // aetherion_binary/src
    ];

    for crate_dir in crate_dirs {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(crate_dir);
        if !path.exists() {
            println!("⚠️ Skipping missing path: {}", path.display());
            continue;
        }

        println!("🔍 Crate: {}\n", path.display());

        for entry in WalkDir::new(&path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            println!("├── {}", entry.path().display());
        }

        println!();
    }

    println!("✅ Module scan complete.\n");
}

/// 🧪 Scans for GDScript-callable Rust methods exposed via #[func]
pub fn print_godot_api_surface() {
    use std::collections::HashMap;
    use std::fs;
    use regex::Regex;

    println!("🧪 API scan triggered");
    println!("📡 Recursively scanning for GDScript-callable API...\n");

    let class_marker = Regex::new(r"#\[\s*derive\s*\(\s*GodotClass\s*\)\s*]").unwrap();
    let method_marker = Regex::new(r"#\[\s*func\s*]").unwrap();
    let fn_signature = Regex::new(r"^\s*(pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)\s*(->\s*.+)?").unwrap();
    let struct_decl = Regex::new(r"^\s*(pub\s+)?struct\s+(\w+)").unwrap();
    let impl_decl = Regex::new(r"^\s*impl\s+(\w+)").unwrap();

    let ignored_impls = ["crate", "std", "Default", "From", "fmt", "WithSignals"];
    let mut godot_api: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut file_count = 0;
    let mut orphan_methods = 0;

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let path = e.path().display().to_string();
            e.path().extension().map_or(false, |ext| ext == "rs")
                && !path.contains("OLD1")
                && !path.contains("OLD2")
        })
    {
        file_count += 1;
        let file_path = entry.path().display().to_string();

        if let Ok(content) = fs::read_to_string(entry.path()) {
            let mut lines = content.lines().peekable();
            let mut last_struct = String::new();
            let mut current_class = String::new();

            while let Some(line) = lines.next() {
                let line = line.trim();

                if class_marker.is_match(line) {
                    for _ in 0..20 {
                        if let Some(next_line) = lines.peek() {
                            if let Some(caps) = struct_decl.captures(next_line.trim()) {
                                last_struct = caps[2].to_string();
                                println!("🔍 Found GodotClass struct: {} in {}", last_struct, file_path);
                                break;
                            }
                            lines.next();
                        }
                    }
                }

                if let Some(caps) = impl_decl.captures(line) {
                    let candidate = caps[1].to_string();
                    if candidate == last_struct && !ignored_impls.contains(&candidate.as_str()) {
                        current_class = candidate;
                        println!("🔧 Matched impl block for: {}", current_class);
                    }
                }

                if method_marker.is_match(line) {
                    let mut signature_lines = Vec::new();
                    let mut lookahead = lines.clone();

                    for _ in 0..10 {
                        if let Some(next_line) = lookahead.next() {
                            let trimmed = next_line.trim();
                            signature_lines.push(trimmed.to_string());
                            if trimmed.contains('{') || trimmed.contains(';') {
                                break;
                            }
                        }
                    }

                    let joined = signature_lines.join(" ");
                    if let Some(caps) = fn_signature.captures(&joined) {
                        let name = &caps[2];
                        let params = &caps[3];
                        let return_type = caps.get(4).map_or("", |m| m.as_str());
                        let signature = format!("fn {}({}) {}", name, params, return_type);

                        if !current_class.is_empty() {
                            println!("✅ Found #[func] method: {} in {}", name, current_class);
                            let key = (current_class.clone(), file_path.clone());
                            godot_api.entry(key).or_default().push(signature);
                        } else {
                            orphan_methods += 1;
                            println!("⚠️ Found #[func] but no class context in {}", file_path);
                        }
                    }
                }
            }
        }
    }

    println!("\n📘 GDScript-Callable Methods");

    if godot_api.is_empty() {
        println!("(No GDScript-callable methods detected. Scanned {} files.)", file_count);
    } else {
        for ((class, file), methods) in &godot_api {
            println!("\n🔹 Node: {}  📁 {}", class, file);
            for method in methods {
                println!("   └── {}", method);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("   Files scanned: {}", file_count);
    println!("   Classes found: {}", godot_api.len());
    println!("   Total methods: {}", godot_api.values().map(|v| v.len()).sum::<usize>());
    println!("   Orphan #[func] methods: {}", orphan_methods);

    println!("\n✅ GDScript-callable methods printed.\n");

    log_info("AetherionBinary", "Scanned GDScript-callable API surface");

}
