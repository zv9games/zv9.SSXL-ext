mod zv9_util_binary_func;
mod zv9_util_binary_func2;
mod zv9_util_binary_func3;
mod zv9_util_binary_menu;
mod zv9_util_binary_func_godot;

use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode};

use aetherion_shared::zv9_util_logging::{init_logging, log_info};
use zv9_util_binary_menu::{build_menu, print_menu};
#[allow(unused_imports)]
use zv9_util_binary_func::*;

/// 🖐️ Optional pause after action
fn wait_for_enter() {
    println!("\nPress Enter to return to menu...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    // 🧠 Startup
    log_info("AetherionBinary", "Interactive CLI for engine diagnostics and control");


    init_logging();
    log_info("Startup", "Engine boot sequence initiated.");

    println!(
        r#"
         (__)
         (oo)
  /-------\/
 / |     ||
*  ||----||
   ~~    ~~
    Aetherion Bull Initialized
"#
    );

    // 🧭 Menu setup
    let menu = build_menu();
    let mut last_keys = HashSet::new();

    // 🔁 Main loop
    loop {
        print_menu(&menu);
        log_info("Console", "Awaiting menu selection...");
        print!("> ");
        io::stdout().flush().unwrap();

        // Wait for keypress
        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        if last_keys.insert(c) {
                            if let Some(item) = menu.iter().find(|m| m.key == c) {
                                log_info("Menu", &format!("Selected: {}", item.label));
                                println!("\n[{}] {}\n", c, item.label);
                                (item.action)();

                                if c == '9' {
                                    log_info("Exit", "Engine shutdown complete.");
                                    return;
                                }

                                wait_for_enter();
                                break; // ✅ Return to outer loop to redraw menu
                            }
                        }
                    }
                }
            } else {
                last_keys.clear(); // 🧹 Reset key memory when idle
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
