//! Core action implementations for the SSXL Dev Console CLI menu.

use std::process::Command;
use tracing::{info, warn, error};

// NEW IMPORTS for Signal Inspector / Concurrency
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use ctrlc;
use std::env;

// PHASE 2 TRANSITION: Import the Conductor types
use ssxl_generate::conductor::{Conductor, ConductorStatus};

// --- CONSTANTS ---
// FIX: Consolidate path constants for re-use in multiple launch functions
const GODOT_EXE_PATH: &str = "./godot.windows.editor.x86_64.exe";
const RELATIVE_PROJECT_PATH_FRAGMENT: &str = "../ssxl_engine_tester";
const GODOT_TEST_SCENE: &str = "res://test_scene/test_ffi_data.tscn";


// --- CORE CLI ACTIONS ---

/// 🚀 Runs the full Rust test suite via Cargo
pub fn run_cargo_tests() {
	println!("🚀 Running full cargo test suite...");

	let status = Command::new("cargo")
		.args(&["test", "--", "--nocapture"])
		.status()
		.expect("Failed to run cargo test");

	if status.success() {
		info!("✅ All tests passed.");
	} else {
		error!("❌ Some tests failed.");
	}
}

/// Helper to calculate the absolute path to the Godot project tester directory.
fn get_godot_project_abs_path() -> Result<String, String> {
    let mut current_dir = env::current_dir()
        .map_err(|e| format!("Failed to determine CWD: {}", e))?;

    current_dir.push(RELATIVE_PROJECT_PATH_FRAGMENT);

    current_dir.canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Cannot resolve project path fragment '{}': {}. Does the directory exist?", RELATIVE_PROJECT_PATH_FRAGMENT, e))
}

// -----------------------------------------------------------------------------
// PHASE 8: NON-HEADLESS CLIENT LAUNCH (UPDATED)
// -----------------------------------------------------------------------------

/// 🚀 Launches the full **Godot Editor** (non-headless) with the project loaded.
/// This is the Phase 8 validation goal for debugging the main scene structure.
pub fn launch_godot_client() {
    info!("🚀 LAUNCHING: Godot Editor (Non-Headless) for scene debugging...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Launch failed: {}", e);
            return;
        }
    };

    info!("Attempting to launch Godot from: {}", GODOT_EXE_PATH);
    info!("Loading project at (Absolute Path): {}", project_path_abs);

    // Launch the Godot process with the --editor flag
    match Command::new(GODOT_EXE_PATH)
        .arg("--editor") // <--- 🔥 CRITICAL FIX: Forces the launch of the Godot Editor UI
        .arg("--path")
        .arg(&project_path_abs)
        .spawn() // Use spawn() for non-blocking launch
    {
        Ok(_) => {
            info!("✅ Godot EDITOR spawned successfully. Please close the Godot window to continue CLI use.");
        }
        Err(e) => {
            error!("❌ Failed to execute Godot command: {}", e);
            warn!("Please ensure the Godot executable is in the current directory: {}", GODOT_EXE_PATH);
        }
    }
}


/// 🎮 Placeholder to launch headless Godot
pub fn launch_headless_godot() {
	warn!("🎮 Placeholder: Attempting to launch headless Godot (simple path check)...");
	
	// Simplified status check for the placeholder
	match Command::new(GODOT_EXE_PATH).arg("--version").status() {
		Ok(status) if status.success() => info!("🚀 Headless Godot launch command ready (path check OK)."),
		_ => error!("❌ Godot executable not found or command failed. Check path: {}", GODOT_EXE_PATH),
	}
}

// -----------------------------------------------------------------------------
// PHASE 7: FFI BRIDGE VALIDATION (E2E FINAL)
// -----------------------------------------------------------------------------

/// 🔥 Runs an end-to-end test of the FFI bridge by launching Godot headless
/// to load the dedicated GDExtension test scene and validate data transfer.
pub fn run_ffi_bridge_validation() {
    info!("🔥 STARTING: FFI Bridge and GDExtension Integration Validation...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return;
        }
    };

    info!("Launching Godot (Headless) from: {}", GODOT_EXE_PATH);
    info!("Running test scene: {} in project (Absolute Path): {}", GODOT_TEST_SCENE, project_path_abs);

    // --- 3. Launch Godot Headless to Execute the GDExtension Test ---
    let godot_command = Command::new(GODOT_EXE_PATH)
        .arg("--headless") // Run without a GUI
        .arg("--path")
        .arg(&project_path_abs) // Pass the calculated absolute path
        .arg("--scene")
        .arg(GODOT_TEST_SCENE)
        .output();

    // --- 4. Process the Output ---
    match godot_command {
        Ok(output) => {
            // Print the standard output from the Godot process
            println!("\n--- GODOT TEST OUTPUT START ---");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("--- GODOT TEST OUTPUT END ---\n");

            if output.status.success() {
                info!("✅ FFI/GDExtension Bridge VALIDATION SUCCEEDED!");
            } else {
                error!("❌ FFI/GDExtension Bridge VALIDATION FAILED! Exit code: {:?}", output.status.code());
                eprintln!("--- GODOT ERROR OUTPUT ---");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            error!("❌ Failed to execute Godot command: {}", e);
            warn!("Please ensure the Godot executable is in the current directory: {}", GODOT_EXE_PATH);
        }
    }
}

