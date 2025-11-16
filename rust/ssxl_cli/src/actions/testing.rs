// FILE: ssxl_cli/src/actions/testing.rs

//! # CLI Actions: Testing Utilities (`ssxl_cli::actions::testing`)
//!
//! Provides a menu for executing various test suites, delegating the complex
//! execution logic to the internal `test_suites` and external `test_core_suites` modules.

use crate::cli_util_menu::{CliAction, CliMenu};
use crate::actions::test_suites;      // Architectural tests
use crate::actions::test_core_suites; // External tests
use std::sync::LazyLock;              // NEW: Used for lazy, thread-safe static initialization

// --- Menu Setup Logic ---

// FIX 1 & 2: Use LazyLock to initialize the actions at runtime (solving E0015) 
// and store them in a Vec<CliAction>. The CliAction struct itself must have 
// been updated in cli_util_menu.rs to include Send + Sync bounds (solving E0277).
#[allow(dead_code)]
static TEST_ACTIONS: LazyLock<Vec<CliAction>> = LazyLock::new(|| vec![
    CliAction::new("cargo", "Run all standard `cargo test` suites (unit/integration)."),
    CliAction::new("ffi", "Run FFI bridge and GDExtension integration validation (Headless Godot)."),
    CliAction::new("channel", "Run Communication Channel Tempo Test."),
    CliAction::new("generation", "Run Map Generation Test (Perlin Generator)."),
    CliAction::new("animation", "Run Animation Conductor Data Contract Test."),
    // FIX 2: Added the "back" action which is used to exit the loop
    CliAction::new("back", "↩️ Return to Main Menu"), 
]);

/// Central entry point for all CLI testing actions.
#[allow(dead_code)]
pub fn execute_testing_menu() -> Result<(), String> {
    // FIX 3: Get a slice from the LazyLock Vec to satisfy CliMenu's 'a [CliAction] requirement.
    let menu = CliMenu::new("Testing and Validation Suites", TEST_ACTIONS.as_slice());
    
    loop {
        // FIX 4: `*s == "id"` and no misplaced `Ok(())` returns are now correct.
        match menu.prompt_action() {
            CliAction { id: ref s, .. } if *s == "cargo" => {
                test_core_suites::run_cargo_tests(); 
            }
            CliAction { id: ref s, .. } if *s == "ffi" => {
                test_core_suites::run_ffi_bridge_validation(); 
            }
            CliAction { id: ref s, .. } if *s == "channel" => {
                test_suites::run_communication_channel_test(); 
            }
            CliAction { id: ref s, .. } if *s == "generation" => {
                test_suites::run_map_generation_test(); 
            }
            CliAction { id: ref s, .. } if *s == "animation" => {
                test_suites::run_animation_conductor_test(); 
            }
            CliAction { id: ref s, .. } if *s == "back" => return Ok(()),
            _ => continue,
        }
    }
}