use std::process::Command;
use tracing::{info, warn, error};
use std::fs;
use std::env;

// Import constants and core pathing utility from the parent module (actions/mod.rs).
// CORRECTED PATH: Changed 'super::super' to 'super'.
use super::{
    get_godot_project_abs_path,
    GODOT_EXE_PATH,
    RELATIVE_PROJECT_PATH_FRAGMENT,
    DLL_NAME,
    SOURCE_DLL_PATH_FRAGMENT,
};


// -----------------------------------------------------------------------------
// DLL COPY FUNCTION
// -----------------------------------------------------------------------------

/// Copies the latest compiled DLL from the Rust target directory to the Godot tester project.
/// Designed to be run automatically during CLI boot.
pub fn copy_dll_to_tester_project_at_boot() -> Result<(), String> {
    info!("Attempting to copy {} to Godot tester project...", DLL_NAME);

    // 1. Construct Source Path
    let mut source = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for source: {}", e))?;
    source.push(SOURCE_DLL_PATH_FRAGMENT);
    source.push(DLL_NAME);
    let source_path = source.as_path();

    // 2. Construct Destination Path
    let mut destination = env::current_dir()
        .map_err(|e| format!("Failed to get current directory for destination: {}", e))?;
    destination.push(RELATIVE_PROJECT_PATH_FRAGMENT);
    destination.push(DLL_NAME);
    let destination_path = destination.as_path();

    // 3. Check if Source DLL exists (implies a successful cargo build run)
    if !source_path.exists() {
        // If the DLL is missing, it's not a fatal error for the CLI, just a warning
        warn!("Source DLL not found at: {}. Have you run `cargo build` recently?", source_path.display());
        return Ok(());
    }

    // 4. Perform Copy
    match fs::copy(source_path, destination_path) {
        Ok(_) => {
            info!("✅ DLL Copied: {} -> {}", source_path.file_name().unwrap_or_default().to_string_lossy(), destination_path.display());
            Ok(())
        }
        Err(e) => {
            // Note: If Godot is running and locked the file, this will fail.
            Err(format!("❌ FAILED to copy DLL. Check permissions/if Godot is running. Error: {}", e))
        }
    }
}


// -----------------------------------------------------------------------------
// GODOT CLIENT LAUNCH
// -----------------------------------------------------------------------------

/// 🚀 Launches the full **Godot Editor** (non-headless) with the project loaded.
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
    info!("Loading project at (Absolute Path): {}", project_path_abs);

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
            warn!("Please ensure the Godot executable is in the correct path relative to your CLI: {}", GODOT_EXE_PATH);
        }
    }
}


/// 🎮 Placeholder to launch headless Godot
pub fn launch_headless_godot() {
	warn!("🎮 Placeholder: Attempting to launch headless Godot (simple path check)...");
	
	match Command::new(GODOT_EXE_PATH).arg("--version").status() {
		Ok(status) if status.success() => info!("🚀 Headless Godot launch command ready (path check OK)."),
		_ => error!("❌ Godot executable not found or command failed. Check path: {}", GODOT_EXE_PATH),
	}
}

// -----------------------------------------------------------------------------
// HARNESS EXECUTION (ADDED)
// -----------------------------------------------------------------------------

/// ⚙️ Executes the Godot test harness, typically running a specific scene 
/// or script in headless mode to validate FFI communication.
/// This function is added to resolve the E0432 unresolved import error in 
/// ssxl_cli/src/actions/mod.rs.
pub fn run_godot_harness() {
    warn!("⚙️ Godot Test Harness execution not yet fully implemented. Placeholder called.");
    // Implementation will go here later: call headless Godot to run a test scene.
}