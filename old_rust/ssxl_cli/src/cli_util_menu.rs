// ============================================================================
// 🧭 SSXL CLI: Developer Console Menu (`ssxl_cli::cli_util_menu`)
// ----------------------------------------------------------------------------
// This module defines the interactive command-line menu system for the SSXL
// engine developer console. It provides a structured way to organize, present,
// and execute various developer actions, including validation tests, benchmarks,
// and inspection utilities.
//
// Purpose:
//   • Offer a unified interface for developers to run tests, benchmarks, and
//     debugging tools directly from the CLI.
//   • Simplify orchestration by exposing a menu-driven workflow rather than
//     requiring manual invocation of individual functions.
//   • Ensure that both Rust-only validation and Godot-linked integration tests
//     can be triggered consistently.
//
// Key Components:
//   • CliAction
//       - Represents a single executable action in the menu.
//       - Contains:
//           • key: the character used to trigger the action.
//           • label: human-readable description shown in the menu.
//           • id: internal identifier for the action.
//           • action: closure wrapping the function to execute.
//       - Provides a `new` constructor for placeholder actions.
//
//   • CliMenu
//       - Represents a group of actions under a common title.
//       - Provides `new` for initialization.
//       - Includes `prompt_action` (currently unimplemented) for interactive
//         selection logic.
//
//   • build_menu
//       - Constructs the full list of available actions for the developer console.
//       - Organizes actions into categories:
//           I. Core Rust Validation (cargo tests, FFI, channels, map generation, animation).
//           II. Headless Integration Tests (Godot-based generation and animation validation).
//           III. Engine Launch & Debugging Tools (launch Godot client/headless, signal inspector).
//           IV. Benchmarks & Utilities (generation tests, grid benchmarks, bitmask conversion).
//           V. System Control (exit).
//       - Each action is mapped to a key and closure for execution.
//
//   • print_menu
//       - Prints the structured menu to the console.
//       - Displays each action’s key and label for user selection.
//       - Provides a clear prompt for interaction.
//
// Workflow:
//   1. Developer runs the CLI console.
//   2. `build_menu` constructs the list of available actions.
//   3. `print_menu` displays the menu with keys and labels.
//   4. User selects an action by pressing the corresponding key.
//   5. The associated closure executes the requested function.
//
// Design Choices:
//   • Actions are closures wrapped in `Box<dyn Fn()>` for flexibility and dynamic dispatch.
//   • Keys are single characters for quick selection in interactive mode.
//   • Labels use emojis and descriptive text for clarity and developer ergonomics.
//   • Menu organization mirrors developer workflows: validation, integration, debugging,
//     benchmarking, and control.
//
// Educational Note:
//   • This module demonstrates how to build a modular CLI menu system in Rust,
//     combining structured data (CliAction, CliMenu) with dynamic execution.
//   • By centralizing developer actions here, contributors gain a predictable,
//     ergonomic interface for testing and debugging the SSXL engine.
// ============================================================================


use crate::actions::{
    run_cargo_tests,
    run_ffi_bridge_validation,
    run_communication_channel_test,
    run_data_channel_test,
    run_map_generation_test,
    run_animation_conductor_test,
    run_headless_generation_integration_test,
    run_headless_animation_tempo_test,
    launch_godot_client,
    launch_headless_godot,
    start_signal_inspector,
};
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};
use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use tracing::warn;

pub struct CliAction {
    pub key: char,
    pub label: &'static str,
    pub id: &'static str,
    pub action: Box<dyn Fn() + Send + Sync + 'static>,
}

impl CliAction {
    pub fn new(id: &'static str, label: &'static str) -> Self {
        CliAction {
            key: '?',
            label,
            id,
            action: Box::new(|| {})
        }
    }
}

pub struct CliMenu<'a> {
    pub title: &'static str,
    pub actions: &'a [CliAction],
}

impl<'a> CliMenu<'a> {
    pub fn new(title: &'static str, actions: &'a [CliAction]) -> Self {
        CliMenu { title, actions }
    }
    
