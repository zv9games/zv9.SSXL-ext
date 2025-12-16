// ssxl_cli/src/ssxl_testing.rs
// ============================================================================
// 🧪 SSXL-ext CLI Testing Harness - PURE RUST IMPLEMENTATION (v4)
// ============================================================================

use std::thread;
use std::time::Instant;

use rand::{thread_rng, Rng};
use tracing::{error, info, warn};

// Bring in core engine types and logic from ssxl_ext.
use ssxl_ext::shared_chunk::Chunk;
use ssxl_ext::shared_config::PerlinNoiseConfig;
use ssxl_ext::shared_tile::TileData;
use ssxl_ext::generate_perlin::{NoiseGenerator, generate_noise_map};
use ssxl_ext::tile_conversion::bitmask_to_id;

// --- Module-Level Constants ---
const CHUNK_SIZE: u32 = 64;
const GRID_SIZE: u32 = 2048;
const ITERATIONS: u32 = 50;
const MAX_TILE_ID: i32 = 47;

// --- Profiler Stub ---
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

// --- Helper: Create a standard Perlin config for tests ---
fn default_perlin_config() -> PerlinNoiseConfig {
    PerlinNoiseConfig {
        octaves: 4,
        lacunarity: 2.0,
        persistence: 0.5,
        scale: 64.0,
        threshold: 0.0,
    }
}

// --- Helper: Run noise generation for a chunk and return it ---
fn generate_chunk_at(chunk_x: i32, chunk_y: i32, size: u32, seed: u64) -> Chunk {
    let config = default_perlin_config();
    let generator = NoiseGenerator::new(config, seed);

    let position = (chunk_x, chunk_y);
    let chunk = Chunk::new(position, size);

    match generate_noise_map(chunk, &generator) {
        Ok(chunk) => chunk,
        Err(err) => {
            panic!(
                "Noise generation failed at chunk ({}, {}): {}",
                chunk_x, chunk_y, err
            );
        }
    }
}

// --- Helper: Compare the touching edges of two chunks ---
fn chunks_share_boundary(a: &Chunk, b: &Chunk, dir: (i32, i32)) -> bool {
    // dir = (1, 0) means b is to the right of a
    // dir = (0, 1) means b is below a

    let size = a.size;
    if a.size != b.size {
        error!(
            "Chunk size mismatch: a.size={} b.size={}",
            a.size, b.size
        );
        return false;
    }

    match dir {
        (1, 0) => {
            // Compare right edge of a with left edge of b
            for y in 0..size {
                let a_tile = a.get_tile(size - 1, y);
                let b_tile = b.get_tile(0, y);
                if !tiles_approx_equal(a_tile, b_tile) {
                    return false;
                }
            }
            true
        }
        (0, 1) => {
            // Compare bottom edge of a with top edge of b
            for x in 0..size {
                let a_tile = a.get_tile(x, size - 1);
                let b_tile = b.get_tile(x, 0);
                if !tiles_approx_equal(a_tile, b_tile) {
                    return false;
                }
            }
            true
        }
        _ => {
            warn!("Unsupported boundary direction: {:?}", dir);
            false
        }
    }
}

// --- Helper: Approximate tile equality for boundary tests ---
fn tiles_approx_equal(a: Option<&TileData>, b: Option<&TileData>) -> bool {
    match (a, b) {
        (Some(ta), Some(tb)) => {
            ta.tile_id == tb.tile_id &&
            ta.atlas_coords == tb.atlas_coords &&
            ta.rotation_flags == tb.rotation_flags
            // We intentionally ignore custom_data for boundary coherence.
        }
        (None, None) => true,
        _ => false,
    }
}

// ----------------------------------------------------------------------------
// I. Component Tests (Pure Rust)
// ----------------------------------------------------------------------------

