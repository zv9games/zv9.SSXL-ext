// ============================================================================
// 🧭 SSXL-ext CLI Developer Console (`ssxl_cli::main`)
// ----------------------------------------------------------------------------
// The main entry point, handling initialization, core stubs, and coordinating
// the interactive menu and source scanning modules.
// ============================================================================

// --- Module Declarations ---
pub mod ssxl_menu;
pub mod ssxl_source_scan;
pub mod ssxl_api_scan;
pub mod ssxl_testing; // ✅ ADDED: New module for all test implementations

use std::io;
use std::fs;
use std::path::PathBuf;
use tracing::{info, error, warn};
use tracing_subscriber::{self, filter::LevelFilter, prelude::*};

// Re-export public functions required by ssxl_menu.rs
pub use ssxl_api_scan::print_godot_api_surface;
// ✅ ADDED: Re-export the single, combined test function
pub use ssxl_testing::run_grand_unified_test;

// --- FFI Declarations (Exported by ssxl_ext) ---
extern "C" {
    // New function to replace the ssxl_start_runtime MOCK
    fn ssxl_boot_core_to_idle() -> i32;

    // FFI functions used by mocked menu actions (kept for linking)
    fn ssxl_set_cell(x: i32, y: i32, tile_id: i32);
    fn ssxl_notify_tilemap_update();
}


// --- RUNTIME BOOT AND UTILITIES ---

/// ✅ REAL: Calls the FFI function in ssxl_ext.dll to initialize the engine core.
fn ssxl_start_runtime() -> bool {
    info!("Engine FFI core: Attempting to boot to idle via DLL...");
    // Safety: Calls an FFI function defined in ssxl_ext.
    unsafe {
        match ssxl_boot_core_to_idle() {
            0 => {
                info!("✅ Engine FFI core initialized (REAL).");
                true
            },
            e => {
                error!("❌ Engine FFI core failed to boot. Exit code: {}", e);
                false
            }
        }
    }
}

/// ✅ REAL: Copies the built ssxl_ext.dll to the Godot tester project folder.
fn copy_dll_to_tester_project_at_boot() -> Result<(), String> {
    let source = PathBuf::from("target/debug/ssxl_ext.dll");
    let destination = PathBuf::from("../SSXLtester2/ssxl_ext.dll");

    if !source.exists() {
        warn!("Source DLL not found: {}. Did you run 'cargo build'?", source.display());
        return Err(format!("Source DLL not found: {}", source.display()));
    }

    match fs::copy(&source, &destination) {
        Ok(_) => {
            info!("✅ DLL Copy: Successfully copied {} to {}.",
                  source.display(), destination.display());
            Ok(())
        },
        Err(e) => {
            if !destination.parent().map_or(false, |p| p.exists()) {
                 return Err(format!("❌ DLL Copy Failed: Destination directory ({}) does not exist. Error: {}",
                                     destination.parent().unwrap_or(&destination).display(), e));
            }
            Err(format!("❌ DLL Copy Failed: Could not copy from {} to {}. Error: {}",
                        source.display(), destination.display(), e))
        }
    }
}

// ⚠️ REMOVED: All mock function definitions (run_fast_test, etc. and print_module_tree) 
// are gone from here. Their functionality is now provided by ssxl_testing.rs 
// and the only needed re-export is run_grand_unified_test.


fn init_logging_and_engine() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    info!("SSXLBinary: Interactive CLI initializing.");

    // This is now a real FFI call
    if !ssxl_start_runtime() {
        error!("Fatal: Engine FFI core failed to initialize. Aborting console boot.");
    }
    
    // This is a real copy operation
    if let Err(e) = copy_dll_to_tester_project_at_boot() {
        error!("DLL Copy Failed: {}", e);
    }
}

fn main() {
    // 1. Logging, FFI Initialization, and DLL Copy
    init_logging_and_engine();
    
    // 2. FFI Linker Guards: These references ensure the linker attempts to resolve the symbols.
    let _ = ssxl_boot_core_to_idle as *const ();
    let _ = ssxl_set_cell as *const ();
    let _ = ssxl_notify_tilemap_update as *const ();
    
    // 3. LOC Report and Banner - ***MODULE CALL***
    ssxl_source_scan::scan_and_report_loc();
    
    println!(
        r#"
                  (__)   
                  (oo)
            /------\/
           / |    ||
          * ||----||
            ~~    ~~
SSXL-ext Engine Console Initialized
"#
    );

    // 4. Interactive Loop - ***MODULE CALL***
    let menu = ssxl_menu::build_menu();
    ssxl_menu::run_interactive_loop(menu);
}