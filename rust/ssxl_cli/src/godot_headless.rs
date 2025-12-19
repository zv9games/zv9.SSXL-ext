use std::process::Command;
use std::path::Path;
use crate::ssxl_config::SsxlConfig;

pub fn run_godot_headless_tests() -> Result<(), String> {
    // Load config
    let config = SsxlConfig::load();

    // Allow override via environment variable
    let godot = std::env::var("SSXL_GODOT_PATH")
        .unwrap_or_else(|_| config.godot.binary.clone());

    // ✅ Your Godot project directory (where project.godot lives)
    let project_dir = "C:/zv9/zv9.SSXL-ext/SSXLtester2";

    // Validate the binary exists
    if !Path::new(&godot).exists() {
        return Err(format!(
            "❌ Godot binary not found at: {}\n\
             Set SSXL_GODOT_PATH or update ssxl_config.toml",
            godot
        ));
    }

    // ✅ Run Godot headless from the correct project directory
    let output = Command::new(&godot)
        .current_dir(project_dir)   // ✅ THIS FIXES THE FILE-NOT-FOUND ERROR
        .args(["--headless", "--script", "res://ssxl_test_runner.gd"])
        .output()
        .map_err(|e| format!("Failed to launch Godot ('{}'): {}", godot, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // If Godot failed, dump everything
    if !output.status.success() {
        return Err(format!(
            "❌ Godot headless tests FAILED\n\
             --- STDOUT ---\n{}\n\
             --- STDERR ---\n{}\n\
             Exit code: {:?}",
            stdout,
            stderr,
            output.status.code()
        ));
    }

    println!("✅ Godot headless tests passed.");
    println!("--- Godot STDOUT ---\n{}", stdout);

    Ok(())
}
