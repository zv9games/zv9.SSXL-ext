// FILE: ssxl_cli\src\actions\test_core_suites.rs

//! # Core Integration and External Test Suites
//!
//! Houses tests that rely on external tools like `cargo test` or involve heavy
//! I/O and process management (e.g., headless Godot execution and FFI bridge validation).

use std::process::{Command, Stdio};
use tracing::{info, error, warn};
use std::thread;
use std::io;

// Import utilities from the actions module.
use super::{
    get_godot_project_abs_path,
    GODOT_EXE_PATH,
    GODOT_TEST_SCENE,
};


// -----------------------------------------------------------------------------
// EXTERNAL TESTING SUITES 
// -----------------------------------------------------------------------------

/// Executes the full, default `cargo test` suite for the entire workspace.
pub fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...");

    let status = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .status()
        .expect("Failed to run cargo test command. Is cargo installed?");

    if status.success() {
        info!("✅ All Rust unit and integration tests passed.");
    } else {
        error!("❌ Some Rust tests failed. See output above.");
    }
}


/// Runs the core integration test against a headless Godot instance to validate
/// the FFI bridge and GDExtension module loading/calls.
pub fn run_ffi_bridge_validation() {
    info!("🔥 STARTING: FFI Bridge and GDExtension Integration Validation...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return;
        }
    };
    
    let mut command = Command::new(GODOT_EXE_PATH);
    command
        .arg("--headless")
        .arg("--path").arg(&project_path_abs)
        .arg("--scene").arg(GODOT_TEST_SCENE)
        .arg("--quit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process: {}", e);
            warn!("Please ensure the Godot executable is in the correct path: {}", GODOT_EXE_PATH);
            return;
        }
    };

    // --- Capture and Read Output Concurrently ---
    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    let stdout_handle = thread::spawn(move || {
        io::read_to_string(stdout).unwrap_or_else(|_| "Failed to read stdout.".to_string())
    });

    let stderr_handle = thread::spawn(move || {
        io::read_to_string(stderr).unwrap_or_else(|_| "Failed to read stderr.".to_string())
    });

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait: {}", e);
            return;
        }
    };

    let stdout_output = stdout_handle.join().unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle.join().unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());

    // --- Report Results ---
    println!("\n--- GODOT TEST OUTPUT START ---");
    println!("{}", stdout_output);
    println!("--- GODOT TEST OUTPUT END ---\n");

    if status.success() {
        info!("✅ FFI/GDExtension Bridge VALIDATION SUCCEEDED!");
    } else {
        error!("❌ FFI/GDExtension Bridge VALIDATION FAILED! Exit code: {:?}", status.code());
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
    }
}