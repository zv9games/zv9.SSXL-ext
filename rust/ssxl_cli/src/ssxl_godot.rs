use std::{path::PathBuf, process::Command};
use tracing::{error, info};

const GODOT_EXE: &str = "../SSXLtester2/Godot_v4.5.1-stable_win64.exe";
const GODOT_PROJECT: &str = "../SSXLtester2/project.godot";

pub fn launch_godot_project() {
    let project_path = PathBuf::from(GODOT_PROJECT);

    // ✅ FIX: Avoid returning a reference to a temporary PathBuf
    let project_dir_buf = project_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".."));

    let project_dir = project_dir_buf.as_path();

    if !project_path.exists() {
        error!("Godot project not found at {}.", GODOT_PROJECT);
        return;
    }

    info!("Launching Godot editor...");

    match Command::new(GODOT_EXE)
        .arg("--editor")
        .arg(project_dir)
        .spawn()
    {
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
