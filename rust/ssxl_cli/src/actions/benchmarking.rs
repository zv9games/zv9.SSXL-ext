use tracing::{info, warn, error};
use std::io::{self, Write};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use ctrlc;

// Import Conductor types and ConductorStatus from the parent module (actions/mod.rs).
// The parent module (super) imports and re-exports these from ssxl_generate.
use super::{Conductor, ConductorStatus}; // CORRECTED PATH: Removed one 'super::'


// -----------------------------------------------------------------------------
// PHASE 4: SIGNAL INSPECTOR / LIVE FEED
// -----------------------------------------------------------------------------

/// 🔮 Starts the live **Signal Inspector** utility (CLI Menu [B]).
/// It monitors the Conductor's state and prints it to the console in real-time.
pub fn start_signal_inspector() {
    warn!("🔮 Initializing Conductor and starting Signal Inspector (Real-Time Feed)...");

    // 1. Initialize Conductor and retrieve the thread-safe state
    // FIX E0308: Updated destructuring to match the new 4-element tuple (Conductor, ConductorState, Receiver, Sender).
    let (conductor, state, _progress_receiver, _request_sender) = match Conductor::new(None) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };

    // Wrap Conductor in Arc<Mutex<Option<>>> for safe, single consumption by the ctrlc handler.
    let conductor_shutdown_safe = Arc::new(Mutex::new(Some(conductor)));
    let shutdown_clone = conductor_shutdown_safe.clone();

    // 2. Setup atomic flag for graceful exit via Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r_for_handler = running.clone();

    // Set a Ctrl-C handler to stop the loop and shut down the Conductor
    if let Err(e) = ctrlc::set_handler(move || {
        r_for_handler.store(false, Ordering::SeqCst);

        if let Some(c) = shutdown_clone.lock().unwrap().take() {
            c.graceful_teardown();
        }
        let _ = writeln!(io::stdout(), "\nInspector: Shutdown signal received. Waiting for Conductor...");
    }) {
        error!("Could not set Ctrl-C handler: {}", e);
        return;
    }

    info!("Inspector: Press Ctrl-C to stop the live feed and gracefully shut down the Conductor.");

    let mut frame_count: u64 = 0;
    const MVG_BASELINE: u64 = 10_000_000;

    // 3. Main Live Feed Loop
    while running.load(Ordering::SeqCst) {
        frame_count += 1;

        // --- REAL-TIME DATA POLLING ---
        let status = state.get_status();
        let queue_depth = state.get_queue_depth();
        let active_id = state.get_active_generator_id();

        // Use carriage return (`\r`) to overwrite the current line.
        print!("\r");
        print!("🔮 LIVE FEED | Frame: {: >4} | Status: {: <12} | Generator: {: <25} | Queue Depth: ~{: >6} | MVG Baseline: {} tiles/s | Press Ctrl-C to exit.",
            frame_count,
            format!("{:?}", status),
            active_id,
            queue_depth,
            MVG_BASELINE
        );
        let _ = io::stdout().flush();

        // Check for internal shutdown signals (e.g., error in Conductor)
        if status == ConductorStatus::ShuttingDown || status == ConductorStatus::Error {
            running.store(false, Ordering::SeqCst);

            if let Some(c) = conductor_shutdown_safe.lock().unwrap().take() {
                c.graceful_teardown();
            }
            break;
        }

        // Wait 50ms (20 FPS refresh)
        thread::sleep(Duration::from_millis(50));
    }

    // 4. Cleanup: Clear the line after exiting the loop
    let _ = writeln!(io::stdout(), "\r{: <200}", " "); // Overwrite and clear the line
    info!("Inspector shutdown complete. Conductor runtime terminated.");
}

// -----------------------------------------------------------------------------
// PHASE 5: BENCHMARK EXECUTION
// -----------------------------------------------------------------------------

/// ⏱️ Executes a full set of engine benchmarks (CLI Menu [B] or [1]).
/// This placeholder function is added to resolve the E0432 unresolved import error
/// in ssxl_cli/src/actions/mod.rs.
pub fn run_benchmark() {
    warn!("⏱️ Benchmark execution not yet implemented. Placeholder called.");
    // Implementation will go here later: setup benchmarks, run, and report results.
}