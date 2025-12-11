// ============================================================================
// 🧪 SSXL CLI: Core Test Runner (`ssxl_cli::actions::test_core_runner`)
// ----------------------------------------------------------------------------
// This module provides utilities for executing both Rust-based unit/integration
// tests and Godot-based headless integration tests. It acts as the backbone of
// the CLI testing framework, ensuring that internal Rust logic and external
// Godot integration are validated consistently.
//
// Key Functions:
//   • run_cargo_tests
//       - Executes the full Rust test suite via `cargo test`.
//       - Runs with `--nocapture` to stream output directly to the console.
//       - Reports success/failure based on Cargo’s exit status.
//
//   • read_all_output_from_stream
//       - Helper function for reading all output from a process stream.
//       - Runs in a separate thread to avoid blocking the main process.
//       - Collects stdout/stderr into strings for later logging.
//
//   • run_godot_test
//       - Generic runner for Godot-based integration tests.
//       - Accepts a test title, scene path, and success message.
//       - Spawns Godot in headless mode with the specified scene.
//       - Captures stdout and stderr concurrently for full visibility.
//       - Logs formatted output and reports success/failure based on Godot’s
//         exit status.
//
// Workflow:
//   1. Rust unit/integration tests are executed via `run_cargo_tests`.
//   2. Godot is launched in headless mode with a specific test scene.
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
//   • This module demonstrates how Rust can orchestrate external processes to
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
    get_godot_project_abs_path, GODOT_EXE_PATH, 
    GODOT_TEST_SCENE, HEADLESS_ANIM_TEST_SCENE, HEADLESS_GEN_TEST_SCENE,
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

pub(crate) fn run_godot_test(
    test_title: &str,
    test_scene: &str,
    success_message: &str,
) -> bool {
    info!("🔥 STARTING: {}...", test_title);

    let project_path_abs = match get_godot_project_abs_path() {
        Ok(path) => path,
        Err(e) => {
            error!("❌ {} failed: {}", test_title, e);
            return false;
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
        .arg(test_scene)
        .arg("--run")
        .arg("--quit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to spawn Godot process for {}: {}", test_title, e);
            warn!(
                "Please ensure the Godot executable is in the correct path: {}",
                GODOT_EXE_PATH
            );
            return false;
        }
    };

    let stdout = child.stdout.take().expect("Failed to capture stdout stream");
    let stderr = child.stderr.take().expect("Failed to capture stderr stream");

    let stdout_handle = thread::spawn(move || read_all_output_from_stream(stdout));
    let stderr_handle = thread::spawn(move || read_all_output_from_stream(stderr));

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("❌ Godot process failed to wait for {}: {}", test_title, e);
            return false;
        }
    };

    let stdout_output = stdout_handle.join().unwrap_or_else(|_| "Stdout reading thread panicked.".to_string());
    let stderr_output = stderr_handle.join().unwrap_or_else(|_| "Stderr reading thread panicked.".to_string());
    
    println!("\n--- GODOT {} TEST OUTPUT START ---", test_title.to_uppercase());
    println!("{}", stdout_output);
    println!("--- GODOT {} TEST OUTPUT END ---\n", test_title.to_uppercase());

    if status.success() {
        info!("✅ {} SUCCEEDED! {}", test_title, success_message);
        true
    } else {
        error!(
            "❌ {} FAILED! Exit code: {:?}",
            test_title,
            status.code()
        );
        eprintln!("--- GODOT ERROR OUTPUT ---");
        eprintln!("{}", stderr_output);
        false
    }
}
