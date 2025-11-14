// ssxl_cli/src/actions/benchmarking.rs

//! # CLI Actions: Monitoring and Benchmarking (`ssxl_cli::actions::benchmarking`)
//!
//! Provides command-line interface tools for interacting with the SSXL engine's
//! asynchronous systems. This includes real-time status inspection of the
//! `Conductor` and facilities for running performance benchmarks.

use tracing::{info, warn, error};
use std::io::{self, Write};
use std::sync::{
    Arc, 
    Mutex, 
    atomic::{AtomicBool, Ordering} // Atomic types for thread-safe signaling
};
use std::thread;
use std::time::Duration;
use ctrlc; // Crate for handling Ctrl-C signals

// FINAL FIX: Importing types directly from the external crates as suggested by the compiler.
// This bypasses any ambiguity or compilation order issues with `crate::actions::` re-exports.
use ssxl_generate::conductor::Conductor;
use ssxl_generate::conductor_state::ConductorStatus;


/// Initializes the main world generation Conductor and begins a live, real-time
/// feed of its status to the command line.
///
/// This function blocks the current thread until the process is terminated via
/// Ctrl-C or the Conductor signals a fatal state (Error/ShuttingDown).
pub fn start_signal_inspector() {
    warn!("🔮 Initializing Conductor and starting Signal Inspector (Real-Time Feed)...");

    // Attempt to initialize the Conductor, which starts the Tokio runtime and worker threads.
    let (conductor, state, _progress_receiver, _request_sender) = match Conductor::new(None) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };

    // --- Graceful Shutdown Setup ---

    // 1. Wrap the Conductor in a structure that allows for shared, mutable ownership
    // and consumption from the signal handler thread.
    let conductor_shutdown_safe = Arc::new(Mutex::new(Some(conductor)));
    let shutdown_clone = conductor_shutdown_safe.clone();

    // 2. Use a thread-safe atomic boolean to control the main inspection loop.
    let running = Arc::new(AtomicBool::new(true));
    let r_for_handler = running.clone();

    // 3. Set the Ctrl-C handler to trigger graceful shutdown.
    if let Err(e) = ctrlc::set_handler(move || {
        // Stop the main inspection loop.
        r_for_handler.store(false, Ordering::SeqCst);

        // Attempt to take the Conductor out of the Mutex/Option and call its teardown method.
        if let Some(c) = shutdown_clone.lock().unwrap().take() {
            c.graceful_teardown();
        }
        let _ = writeln!(io::stdout(), "\nInspector: Shutdown signal received. Waiting for Conductor...");
    }) {
        error!("Could not set Ctrl-C handler: {}", e);
        return;
    }

    info!("Inspector: Press Ctrl-C to stop the live feed and gracefully shut down the Conductor.");

    // --- Live Feed Loop ---

    let mut frame_count: u64 = 0;
    // Arbitrary maximum tile generation rate used for display.
    const MVG_BASELINE: u64 = 10_000_000;

    while running.load(Ordering::SeqCst) {
        frame_count += 1;

        // Fetch current state variables from the thread-safe state tracker.
        let status = state.get_status();
        let queue_depth = state.get_queue_depth();
        let active_id = state.get_active_generator_id();

        // Print the status line, using '\r' (carriage return) to overwrite the line
        // without scrolling the console.
        print!("\r");
        print!("🔮 LIVE FEED | Frame: {: >4} | Status: {: <12} | Generator: {: <25} | Queue Depth: ~{: >6} | MVG Baseline: {} tiles/s | Press Ctrl-C to exit.",
            frame_count,
            format!("{:?}", status),
            active_id,
            queue_depth,
            MVG_BASELINE
        );
        // Flush stdout to ensure the output is immediately written to the terminal.
        let _ = io::stdout().flush();

        // Check for internal shutdown signals from the Conductor itself (e.g., internal error).
        if status == ConductorStatus::ShuttingDown || status == ConductorStatus::Error {
            // Signal the loop to stop and try to manually trigger cleanup one last time.
            running.store(false, Ordering::SeqCst);

            if let Some(c) = conductor_shutdown_safe.lock().unwrap().take() {
                c.graceful_teardown();
            }
            break;
        }

        // Pause for a brief duration (50ms refresh rate).
        thread::sleep(Duration::from_millis(50));
    }

    // Overwrite the last printed line with spaces to clear it cleanly.
    let _ = writeln!(io::stdout(), "\r{: <200}", " ");
    info!("Inspector shutdown complete. Conductor runtime terminated.");
}


/// Placeholder function for running engine performance benchmarks.
#[allow(dead_code)] // Suppressing the warning as this is a public API for the CLI.
pub fn run_benchmark() {
    warn!("⏱️ Benchmark execution not yet implemented. Placeholder called.");
}