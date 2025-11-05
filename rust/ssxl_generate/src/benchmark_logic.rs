// ssxl_generate/src/benchmark_logic.rs

use tracing::{info, warn};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}}; // ADDED: For concurrent progress reporting

pub fn benchmark_generation_workload(
    workload_tiles: u64, 
    processed_tiles_counter: Arc<AtomicU64> // NEW: Shared atomic counter for progress
) {
    warn!("Generator: Executing generation workload for {} tiles...", workload_tiles);

    const UPDATE_INTERVAL: u64 = 10_000_000; 

    let mut result: u64 = 0;
    
    for i in 0..workload_tiles {
        result = result.wrapping_add(i % 17);
        
        if (i + 1) % UPDATE_INTERVAL == 0 {
            processed_tiles_counter.fetch_add(UPDATE_INTERVAL, Ordering::Relaxed);
        }
    }

    let final_update = workload_tiles % UPDATE_INTERVAL;
    if final_update > 0 {
        processed_tiles_counter.fetch_add(final_update, Ordering::Relaxed);
    }
    
    info!("Generator: Workload simulation complete. Final check value: {}", result);
}