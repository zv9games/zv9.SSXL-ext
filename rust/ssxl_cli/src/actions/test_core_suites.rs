// ============================================================================
// 🧪 SSXL CLI: Core Test Suites (`ssxl_cli::actions::test_core_suites`)
// ----------------------------------------------------------------------------
// This module defines the core integration tests that validate communication
// between the Rust engine and the Godot runtime. Unlike lightweight unit tests,
// these tests spawn external processes (Cargo and Godot) to ensure that the
// full pipeline—from compilation to FFI bridge to headless execution—works
// correctly in real runtime conditions.
//
// Key Functions:
//   • run_cargo_tests
//       - Executes the full Rust test suite via `cargo test`.
//       - Runs with `--nocapture` to stream output directly to the console.
//       - Validates that all Rust unit and integration tests pass before
//         attempting Godot integration.
//
//   • run_ffi_bridge_validation
//       - Launches Godot in headless mode with the FFI bridge validation scene.
//       - Confirms that the Rust dynamic library is correctly loaded via
//         GDExtension and that communication between Rust and Godot succeeds.
//       - Captures stdout/stderr from the Godot process for detailed logging.
//       - Reports success/failure based on Godot’s exit status.
//
//   • run_headless_generation_integration_test
//       - Executes a headless Godot scene that validates the full map generation
//         pipeline.
//       - Ensures procedural generation, streaming, and integration logic work
//         without requiring a graphical interface.
//       - Captures and logs Godot output for debugging and validation.
//
//   • run_headless_animation_tempo_test
//       - Runs a headless Godot scene focused on animation conductor tempo.
//       - Validates high-frequency signal emission and latency handling in the
//         conductor loop.
//       - Confirms that the conductor maintains expected performance under load.
//
// Workflow:
//   1. Rust unit/integration tests are executed via `run_cargo_tests`.
//   2. Godot is launched in headless mode with specific test scenes.
//   3. Output streams (stdout/stderr) are captured in parallel threads.
//   4. Results are logged to the console, including success/failure messages.
//   5. Exit codes and captured output provide detailed diagnostics.
//
// Design Choices:
//   • `std::process::Command` is used to spawn external processes (Cargo, Godot).
//   • `Stdio::piped` allows capturing stdout/stderr for real-time logging.
//   • Threads are used to read output streams concurrently, preventing blocking.
//   • `tracing` macros (`info`, `warn`, `error`) provide structured logging for
//     visibility and debugging.
//   • Headless Godot execution ensures tests can run in CI/CD pipelines without
//     requiring a GUI.
//
// Educational Note:
//   • These tests demonstrate how Rust can orchestrate external processes to
//     validate integration with another engine (Godot).
//   • By combining Cargo tests with Godot headless scenes, developers gain
//     confidence that both the Rust engine and its FFI bridge are functioning
//     correctly in real runtime conditions.
// ============================================================================


use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use tracing::{error, info, warn};

use super::{
    get_godot_project_abs_path, GODOT_EXE_PATH, GODOT_TEST_SCENE, HEADLESS_ANIM_TEST_SCENE,
    HEADLESS_GEN_TEST_SCENE,
};

pub fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...");

    let status = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .status()
        .expect("Failed to run cargo test command. Is cargo installed?");

    if status.success() {
        info!("✅ All Rust unit and integration tests passed.");
    } else {
        error!("❌ Some Rust tests failed. See output above.");
    }
}

fn read_all_output_from_stream<R: io::Read + Send + 'static>(stream: R) -> String {
    let mut reader = BufReader::new(stream);
    let mut output = String::new();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => output.push_str(&line),
            Err(_) => break,
        }
    }
    output
}

