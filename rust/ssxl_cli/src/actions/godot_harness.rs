// FILE: ssxl_cli/src/actions/godot_harness.rs

//! # CLI Actions: Godot Harness (`ssxl_cli::actions::godot_harness`)
//!
//! Utilities for managing and launching the Godot editor or the Godot game
//! client from the command line, including necessary setup steps like copying
//! the compiled Rust dynamic library (DLL/SO/DYLIB) into the Godot project's
//! GDExtension directory.

use std::process::Command;
use tracing::{info, warn, error};
use std::fs;
use std::env;

// Imports of constants and utility functions from the parent module.
use super::{
    get_godot_project_abs_path,
    GODOT_EXE_PATH,
    RELATIVE_PROJECT_PATH_FRAGMENT,
    DLL_NAME,
    SOURCE_DLL_PATH_FRAGMENT,
};


/// Copies the compiled Rust dynamic library (DLL/SO/DYLIB) from the `target/release`
/// or `target/debug` folder to the Godot tester project's GDExtension directory.
///
/// This is a critical step to ensure Godot loads the latest engine code.
pub fn copy_dll_to_tester_project_at_boot() -> Result<(), String> {
    info!("Attempting to copy {} to Godot tester project...", DLL_NAME);

    // --- 1. Construct Source Path ---
    let mut source = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for source path construction: {}", e))?;
    // Navigate to the target directory (e.g., `target/debug/`).
    source.push(SOURCE_DLL_PATH_FRAGMENT);
    // Add the DLL file name (e.g., `SSXL_engine.dll`).
    source.push(DLL_NAME);
    let source_path = source.as_path();

    // --- 2. Construct Destination Path ---
    let mut destination = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for destination path construction: {}", e))?;
    // Navigate to the Godot project's GDExtension folder.
    destination.push(RELATIVE_PROJECT_PATH_FRAGMENT);
    // Add the DLL file name.
    destination.push(DLL_NAME);
    let destination_path = destination.as_path();

    // --- 3. Validation and Copy ---

    if !source_path.exists() {
        warn!(
            "Source DLL not found at: {}. Have you run `cargo build` recently?", 
            source_path.display()
        );
        // Treat missing source as a non-fatal warning to continue CLI usage.
        return Ok(());
    }

    match fs::copy(source_path, destination_path) {
        Ok(_) => {
            info!(
                "✅ DLL Copied: {} -> {}", 
                source_path.file_name().unwrap_or_default().to_string_lossy(), 
                destination_path.display()
            );
            Ok(())
        }
        Err(e) => {
            // This often fails if the target DLL is locked by a running Godot instance.
            Err(format!(
                "❌ FAILED to copy DLL. Check permissions or if the Godot Editor is currently running and locking the file. Error: {}", 
                e
            ))
        }
    }
}


/// Launches the Godot Editor in a non-blocking subprocess.
pub fn launch_godot_client() {
    info!("🚀 LAUNCHING: Godot Editor (Non-Headless) for scene debugging...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Launch failed: {}", e);
            return;
        }
    };

    info!("Attempting to launch Godot from: {}", GODOT_EXE_PATH);
    // FIX: Call .display() on PathBuf to implement Display trait
    info!("Loading project at (Absolute Path): {}", project_path_abs.display()); 

    // Execute the Godot process.
    match Command::new(GODOT_EXE_PATH)
        // Flag to launch the editor window instead of running the game directly.
        .arg("--editor")
        // Argument specifying the path to the Godot project folder.
        .arg("--path")
        .arg(&project_path_abs)
        // Use `spawn()` to run the command asynchronously, allowing the CLI process to continue.
        .spawn()
    {
        Ok(_) => {
            info!("✅ Godot EDITOR spawned successfully. Please close the Godot window to continue CLI use.");
        }
        Err(e) => {
            error!("❌ Failed to execute Godot command: {}", e);
            warn!("Please ensure the Godot executable is correctly set in the path configuration: {}", GODOT_EXE_PATH);
        }
    }
}

/// Launches Godot in a non-blocking subprocess using the `--headless` flag.
/// 
/// This is used for automated testing where no GUI is needed.
pub fn launch_headless_godot() {
    info!("🚀 LAUNCHING: Godot Headless Client...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Launch failed: {}", e);
            return;
        }
    };

    info!("Attempting to launch Godot from: {}", GODOT_EXE_PATH);
    // FIX: Call .display() on PathBuf to implement Display trait
    info!("Loading project at (Absolute Path): {}", project_path_abs.display()); 

    match Command::new(GODOT_EXE_PATH)
        // Flag to run Godot without a graphical interface.
        .arg("--headless")
        // Argument specifying the path to the Godot project folder.
        .arg("--path")
        .arg(&project_path_abs)
        // Use `spawn()` to run the command asynchronously.
        .spawn()
    {
        Ok(_) => {
            info!("✅ Godot HEADLESS client spawned successfully.");
        }
        Err(e) => {
            error!("❌ Failed to execute Godot headless command: {}", e);
            warn!(
                "Please ensure the Godot executable is correctly set in the path configuration: {}", 
                GODOT_EXE_PATH
            );
        }
    }
}