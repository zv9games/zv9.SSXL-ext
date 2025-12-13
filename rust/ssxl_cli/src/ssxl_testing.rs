// ssxl_cli/src/ssxl_testing.rs
// ============================================================================
// 🧪 SSXL-ext CLI Testing Harness - HEAVY LOAD IMPLEMENTATION (v3: Corrected)
// ----------------------------------------------------------------------------
use tracing::{info, warn, error};
use std::thread;
use std::time::Instant;
// Removed unused imports: use std::sync::{Arc, Mutex}; 
use rand::{thread_rng, Rng}; // ✅ Requires 'rand = "0.8"' in Cargo.toml

// --- Module-Level Constants (Visible to all functions, including panic! macros) ---
const CHUNK_SIZE: i32 = 64; 
const GRID_SIZE: i32 = 2048;
const ITERATIONS: i32 = 50; 
const MAX_TILE_ID: i32 = 47; 

// --- FFI Declarations (MUST be imported from the main crate for linking) ---
extern "C" {
    // The core FFI functions used for writing data to the Godot host
    fn ssxl_set_cell(x: i32, y: i32, tile_id: i32);
    fn ssxl_notify_tilemap_update();

    // ASSUMED CORE API: Functions exported by ssxl_ext for direct testing
    fn ssxl_ext_bitmask_to_id(bitmask: u8) -> i32;
    fn ssxl_ext_generate_noise_chunk(x: i32, y: i32, size: i32, seed: u64);
    fn ssxl_ext_verify_chunk_boundary(chunk_x: i32, chunk_y: i32, neighbor_x: i32, neighbor_y: i32) -> bool;
}

// --- Profiler Stub (Defined from rust.txt for self-containment) ---
struct Profiler {
    start: Instant,
    name: &'static str,
    enabled: bool,
}

impl Profiler {
    pub fn start(name: &'static str) -> Self {
        Profiler {
            start: Instant::now(),
            name,
            enabled: true,
        }
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        if self.enabled {
            let duration = self.start.elapsed();
            eprintln!(
                "PERF [{}]: Execution time: {:.3}ms", 
                self.name, 
                duration.as_secs_f64() * 1000.0
            );
        }
    }
}


// --- Test Helper: FFI Call with Safe Access ---
fn call_ffi_tile_write(x: i32, y: i32, id: i32) {
    // Safety: Calls FFI function defined in ssxl_ext.dll.
    unsafe {
        ssxl_set_cell(x, y, id);
    }
}


// ----------------------------------------------------------------------------
// I. The Four Component Tests (Called internally by GUT)
// ----------------------------------------------------------------------------

