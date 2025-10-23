// ssxl_cli/src/main.rs

// --- MODULES ---
// These files must be created in the ssxl_cli/src directory.
mod cli_util_actions;	// Contains the menu action functions (e.g., run_tests)
mod cli_util_inspect;	// Contains inspection functions (API surface, module tree)
mod cli_util_menu;		// Contains MenuItem struct, build_menu, and print_menu
mod cli_util_bench;		// Contains benchmark/conversion functions

// --- EXTERNAL IMPORTS ---
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};

// Tracing imports are correct.
use tracing::{info, error};
use tracing_subscriber::{self, filter::LevelFilter, prelude::*}; 

// --- INTERNAL IMPORTS ---
use crate::cli_util_menu::{build_menu, print_menu};
use ssxl_engine_ffi::ssxl_initialize_engine; // To be called once on startup

/// 🖐️ Optional pause after action
fn wait_for_enter() {
	println!("\nPress Enter to return to menu...");
	let _ = io::stdin().read_line(&mut String::new());
}

fn init_logging_and_engine() {
	// This sets up a simple console logger for the CLI environment.
	tracing_subscriber::registry()
        // Whitespace cleaned here
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout) // Direct output to stdout
                .with_filter(LevelFilter::INFO),
        )
        .init();

	info!("SSXLBinary: Interactive CLI initializing.");

	// Call the FFI initialization (placeholder for engine boot)
	if ssxl_initialize_engine() {
		info!("Engine FFI core initialized.");
	} else {
		error!("Failed to initialize Engine FFI core.");
	}
}

fn main() {
	// 🧠 Startup
	init_logging_and_engine();


	println!(
    r#"
         (__)
         (oo)
  /-------\/
 / |     ||
*  ||----||
    ~~    ~~
SSXL Engine Console Initialized
"#
);



	// 🧭 Menu setup
	let menu = build_menu();
	let mut last_keys = HashSet::new();

	// 🔁 Main loop
	loop {
		print_menu(&menu);
		info!("Console: Awaiting menu selection...");
		print!("> ");
		io::stdout().flush().unwrap();

		// Wait for keypress
		loop {
			if event::poll(Duration::from_millis(500)).unwrap() {
				if let Event::Key(key_event) = event::read().unwrap() {
					if let KeyCode::Char(c) = key_event.code {
						if last_keys.insert(c) {
							if let Some(item) = menu.iter().find(|m| m.key == c) {
								info!("Menu: Selected: {}", item.label);
								println!("\n[{}] {}\n", c, item.label);
								(item.action)();

								if c == '9' {
									info!("Exit: Engine shutdown complete.");
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