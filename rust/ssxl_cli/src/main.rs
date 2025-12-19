pub mod ssxl_menu;
pub mod ssxl_source_scan;
pub mod ssxl_api_scan;
pub mod ssxl_testing;
pub mod ssxl_godot;
pub mod godot_headless;
pub mod ssxl_config;

pub use ssxl_godot::launch_godot_project;
pub use ssxl_testing::run_grand_unified_test;

use std::{fs, io, process::Command};
use tracing::{error, info};
use tracing_subscriber::{filter::LevelFilter, prelude::*};
use std::io::Write;
use std::{io::stdout, thread, time::Duration};
use rand::Rng;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    execute,
};

// -----------------------------------------------------------------------------
// NEW PIPELINE CONSTANTS
// -----------------------------------------------------------------------------

const DLL_SRC: &str = "C:/zv9/zv9.SSXL-ext/rust/target/release/ssxl_ext.dll";
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
    kill_stale_godot();
    build_extension();
    deploy_dll();
}

// -----------------------------------------------------------------------------
// ENGINE RUNTIME
// -----------------------------------------------------------------------------

pub fn start_runtime() -> bool {
    true
}

// -----------------------------------------------------------------------------
// INIT + MAIN
// -----------------------------------------------------------------------------

fn init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stderr)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    info!("SSXL CLI initializing...");
    ensure_extension_fresh();
}

fn main() {
    init();
    ssxl_source_scan::scan_and_report_loc();

    let mut stdout_main = stdout();
    execute!(
        &mut stdout_main,
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();

    // ---------- Scene constants ----------
    const WIDTH: usize = 80;
    const HEIGHT: usize = 13;
    const SNOW_COUNT: usize = 45;
    const SCENE_Y: u16 = 0;

    // ---------- Animation control flag ----------
    let anim_running = Arc::new(AtomicBool::new(true));

    // ---------- Background layer ----------
    let mut background = vec![vec![' '; WIDTH]; HEIGHT];
    let mut put_str = |x: usize, y: usize, s: &str| {
        if y < HEIGHT {
            for (i, ch) in s.chars().enumerate() {
                let xx = x + i;
                if xx < WIDTH {
                    background[y][xx] = ch;
                }
            }
        }
    };

    // ✅ ASCII ART (Bull + Tree)
    put_str(12, 1, " (__)");
    put_str(12, 2, " (oo)");
    put_str(6,  3, "/------ \\/");
    put_str(5,  4, "/ |     ||");
    put_str(4,  5, "*  ||----||");
    put_str(7,  6, "~~    ~~");

    put_str(28, 1, "     *");
    put_str(28, 2, "    ***");
    put_str(28, 3, "   *****");
    put_str(28, 4, "  *******");
    put_str(28, 5, " *********");
    put_str(30, 6, "  |||");

    put_str(0, 10, "SSXL-ext Engine Console Initialized");
    put_str(16, 12, "Merry Christmas from the SSXL TEAM!");

    // ---------- Snow + Light layer ----------
    #[derive(Clone, Copy)]
    struct Snow { x: usize, y: usize }
    #[derive(Clone, Copy)]
    struct Bulb { x: usize, y: usize }

    let mut rng = rand::thread_rng();
    let snowflakes: Vec<Snow> = (0..SNOW_COUNT)
        .map(|_| Snow {
            x: rng.gen_range(0..WIDTH),
            y: rng.gen_range(0..HEIGHT),
        })
        .collect();

    let bulbs = vec![
        Bulb { x: 32, y: 2 }, Bulb { x: 34, y: 2 },
        Bulb { x: 31, y: 3 }, Bulb { x: 35, y: 3 },
        Bulb { x: 30, y: 4 }, Bulb { x: 36, y: 4 },
        Bulb { x: 29, y: 5 }, Bulb { x: 37, y: 5 },
    ];

    // ---------- Spawn render thread ----------
    let background_clone = background.clone();
    let bulbs_clone = bulbs.clone();
    let anim_flag = anim_running.clone();

    thread::spawn(move || {
        let mut stdout = stdout();
        let mut rng = rand::thread_rng();
        let mut snowflakes = snowflakes;

        while anim_flag.load(Ordering::Relaxed) {
            for flake in &mut snowflakes {
                if flake.y + 1 >= HEIGHT {
                    flake.y = 0;
                    flake.x = rng.gen_range(0..WIDTH);
                } else {
                    flake.y += 1;
                    let drift: i32 = rng.gen_range(-1..=1);
                    let new_x = flake.x as i32 + drift;
                    if new_x >= 0 && new_x < WIDTH as i32 {
                        flake.x = new_x as usize;
                    }
                }
            }

            let mut frame = background_clone.clone();

            for flake in &snowflakes {
                if flake.y < HEIGHT && flake.x < WIDTH {
                    frame[flake.y][flake.x] = '*';
                }
            }

            for bulb in &bulbs_clone {
                if rng.gen_bool(0.6) {
                    let symbols = ['*', 'o', 'O', '@'];
                    let ch = symbols[rng.gen_range(0..symbols.len())];
                    if bulb.y < HEIGHT && bulb.x < WIDTH {
                        frame[bulb.y][bulb.x] = ch;
                    }
                }
            }

            for (i, row) in frame.iter().enumerate() {
                let y = SCENE_Y + i as u16;
                if y >= SCENE_Y + HEIGHT as u16 {
                    continue;
                }

                let line: String = row.iter().collect();
                let padded = format!("{:<width$}", line, width = WIDTH);
                execute!(&mut stdout, MoveTo(0, y)).unwrap();
                write!(stdout, "{padded}").unwrap();
            }

            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(140));
        }
    });

    // ---------- Launch menu below the scene ----------
    let menu_y: u16 = SCENE_Y + HEIGHT as u16 + 1;

    // ✅ Clear prompt area to remove any leftover snowflakes or bulbs
    execute!(
        &mut stdout_main,
        MoveTo(0, menu_y),
        Clear(ClearType::FromCursorDown)
    ).unwrap();

    execute!(&mut stdout_main, MoveTo(0, menu_y - 1)).unwrap();
    println!();

    execute!(&mut stdout_main, MoveTo(0, menu_y)).unwrap();

    ssxl_menu::run_interactive_loop_with_anim_flag(
        ssxl_menu::build_menu(),
        menu_y,
        anim_running,
    );
}