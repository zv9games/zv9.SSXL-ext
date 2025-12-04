use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use tracing::{error, info, warn};

use super::{
    get_godot_project_abs_path, GODOT_EXE_PATH, 
    // Keep constants here for completeness, or only required ones if splitting strictly
    // by dependencies. Keeping them here allows `run_godot_test` to access them.
    GODOT_TEST_SCENE, HEADLESS_ANIM_TEST_SCENE, HEADLESS_GEN_TEST_SCENE,
};

/// Runs the full cargo test suite.
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

/// Helper function to read all output from a stream in a non-blocking way. (O(n) on stream size)
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

/// Generic runner for Godot-based integration tests.
/// Extracts all boilerplate for spawning and output piping.
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
    
    // Output formatting and final result logging.
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