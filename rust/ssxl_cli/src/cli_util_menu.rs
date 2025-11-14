// ssxl_cli/src/cli_util_menu.rs

//! # CLI Utilities: Dev Console Menu (`ssxl_cli::cli_util_menu`)
//!
//! Defines the interactive main menu structure and logic for the SSXL-ext
//! developer console. It maps user-input keys to specific actions (tests,
//! benchmarks, inspection, or external tool launches).

// Imports of action functions from sibling modules within `ssxl_cli`.
use crate::actions::{
    run_cargo_tests,
    launch_godot_client,
    launch_headless_godot,
    start_signal_inspector,
    run_ffi_bridge_validation,
    // NEW FOCUSED TESTS
    run_communication_channel_test,
    run_data_channel_test,
    run_map_generation_test,
    run_animation_conductor_test,
};
use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};
use tracing::warn;


// --- FIX: Add missing required types and adjust visibility ---

/// FIX: Renamed from `MenuItem` to `CliAction` to resolve E0432 in testing.rs.
/// Represents a single selectable entry in the CLI menu.
pub struct CliAction {
    /// The character key the user presses to execute this item's action.
    pub key: char,
    /// The descriptive label displayed in the console menu.
    pub label: &'static str,
    /// FIX: Changing to `pub` so `testing.rs` can use pattern matching on the field.
    pub id: &'static str, // Added required `id` field based on usage in testing.rs
    /// A Boxed trait object holding the function to be executed when the item is selected.
    // FIX: Added `Send + Sync` bounds to make the type thread-safe for use in `static` or `LazyLock`.
    pub action: Box<dyn Fn() + Send + Sync + 'static>,
}

impl CliAction {
    // FIX: Removed `const` keyword to allow use of `Box::new` and the closure, resolving E0015.
    pub fn new(id: &'static str, label: &'static str) -> Self {
        // Note: This implementation is for `testing.rs` which uses the actions in an array.
        // It requires an action field, but here we can only define `id` and `label`.
        // The `testing.rs` file does not need the action in its array definition, 
        // so we can use a dummy action for array creation.
        // This is a common pattern when action structs are reused for different menu layers.
        CliAction { 
            key: '?', // Key is not used in the testing action list
            label, 
            id, 
            // The dummy closure must also be Send + Sync. Since it captures nothing, it is by default.
            action: Box::new(|| {}) 
        }
    }
}


/// FIX: Added `CliMenu` struct to resolve E0432 in testing.rs.
/// Represents a collection of actions for a sub-menu.
pub struct CliMenu<'a> {
    pub title: &'static str,
    pub actions: &'a [CliAction],
}

impl<'a> CliMenu<'a> {
    // FIX: Removed `const` keyword for consistency, though this function could be const if `actions` were static.
    pub fn new(title: &'static str, actions: &'a [CliAction]) -> Self {
        CliMenu { title, actions }
    }
    
    // Placeholder implementation for `prompt_action`
    pub fn prompt_action(&self) -> CliAction {
        // Since this is a utility file, this placeholder satisfies the compiler.
        unimplemented!() 
    }
}

// --- Menu Construction Logic ---

/// Constructs the complete list of menu items for the developer console.
///
/// Each item is a `CliAction` struct mapping a key to a specific action function.
// FIX: Update the return type to use the new CliAction name.
pub fn build_menu() -> Vec<CliAction> {
    vec![
        // --- Core Testing & Validation (Keys 0-5) ---
        // NOTE: The `id` field is redundant for this main menu but required by the struct definition.
        CliAction { key: '0', label: "✅ Run: Full Cargo Test Suite", id: "cargo_all", action: Box::new(run_cargo_tests) },
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

        // --- External Harness & Inspection (Keys L, H, S) ---
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

        // --- Benchmarks & Utilities (Keys T, B, P, A, I, R) ---
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
            label: "✅ Inspect: Godot-Callable API Surface", 
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
            // Inline action for a simple placeholder warning.
            action: Box::new(|| warn!("TODO: Trailkeeper scan not yet implemented.")) 
        },
        
        // --- Exit ---
        CliAction { key: 'E', label: "✅ Exit Console", id: "exit", action: Box::new(|| {}) },
    ]
}


/// Prints the formatted menu to the console, showing the key and label for each item.
// FIX: Update the parameter type to use the new CliAction name.
pub fn print_menu(menu: &[CliAction]) {
    
    println!("\n🧭 SSXL-ext Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nSelect an option by pressing its number key or letter key...\n");
}