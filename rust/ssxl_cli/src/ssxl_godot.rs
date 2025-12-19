use std::{path::PathBuf, process::Command};
use tracing::{error, info};

const GODOT_EXE: &str = "../SSXLtester2/Godot_v4.5.1-stable_win64.exe";
const GODOT_PROJECT: &str = "../SSXLtester2/project.godot";

pub fn launch_godot_project() {
    let project_path = PathBuf::from(GODOT_PROJECT);
    let godot_exe_path = PathBuf::from(GODOT_EXE);

    // ✅ Validate Godot executable exists
    if !godot_exe_path.exists() {
        error!("Godot executable not found at {}.", GODOT_EXE);
        return;
    }

    // ✅ Validate project file exists
    if !project_path.exists() {
        error!("Godot project not found at {}.", GODOT_PROJECT);
        return;
    }

    // ✅ Compute project directory safely
    let project_dir = project_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".."));

    info!("Launching Godot editor...");

    let spawn_result = Command::new(&godot_exe_path)
        .arg("--editor")
        .arg(&project_dir)
        .spawn();

    match spawn_result {
        Ok(mut child) => {
            info!("Godot launched. Waiting for editor to close...");
            if let Err(e) = child.wait() {
                error!("Error waiting for Godot: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to launch Godot: {}", e);
        }
    }
}
