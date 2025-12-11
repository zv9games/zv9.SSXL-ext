// rust/SSXL-ext/src/host_tilemap_status.rs

use godot::prelude::*;

use crate::host_state::HostState;
use crate::generate_conductor_state::GenerationMetrics;
use crate::{ssxl_error}; // For logging if the state is uninitialized

/// Converts the core generation metrics into a Godot Dictionary for GDScript consumption.
/// 
/// This function is called from the Godot main thread loop (via host_tick.rs).
/// 
/// Note: The function name must exactly match the name referenced in host_tick.rs:
/// `get_status_report_dict`.
pub fn get_status_report_dict(host_state: &HostState) -> Dictionary {
    let mut dict = Dictionary::new();

    // The HostState should contain the GenerateConductor instance.
    if !host_state.is_core_ready {
        ssxl_error!("Called get_status_report_dict before core was ready.");
        dict.insert("is_core_ready", false.to_variant());
        dict.insert("conductor_state", "Uninitialized".to_variant());
        dict.insert("total_chunks", 0.to_variant());
        dict.insert("completed_chunks", 0.to_variant());
        return dict;
    }

    // 1. Get the current state and metrics from the Conductor
    let conductor = &host_state.conductor;
    let metrics = conductor.get_metrics();
    let conductor_state = conductor.get_state_container().get_state();

    // 2. Map metrics into the Dictionary
    dict.insert("is_core_ready", host_state.is_core_ready.to_variant());
    dict.insert("conductor_state", conductor_state.to_string().to_variant());
    
    // Convert GenerationMetrics into Dictionary entries
    dict.insert("total_chunks", metrics.total_chunks.to_variant());
    dict.insert("completed_chunks", metrics.completed_chunks.to_variant());
    
    // Calculate progress as a float
    let progress = if metrics.total_chunks > 0 {
        metrics.completed_chunks as f64 / metrics.total_chunks as f64
    } else {
        0.0
    };
    dict.insert("progress", progress.to_variant());

    dict
}