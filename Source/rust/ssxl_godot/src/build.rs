// rust/SSXL_godot/build.rs

use std::env;
use std::path::PathBuf;
use std::process::Command; // 🛑 New import for executing shell commands

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // The directory where cargo.toml resides (e.g., .../rust/aetherion_godot)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // The name of the resulting GDExtension DLL (must match your .gdextension file)
    let dll_name = "ssxl_engine.dll"; 
    
    // Determine the profile (debug or release)
    let profile = env::var("PROFILE").unwrap();    

    // --- 1. Calculate Source Path ---
    let src_path = manifest_dir
        .parent().unwrap() // .. to rust/
        .join("target")
        .join(&profile)
        .join(dll_name);

    // --- 2. Calculate Destination Path ---
    let dst_path = manifest_dir
        .parent().unwrap()      // up to rust/
        .parent().unwrap()      // up to zv9.aetherion/
        .join("ssxl_engine_tester")
        .join(dll_name);

    // --- 3. Execute Native Windows Copy Command ---
    // Using `cmd /C copy /Y` is more robust on Windows than fs::copy.
    let status = Command::new("cmd")
        // /C runs the command and terminates; copy /Y overwrites without prompt
        .args(&["/C", "copy", "/Y"]) 
        .arg(&src_path)
        .arg(&dst_path)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=✅ SUCCESS (CMD): Deployed {} to {}", dll_name, dst_path.display());
        }
        _ => {
            println!("cargo:warning=🚨 CRITICAL FAILURE (CMD): DLL deployment failed!");
            println!("cargo:warning=  Source Path: {}", src_path.display());
            println!("cargo:warning=  Dest Path: {}", dst_path.display());
            println!("cargo:warning=  Note: The shell command failed. Check file locks or path permissions.");
        }
    }
}