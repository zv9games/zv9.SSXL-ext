
use std::process::Command;
use std::io::{self, Write};
#[allow(unused_imports)]
//use aetherion_core::trailkeeper::{
//    Trailkeeper,
//    check_config_change,
//    scan_git_diff,
//    LogEntry,
//   EventType,
//};


#[allow(unused_imports)]
use aetherion_core::core::{Conductor, ProcCommand};

/// 🚀 Runs the full Rust test suite via Cargo
pub fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...\n");

    let status = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .status()
        .expect("Failed to run cargo test");

    if status.success() {
        println!("✅ All tests passed.");
    } else {
        println!("❌ Some tests failed.");
    }
}


/*
/// 🔍 Runs a Trailkeeper scan for changes and config diffs
pub fn run_trailkeeper_scan() {
    println!("🔍 Running Trailkeeper scan...\n");

    scan_git_diff();
    check_config_change();

    for log in Trailkeeper::all() {
        println!("{:?}", log);
    }

    println!("\n✅ Trailkeeper scan complete.\n");
}
*/
// 📜 Interactive viewer for Trailkeeper logs
/*
#[allow(dead_code)]
pub fn view_trailkeeper_logs() {
    println!("\n📜 Trailkeeper Log Registry:\n");

    let logs = Trailkeeper::all();
    if logs.is_empty() {
        println!("(No logs recorded yet.)");
        return;
    }

    let stdin = io::stdin();
    let mut buffer = String::new();

    for (i, log) in logs.iter().enumerate() {
        print_log_entry(i + 1, log);
        print!("Press Enter to continue, or type 9 to quit: ");
        io::stdout().flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        if buffer.trim() == "9" {
            println!("\n🚪 Exiting log viewer...\n");
            break;
        }
    }

    println!("\n✅ Log inspection complete.\n");
}

*/
/*
/// 🧾 Prints a formatted Trailkeeper log entry
#[allow(dead_code)]
pub fn print_log_entry(index: usize, log: &LogEntry) {
    println!("──────────────────────────────────────────────");
    println!("📄 Entry #{}", index);
    println!("🕒 Timestamp: {}", log.timestamp.to_rfc3339());
    println!("🧠 Event Type: {:?}", log.event_type);
    println!("👤 Actor: {}", log.actor);
    println!("📝 Description: {}", log.description);
    println!("📦 Components: {:?}", log.affected_components);
    println!("⚠️ Status: {:?}", log.status);
    println!("──────────────────────────────────────────────");
}
*/