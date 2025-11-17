//! # SSXL-ext CLI Developer Console (`ssxl_cli::main`)
//!
//! The main entry point for the interactive developer console. This utility manages
//! initialization, logging, the main menu loop, and delegates tasks to action modules
//! for testing, benchmarking, and external tool execution.

mod actions;               // Core functions for tests, benchmarks, and Godot interaction.
mod cli_util_inspect;      // Utilities for scanning the codebase and API surface.

// FIX: Change to `pub mod` so its types (CliAction, CliMenu) are accessible
// by other modules in the crate (like actions/testing.rs).
pub mod cli_util_menu;     // Menu structure and display logic.

mod cli_util_bench;        // Functions for running generation tests and benchmarks.
mod cli_util_loc;          // <--- CHANGE 1: Declare the new LOC utility module.

use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};

use tracing::{info, error};
use tracing_subscriber::{self, filter::LevelFilter, prelude::*};

use crate::cli_util_menu::{build_menu, print_menu};
use ssxl_engine_ffi::ssxl_initialize_engine; // External FFI function to bootstrap the engine core.
use crate::cli_util_loc::scan_and_report_loc; // <--- CHANGE 2: Update the import path and function name.
use crate::actions::copy_dll_to_tester_project_at_boot; // Action to ensure the latest DLL is in the Godot project.


/// Prompts the user to press Enter before returning to the main menu.
fn wait_for_enter() {
    println!("\nPress Enter to return to menu...");
    // Read a line from stdin and discard the result.
    let _ = io::stdin().read_line(&mut String::new());
}

/// Sets up the logging system and performs critical engine initialization steps.
fn init_logging_and_engine() {
    // 1. Initialize Tracing/Logging Subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout) // Direct log output to stdout.
                .with_filter(LevelFilter::INFO), // Set the minimum logging level to INFO.
        )
        .init();

    info!("SSXLBinary: Interactive CLI initializing.");

    // 2. Initialize the Rust Core via FFI
    // Calls the external C-compatible function to boot the engine's core state and runtime.
    if ssxl_initialize_engine() {
        info!("Engine FFI core initialized.");
    } else {
        // We log the failure but allow the CLI to continue for non-engine tasks (like LOC scan).
        error!("Failed to initialize Engine FFI core.");
    }
    
    // 3. Copy DLL to Godot Project
    // Ensure the compiled GDExtension DLL is copied into the Godot tester project
    // before any Godot-related actions are run.
    if let Err(e) = copy_dll_to_tester_project_at_boot() {
        // This is a critical warning, as Godot interaction will fail without the DLL.
        error!("DLL Copy Failed: {}", e);
    }
}

fn main() {
    // Perform initial setup: logging, FFI, and DLL copy.
    init_logging_and_engine();
    
    // Run a Lines of Code (LOC) scan on the codebase at startup.
    // Note: The scan function no longer takes an argument.
    scan_and_report_loc(); // <--- CHANGE 3: Update function call and remove unnecessary argument.
    
    // Print welcome ASCII art.
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

    // Build the menu structure.
    let menu = build_menu();
    // Set for input debouncing to prevent multiple actions from a single key press hold.
    let mut last_keys = HashSet::new();

    // --- Main Interactive Console Loop ---
    loop {
        // Display the menu options.
        print_menu(&menu);
        info!("Console: Awaiting menu selection...");
        print!("> ");
        // Ensure the prompt character is immediately visible.
        io::stdout().flush().unwrap();

        // Inner loop handles key polling and processing.
        loop {
            // Poll for key events with a timeout to keep the loop responsive (500ms tempo).
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        let c = c.to_ascii_uppercase(); // Normalize input to uppercase.

                        // Input Debounce Check: Only process if the key hasn't been seen recently.
                        if last_keys.insert(c) {
                            // Find the corresponding menu item.
                            if let Some(item) = menu.iter().find(|m| m.key == c) {
                                info!("Menu: Selected: {}", item.label);
                                println!("\n[{}] {}\n", c, item.label);
                                
                                // Execute the action associated with the menu item.
                                (item.action)();

                                // Check for the exit key ('E').
                                if c == 'E' {
                                    info!("Exit: Console closed. Engine shutdown complete.");
                                    return; // Exit the main function, terminating the CLI.
                                }

                                // Wait for user acknowledgment before returning to the main menu screen.
                                wait_for_enter();
                                // Break the inner polling loop to redraw the menu.
                                break;
                            }
                        }
                    }
                }
            } else {
                // If the poll times out, clear the debounce set, allowing a new key press to be registered.
                last_keys.clear();
            }

            // Short pause for general loop control.
            thread::sleep(Duration::from_millis(10));
        }
    }
}