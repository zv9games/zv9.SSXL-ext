use crate::actions::{
    // --- Core Test & Validation Functions (Godot/Rust interaction) ---
    run_cargo_tests,
    run_ffi_bridge_validation,
    run_communication_channel_test,
    run_data_channel_test,
    run_map_generation_test,
    run_animation_conductor_test,
    
    // --- NEW: Headless Integration Tests for Rendering Logic (Fixing Generate/Animate) ---
    run_headless_generation_integration_test, // Validates 'generate' FFI data pipeline.
    run_headless_animation_tempo_test,        // Validates 'animate' conductor tempo/latency.
    
    // --- Launch & Debug Functions ---
    launch_godot_client,
    launch_headless_godot,
    start_signal_inspector,
};
// Benchmark/Utility functions are correctly imported from their dedicated module:
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};
use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use tracing::warn;


/// Defines a single executable action within the Command Line Interface menu.
/// Each action is a closure wrapped in a Box for dynamic execution.
pub struct CliAction {
    pub key: char,
    pub label: &'static str,
    pub id: &'static str,
    pub action: Box<dyn Fn() + Send + Sync + 'static>,
}

impl CliAction {
    /// Helper constructor (primarily for TODO/unimplemented actions).
    pub fn new(id: &'static str, label: &'static str) -> Self {
        CliAction {
            key: '?',
            label,
            id,
            action: Box::new(|| {}) // Default no-op action
        }
    }
}


/// Defines a structure for organizing and presenting a group of CLI actions.
pub struct CliMenu<'a> {
    pub title: &'static str,
    pub actions: &'a [CliAction],
}

impl<'a> CliMenu<'a> {
    pub fn new(title: &'static str, actions: &'a [CliAction]) -> Self {
        CliMenu { title, actions }
    }
    
    // Note: prompt_action implementation is omitted as it is environment-specific.
    pub fn prompt_action(&self) -> CliAction {
        unimplemented!()
    }
}


/// Constructs the complete list of available actions for the SSXL Engine Dev Console.
pub fn build_menu() -> Vec<CliAction> {
    vec![
        // =======================================================================
        // I. CORE RUST VALIDATION (Internal Checks)
        // =======================================================================
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

        // =======================================================================
        // II. HEADLESS INTEGRATION TESTS (Simulating Engine in Godot)
        // This targets the root of our 'generate' and 'animate' rendering errors.
        // =======================================================================
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

        // =======================================================================
        // III. ENGINE LAUNCH & DEBUGGING TOOLS
        // =======================================================================
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

        // =======================================================================
        // IV. BENCHMARKS & UTILITIES
        // =======================================================================
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
        
        // =======================================================================
        // V. SYSTEM CONTROL
        // =======================================================================
        CliAction { key: 'E', label: "✅ Exit Console", id: "exit", action: Box::new(|| {}) },
    ]
}


/// Prints the structured menu to the console, ready for user selection.
pub fn print_menu(menu: &[CliAction]) {
    
    println!("\n🧭 SSXL-ext Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nSelect an option by pressing its number key or letter key...\n");
}