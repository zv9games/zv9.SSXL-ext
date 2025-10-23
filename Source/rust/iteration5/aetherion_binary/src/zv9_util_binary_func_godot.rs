use std::process::Command;
#[allow(dead_code)]
pub fn launch_headless_godot() {
    let godot_path = "C:/ZV9/Godot/Godot.exe"; // adjust this
    let project_path = "C:/ZV9/zv9.aetherion/godot_project";
    let scene_path = "res://scenes/Pacman2.tscn";

    let result = Command::new(godot_path)
        .args(&["--headless", "--path", project_path, "--scene", scene_path])
        .spawn();

    match result {
        Ok(_) => println!("🚀 Headless Godot launched successfully."),
        Err(e) => eprintln!("❌ Failed to launch Godot: {}", e),
    }
}