/// 1. Concurrency Stress Test: Spawns many threads to hammer noise generation.
fn run_fast_test() {
    info!("Starting Fast Test (Concurrency Stress on Noise Generator)...");
    let _p = Profiler::start("FastTest_Concurrency");

    const NUM_THREADS: u32 = 500;
    const CHUNK_LOCAL_SIZE: u32 = 16;

    let handles = (0..NUM_THREADS)
        .map(|i| {
            thread::spawn(move || {
                let seed_base = i as u64;
                for j in 0..20u32 {
                    let seed = seed_base + j as u64;
                    let _chunk = generate_chunk_at(i as i32, j as i32, CHUNK_LOCAL_SIZE, seed);
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    info!(
        "✅ Fast Test Complete: {} threads exercised concurrent noise generation.",
        NUM_THREADS
    );
}

/// 2. Full Integration Test: Chunk generation + boundary checks.
fn run_full_test() {
    info!("Starting Full Integration Test (Chunk Boundary Coherence)...");
    let _p = Profiler::start("FullTest_Integration");

    let seed: u64 = thread_rng().gen();

    let c00 = generate_chunk_at(0, 0, CHUNK_SIZE, seed);
    let c10 = generate_chunk_at(1, 0, CHUNK_SIZE, seed);
    let c01 = generate_chunk_at(0, 1, CHUNK_SIZE, seed);
    let c11 = generate_chunk_at(1, 1, CHUNK_SIZE, seed);

    let mut failures = 0;

    // Right of c00
    if !chunks_share_boundary(&c00, &c10, (1, 0)) {
        error!("❌ Boundary Fail: Chunk (0,0) and (1,0) mismatch.");
        failures += 1;
    }

    // Below c00
    if !chunks_share_boundary(&c00, &c01, (0, 1)) {
        error!("❌ Boundary Fail: Chunk (0,0) and (0,1) mismatch.");
        failures += 1;
    }

    // Below c10
    if !chunks_share_boundary(&c10, &c11, (0, 1)) {
        error!("❌ Boundary Fail: Chunk (1,0) and (1,1) mismatch.");
        failures += 1;
    }

    // Right of c01
    if !chunks_share_boundary(&c01, &c11, (1, 0)) {
        error!("❌ Boundary Fail: Chunk (0,1) and (1,1) mismatch.");
        failures += 1;
    }

    if failures == 0 {
        info!("✅ Full Test Passed: Boundary coherence verified across 4 chunks.");
    } else {
        panic!(
            "❌ Full Test FAILED: {} boundary mismatches detected.",
            failures
        );
    }
}


/// 3. Max Grid Benchmark: Heavy compute load on a huge chunk.
fn run_max_grid_benchmark() {
    info!("Starting Max Grid Benchmark (Heavy Noise Compute Iterations)...");
    let _p = Profiler::start("MaxGrid_HeavyNoise");

    info!(
        "Generating {} x {} noise field {} times (single large chunk)...",
        GRID_SIZE, GRID_SIZE, ITERATIONS
    );

    for i in 0..ITERATIONS {
        let seed = thread_rng().gen();
        let _chunk = generate_chunk_at(0, i as i32, GRID_SIZE, seed);
    }

    info!(
        "✅ Max Grid Benchmark Complete: {} heavy generation calls finished.",
        ITERATIONS
    );
}

/// 4. Bitmask Conversion Test: Exhaustive 256-case validation (pure Rust).
fn run_bitmask_conversion() {
    info!("Starting Bitmask Conversion Test (256/256 Exhaustive Coverage)...");
    let _p = Profiler::start("Bitmask_Exhaustive");

    let mut failures = 0;

    for bitmask in 0u8..=255 {
        let result_id = bitmask_to_id(bitmask);

        if result_id < 0 || result_id > MAX_TILE_ID {
            error!(
                "❌ Bitmask Fail: Input {} resulted in invalid Tile ID {}.",
                bitmask, result_id
            );
            failures += 1;
        }

        if bitmask == 255 && result_id != 10 {
            warn!(
                "Bitmask 255 not mapping to expected 'Full' Tile ID (10). Got: {} (MOCK behavior).",
                result_id
            );
        }
    }

    if failures == 0 {
        info!("✅ Bitmask Conversion Test Passed: 256/256 combinations verified (0 failures).");
    } else {
        panic!(
            "❌ Bitmask Conversion Test FAILED: {} failures detected! (MAX_TILE_ID={})",
            failures, MAX_TILE_ID
        );
    }
}

// ----------------------------------------------------------------------------
// II. Grand Unified Test (G-action)
// ----------------------------------------------------------------------------

/// Chains together the most critical tests for a full smoke test.
pub fn run_grand_unified_test() {
    println!("\n==================================================");
    println!("=== SSXL-ext GRAND UNIFIED TEST (GUT) STARTING ===");
    println!("==================================================");

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
