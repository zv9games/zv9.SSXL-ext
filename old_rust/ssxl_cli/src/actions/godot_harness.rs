// ============================================================================
// 🎮 SSXL CLI: Godot Harness (`ssxl_cli::actions::godot_harness`)
// ----------------------------------------------------------------------------
// This module provides command-line utilities for integrating the SSXL engine
// with the Godot editor and runtime. It automates critical setup tasks such as
// copying the compiled Rust dynamic library into the Godot project and
// launching Godot in either editor or headless mode.
//
// Key Functions:
//   • copy_dll_to_tester_project_at_boot
//       - Copies the compiled Rust dynamic library (DLL/SO/DYLIB) from the
//         `target` build directory into the Godot project’s GDExtension folder.
//       - Ensures Godot loads the latest engine code when the project runs.
//       - Validates source path existence, logs warnings if missing, and
//         handles errors gracefully (e.g., file locked by running Godot).
//
//   • launch_godot_client
//       - Launches the Godot Editor in a non-blocking subprocess.
//       - Useful for debugging scenes and testing engine integration with GUI.
//       - Uses `--editor` flag to open the editor instead of running the game.
//       - Logs success/failure and provides guidance if the executable path is
//         misconfigured.
//
//   • launch_headless_godot
//       - Launches Godot in headless mode (no GUI).
//       - Ideal for automated testing, CI pipelines, or server-side execution.
//       - Uses `--headless` flag to run without a graphical interface.
//       - Logs success/failure and ensures correct path configuration.
//
// Workflow:
//   1. Build Rust engine (`cargo build`).
//   2. Run `copy_dll_to_tester_project_at_boot` to sync DLL into Godot project.
//   3. Launch Godot via either `launch_godot_client` (editor mode) or
//      `launch_headless_godot` (headless mode).
//   4. Godot loads the DLL from its GDExtension folder, enabling engine
//      integration.
//
// Design Choices:
//   • `std::process::Command` provides portable subprocess execution.
//   • `tracing` macros (`info`, `warn`, `error`) ensure structured logging for
//     visibility and debugging.
//   • Path construction uses `env::current_dir` combined with project-specific
//     constants for portability across environments.
//   • Non-blocking subprocesses (`spawn`) allow the CLI to continue running
//     while Godot executes.
//
// Educational Note:
//   • This harness demonstrates how Rust can act as a build-time and runtime
//     companion to Godot, automating repetitive tasks and ensuring smooth
//     integration.
//   • By centralizing DLL copying and process launching here, developers avoid
//     manual setup errors and streamline the workflow between Rust and Godot.
// ============================================================================


use std::process::Command;
use tracing::{info, warn, error};
use std::fs;
use std::env;

use super::{
    get_godot_project_abs_path,
    GODOT_EXE_PATH,
    RELATIVE_PROJECT_PATH_FRAGMENT,
    DLL_NAME,
    SOURCE_DLL_PATH_FRAGMENT,
};

pub fn copy_dll_to_tester_project_at_boot() -> Result<(), String> {
    info!("Attempting to copy {} to Godot tester project...", DLL_NAME);

    let mut source = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for source path construction: {}", e))?;
    source.push(SOURCE_DLL_PATH_FRAGMENT);
    source.push(DLL_NAME);
    let source_path = source.as_path();

    let mut destination = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for destination path construction: {}", e))?;
    destination.push(RELATIVE_PROJECT_PATH_FRAGMENT);
    destination.push(DLL_NAME);
    let destination_path = destination.as_path();

    if !source_path.exists() {
        warn!(
            "Source DLL not found at: {}. Have you run `cargo build` recently?", 
            source_path.display()
        );
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
            Err(format!(
                "❌ FAILED to copy DLL. Check permissions or if the Godot Editor is currently running and locking the file. Error: {}", 
                e
            ))
        }
    }
}

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
    info!("Loading project at (Absolute Path): {}", project_path_abs.display()); 

    match Command::new(GODOT_EXE_PATH)
        .arg("--editor")
        .arg("--path")
        .arg(&project_path_abs)
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
    info!("Loading project at (Absolute Path): {}", project_path_abs.display()); 

    match Command::new(GODOT_EXE_PATH)
        .arg("--headless")
        .arg("--path")
        .arg(&project_path_abs)
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
