// ssxl_ext\src\host_init.rs

use godot::prelude::*;
use crate::host_state::init_host_state;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor;
use crate::config::GlobalConfig;
use std::sync::Arc;

// --- CORE LOGIC (Godot-independent and safe to call from CLI) ---

/// Internal function performing all core logic (Config loading, Conductor init, HostState init),
/// isolated from Godot-specific API calls like godot_print!.
fn _do_initialization() -> Result<(), String> {
    
    // 1. Load Global Configuration
    let config = match GlobalConfig::load_or_default() {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to load configuration: {}", e)),
    };
    let config = Arc::new(config);

    // 2. Initialize the Conductor
    let num_workers = config.threading.generation_worker_count;
    let conductor = GenerateConductor::new(
        num_workers,
        Arc::clone(&config),
    );

    // 3. Initialize the Animation Conductor
    let anim_conductor = AnimConductor::new();
    
    // 4. Initialize the Host State Singleton
    // FIX: Map the custom SSXLCoreError returned by init_host_state to a generic String.
    init_host_state(conductor, anim_conductor, Arc::clone(&config))
        .map_err(|e| format!("HostState initialization failed: {}", e))
}


// --- FFI ENTRY POINT (CLI-safe) ---

/// C-exported entry point for the ssxl_cli tool to initialize the core and wait in idle.
/// This function is the real target for the FFI call in the CLI's main.rs.
/// Returns 0 on success, and a non-zero error code on failure (following C conventions).
#[no_mangle]
pub extern "C" fn ssxl_boot_core_to_idle() -> i32 {
    match _do_initialization() { 
        Ok(_) => {
            // Use standard console output (eprintln!) for the CLI environment.
            eprintln!("✅ SSXL-ext CLI Core: Initialization complete. Ready for FFI work.");
            0 // Success
        },
        Err(e) => {
            // Use standard console error output for the CLI environment.
            eprintln!("❌ CLI FFI Boot Error: {}", e);
            1 // Generic error code for boot failure
        }
    }
}


// --- GODOT ENTRY POINT (Original logic, retained for GDExtension hook) ---

/// The public function used by the Godot GDExtension lifecycle hook.
/// It wraps the core logic with Godot's logging API.
pub fn initialize_ssxl_core() -> Result<(), String> {
    
    godot_print!("SSXL-ext Core: Starting initialization (v9.1.seed).");
    
    match _do_initialization() { 
        Ok(_) => {
            godot_print!("SSXL-ext Core: Initialization complete. Ready for work.");
            Ok(())
        },
        Err(e) => {
            // Retain the original use of godot_print! for logging inside the Godot environment
            godot_print!("❌ SSXL-ext Core FFI Error: {}", e);
            Err(e)
        }
    }
}