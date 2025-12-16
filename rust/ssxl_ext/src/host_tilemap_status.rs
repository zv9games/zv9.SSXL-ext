// rust/SSXL-ext/src/host_tilemap_status.rs (FIXED)

// 🎯 CRITICAL FIX: Gate Godot imports
#[cfg(feature = "godot-binding")]
use godot::prelude::*;

#[cfg(feature = "godot-binding")]
use godot::builtin::VarDictionary;

use crate::host_state::HostState;

// NOTE: ConductorState should be visible without gating.

/// Generates a `VarDictionary` containing the current status and metrics of the SSXL core.
// 🎯 CRITICAL FIX: Gate the entire function implementation
#[cfg(feature = "godot-binding")]
pub fn get_status_report_dict(host_state: &HostState) -> VarDictionary {
    let mut dict = VarDictionary::new();

    if !host_state.is_core_ready {
        // ssxl_error! is already feature-gated and will work correctly.
        crate::ssxl_error!("Called get_status_report_dict before core was ready.");
        let _ = dict.insert("is_core_ready", false.to_variant());
        let _ = dict.insert("conductor_state", "Uninitialized".to_variant());
        let _ = dict.insert("total_chunks", 0.to_variant());
        let _ = dict.insert("completed_chunks", 0.to_variant());
        return dict;
    }

    // 1. Get the current state and metrics from the Conductor
    let conductor = &host_state.conductor;
    let metrics = conductor.get_metrics();
    let conductor_state = conductor.get_state_container().get_state();

    // 2. Map metrics into the Dictionary
    let _ = dict.insert("is_core_ready", host_state.is_core_ready.to_variant());
    let _ = dict.insert("conductor_state", conductor_state.to_string().to_variant());
    
    // Convert GenerationMetrics into Dictionary entries
    let _ = dict.insert("total_chunks", metrics.total_chunks.to_variant());
    let _ = dict.insert("completed_chunks", metrics.completed_chunks.to_variant());
    
    // Calculate progress as a float
    let progress = if metrics.total_chunks > 0 {
        metrics.completed_chunks as f64 / metrics.total_chunks as f64
    } else {
        0.0
    };
    let _ = dict.insert("progress", progress.to_variant());

    dict
}

// 🎯 FALLBACK: Provide a mock function for the CLI build to satisfy cross-module calls
#[cfg(not(feature = "godot-binding"))]
// Define a placeholder type for VarDictionary in the CLI context if needed, or use a simpler return type.
// Since the caller (SSXLConductor) is also gated, we can use a simpler signature.
pub fn get_status_report_dict(_host_state: &HostState) -> () {
    // No-op for the CLI build
    ()
}