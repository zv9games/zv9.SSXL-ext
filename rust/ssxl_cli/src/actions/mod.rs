// ============================================================================
// ⚙️ SSXL CLI Actions Module (`ssxl_cli::actions`)
// ----------------------------------------------------------------------------
// This module acts as the central façade for all command-line actions in the
// SSXL engine CLI. It aggregates constants, utilities, and exports from child
// modules, providing a unified interface for launching Godot, running tests,
// monitoring conductor status, and initiating benchmarks.
//
// Structure:
//   • Submodules:
//       - benchmarking: Tools for real-time monitoring of the Conductor and
//         placeholders for benchmark logic.
//       - godot_harness: Utilities for launching and managing the external
//         Godot tester project (editor and headless modes).
//       - testing: Menu and orchestration logic for executing test suites.
//       - test_suites: Self-contained architectural and data validation tests.
//       - test_core_suites: Tests requiring external processes like `cargo`
//         builds and Godot FFI validation.
//
//   • Configuration Constants:
//       - GODOT_EXE_PATH: Absolute path to the Godot executable.
//       - RELATIVE_PROJECT_PATH_FRAGMENT: Relative path to the Godot tester
//         project’s GDExtension folder.
//       - DLL_NAME: Expected name of the compiled Rust dynamic library.
//       - SOURCE_DLL_PATH_FRAGMENT: Path fragment where the compiled DLL is
//         found (e.g., `target/debug/`).
//       - GODOT_TEST_SCENE: Scene path for FFI bridge validation.
//       - HEADLESS_GEN_TEST_SCENE: Scene path for headless generation pipeline
//         validation.
//       - HEADLESS_ANIM_TEST_SCENE: Scene path for headless animation tempo
//         validation.
//
//   • Utility Functions:
//       - get_godot_project_abs_path: Calculates the absolute path to the Godot
//         tester project root, ensuring portability across environments.
//
//   • Public Exports (Façade):
//       - start_signal_inspector: Real-time conductor monitoring feed.
//       - copy_dll_to_tester_project_at_boot, launch_godot_client,
//         launch_headless_godot: Godot harness utilities.
//       - run_cargo_tests, run_ffi_bridge_validation,
//         run_headless_generation_integration_test,
//         run_headless_animation_tempo_test: Core test suite exports.
//       - run_communication_channel_test, run_data_channel_test,
//         run_map_generation_test, run_animation_conductor_test: General test
//         suite exports.
//       - execute_testing_menu: Entry point for orchestrating test execution.
//
// Design Choices:
//   • Modular organization ensures separation of concerns between harnessing,
//     benchmarking, and testing logic.
//   • Constants centralize configuration for portability and maintainability.
//   • Re-exports provide a clean façade, allowing external callers to interact
//     with CLI actions without needing to know internal module structure.
//
// Educational Note:
//   • This module demonstrates the façade pattern in Rust: exposing a curated
//     set of functions and constants from multiple submodules to simplify
//     external usage.
//   • By consolidating CLI actions here, developers gain a single, predictable
//     entry point for managing Godot integration, conductor monitoring, and
//     validation pipelines.
// ============================================================================


use std::env;
use std::path::PathBuf;

mod benchmarking;
mod godot_harness;
mod testing;
mod test_suites;
mod test_core_suites;

pub const GODOT_EXE_PATH: &str = "C:/ZV9/zv9.SSXL-ext/SSXL_engine_tester/godot.windows.editor.x86_64.exe"; 
pub const RELATIVE_PROJECT_PATH_FRAGMENT: &str = "../SSXLtester2/";
pub const DLL_NAME: &str = "ssxl_engine.dll";
pub const SOURCE_DLL_PATH_FRAGMENT: &str = "target/debug/";
pub const GODOT_TEST_SCENE: &str = "res://tests/ffi_bridge_test.tscn"; 
pub const HEADLESS_GEN_TEST_SCENE: &str = "res://tests/headless_gen_pipeline.tscn";
pub const HEADLESS_ANIM_TEST_SCENE: &str = "res://tests/headless_anim_tempo.tscn";

pub fn get_godot_project_abs_path() -> Result<PathBuf, String> {
    let mut path = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    path.push(RELATIVE_PROJECT_PATH_FRAGMENT);

    if path.pop() {
        Ok(path)
    } else {
        Err(format!(
            "Failed to determine parent directory for project path: {}",
            path.display()
        ))
    }
}

pub use benchmarking::start_signal_inspector;

pub use godot_harness::{
    copy_dll_to_tester_project_at_boot,
    launch_godot_client,
    launch_headless_godot,
};

pub use test_core_suites::{
    run_cargo_tests,
    run_ffi_bridge_validation,
    run_headless_generation_integration_test,
    run_headless_animation_tempo_test,
};

pub use test_suites::{
    run_communication_channel_test,
    run_data_channel_test,
    run_map_generation_test,
    run_animation_conductor_test,
};

#[allow(unused_imports)]
pub use testing::execute_testing_menu;
