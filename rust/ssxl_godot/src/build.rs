// rust/SSXL_godot/src/build.rs

//! # Build Script for SSXL-ext GDExtension Deployment
//!
//! This script runs *before* the main compilation of the `ssxl_godot` crate.
//! Its purpose is to **automate the deployment** of the compiled GDExtension
//! dynamic library (`ssxl_engine.dll`) from the Cargo `target` directory
//! directly into the Godot test project (`ssxl_engine_tester`). This enables
//! fast iteration and "faster than light speed" development **tempo**.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 1. CARGO INSTRUCTION: Rerun on change
    // Tells Cargo to re-run this build script if the script itself is modified.
    println!("cargo:rerun-if-changed=build.rs");

    // 2. PATH SETUP: Determine the root of the current crate's manifest.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set"));

    // 3. ARTIFACT NAME: The name of the resulting dynamic library (DLL on Windows).
    let dll_name = "ssxl_engine.dll";

    // 4. PROFILE: Determine the build profile (e.g., "debug" or "release").
    // This is crucial as the DLL location depends on the profile folder within 'target/'.
    let profile = env::var("PROFILE")
        .expect("PROFILE environment variable not set");

    // --- Path Calculation ---

    // 5. SOURCE PATH: Calculate the path to the newly compiled DLL.
    // The structure is generally: /workspace/rust/ssxl_godot -> /workspace/rust -> target/profile/ssxl_engine.dll
    let src_path = manifest_dir
        .parent().unwrap()      // Moves from `/rust/ssxl_godot` to `/rust`
        .join("target")
        .join(&profile)         // Inserts "debug" or "release"
        .join(dll_name);

    // 6. DESTINATION PATH: Calculate the path to the Godot test project folder.
    // The structure is: /workspace/rust/ssxl_godot -> /workspace/rust -> /workspace -> ssxl_engine_tester/ssxl_engine.dll
    let dst_path = manifest_dir
        .parent().unwrap()      // Moves from `/rust/ssxl_godot` to `/rust`
        .parent().unwrap()      // Moves from `/rust` to `/workspace` root
        .join("ssxl_engine_tester") // The Godot project folder
        .join(dll_name);

    // --- Deployment Execution (Windows-Specific) ---

    // 7. COMMAND: Execute a Windows shell command (`cmd /C copy`) to perform the deployment.
    // `/C` executes the command and then terminates.
    // `/Y` suppresses prompting to confirm overwriting the existing file.
    let status = Command::new("cmd")
        .args(&["/C", "copy", "/Y"])
        .arg(&src_path)
        .arg(&dst_path)
        .status();

    // 8. RESULT HANDLING: Report the outcome using Cargo warnings.
    match status {
        Ok(s) if s.success() => {
            // Success: The experiment's result is deployed and ready for testing!
            println!("cargo:warning=✅ SUCCESS (CMD): Deployed {} to {}", dll_name, dst_path.display());
        }
        Ok(s) => {
            // Failure with status code: Common if files are locked (e.g., Godot is running).
            println!("cargo:warning=🚨 CRITICAL FAILURE (CMD): DLL deployment failed with exit code: {:?}", s.code());
            println!("cargo:warning=  Source Path: {}", src_path.display());
            println!("cargo:warning=  Dest Path: {}", dst_path.display());
            println!("cargo:warning=  Note: The shell command failed. Check file locks (is Godot editor running?) or path permissions.");
        }
        Err(e) => {
             // System error: Command could not even be executed.
            println!("cargo:warning=❌ SYSTEM ERROR (CMD): Failed to execute copy command: {}", e);
        }
    }
}