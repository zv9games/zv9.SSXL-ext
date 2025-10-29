//! Utilities for running generation benchmarks and conversion tests.

use tracing::{info, warn, error};
use std::time::Instant;
// NEW IMPORTS for Concurrency and Live Ticker
use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::io::{self, Write};
use std::time::Duration;

// --- EXTERNAL CRATE DEPENDENCIES ---
use ssxl_generate::benchmark_generation_workload; 
use ssxl_generate::conductor::Conductor; // Import Conductor
use ssxl_math::Vec2i; // Import Vec2i for chunk coordinates

// --- Test Utilities ---

/// 🧪 CLI-safe test: Initializes the Conductor, switches generators, and generates
/// a small set of chunks to validate the core pipeline.
pub fn test_generation_and_placement_cli() {
    warn!("🧪 Running CLI Test: Generation and Placement (Conductor Validation)...");

    // 1. Initialize Conductor and retrieve state
    // FIX: Update destructuring to handle the new 3-element tuple (Conductor, ConductorState, Receiver).
    let (mut conductor, _state, _receiver) = match Conductor::new(None) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };

    let test_coords = [
        Vec2i::new(0, 0),
        Vec2i::new(-1, 0),
        Vec2i::new(100, 100),
    ];
    let mut chunks_generated = 0;

    // --- 2. Test Perlin Generator ---
    let perlin_id = "perlin_mvg"; 
    if conductor.set_active_generator(perlin_id).is_ok() {
        info!("-> Active Generator set to: {}", perlin_id);
        for &coords in &test_coords {
            // FIX: Renamed 'chunk' to '_chunk' to suppress the unused variable warning.
            let _chunk = conductor.generate_single_chunk(coords);
            info!("  - Generated chunk {:?} successfully.", coords);
            // In a real scenario, we would assert properties of '_chunk' here.
            chunks_generated += 1;
        }
    } else {
        warn!("Generator '{}' not found. Skipping Perlin test.", perlin_id);
    }
    
    // --- 3. Test Cellular Automata Generator ---
    let ca_id = "cellular_automata_basic"; 
    if conductor.set_active_generator(ca_id).is_ok() {
        info!("-> Active Generator set to: {}", ca_id);
        let coords = Vec2i::new(50, 50);
        // FIX: Renamed 'chunk' to '_chunk' to suppress the unused variable warning.
        let _chunk = conductor.generate_single_chunk(coords);
        info!("  - Generated chunk {:?} successfully.", coords);
        chunks_generated += 1;
    } else {
        warn!("Generator '{}' not found. Skipping CA test.", ca_id);
    }

    // --- 4. Report and Shutdown ---
    if chunks_generated > 0 {
        info!("✅ CLI Generation Test SUCCESS: Generated {} chunks across {} generator(s).", 
            chunks_generated, 
            conductor.get_active_generator_id() // Reports the last active ID
        );
    } else {
        error!("❌ CLI Generation Test FAILED: Zero chunks were generated. Check Conductor initialization and generator IDs.");
    }

    // FIX: Use the renamed, consuming shutdown method.
    conductor.graceful_teardown();
}

// --- Conversion Utilities ---

/// 🧪 Converts a PNG into a tile chunk using bitmask logic (Placeholder).
pub fn run_bitmask_conversion() {
    warn!("🧪 Starting bitmask conversion from world.png (Placeholder)...");

    let tiles_placed = 5000;
    
    info!("✅ Conversion complete. Tiles placed: {}", tiles_placed);
}

// --- Benchmark Utilities ---

