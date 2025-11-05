// ssxl_cli/src/cli_util_menu.rs

use crate::actions::{
    run_cargo_tests,
    launch_godot_client,
    launch_headless_godot,
    run_priority_1_tests,
    start_signal_inspector,
    run_ffi_bridge_validation,
};
use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};
use tracing::warn;

/// 🧩 Menu item definition (Kept for completeness, not edited)
pub struct MenuItem {
    pub key: char,
    pub label: &'static str,
    pub action: Box<dyn Fn()>,
}

/// 🧭 Builds the interactive dev console menu, prioritized for completion.
pub fn build_menu() -> Vec<MenuItem> {
    vec![
        // --- 0. CORE VALIDATION (The Essential Checklist) ---
        MenuItem { key: '0', label: "✅ Run: Full Cargo Test Suite", action: Box::new(run_cargo_tests) },
        MenuItem { 
            key: '1', 
            label: "✅ Validate: FFI Bridge Data Transfer (E2E Final)", 
            action: Box::new(run_ffi_bridge_validation) // Formerly key '9'
        },
        MenuItem { 
            key: '2', 
            label: "✅ Validate: Phase 1 Final Integration Check", 
            action: Box::new(run_priority_1_tests) // Formerly key 'C'
        },
        MenuItem { 
            key: '3', 
            label: "✅ Inspect: Godot-Callable API Surface", 
            action: Box::new(print_godot_api_surface) 
        },

        // --- 1. RUNTIME & INTEGRATION (The Bulldozer Launch) ---
        MenuItem { 
            key: '4', 
            label: "🚀 Launch: Godot Client (Non-Headless)", 
            action: Box::new(launch_godot_client) 
        },
        MenuItem { 
            key: '5', 
            label: "🎮 Launch: Headless Godot (External)", 
            action: Box::new(launch_headless_godot) 
        },

        // --- 2. GENERATION & PERFORMANCE (The Tempo Checks) ---
        MenuItem { 
            key: '6', 
            label: "🧪 Test: Generation & Placement CLI", 
            action: Box::new(test_generation_and_placement_cli) 
        },
        MenuItem { 
            key: '7', 
            label: "🧪 Benchmark: Max Grid Placement", 
            action: Box::new(run_max_grid_benchmark) 
        },

        // --- 3. UTILITIES & INSPECTION (The Maintenance Layer) ---
        MenuItem { 
            key: '8', 
            label: "✅ Perform: Bitmask PNG Conversion", 
            action: Box::new(run_bitmask_conversion) 
        },
        MenuItem { 
            key: '9', 
            label: "✅ Inspect: Rust Module Tree", 
            action: Box::new(print_module_tree) 
        },

        // --- 4. FUTURE EXPANSION / TODOS ---
        MenuItem { 
            key: 'A', 
            label: "🔮 Start: Signal Inspector / Live Feed (TODO)", 
            action: Box::new(start_signal_inspector) 
        },
        MenuItem { 
            key: 'B', 
            label: "⚠️ Run: Trailkeeper Scan (TODO)", 
            action: Box::new(|| warn!("TODO: Trailkeeper scan not yet implemented.")) 
        },
        
        // --- 5. EXIT ---
        MenuItem { key: 'E', label: "✅ Exit Console", action: Box::new(|| {}) },
        // Item 'F' could be used for 'Export Chunk Hashes for Streaming' if that becomes a priority.
    ]
}

// ... print_menu function remains the same ...

/// 🖥 Prints the menu to the console
pub fn print_menu(menu: &[MenuItem]) {
	
	println!("\n🧭 SSXL-ext Engine Dev Console\n");
	for item in menu {
		println!("[{}] {}", item.key, item.label);
	}
	println!("\nSelect an option by pressing its number key or letter key...\n");
}