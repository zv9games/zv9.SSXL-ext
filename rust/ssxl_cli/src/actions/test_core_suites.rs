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