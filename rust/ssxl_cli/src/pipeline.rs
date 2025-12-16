use std::fs;
use std::process::Command;
use std::path::PathBuf;

pub fn run_pipeline() -> anyhow::Result<()> {
    // 1. Kill stale Godot processes
    let _ = Command::new("taskkill")
        .args(&["/IM", "godot.exe", "/F"])
        .status();

    // 2. Build DLL
    Command::new("cargo")
        .args(&["build", "-p", "ssxl_ext", "--release", "--features", "godot-binding"])
        .status()?;

    // 3. Locate DLL
    let dll_src = PathBuf::from("rust/target/release/ssxl_ext.dll");

    // 4. Copy into both Godot project roots
    let tester2 = PathBuf::from("SSXLtester2/SSXL_ext.dll");
    let engine_tester = PathBuf::from("SSXL_engine_tester/SSXL_ext.dll");

    fs::copy(&dll_src, &tester2)?;
    fs::copy(&dll_src, &engine_tester)?;

    println!("✅ DLL deployed to both Godot projects");

    // 5. Optionally launch Godot (example: SSXLtester2)
    Command::new("godot")
        .current_dir("SSXLtester2")
        .status()?;

    Ok(())
}
