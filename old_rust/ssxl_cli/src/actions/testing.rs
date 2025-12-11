// ============================================================================
// 🧪 SSXL CLI: Testing Menu (`ssxl_cli::actions::testing`)
// ----------------------------------------------------------------------------
// This module provides a command-line menu for executing various validation
// suites. It acts as the central dispatcher, delegating execution to both
// internal architectural tests and external integration tests.
//
// Purpose:
//   • Offer a unified interface for developers to run different categories of
//     tests directly from the CLI.
//   • Simplify orchestration by exposing a menu-driven workflow rather than
//     requiring manual invocation of individual test functions.
//   • Ensure that both Rust-only validation (unit/integration tests) and
//     Godot-linked integration tests can be triggered consistently.
//
// Key Components:
//   • TEST_ACTIONS (LazyLock<Vec<CliAction>>)
//       - Defines the available test actions as menu items.
//       - Each action has an identifier (`id`) and a description.
//       - LazyLock ensures thread-safe, runtime initialization of the menu.
//
//   • execute_testing_menu
//       - Central entry point for the testing menu.
//       - Creates a `CliMenu` with all available test actions.
//       - Enters a loop, prompting the user to select an action.
//       - Matches the selected action against its `id` and delegates execution
//         to the appropriate test function.
//       - Supports returning to the main menu via the "back" action.
//
// Supported Actions:
//   • "cargo"      → Runs all standard Rust unit/integration tests via Cargo.
//   • "ffi"        → Runs FFI bridge and GDExtension integration validation
//                    (headless Godot).
//   • "channel"    → Runs communication channel tempo test (non-blocking mpsc).
//   • "generation" → Runs Perlin-based map generation test.
//   • "animation"  → Runs animation conductor data contract test.
//   • "back"       → Exits the testing menu and returns to the main CLI menu.
//
// Design Choices:
//   • `CliAction` and `CliMenu` abstract away menu logic, keeping this module
//     focused on orchestration.
//   • LazyLock ensures initialization happens only once, avoiding global
//     mutable state issues.
//   • Pattern matching on `id` strings provides a simple, extensible way to
//     add new test actions in the future.
//
// Educational Note:
//   • This module demonstrates how to build a façade for test orchestration,
//     combining multiple subsystems into a single, developer-friendly entry
//     point.
//   • By centralizing test execution here, developers can quickly validate
//     both internal architecture and external integration without switching
//     contexts or running multiple commands manually.
// ============================================================================


use crate::cli_util_menu::{CliAction, CliMenu};
use crate::actions::test_suites;
use crate::actions::test_core_suites;
use std::sync::LazyLock;

#[allow(dead_code)]
static TEST_ACTIONS: LazyLock<Vec<CliAction>> = LazyLock::new(|| vec![
    CliAction::new("cargo", "Run all standard `cargo test` suites (unit/integration)."),
    CliAction::new("ffi", "Run FFI bridge and GDExtension integration validation (Headless Godot)."),
    CliAction::new("channel", "Run Communication Channel Tempo Test."),
    CliAction::new("generation", "Run Map Generation Test (Perlin Generator)."),
    CliAction::new("animation", "Run Animation Conductor Data Contract Test."),
    CliAction::new("back", "↩️ Return to Main Menu"), 
]);

#[allow(dead_code)]
pub fn execute_testing_menu() -> Result<(), String> {
    let menu = CliMenu::new("Testing and Validation Suites", TEST_ACTIONS.as_slice());
    
    loop {
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
