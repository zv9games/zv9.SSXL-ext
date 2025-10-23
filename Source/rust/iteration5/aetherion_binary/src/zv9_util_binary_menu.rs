use aetherion_core::core::start;
use aetherion_core::zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;
#[allow(unused_imports)]
use crate::zv9_util_binary_func::run_cargo_tests; /*run_trailkeeper_scan, view_trailkeeper_logs};*/
use crate::zv9_util_binary_func2::{print_godot_api_surface, print_module_tree};
use crate::zv9_util_binary_func3::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};
#[allow(unused_imports)]
use aetherion_engine::inspect_pending_queue;

/// 🧩 Menu item definition
pub struct MenuItem {
    pub key: char,
    pub label: &'static str,
    pub action: Box<dyn Fn()>,
}

/// 🧭 Builds the interactive dev console menu
pub fn build_menu() -> Vec<MenuItem> {
    vec![
        // ✅ Stable Tools
        MenuItem { 
            key: '0', 
            label: "✅ Run: Cargo Test Suite [zv9_util_binary_func.rs]", 
            action: Box::new(run_cargo_tests),
        },
        MenuItem { 
            key: '1', 
            label: "✅ Inspect: Godot-Callable API Surface [zv9_util_binary_func2.rs]", 
            action: Box::new(print_godot_api_surface),
        },
        MenuItem { 
            key: '2', 
            label: "✅ Inspect: Rust Module Tree [zv9_util_binary_func2.rs]", 
            action: Box::new(print_module_tree),
        },
        MenuItem { 
            key: '3', 
            label: "⚠️ Run: Trailkeeper Scan [zv9_util_binary_func.rs]", 
            action: Box::new(|| println!("TODO: Trailkeeper scan not yet implemented.")),
        },
        MenuItem { 
            key: '4', 
            label: "⚠️ Start: Aetherion Runtime [inline in zv9_util_binary_menu.rs]", 
            action: Box::new(|| start(DummyDelivery::new())),
        },
        MenuItem { 
            key: '5', 
            label: "🧪 Test: Generation & Placement [zv9_util_binary_func3.rs]", 
            action: Box::new(test_generation_and_placement_cli),
        },
        MenuItem { 
            key: '6', 
            label: "✅ Perform: Bitmask PNG Conversion [zv9_util_binary_func3.rs]", 
            action: Box::new(run_bitmask_conversion),
        },
        MenuItem { 
            key: '7', 
            label: "🧪 Benchmark: Max Grid Placement [zv9_util_binary_func3.rs]", 
            action: Box::new(run_max_grid_benchmark),
        },
        MenuItem { 
            key: '9', 
            label: "✅ Exit [inline]", 
            action: Box::new(|| {}),
        },

        // 🔮 Future Expansion / TODOs
        MenuItem { key: 'A', label: "🔮 TODO: Export Chunk Hashes for Streaming", action: Box::new(|| println!("TODO: Chunk hashing not yet implemented.")) },
        MenuItem { key: 'B', label: "🔮 TODO: Signal Inspector / Live Feed", action: Box::new(|| println!("TODO: Signal inspector not yet implemented.")) },
        MenuItem { key: 'C', label: "🔮 TODO: Generate Pacman 2.0 Data Package", action: Box::new(|| println!("TODO: Pacman 2.0 data export not yet implemented.")) },
        MenuItem { key: 'D', label: "🔮 TODO: Configure Plugin Mode for External Engines", action: Box::new(|| println!("TODO: Plugin mode not yet implemented.")) },
        MenuItem { key: 'E', label: "🔮 TODO: Memory Usage & Performance Diagnostics", action: Box::new(|| println!("TODO: Diagnostics not yet implemented.")) },
        MenuItem { key: 'F', label: "🔮 TODO: Export TileMap to Godot Scene", action: Box::new(|| println!("TODO: TileMap export not yet implemented.")) },
        MenuItem { key: 'G', label: "🔮 TODO: Generate Procedural Biome Layer", action: Box::new(|| println!("TODO: Biome generation not yet implemented.")) },
        MenuItem { key: 'H', label: "🔮 TODO: Hash & Cache Chunk Data", action: Box::new(|| println!("TODO: Chunk caching not yet implemented.")) },
        MenuItem { key: 'I', label: "🔮 TODO: Inspect Chunk Merge Performance", action: Box::new(|| println!("TODO: Merge diagnostics not yet implemented.")) },
        MenuItem { key: 'J', label: "🔮 TODO: Launch Headless Batch Generator", action: Box::new(|| println!("TODO: Batch generator not yet implemented.")) },
        MenuItem { key: 'K', label: "🔮 TODO: Configure Plugin Mode for Unity", action: Box::new(|| println!("TODO: Unity plugin mode not yet implemented.")) },
    ]
}



/// 🖥 Prints the menu to the console
pub fn print_menu(menu: &[MenuItem]) {
    println!("\n🧭 Aetherion Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nSelect an option by pressing its number key...\n");
}