// -----------------------------------------------------------------------------
// PHASE 1: FOUNDATION VALIDATION
// -----------------------------------------------------------------------------

/// Runs only the unit tests defined in the Phase 1 Foundation Layer packages.
pub fn run_priority_1_tests() {
	info!("Running Phase 1 Foundation Layer (P1) test suite...");

	// Target packages: aetherion_shared, aetherion_math, aetherion_sync
	let result = Command::new("cargo")
		.arg("test")
		.arg("--package")
		.arg("aetherion_shared")
		.arg("--package")
		.arg("aetherion_math")
		.arg("--package")
		.arg("aetherion_sync")
		.arg("--all-targets")
		.status();

	match result {
		Ok(status) if status.success() => {
			info!("✅ Phase 1 Validation Complete: All foundation tests passed successfully.");
		}
		_ => {
			error!("❌ Phase 1 Validation Failed. Check the errors above.");
		}
	}
}

// -----------------------------------------------------------------------------
// PHASE 4: SIGNAL INSPECTOR / LIVE FEED
// -----------------------------------------------------------------------------

/// 🔮 Starts the live **Signal Inspector** utility (CLI Menu [B]).
///
/// This function initializes the **Conductor** and begins a real-time inspection loop
/// using the shared `ConductorState`.
pub fn start_signal_inspector() {
    warn!("🔮 Initializing Conductor and starting Signal Inspector (Real-Time Feed)...");

    // 1. Initialize Conductor and retrieve the thread-safe state
    // FIX: Update destructuring to handle the new 3-element tuple returned by Conductor::new(None)
    let (conductor, state, _receiver) = match Conductor::new(None) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };

    // Wrap Conductor in Arc<Mutex<Option<>>> for safe, single consumption by the ctrlc handler.
    let conductor_shutdown_safe = Arc::new(Mutex::new(Some(conductor)));
    let shutdown_clone = conductor_shutdown_safe.clone();

    // 2. Setup atomic flag for graceful exit via Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r_for_handler = running.clone();

    // Set a Ctrl-C handler to stop the loop and shut down the Conductor
    if let Err(e) = ctrlc::set_handler(move || {
        r_for_handler.store(false, Ordering::SeqCst);

        // Atomically take the Conductor out of the Mutex and shut it down once.
        if let Some(c) = shutdown_clone.lock().unwrap().take() {
            c.graceful_teardown();
        }
        let _ = writeln!(io::stdout(), "\nInspector: Shutdown signal received. Waiting for Conductor...");
    }) {
        error!("Could not set Ctrl-C handler: {}", e);
        return;
    }

    info!("Inspector: Press Ctrl-C to stop the live feed and gracefully shut down the Conductor.");

    let mut frame_count: u64 = 0;
    const MVG_BASELINE: u64 = 10_000_000;

    // 3. Main Live Feed Loop
    while running.load(Ordering::SeqCst) {
        frame_count += 1;

        // --- REAL-TIME DATA POLLING ---
        let status = state.get_status();
        let queue_depth = state.get_queue_depth();
        let active_id = state.get_active_generator_id();

        // Use carriage return (`\r`) to overwrite the current line.
        print!("\r");
        print!("🔮 LIVE FEED | Frame: {: >4} | Status: {: <12} | Generator: {: <25} | Queue Depth: ~{: >6} | MVG Baseline: {} tiles/s | Press Ctrl-C to exit.",
            frame_count,
            format!("{:?}", status),
            active_id,
            queue_depth,
            MVG_BASELINE
        );
        let _ = io::stdout().flush();

        // Check for internal shutdown signals (e.g., error in Conductor)
        if status == ConductorStatus::ShuttingDown || status == ConductorStatus::Error {
            running.store(false, Ordering::SeqCst);

            if let Some(c) = conductor_shutdown_safe.lock().unwrap().take() {
                c.graceful_teardown();
            }
            break;
        }

        // Wait 50ms (20 FPS refresh)
        thread::sleep(Duration::from_millis(50));
    }

    // 4. Cleanup: Clear the line after exiting the loop
    let _ = writeln!(io::stdout(), "\r{: <200}", " "); // Overwrite and clear the line
    info!("Inspector shutdown complete. Conductor runtime terminated.");
}