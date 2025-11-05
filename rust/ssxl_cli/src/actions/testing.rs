use std::process::{Command, Stdio};
use tracing::{info, error, warn};
use std::thread;
use std::io;

// Import constants and core pathing utility from the parent module (actions/mod.rs).
// CORRECTED PATH: Changed 'super::super' to 'super'.
use super::{
    get_godot_project_abs_path,
    GODOT_EXE_PATH,
    GODOT_TEST_SCENE,
};

// -----------------------------------------------------------------------------
// CARGO AND PRIORITY TESTS
// -----------------------------------------------------------------------------

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

/// Runs only the unit tests defined in the Phase 1 Foundation Layer packages.
pub fn run_priority_1_tests() {
	info!("Running Phase 1 Foundation Layer (P1) test suite...");

	let result = Command::new("cargo")
		.arg("test")
		.arg("--package")
		.arg("ssxl_shared")
		.arg("--package")
		.arg("ssxl_math")
		.arg("--package")
		.arg("ssxl_sync")
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
// FFI BRIDGE VALIDATION (E2E FINAL)
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
    let mut command = Command::new(GODOT_EXE_PATH);
    command
        .arg("--headless") 
        .arg("--path")
        .arg(&project_path_abs)
        .arg("--scene")
        .arg(GODOT_TEST_SCENE)
        .arg("--quit") // CRITICAL FIX: Ensures Godot process exits immediately after running the scene
        .stdout(Stdio::piped()) // FIX: Explicitly pipe stdout for reading
        .stderr(Stdio::piped()); // FIX: Explicitly pipe stderr for reading

    // Spawn the process
    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process: {}", e);
            warn!("Please ensure the Godot executable is in the correct path relative to your CLI: {}", GODOT_EXE_PATH);
            return;
        }
    };

    // Capture streams BEFORE waiting for the process to exit
    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    // 1. Concurrently read STDOUT in a separate thread to prevent pipe deadlock
    let stdout_handle = thread::spawn(move || {
        io::read_to_string(stdout).unwrap_or_else(|_| "Failed to read stdout.".to_string())
    });

    // 2. Concurrently read STDERR in another thread to prevent pipe deadlock
    let stderr_handle = thread::spawn(move || {
        io::read_to_string(stderr).unwrap_or_else(|_| "Failed to read stderr.".to_string())
    });

    // 3. Main thread waits for the Godot process to exit
    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait: {}", e);
            return;
        }
    };

    // 4. Retrieve captured output from the concurrent threads
    let stdout_output = stdout_handle.join().unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle.join().unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());

    // --- 5. Process the Output ---
    println!("\n--- GODOT TEST OUTPUT START ---");
    println!("{}", stdout_output);
    println!("--- GODOT TEST OUTPUT END ---\n");

    if status.success() {
        info!("✅ FFI/GDExtension Bridge VALIDATION SUCCEEDED!");
    } else {
        error!("❌ FFI/GDExtension Bridge VALIDATION FAILED! Exit code: {:?}", status.code());
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
    }
}

// -----------------------------------------------------------------------------
// CORE TEST SUITE EXECUTION (ADDED)
// -----------------------------------------------------------------------------

/// 🧪 Runs a consolidated test suite, typically including priority tests,
/// cargo tests, and the final FFI bridge validation.
/// This function is added to resolve the E0432 unresolved import error in 
/// ssxl_cli/src/actions/mod.rs.
pub fn run_test_suite() {
    warn!("🧪 Full Test Suite execution not yet fully implemented. Placeholder called.");
    
    // In a final implementation, this would likely chain:
    // run_priority_1_tests();
    // run_cargo_tests();
    // run_ffi_bridge_validation();
}