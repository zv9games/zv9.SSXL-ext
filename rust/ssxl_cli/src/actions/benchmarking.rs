use tracing::{info, warn, error};
use std::io::{self, Write};
use std::sync::{
    Arc, 
    Mutex, 
    atomic::{AtomicBool, Ordering}
};
use std::thread;
use std::time::Duration;
use ctrlc;

// FIX E0432: Conductor and ConductorStatus are found in the `conductor` module.
use ssxl_generate::conductor::{Conductor, ConductorStatus};


pub fn start_signal_inspector() {
    warn!("🔮 Initializing Conductor and starting Signal Inspector (Real-Time Feed)...");

    // FIX E0308: Conductor::new now returns 4 elements. We destructure the fourth 
    // element (_progress_receiver) to ignore it here, as the CLI inspects the 
    // ConductorState directly, not the message channel.
    let (conductor, state, _request_sender, _progress_receiver) = match Conductor::new(None) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };

    let conductor_shutdown_safe = Arc::new(Mutex::new(Some(conductor)));
    let shutdown_clone = conductor_shutdown_safe.clone();

    let running = Arc::new(AtomicBool::new(true));
    let r_for_handler = running.clone();

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

    while running.load(Ordering::SeqCst) {
        frame_count += 1;

        let status = state.get_status();
        let queue_depth = state.get_queue_depth();
        let active_id = state.get_active_generator_id();

        print!("\r");
        print!("🔮 LIVE FEED | Frame: {: >4} | Status: {: <12} | Generator: {: <25} | Queue Depth: ~{: >6} | MVG Baseline: {} tiles/s | Press Ctrl-C to exit.",
            frame_count,
            format!("{:?}", status),
            active_id,
            queue_depth,
            MVG_BASELINE
        );
        let _ = io::stdout().flush();

        if status == ConductorStatus::ShuttingDown || status == ConductorStatus::Error {
            running.store(false, Ordering::SeqCst);

            if let Some(c) = conductor_shutdown_safe.lock().unwrap().take() {
                c.graceful_teardown();
            }
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }

    let _ = writeln!(io::stdout(), "\r{: <200}", " ");
    info!("Inspector shutdown complete. Conductor runtime terminated.");
}

#[allow(dead_code)]
pub fn run_benchmark() {
    warn!("⏱️ Benchmark execution not yet implemented. Placeholder called.");
}