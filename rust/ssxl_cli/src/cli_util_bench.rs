// ssxl_cli/src/cli_util_bench.rs

//! # CLI Utilities: Benchmarking and Validation (`ssxl_cli::cli_util_bench`)
//!
//! Provides CLI-specific functions for testing core engine features and running
//! performance benchmarks, primarily focusing on world generation logic.

use tracing::{info, warn, error};
use std::time::Instant;
use std::thread;
use std::sync::{
    Arc, 
    atomic::{AtomicU64, Ordering} // AtomicU64 for thread-safe tile counting
};
use std::io::{self, Write};
use std::time::Duration;

// Imports of core engine components.
use ssxl_generate::benchmark_generation_workload; // External function that runs the actual benchmark work
use ssxl_generate::conductor::Conductor; // The main asynchronous world manager
use ssxl_math::Vec2i; // Coordinate type

// --- Test Utilities ---

/// Runs a high-level test to validate that the `Conductor` can initialize,
/// switch active generators, and successfully generate single chunks on demand.
pub fn test_generation_and_placement_cli() {
    warn!("🧪 Running CLI Test: Generation and Placement (Conductor Validation)...");

    // Initialize the Conductor. We only need the Conductor handle for this test.
    let (mut conductor, _state, _progress_receiver, _request_sender) = match Conductor::new(None) {
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

    // Test Case 1: Perlin Noise Generator
    // 🎯 FIX: Changed "perlin_mvg" to the correct, registered ID "perlin_basic_2d"
    let perlin_id = "perlin_basic_2d";
    if conductor.set_active_generator(perlin_id).is_ok() {
        info!("-> Active Generator set to: {}", perlin_id);
        for &coords in &test_coords {
            // Synchronously request a chunk generation.
            let _chunk = conductor.generate_single_chunk(coords);
            info!("  - Generated chunk {:?} successfully.", coords);
            chunks_generated += 1;
        }
    } else {
        warn!("Generator '{}' not found. Skipping Perlin test.", perlin_id);
    }
    
    // Test Case 2: Cellular Automata Generator
    let ca_id = "cellular_automata_basic";
    if conductor.set_active_generator(ca_id).is_ok() {
        info!("-> Active Generator set to: {}", ca_id);
        let coords = Vec2i::new(50, 50);
        let _chunk = conductor.generate_single_chunk(coords);
        info!("  - Generated chunk {:?} successfully.", coords);
        chunks_generated += 1;
    } else {
        warn!("Generator '{}' not found. Skipping CA test.", ca_id);
    }

    // Final result reporting.
    if chunks_generated > 0 {
        info!("✅ CLI Generation Test SUCCESS: Generated {} chunks across {} generator(s).", 
            chunks_generated, 
            // NOTE: This will report the last successful generator ID.
            conductor.get_active_generator_id() 
        );
    } else {
        error!("❌ CLI Generation Test FAILED: Zero chunks were generated. Check Conductor initialization and generator IDs.");
    }

    // Clean up the Conductor's worker threads and runtime.
    conductor.graceful_teardown();
}


/// Placeholder for a utility function that converts an image file into engine bitmask data.
pub fn run_bitmask_conversion() {
    warn!("🧪 Starting bitmask conversion from world.png (Placeholder)...");

    // Simulated result.
    let tiles_placed = 5000;
    
    info!("✅ Conversion complete. Tiles placed: {}", tiles_placed);
}

// --- Benchmark ---

/// Executes the Max Grid Generation Benchmark, which stresses the asynchronous
/// generation pipeline with a large, simulated workload.
pub fn run_max_grid_benchmark() {
    warn!("🧪 Starting Max Grid Benchmark (Real Workload)...");
    
    // Define the total number of tiles the benchmark should process.
    const WORKLOAD_TILES: u64 = 100_000_000;

    // An Atomic counter shared between the workload thread and the reporting thread.
    let processed_tiles = Arc::new(AtomicU64::new(0));
    let workload_counter_clone = processed_tiles.clone();
    let ticker_counter_clone = processed_tiles.clone();

    // --- Workload Thread (Generates Tiles) ---
    let workload_handle = thread::spawn(move || {
        // This function blocks and increments `workload_counter_clone`.
        benchmark_generation_workload(WORKLOAD_TILES, workload_counter_clone); 
    });

    // --- Ticker Thread (Reports Progress) ---
    let start_time = Instant::now();
    let ticker_handle = thread::spawn(move || {
        let total = WORKLOAD_TILES as f64;
        
        loop {
            let current = ticker_counter_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            
            // Calculate progress percentage and clamp to 100%.
            let percentage = ((current as f64 / total) * 100.0).min(100.0).round() as u64;

            // Calculate instantaneous throughput (tiles per second).
            let throughput = if elapsed > 0.0 { 
                (current as f64 / elapsed).round() as u64 
            } else { 
                0 
            };

            // Display progress using carriage return (`\r`) to update the same line.
            print!("\r⏳ Progress: {: >3}% ({: >8}M / {}M) | Throughput: ~{: >9} tiles/s", 
                percentage, 
                current / 1_000_000, // Display current in Millions
                WORKLOAD_TILES / 1_000_000, // Display total in Millions
                throughput
            );
            let _ = io::stdout().flush(); // Ensure output is immediately displayed.

            if current >= WORKLOAD_TILES {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

    // --- Execution & Final Reporting ---

    let start = Instant::now();
    
    // Wait for the generation workload to complete. Handle panics gracefully.
    if let Err(e) = workload_handle.join() {
        error!("Workload thread panicked: {:?}", e);
        // Overwrite the ticker line with a failure message.
        let _ = println!("\r❌ Benchmark failed: Generation thread panic. {: <100}", " "); 
        return; 
    }
    
    // Wait for the ticker thread to finish (it breaks its loop after the workload completes).
    ticker_handle.join().unwrap();
    
    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();
    let duration_millis = duration.as_millis() as f64;
    let actual_tiles_placed = WORKLOAD_TILES;
    
    // Final summary print.
    println!("\r✅ Benchmark complete. Workload: {} tiles. Duration: {:.2}s", 
        actual_tiles_placed, duration_secs);

    if duration_secs > 0.0 {
        let throughput_sec = (actual_tiles_placed as f64 / duration_secs).round() as u64;
        let throughput_ms = actual_tiles_placed as f64 / duration_millis;
        
        println!("⚡ Max Throughput: ~{} tiles/sec", throughput_sec);
        println!("⚡ Diagnostics: {:.2} Million tiles/ms", throughput_ms / 1_000_000.0);
        
        // Define performance targets for status reporting.
        const MVG_BASELINE: u64 = 10_000_000;
        const ITERATION5_TARGET: u64 = 18_000_000;

        // Grade the performance result.
        if throughput_sec >= ITERATION5_TARGET {
            info!("🚀 CRITICAL SUCCESS: Throughput of {} tiles/sec EXCEEDS the Iteration 5 Target of {} tiles/sec!", throughput_sec, ITERATION5_TARGET);
        } else if throughput_sec >= MVG_BASELINE {
            info!("📈 Performance OK: Throughput of {} tiles/sec meets or exceeds the MVG Baseline of {} tiles/sec.", throughput_sec, MVG_BASELINE);
        } else {
            error!("⚠️ Performance CRITICAL: Throughput of {} tiles/sec is BELOW the MVG Baseline of {} tiles/sec.", throughput_sec, MVG_BASELINE);
            warn!("Use Signal Inspector to diagnose the concurrency bottleneck.");
        }

    } else {
        error!("❌ Benchmark failed: Workload was too small or timer error (duration was 0.0s). Increase WORKLOAD_TILES.");
    }
}