    pub fn prompt_action(&self) -> CliAction {
        unimplemented!()
    }
}

pub fn build_menu() -> Vec<CliAction> {
    vec![
        CliAction { 
            key: '0', 
            label: "✅ Run: Full Cargo Test Suite", 
            id: "cargo_all", 
            action: Box::new(run_cargo_tests) 
        },
        CliAction {    
            key: '1',    
            label: "✅ Validate: FFI Bridge Data Transfer (Data Integrity)",    
            id: "ffi_validate",
            action: Box::new(run_ffi_bridge_validation)
        },
        CliAction {    
            key: '2',    
            label: "✅ Validate: Async Communication Channels (Godot <-> Rust)",    
            id: "async_channel",
            action: Box::new(run_communication_channel_test)
        },
        CliAction {    
            key: '3',    
            label: "✅ Validate: Chunk/Tile Data Channels (Crypto Coded)",    
            id: "data_channel",
            action: Box::new(run_data_channel_test)
        },
        CliAction {    
            key: '4',    
            label: "✅ Validate: Map Generation Logic (Procedural Purity)",    
            id: "map_gen",
            action: Box::new(run_map_generation_test)
        },
        CliAction {    
            key: '5',    
            label: "✅ Validate: Animation Conductor Tempo (Frame Consistency)",    
            id: "animation_tempo",
            action: Box::new(run_animation_conductor_test)
        },
        CliAction { 
            key: '6', 
            label: "✅ Validate: Headless Generation Integration (Full Pipeline Simulation)", 
            id: "headless_gen_integration",
            action: Box::new(run_headless_generation_integration_test)
        },
        CliAction { 
            key: '7', 
            label: "✅ Validate: Headless Animation Tempo Integration (Signal Latency Check)", 
            id: "headless_anim_tempo",
            action: Box::new(run_headless_animation_tempo_test)
        },
        CliAction {    
            key: 'L',    
            label: "🚀 Launch: Godot Client (Non-Headless)",    
            id: "launch_client",
            action: Box::new(launch_godot_client)    
        },
        CliAction {    
            key: 'H',    
            label: "🎮 Launch: Headless Godot (External)",    
            id: "launch_headless",
            action: Box::new(launch_headless_godot)    
        },
        CliAction {    
            key: 'S',    
            label: "🔮 Start: Signal Inspector / Live Feed (TODO)",    
            id: "start_inspector",
            action: Box::new(start_signal_inspector)    
        },
        CliAction {    
            key: 'T',    
            label: "🧪 Test: Generation & Placement CLI",    
            id: "gen_cli_test",
            action: Box::new(test_generation_and_placement_cli)    
        },
        CliAction {    
            key: 'B',    
            label: "🧪 Benchmark: Max Grid Placement",    
            id: "max_grid_bench",
            action: Box::new(run_max_grid_benchmark)    
        },
        CliAction {    
            key: 'P',    
            label: "✅ Perform: Bitmask PNG Conversion",    
            id: "bitmask_convert",
            action: Box::new(run_bitmask_conversion)    
        },
        CliAction {    
            key: 'A',    
            label: "✅ Inspect: SSXL Developer API Surface (Callable + Signals)",    
            id: "api_surface",
            action: Box::new(print_godot_api_surface)    
        },
        CliAction {    
            key: 'I',    
            label: "✅ Inspect: Rust Module Tree",    
            id: "module_tree",
            action: Box::new(print_module_tree)    
        },
        CliAction {    
            key: 'R',    
            label: "⚠️ Run: Trailkeeper Scan (TODO)",    
            id: "trailkeeper_scan",
            action: Box::new(|| warn!("TODO: Trailkeeper scan not yet implemented."))    
        },
        CliAction { key: 'E', label: "✅ Exit Console", id: "exit", action: Box::new(|| {}) },
    ]
}

pub fn print_menu(menu: &[CliAction]) {
    println!("\n🧭 SSXL-ext Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nSelect an option by pressing its number key or letter key...\n");
}
