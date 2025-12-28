// ssxl_ext/src/host_init.rs

// Conditionally include Godot dependencies only when needed.
#[cfg(feature = "godot-binding")]
use godot::prelude::*;

use crate::host_state::init_host_state;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor;
use crate::config::{GlobalConfig, DEBUG_HOST_INIT};
use std::sync::Arc;

// --- CORE LOGIC (Godot-independent and safe to call from CLI) ---

/// Internal function performing all core logic (Config loading, Conductor init, HostState init),
/// isolated from Godot-specific API calls.
fn _do_initialization() -> Result<(), String> {

    // 1. Load Global Configuration
    if DEBUG_HOST_INIT {
        eprintln!("DEBUG_HOST_INIT: Loading GlobalConfig...");
    }

    let config = match GlobalConfig::load_or_default() {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to load configuration: {}", e)),
    };
    let config = Arc::new(config);

    // 2. Initialize the Conductor
    if DEBUG_HOST_INIT {
        eprintln!("DEBUG_HOST_INIT: Initializing GenerateConductor...");
    }

    let num_workers = config.threading.generation_worker_count;
    let conductor = GenerateConductor::new(num_workers, Arc::clone(&config));

    // 3. Initialize the Animation Conductor
    if DEBUG_HOST_INIT {
        eprintln!("DEBUG_HOST_INIT: Initializing AnimConductor...");
    }

    let anim_conductor = AnimConductor::new();

    // 4. Initialize the Host State Singleton
    if DEBUG_HOST_INIT {
        eprintln!("DEBUG_HOST_INIT: Initializing HostState...");
    }

    init_host_state(conductor, anim_conductor, Arc::clone(&config))
        .map_err(|e| format!("HostState initialization failed: {}", e))
}


// --- FFI ENTRY POINT (CLI-safe) ---

#[cfg(not(feature = "godot-binding"))]
#[no_mangle]
pub extern "C" fn ssxl_boot_core_to_idle() -> i32 {
    match _do_initialization() {
        Ok(_) => {
            // CLI logs always print
            eprintln!("✅ SSXL-ext CLI Core: Initialization complete. Ready for FFI work.");
            0
        }
        Err(e) => {
            eprintln!("❌ CLI FFI Boot Error: {}", e);
            1
        }
    }
}


// --- GODOT ENTRY POINT (Feature-gated for GDExtension hook) ---

#[cfg(feature = "godot-binding")]
pub fn initialize_ssxl_core() -> Result<(), String> {

    if DEBUG_HOST_INIT {
        godot_print!("SSXL-ext Core: Starting initialization (v9.1.seed).");
    }

    match _do_initialization() {
        Ok(_) => {
            if DEBUG_HOST_INIT {
                godot_print!("SSXL-ext Core: Initialization complete. Ready for work.");
            }
            Ok(())
        }
        Err(e) => {
            // Errors always print
            godot_print!("❌ SSXL-ext Core FFI Error: {}", e);
            Err(e)
        }
    }
}
