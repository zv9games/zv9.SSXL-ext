pub mod ssxl_menu;
pub mod ssxl_source_scan;
pub mod ssxl_api_scan;
pub mod ssxl_testing;
pub mod ssxl_godot;

pub use ssxl_godot::launch_godot_project;
pub use ssxl_testing::run_grand_unified_test;

use std::{fs, io, process::Command};
use tracing::{error, info};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

extern "C" {
    fn ssxl_boot_core_to_idle() -> i32;
}

// -----------------------------------------------------------------------------
// NEW PIPELINE CONSTANTS
// -----------------------------------------------------------------------------

// Build output (always release for Godot)
const DLL_SRC: &str = "C:/zv9/zv9.SSXL-ext/rust/target/release/ssxl_ext.dll";

// Dual deployment targets
const DLL_DST_TESTER2: &str = "C:/zv9/zv9.SSXL-ext/SSXLtester2/ssxl_ext.dll";
const DLL_DST_ENGINE: &str = "C:/zv9/zv9.SSXL-ext/SSXL_engine_tester/ssxl_ext.dll";

// -----------------------------------------------------------------------------
// PIPELINE HELPERS
// -----------------------------------------------------------------------------

fn kill_stale_godot() {
    let _ = Command::new("taskkill")
        .args(&["/IM", "godot.exe", "/F"])
        .status();
}

fn build_extension() {
    info!("Building ssxl_ext (release, godot-binding)...");

    let status = Command::new("cargo")
        .args([
            "build",
            "-p",
            "ssxl_ext",
            "--release",
            "--features",
            "godot-binding",
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        error!("ssxl_ext build failed.");
        std::process::exit(1);
    }
}

fn deploy_dll() {
    info!("Deploying DLL to both Godot projects...");

    if let Err(e) = fs::copy(DLL_SRC, DLL_DST_TESTER2) {
        error!("Failed to copy to SSXLtester2: {}", e);
    }

    if let Err(e) = fs::copy(DLL_SRC, DLL_DST_ENGINE) {
        error!("Failed to copy to SSXL_engine_tester: {}", e);
    }

    info!("✅ DLL deployed to SSXLtester2 and SSXL_engine_tester");
}

fn ensure_extension_fresh() {
    // NEW: Always rebuild on CLI start.
    // No timestamp logic. No stale detection.
    // The pipeline is authoritative.
    kill_stale_godot();
    build_extension();
    deploy_dll();
}

// -----------------------------------------------------------------------------
// ENGINE RUNTIME
// -----------------------------------------------------------------------------

pub fn start_runtime() -> bool {
    unsafe {
        match ssxl_boot_core_to_idle() {
            0 => {
                info!("Engine runtime initialized.");
                true
            }
            code => {
                error!("Engine failed to boot. Code {}", code);
                false
            }
        }
    }
}

// -----------------------------------------------------------------------------
// INIT + MAIN
// -----------------------------------------------------------------------------

fn init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    info!("SSXL CLI initializing...");

    // NEW: Always rebuild + deploy DLL before any FFI call.
    ensure_extension_fresh();
}

fn main() {
    init();

    ssxl_source_scan::scan_and_report_loc();

    println!(
        r#"
            (__)  
            (oo)
      /------\/
     / |    ||
    * ||----||
      ~~    ~~
SSXL-ext Engine Console Initialized
"#
    );

    ssxl_menu::run_interactive_loop(ssxl_menu::build_menu());
}