pub fn run_ffi_bridge_validation() {
    info!("🔥 STARTING: FFI Bridge and GDExtension Integration Validation...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return;
        }
    };

    let mut command = Command::new(GODOT_EXE_PATH);
    command
        .arg("--path")
        .arg(&project_path_abs)
        .arg("--headless")
        .arg("--no-window")
        .arg("--verbose")
        .arg("--display-print")
        .arg("--render-loop-time-slice")
        .arg("1")
        .arg("--main-scene")
        .arg(GODOT_TEST_SCENE)
        .arg("--run")
        .arg("--quit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process: {}", e);
            warn!(
                "Please ensure the Godot executable is in the correct path: {}",
                GODOT_EXE_PATH
            );
            return;
        }
    };

    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    let stdout_handle = thread::spawn(move || read_all_output_from_stream(stdout));
    let stderr_handle = thread::spawn(move || read_all_output_from_stream(stderr));

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait: {}", e);
            return;
        }
    };

    let stdout_output = stdout_handle
        .join()
        .unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle
        .join()
        .unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());

    println!("\n--- GODOT TEST OUTPUT START ---");
    println!("{}", stdout_output);
    println!("--- GODOT TEST OUTPUT END ---\n");

    if status.success() {
        info!("✅ FFI/GDExtension Bridge VALIDATION SUCCEEDED!");
    } else {
        error!(
            "❌ FFI/GDExtension Bridge VALIDATION FAILED! Exit code: {:?}",
            status.code()
        );
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
    }
}

pub fn run_headless_generation_integration_test() {
    info!("🔥 STARTING: Headless Map Generation Integration Test (Full Pipeline Validation)...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return;
        }
    };

    let mut command = Command::new(GODOT_EXE_PATH);
    command
        .arg("--path")
        .arg(&project_path_abs)
        .arg("--headless")
        .arg("--no-window")
        .arg("--verbose")
        .arg("--display-print")
        .arg("--render-loop-time-slice")
        .arg("1")
        .arg("--main-scene")
        .arg(HEADLESS_GEN_TEST_SCENE)
        .arg("--run")
        .arg("--quit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process: {}", e);
            warn!(
                "Please ensure the Godot executable is in the correct path: {}",
                GODOT_EXE_PATH
            );
            return;
        }
    };

    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    let stdout_handle = thread::spawn(move || read_all_output_from_stream(stdout));
    let stderr_handle = thread::spawn(move || read_all_output_from_stream(stderr));

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait: {}", e);
            return;
        }
    };

    let stdout_output = stdout_handle
        .join()
        .unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle
        .join()
        .unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());

    println!("\n--- GODOT HEADLESS GENERATION TEST OUTPUT START ---");
    println!("{}", stdout_output);
    println!("--- GODOT HEADLESS GENERATION TEST OUTPUT END ---\n");

    if status.success() {
        info!("✅ Headless Generation Integration Test SUCCEEDED! Quantum alignment confirmed.");
    } else {
        error!(
            "❌ Headless Generation Integration Test FAILED! Exit code: {:?}",
            status.code()
        );
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
    }
}

pub fn run_headless_animation_tempo_test() {
    info!("🔥 STARTING: Headless Animation Conductor Tempo Test (Signal Latency Check)...");

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return;
        }
    };

    let mut command = Command::new(GODOT_EXE_PATH);
    command
        .arg("--path")
        .arg(&project_path_abs)
        .arg("--headless")
        .arg("--no-window")
        .arg("--verbose")
        .arg("--display-print")
        .arg("--render-loop-time-slice")
        .arg("1")
        .arg("--main-scene")
        .arg(HEADLESS_ANIM_TEST_SCENE)
        .arg("--run")
        .arg("--quit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process: {}", e);
            warn!(
                "Please ensure the Godot executable is in the correct path: {}",
                GODOT_EXE_PATH
            );
            return;
        }
    };

    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    let stdout_handle = thread::spawn(move || read_all_output_from_stream(stdout));
    let stderr_handle = thread::spawn(move || read_all_output_from_stream(stderr));

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait: {}", e);
            return;
        }
    };

    let stdout_output = stdout_handle
        .join()
        .unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle
        .join()
        .unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());

    println!("\n--- GODOT HEADLESS ANIMATION TEMPO TEST OUTPUT START ---\n");
    println!("{}", stdout_output);
    println!("--- GODOT HEADLESS ANIMATION TEMPO TEST OUTPUT END ---\n");

    if status.success() {
        info!("✅ Headless Animation Tempo Test SUCCEEDED! Faster-than-light speed confirmed.");
    } else {
        error!(
            "❌ Headless Animation Tempo Test FAILED! Exit code: {:?}",
            status.code()
        );
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
    }
}