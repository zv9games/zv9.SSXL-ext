// FILE: ssxl_cli/src/actions/mod.rs

//! # CLI Actions Module (`ssxl_cli::actions`)
//!
//! This module acts as the public interface (façade) for all complex command-line
//! actions, such as launching Godot, running tests, or initiating benchmarks.
//! It aggregates constants, external dependencies, and exports from its child modules.

use std::env;
use std::path::PathBuf;

// --- Internal Modules ---

/// Tools for real-time monitoring of the Conductor and placeholder for benchmark logic.
mod benchmarking;
/// Utilities for launching and managing the external Godot engine tester project.
mod godot_harness;
/// The main menu and delegation stub for all test suites.
mod testing;
/// Contains self-contained architectural and data validation tests.
mod test_suites;
/// Contains tests requiring external processes like `cargo` and Godot FFI validation.
mod test_core_suites;

// --- Public Re-exports from Sibling Crates ---

// --- Configuration Constants ---

/// The absolute path to the Godot executable file.
pub const GODOT_EXE_PATH: &str = "C:/ZV9/zv9.SSXL-ext/SSXL_engine_tester/godot.windows.editor.x86_64.exe"; 
/// The project-relative path fragment pointing to the GDExtension folder (e.g., `godot_tester_project/gde/`).
pub const RELATIVE_PROJECT_PATH_FRAGMENT: &str = "../SSXL_engine_tester/";
/// The expected file name of the compiled Rust dynamic library (e.g., `ssxl_engine.dll`).
pub const DLL_NAME: &str = "ssxl_engine.dll";
/// The project-relative path fragment where the compiled DLL is found (e.g., `target/debug/`).
pub const SOURCE_DLL_PATH_FRAGMENT: &str = "target/debug/";
/// The scene path within the Godot project used for FFI bridge validation tests.
pub const GODOT_TEST_SCENE: &str = "res://tests/ffi_bridge_test.tscn"; // <--- ADDED THIS LINE

// --- Utility Functions ---

/// Calculates the absolute path to the Godot tester project root.
///
/// The calculation starts from the current working directory, navigates to the 
/// GDExtension path, and then steps back one level to find the project root.
pub fn get_godot_project_abs_path() -> Result<PathBuf, String> {
    // Start from the current working directory (usually the crate root)
    let mut path = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Append the relative project fragment path (e.g., 'godot_tester_project/gde/')
    path.push(RELATIVE_PROJECT_PATH_FRAGMENT);

    // Remove the last component (the `gde/` part) to get the project root
    if path.pop() {
        Ok(path)
    } else {
        Err(format!(
            "Failed to determine parent directory for project path: {}",
            path.display()
        ))
    }
}


// --- Public Module Exports (Façade) ---

/// Re-export for starting the **real-time status feed**.
pub use benchmarking::start_signal_inspector;

// 🎯 FIX: Add the missing pub use statements for all functions required by cli_util_menu.rs.
// Exports from godot_harness (Includes the requested launch_headless_godot).
pub use godot_harness::{
    copy_dll_to_tester_project_at_boot,
    launch_godot_client,
    launch_headless_godot,
};

// Exports from test_core_suites.
pub use test_core_suites::{
    run_cargo_tests,
    run_ffi_bridge_validation,
};

// Exports from test_suites.
pub use test_suites::{
    run_communication_channel_test,
    run_data_channel_test,
    run_map_generation_test,
    run_animation_conductor_test,
};


/// Exports related to **testing and validation**.
#[allow(unused_imports)]
pub use testing::execute_testing_menu;