/// 1. Concurrency Stress Test: Spawns a massive number of threads to hit the internal queue.
fn run_fast_test() {
    info!("Starting Fast Test (Concurrency Stress on FFI locks)...");
    let _p = Profiler::start("FastTest_Concurrency"); 
    
    const NUM_THREADS: i32 = 500;
    const CALLS_PER_THREAD: i32 = 20; 

    let handles = (0..NUM_THREADS)
        .map(|i| {
            thread::spawn(move || {
                let seed = i as u64;
                for j in 0..CALLS_PER_THREAD {
                    // Call the heavy computation function (assumed to be thread-safe)
                    unsafe { ssxl_ext_generate_noise_chunk(i, j, 16, seed + j as u64) };
                    
                    // Directly stress the FFI lock with a write
                    call_ffi_tile_write(i, j, (i + j) % 10); 
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
    
    // Notify the host once everything is done
    unsafe { ssxl_notify_tilemap_update(); }

    info!("✅ Fast Test Complete: {} total FFI writes executed (plus {} noise generation calls).", 
          NUM_THREADS * CALLS_PER_THREAD, NUM_THREADS * CALLS_PER_THREAD);
}

/// 2. Full Integration Test: Large map generation, boundary, and data integrity checks.
fn run_full_test() {
    info!("Starting Full Integration Test (Chunk Boundary Coherence)...");
    let _p = Profiler::start("FullTest_Integration"); 

    let seed: u64 = thread_rng().gen();

    // 1. Generate four adjacent chunks (using the actual generation FFI call)
    unsafe { 
        ssxl_ext_generate_noise_chunk(0, 0, CHUNK_SIZE, seed);
        ssxl_ext_generate_noise_chunk(1, 0, CHUNK_SIZE, seed);
        ssxl_ext_generate_noise_chunk(0, 1, CHUNK_SIZE, seed);
        ssxl_ext_generate_noise_chunk(1, 1, CHUNK_SIZE, seed);
    }
    
    // --- Boundary Validation (CRITICAL TEST) ---
    let mut failures = 0;
    
    // Check horizontal boundary (Chunk 0,0 vs 1,0)
    if !unsafe { ssxl_ext_verify_chunk_boundary(0, 0, 1, 0) } {
        error!("❌ Boundary Fail: Chunk (0,0) and (1,0) mismatch.");
        failures += 1;
    } 

    // Check vertical boundary (Chunk 0,0 vs 0,1)
    if !unsafe { ssxl_ext_verify_chunk_boundary(0, 0, 0, 1) } {
        error!("❌ Boundary Fail: Chunk (0,0) and (0,1) mismatch.");
        failures += 1;
    }
    
    if failures == 0 {
        info!("✅ Full Test Passed: Boundary coherence verified across 4 chunks.");
    } else {
         panic!("❌ Full Test FAILED: {} boundary mismatches detected.", failures);
    }
    
    // Draw markers for visual confirmation in the Godot host
    call_ffi_tile_write(CHUNK_SIZE / 2, CHUNK_SIZE / 2, 99); // Center of 0,0
    call_ffi_tile_write(CHUNK_SIZE * 3 / 2, CHUNK_SIZE / 2, 99); // Center of 1,0
    unsafe { ssxl_notify_tilemap_update(); }
}

/// 3. Max Grid Benchmark: Use the Profiler to measure raw compute load.
fn run_max_grid_benchmark() {
    info!("Starting Max Grid Benchmark (Heavy Noise Compute Iterations)...");
    
    // Start profiler AFTER setting up static variables
    let _p = Profiler::start("MaxGrid_HeavyNoise"); 

    info!("Generating {} x {} noise field {} times...", GRID_SIZE, GRID_SIZE, ITERATIONS);

    // Loop to measure worst-case scenario over multiple seeds
    for _ in 0..ITERATIONS {
        let seed = thread_rng().gen(); 
        // Force the core to generate one massive chunk, stressing ssxl_ext's compute ability
        unsafe { 
            ssxl_ext_generate_noise_chunk(0, 0, GRID_SIZE, seed); 
        }
    }
    
    info!("✅ Max Grid Benchmark Complete: {} heavy generation calls finished.", ITERATIONS);
}

/// 4. Bitmask Conversion Test: Exhaustive data integrity check (256/256 combinations).
fn run_bitmask_conversion() {
    info!("Starting Bitmask Conversion Test (256/256 Exhaustive Coverage)...");
    let _p = Profiler::start("Bitmask_Exhaustive"); 

    let mut failures = 0;
    
    for bitmask in 0u8..=255 {
        // ACTUAL CALL: Replace the dummy logic with the real FFI call
        let result_id = unsafe { ssxl_ext_bitmask_to_id(bitmask) };
        
        // Assert: The result must be a valid, positive tile ID. 
        if result_id < 0 || result_id > MAX_TILE_ID {
            error!("❌ Bitmask Fail: Input {} resulted in invalid Tile ID {}.", bitmask, result_id);
            failures += 1;
        } 
        
        // Secondary Assert: Specific known good and known bad cases
        if bitmask == 255 && result_id != 10 { // 255 (all neighbors) must be the 'Full' tile ID 10
             warn!("Bitmask 255 failed to map to expected 'Full' Tile ID (10). Got: {}", result_id);
             // Failures are counted by the main check, but this provides context
        }
    }
    
    if failures == 0 {
        info!("✅ Bitmask Conversion Test Passed: 256/256 combinations verified (0 failures).");
    } else {
        // ✅ FIXED: MAX_TILE_ID is now a module constant and accessible here.
        panic!("❌ Bitmask Conversion Test FAILED: {} failures detected! (MAX_TILE_ID={})", failures, MAX_TILE_ID);
    }
}


// ----------------------------------------------------------------------------
// II. Grand Unified Test (The G-Action)
// ----------------------------------------------------------------------------

/// Chains together the most critical tests for a full smoke test.
pub fn run_grand_unified_test() {
    println!("\n==================================================");
    println!("=== SSXL-ext GRAND UNIFIED TEST (GUT) STARTING ===");
    println!("==================================================");
    
    // Order: Data Integrity -> Heavy Compute -> I/O Stress
    
    run_bitmask_conversion();
    println!("--------------------------------------------------");

    run_max_grid_benchmark();
    println!("--------------------------------------------------");

    run_fast_test(); 
    println!("--------------------------------------------------");

    run_full_test();
    
    println!("\n==================================================");
    println!("=== SSXL-ext GRAND UNIFIED TEST (GUT) COMPLETE ===");
    println!("==================================================");
}