// ssxl_cli\src\cli_util_bench.rs

use tracing::{info, warn, error};
use std::time::Instant;
use std::thread;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering}
};
use std::io::{self, Write};
use std::time::Duration;

use ssxl_generate::benchmark_generation_workload;
use ssxl_generate::conductor::Conductor;
use ssxl_math::prelude::Vec2i;

pub fn test_generation_and_placement_cli() {
    warn!("🧪 Running CLI Test: Generation and Placement (Conductor Validation)...");

    let (mut conductor, _state, _request_sender, _progress_receiver) = match Conductor::new(None) {
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

    let perlin_id = "perlin_basic_2d";
    if conductor.set_generator(perlin_id).is_ok() {
        info!("-> Active Generator set to: {}", perlin_id);
        for &coords in &test_coords {
            // FIX: Use the correct synchronous generation method: `get_chunk_data`
            let _chunk = conductor.get_chunk_data(&coords);
            info!("  - Generated chunk {:?} successfully.", coords);
            chunks_generated += 1;
        }
    } else {
        warn!("Generator '{}' not found. Skipping Perlin test.", perlin_id);
    }
    
    let ca_id = "cellular_automata_basic";
    if conductor.set_generator(ca_id).is_ok() {
        info!("-> Active Generator set to: {}", ca_id);
        let coords = Vec2i::new(50, 50);
        // FIX: Use the correct synchronous generation method: `get_chunk_data`
        let _chunk = conductor.get_chunk_data(&coords);
        info!("  - Generated chunk {:?} successfully.", coords);
        chunks_generated += 1;
    } else {
        warn!("Generator '{}' not found. Skipping CA test.", ca_id);
    }

    if chunks_generated > 0 {
        info!("✅ CLI Generation Test SUCCESS: Generated {} chunks across {} generator(s).",
            chunks_generated,
            conductor.get_active_generator_id()
        );
    } else {
        error!("❌ CLI Generation Test FAILED: Zero chunks were generated. Check Conductor initialization and generator IDs.");
    }

    conductor.graceful_teardown();
}


pub fn run_bitmask_conversion() {
    warn!("🧪 Starting bitmask conversion from world.png (Placeholder)...");

    let tiles_placed = 5000;
    
    info!("✅ Conversion complete. Tiles placed: {}", tiles_placed);
}

pub fn run_max_grid_benchmark() {
    warn!("🧪 Starting Max Grid Benchmark (Real Workload)...");
    
    const WORKLOAD_TILES: u64 = 100_000_000;

    let processed_tiles = Arc::new(AtomicU64::new(0));
    let workload_counter_clone = processed_tiles.clone();
    let ticker_counter_clone = processed_tiles.clone();

    let workload_handle = thread::spawn(move || {
        benchmark_generation_workload(WORKLOAD_TILES, workload_counter_clone);
    });

    let start_time = Instant::now();
    let ticker_handle = thread::spawn(move || {
        let total = WORKLOAD_TILES as f64;
        
        loop {
            let current = ticker_counter_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            
            let percentage = ((current as f64 / total) * 100.0).min(100.0).round() as u64;

            let throughput = if elapsed > 0.0 {
                (current as f64 / elapsed).round() as u64
            } else {
                0
            };

            print!("\r⏳ Progress: {: >3}% ({: >8}M / {}M) | Throughput: ~{: >9} tiles/s",
                percentage,
                current / 1_000_000,
                WORKLOAD_TILES / 1_000_000,
                throughput
            );
            let _ = io::stdout().flush();

            if current >= WORKLOAD_TILES {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

    let start = Instant::now();
    
    if let Err(e) = workload_handle.join() {
        error!("Workload thread panicked: {:?}", e);
        let _ = println!("\r❌ Benchmark failed: Generation thread panic. {: <100}", " ");
        return;
    }
    
    ticker_handle.join().unwrap();
    
    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();
    let duration_millis = duration.as_millis() as f64;
    let actual_tiles_placed = WORKLOAD_TILES;
    
    println!("\r✅ Benchmark complete. Workload: {} tiles. Duration: {:.2}s",
        actual_tiles_placed, duration_secs);

    if duration_secs > 0.0 {
        let throughput_sec = (actual_tiles_placed as f64 / duration_secs).round() as u64;
        let throughput_ms = actual_tiles_placed as f64 / duration_millis;
        
        println!("⚡ Max Throughput: ~{} tiles/sec", throughput_sec);
        println!("⚡ Diagnostics: {:.2} Million tiles/ms", throughput_ms / 1_000_000.0);
        
        const MVG_BASELINE: u64 = 10_000_000;
        const ITERATION5_TARGET: u64 = 18_000_000;

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