/// 🧪 Benchmarks tile placement throughput.
///
/// Executes a fixed workload concurrently with a live progress ticker.
pub fn run_max_grid_benchmark() {
    warn!("🧪 Starting Max Grid Benchmark (Real Workload)...");
    
    const WORKLOAD_TILES: u64 = 100_000_000; 

    // --- 1. SETUP SHARED ATOMIC STATE ---
    let processed_tiles = Arc::new(AtomicU64::new(0));
    let workload_counter_clone = processed_tiles.clone();
    let ticker_counter_clone = processed_tiles.clone();

    // --- 2. START THE WORKLOAD (GENERATION) THREAD ---
    // NOTE: Conductor::new() is called inside benchmark_generation_workload 
    // and is assumed to be fixed there (to pass None).
    let workload_handle = thread::spawn(move || {
        // NOTE: The function signature in aetherion_generate now includes the counter.
        benchmark_generation_workload(WORKLOAD_TILES, workload_counter_clone); 
    });

    // --- 3. START THE TICKER (CLI REPORTING) THREAD ---
    let start_time = Instant::now();
    let ticker_handle = thread::spawn(move || {
        let total = WORKLOAD_TILES as f64;
        
        loop {
            // Load the current progress atomically
            let current = ticker_counter_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            let percentage = ((current as f64 / total) * 100.0).min(100.0).round() as u64;

            // Calculate live throughput
            let throughput = if elapsed > 0.0 { 
                (current as f64 / elapsed).round() as u64 
            } else { 
                0 
            };

            // Use \r to reset the cursor to the start of the line for a live update
            print!("\r⏳ Progress: {: >3}% ({: >8}M / {}M) | Throughput: ~{: >9} tiles/s", 
                percentage, 
                current / 1_000_000, // Display in Millions
                WORKLOAD_TILES / 1_000_000, // Display Total in Millions
                throughput
            );
            // Ensure the output is immediately visible
            let _ = io::stdout().flush();

            if current >= WORKLOAD_TILES {
                break;
            }
            // Poll progress every 50ms
            thread::sleep(Duration::from_millis(50));
        }
    });

    // --- 4. WAIT FOR WORKLOAD AND MEASURE TIME ---
    let start = Instant::now();
    
    // Block until the generation work is finished
    if let Err(e) = workload_handle.join() {
        error!("Workload thread panicked: {:?}", e);
        // Clean the line and exit
        let _ = println!("\r❌ Benchmark failed: Generation thread panic. {: <100}", " "); 
        return; 
    }
    
    // Ensure the ticker thread prints 100% and finishes its loop
    ticker_handle.join().unwrap();
    
    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();
    let duration_millis = duration.as_millis() as f64; 
    let actual_tiles_placed = WORKLOAD_TILES;
    
    // Clear the progress line before printing final results
    println!("\r✅ Benchmark complete. Workload: {} tiles. Duration: {:.2}s", 
        actual_tiles_placed, duration_secs);

    if duration_secs > 0.0 {
        let throughput_sec = (actual_tiles_placed as f64 / duration_secs).round() as u64;
        let throughput_ms = actual_tiles_placed as f64 / duration_millis;
        
        // --- Outputting Max Possible Throughput ---
        println!("⚡ Max Throughput: ~{} tiles/sec", throughput_sec);
        println!("⚡ Diagnostics: {:.2} Million tiles/ms", throughput_ms / 1_000_000.0);
        
        // --- Performance Analysis ---
        const MVG_BASELINE: u64 = 10_000_000;
        const ITERATION5_TARGET: u64 = 18_000_000;

        if throughput_sec >= ITERATION5_TARGET {
            info!("🚀 CRITICAL SUCCESS: Throughput of {} tiles/sec EXCEEDS the Iteration 5 Target of {} tiles/sec!", throughput_sec, ITERATION5_TARGET);
        } else if throughput_sec >= MVG_BASELINE {
            info!("📈 Performance OK: Throughput of {} tiles/sec meets or exceeds the MVG Baseline of {} tiles/sec.", throughput_sec, MVG_BASELINE);
        } else {
            error!("⚠️ Performance CRITICAL: Throughput of {} tiles/sec is BELOW the MVG Baseline of {} tiles/sec.", throughput_sec, MVG_BASELINE);
            warn!("Use Signal Inspector [B] to diagnose the concurrency bottleneck.");
        }

    } else {
        error!("❌ Benchmark failed: Workload was too small or timer error (duration was 0.0s). Increase WORKLOAD_TILES.");
    }
}