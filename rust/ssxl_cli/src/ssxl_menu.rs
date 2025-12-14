use std::collections::HashSet;
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use crossterm::event::{self, Event, KeyCode};
use tracing::info;

// Note: We use extern crate self as ssxl_cli to resolve actions defined in main.rs
extern crate self as ssxl_cli;
// FIX 1: Import the new action function from main.rs
use ssxl_cli::{
    run_grand_unified_test,
    launch_godot_project // <--- NEW IMPORT
};

/// Structure representing a single menu item and its action.
pub struct CliAction {
    pub key: char,
    pub label: &'static str,
    pub id: &'static str,
    pub action: Box<dyn Fn()>,
}

pub fn build_menu() -> Vec<CliAction> {
    vec![
        // FIX 2: Add the new action to launch Godot
        CliAction {
            key: 'L',
            label: "🚀 press L: Launch Godot Project (Full Integration Test)",
            id: "launch_godot",
            action: Box::new(launch_godot_project)
        },
        // Only the Grand Unified Test (GUT) remains for one-click validation.
        CliAction {
            key: 'G',
            label: "✅ press G: Run Grand Unified Test (GUT)",
            id: "grand_unified_test",
            action: Box::new(run_grand_unified_test)
        },
        // Corrected menu item for exit
        CliAction { key: 'U', label: "✅ press U: EXIT Console", id: "exit", action: Box::new(|| {}) },
    ]
}

fn print_menu(menu: &[CliAction]) {
    println!("\n--- SSXL-ext Main Menu ---");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
}

fn wait_for_enter() {
    println!("\nPress Enter to return to menu...");
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn run_interactive_loop(menu: Vec<CliAction>) {
    let mut last_keys = HashSet::new();

    loop {
        print_menu(&menu);
        info!("Console: Awaiting menu selection...");
        print!("> ");
        io::stdout().flush().unwrap();

        loop {
            // Debounced input poll
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        let c = c.to_ascii_uppercase();

                        if last_keys.insert(c) {
                            if let Some(item) = menu.iter().find(|m| m.key == c) {
                                info!("Menu: Selected: {}", item.label);
                                println!("\n[{}] {}\n", c, item.label);
                                
                                (item.action)(); // Execute the action closure

                                if c == 'U' {    
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