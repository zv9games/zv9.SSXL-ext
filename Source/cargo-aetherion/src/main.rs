use std::{fs, path::PathBuf, process::Command, thread, time::Duration};

fn main() {
    // Define project structure
    let root_dir = PathBuf::from(r"C:\zv9\zv9.aetherion");
    let engine_dir = root_dir.join("rust");
    let tester_dir = root_dir.join("aetherion_engine_tester");

    let dll_name = "aetherion_engine.dll";
    let exe_name = "aetherion_engine.exe";

    let dll_path = engine_dir.join("target/debug").join(&dll_name);
    let exe_path = engine_dir.join("target/debug").join(&exe_name);
    let destination = tester_dir.join(&dll_name);

    println!("🔧 Aetherion deployment initiated...");

    // Step 1: Build the engine crate in standard debug mode
    println!("🛠️ Building engine at: {:?}", engine_dir);
    let build_status = Command::new("cargo")
        .arg("build")
        .current_dir(&engine_dir)
        .status()
        .expect("❌ Failed to start cargo build");

    if !build_status.success() {
        eprintln!("❌ Build failed");
        std::process::exit(1);
    }

    println!("📦 Build complete.");

    // Step 2: Wait for DLL to appear
    println!("🔍 Waiting for DLL at: {:?}", dll_path);
    let mut attempts = 0;
    while !dll_path.exists() && attempts < 10 {
        println!("⏳ Attempt {}: DLL not found yet...", attempts + 1);
        thread::sleep(Duration::from_millis(500));
        attempts += 1;
    }

    if !dll_path.exists() {
        eprintln!("🚨 DLL not found after waiting!");
        std::process::exit(1);
    }

    // Step 3: Copy DLL to tester directory
    fs::create_dir_all(&tester_dir).expect("❌ Failed to create tester directory");

    match fs::copy(&dll_path, &destination) {
        Ok(_) => println!("✅ DLL copied to tester directory:\n  {:?}", destination),
        Err(err) => {
            eprintln!("❌ Failed to copy DLL: {}", err);
            std::process::exit(1);
        }
    }

    // Step 4: Run the engine binary directly
    println!("🚀 Launching engine binary: {:?}", exe_path);
    let run_status = Command::new(&exe_path)
        .status()
        .expect("❌ Failed to start engine binary");

    if !run_status.success() {
        eprintln!("❌ Engine run failed");
        std::process::exit(1);
    }

    println!("✅ Engine run complete");
}
