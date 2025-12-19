use std::collections::HashSet;
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use crossterm::{cursor::MoveTo, execute};
use crossterm::event::{self, Event, KeyCode};
use tracing::info;

// Import actions from crate root (main.rs re-exports them)
use crate::{
    run_grand_unified_test,
    launch_godot_project,
};

// ✅ Import start_runtime so L can load the engine on demand
use crate::start_runtime;

// ✅ NEW: Import tank‑mode headless Godot runner
use crate::godot_headless::run_godot_headless_tests;

/// Structure representing a single menu item and its action.
/// ✅ Updated: action now receives action_y
pub struct CliAction {
    pub key: char,
    pub label: &'static str,
    pub id: &'static str,
    pub action: Box<dyn Fn(u16)>,
}

/// ✅ Updated menu with new TANK MODE option
pub fn build_menu() -> Vec<CliAction> {
    vec![
        CliAction {
            key: 'A',
            label: "📘 press A: Show GDScript API (Auto-Generated)",
            id: "show_gdscript_api",
            action: Box::new(|action_y| {
                crate::ssxl_api_scan::print_godot_api_surface(action_y);
            }),
        },

        CliAction {
            key: 'L',
            label: "🚀 press L: Launch Godot Project (Full Integration Test)",
            id: "launch_godot",
            action: Box::new(|_action_y| {
                if start_runtime() {
                    launch_godot_project();
                } else {
                    println!("❌ Runtime failed to start. Cannot launch Godot.");
                }
            }),
        },

        CliAction {
            key: 'G',
            label: "✅ press G: Run Grand Unified Test (GUT)",
            id: "grand_unified_test",
            action: Box::new(|_action_y| {
                run_grand_unified_test();
            }),
        },

        CliAction {
            key: 'T',
            label: "🧪 press T: Run Godot Headless Tests (Tank Mode)",
            id: "godot_headless_tests",
            action: Box::new(|action_y| {
                let mut stdout = io::stdout();
                execute!(&mut stdout, MoveTo(0, action_y)).unwrap();

                println!("=== SSXL Headless Godot Test (Tank Mode) ===");
                match run_godot_headless_tests() {
                    Ok(_) => println!("✅ Godot headless tests passed."),
                    Err(e) => println!("❌ Godot headless tests failed:\n{}", e),
                }
            }),
        },

        CliAction {
            key: 'U',
            label: "✅ press U: EXIT Console",
            id: "exit",
            action: Box::new(|_action_y| {}),
        },
    ]
}

/// ✅ Print menu at a fixed anchored Y position
fn print_menu(menu: &[CliAction], menu_y: u16) {
    let mut stdout = io::stdout();
    execute!(&mut stdout, MoveTo(0, menu_y)).unwrap();

    println!("--- SSXL-ext Main Menu ---");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
}

/// ✅ Wait prompt also anchored
fn wait_for_enter(menu_y: u16) {
    let mut stdout = io::stdout();
    let wait_y = menu_y + 20;
    execute!(&mut stdout, MoveTo(0, wait_y)).unwrap();
    println!("Press Enter to return to menu...");
    let _ = io::stdin().read_line(&mut String::new());
}

/// ✅ MAIN LOOP — now anchored safely below the scene
/// ✅ NEW: accepts anim_flag so we can shut down the animation thread
pub fn run_interactive_loop_with_anim_flag(
    menu: Vec<CliAction>,
    menu_y: u16,
    anim_flag: Arc<AtomicBool>,
) {
    let mut last_keys = HashSet::new();

    loop {
        // Print menu at anchored position
        print_menu(&menu, menu_y);

        let mut stdout = io::stdout();
        let prompt_y = menu_y + menu.len() as u16 + 2;

        // ✅ Clear prompt area AND everything below it
        use crossterm::terminal::{Clear, ClearType};
        execute!(
            &mut stdout,
            MoveTo(0, prompt_y),
            Clear(ClearType::FromCursorDown)
        ).unwrap();

        // ✅ Anchor and print prompt block cleanly
        execute!(&mut stdout, MoveTo(0, prompt_y)).unwrap();
        println!("Awaiting menu selection...");
        execute!(&mut stdout, MoveTo(0, prompt_y + 1)).unwrap();
        print!("> ");
        io::stdout().flush().unwrap();

        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        let c = c.to_ascii_uppercase();

                        if last_keys.insert(c) {
                            if let Some(item) = menu.iter().find(|m| m.key == c) {

                                // ✅ STOP ANIMATION IMMEDIATELY
                                anim_flag.store(false, Ordering::Relaxed);

                                // ✅ Dynamic action anchor
                                let action_y = menu_y + menu.len() as u16 + 4;

                                // ✅ Clear action area before printing
                                execute!(
                                    &mut stdout,
                                    MoveTo(0, action_y),
                                    Clear(ClearType::FromCursorDown)
                                ).unwrap();

                                // ✅ Print selection label at fixed anchor
                                execute!(&mut stdout, MoveTo(0, action_y)).unwrap();

                                // ✅ FIX: ensure stderr logs don't collide with stdout
                                eprintln!(); // newline for stderr safety
                                info!("Menu: Selected: {}", item.label);

                                println!("[{}] {}", c, item.label);

                                // ✅ Run action with action_y
                                (item.action)(action_y);

                                if c == 'U' {
                                    return;
                                }

                                wait_for_enter(menu_y);
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
