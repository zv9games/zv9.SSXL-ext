// ============================================================================
// 🧭 SSXL-ext CLI Developer Console (`ssxl_cli::main`)
// ----------------------------------------------------------------------------
// This module is the main entry point for the interactive developer console
// of the SSXL-ext engine. It orchestrates initialization, logging, FFI runtime
// setup, menu construction, and the interactive loop that allows developers
// to run validation tests, benchmarks, and inspection utilities.
//
// Purpose:
//   • Provide a unified, interactive interface for developers to control and
//     validate the SSXL engine from the command line.
//   • Simplify workflows by centralizing access to tests, benchmarks, and
//     external Godot integration tools.
//   • Ensure critical initialization steps (logging, FFI runtime, DLL copy)
//     are performed before any developer actions are executed.
//
// Key Components:
//   • Module Imports
//       - `actions`: Core functions for tests, benchmarks, and Godot interaction.
//       - `cli_util_inspect`: Utilities for scanning the Rust workspace and API surface.
//       - `cli_util_menu`: Menu structure and display logic (public for reuse).
//       - `cli_util_bench`: Benchmark and generation workload utilities.
//       - `cli_util_loc`: LOC (Lines of Code) analysis utilities.
//
//   • wait_for_enter
//       - Pauses execution until the user presses Enter.
//       - Used to provide acknowledgment before returning to the main menu.
//
//   • init_logging_and_engine
//       - Configures the tracing/logging system with INFO-level output.
//       - Initializes the SSXL engine core via FFI (`ssxl_start_runtime`).
//       - Ensures the latest compiled DLL is copied into the Godot tester project.
//
//   • main
//       - Performs initialization (logging, FFI, DLL copy).
//       - Declares symbolic references to FFI functions to ensure linker inclusion.
//       - Runs a LOC scan at startup for visibility into codebase size.
//       - Prints ASCII art banner to signal console readiness.
//       - Builds the menu structure via `build_menu`.
//       - Enters the main interactive loop:
//           1. Displays menu options.
//           2. Polls for key events using `crossterm`.
//           3. Debounces input to prevent repeated triggers.
//           4. Executes the selected action closure.
//           5. Waits for user acknowledgment before returning to menu.
//           6. Exits gracefully when the 'E' key is pressed.
//
// Workflow:
//   1. Initialize logging and engine runtime.
//   2. Copy DLL into Godot tester project.
//   3. Perform LOC scan and print startup banner.
//   4. Build and display the interactive menu.
//   5. Await user input and execute corresponding actions.
//   6. Continue until exit key is pressed.
//
// Design Choices:
//   • `tracing` and `tracing_subscriber` provide structured, leveled logging.
//   • `crossterm` enables cross-platform key event polling for interactive input.
//   • Input debouncing ensures smooth user experience without repeated triggers.
//   • Modular design separates concerns: actions, benchmarks, inspection, menu,
//     and LOC utilities are imported and orchestrated here.
//   • ASCII art banner provides a friendly, recognizable startup signal.
//
// Educational Note:
//   • This module demonstrates how Rust can be used to build an interactive,
//     developer-focused CLI with structured logging, FFI integration, and
//     modular action orchestration.
//   • By centralizing initialization and menu-driven workflows, developers gain
//     a predictable and ergonomic interface for testing and debugging the SSXL engine.
// ============================================================================


mod actions;
mod cli_util_inspect;
pub mod cli_util_menu;
mod cli_util_bench;
mod cli_util_loc;

use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};

use tracing::{info, error};
use tracing_subscriber::{self, filter::LevelFilter, prelude::*};

use crate::cli_util_menu::{build_menu, print_menu};
use ssxl_shared::ssxl_start_runtime;
use crate::cli_util_loc::scan_and_report_loc;
use crate::actions::copy_dll_to_tester_project_at_boot;

fn wait_for_enter() {
    println!("\nPress Enter to return to menu...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn init_logging_and_engine() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    info!("SSXLBinary: Interactive CLI initializing.");

    if ssxl_start_runtime() {
        info!("Engine FFI core initialized.");
    } else {
        error!("Failed to initialize Engine FFI core.");
    }
    
    if let Err(e) = copy_dll_to_tester_project_at_boot() {
        error!("DLL Copy Failed: {}", e);
    }
}

fn main() {
    init_logging_and_engine();
    
    extern "C" {
        fn ssxl_set_cell(x: i32, y: i32, tile_id: i32);
        fn ssxl_notify_tilemap_update();
    }

    let _ = ssxl_set_cell as *const ();
    let _ = ssxl_notify_tilemap_update as *const ();
    
    scan_and_report_loc();
    
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

    let menu = build_menu();
    let mut last_keys = HashSet::new();

    loop {
        print_menu(&menu);
        info!("Console: Awaiting menu selection...");
        print!("> ");
        io::stdout().flush().unwrap();

        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        let c = c.to_ascii_uppercase();

                        if last_keys.insert(c) {
                            if let Some(item) = menu.iter().find(|m| m.key == c) {
                                info!("Menu: Selected: {}", item.label);
                                println!("\n[{}] {}\n", c, item.label);
                                
                                (item.action)();

                                if c == 'E' {
                                    info!("Exit: Console closed. Engine shutdown complete.");
                                    return;
                                }

                                wait_for_enter();
                                break;
                            }
                        }
                    }
                }
            } else {
                last_keys.clear();
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
