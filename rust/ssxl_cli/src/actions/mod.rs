// ssxl_cli/src/actions/mod.rs (Consolidated and Cleaned)

// --- MODULE DECLARATION (Only once) ---
mod benchmarking;
mod godot_harness;
mod testing;

// --- CRATE IMPORTS & RE-EXPORTS ---
// OLD: use ssxl_generate::Conductor; // <-- DELETE THIS LINE
use ssxl_tools::get_config_from_path;
use tracing::{info, error, warn};

// Import necessary std libraries
use std::process::{Command, Stdio};
use std::io;
use std::env;
use std::fs;

// Import Conductor types for re-export (This line correctly handles the import and re-export)
pub use ssxl_generate::conductor::Conductor; // Conductor is pub in its module
// FIX: Import ConductorStatus from its correct, publicly accessible path, as it is not
// directly re-exported by the 'conductor' module.
pub use ssxl_generate::conductor_state::ConductorStatus;


// --- CONSTANTS ---
pub const GODOT_EXE_PATH: &str = "./../SSXL_engine_tester/godot.windows.editor.x86_64.exe";
pub const RELATIVE_PROJECT_PATH_FRAGMENT: &str = "../SSXL_engine_tester";
pub const GODOT_TEST_SCENE: &str = "res://test_scene/test_ffi_data.tscn";
pub const DLL_NAME: &str = "SSXL_engine.dll";
pub const SOURCE_DLL_PATH_FRAGMENT: &str = "target/debug/";


// --- CORE CLI ACTIONS ---

/// Helper to calculate the absolute path to the Godot project tester directory.
pub fn get_godot_project_abs_path() -> Result<String, String> {
    let mut current_dir = env::current_dir()
        .map_err(|e| format!("Failed to determine CWD: {}", e))?;

    current_dir.push(RELATIVE_PROJECT_PATH_FRAGMENT);

    current_dir.canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Cannot resolve project path fragment '{}': {}. Does the directory exist?", RELATIVE_PROJECT_PATH_FRAGMENT, e))
}

// --- PUBLIC RE-EXPORTS ---

// Re-export functions from the three sub-modules for the CLI's main entry point.
pub use benchmarking::start_signal_inspector;
pub use benchmarking::run_benchmark;

pub use godot_harness::{
    copy_dll_to_tester_project_at_boot,
    launch_godot_client,
    launch_headless_godot,
    run_godot_harness,
};

pub use testing::{
    run_cargo_tests,
    run_ffi_bridge_validation,
    run_priority_1_tests,
    run_test_suite,
};