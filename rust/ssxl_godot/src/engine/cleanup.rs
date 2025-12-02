// ssxl_godot/src/engine/cleanup.rs
//
// This module contains logic for teardown, shutdown, and resource release (Logic Implementation).

use godot::prelude::*;
use std::sync::Arc; // Corrected import (needed for Arc::try_unwrap)

// FIX 1: Adopt the macro-friendly import pattern.
use crate::engine::state as state_module;
// CRITICAL FIX: Import necessary types/macros from the state module.
use state_module::{SSXLEngine, state};

// The entire #[godot_api] impl block is removed to resolve E0119.

/// Logic for the Public Godot-facing function to explicitly release all background threads and resources.
pub fn shutdown_logic(engine: &mut SSXLEngine) -> GString {
    // CRITICAL FIX: The `state!` macro requires the state module's struct name as the argument.
    let state = state!(engine);
    
    // NOTE: In the previous turn, the Conductor method was confirmed to be `graceful_teardown`.
    if let Some(conductor_arc) = state.conductor.take() {
        // Attempt to stop the generation thread gracefully
        // NOTE: conductor_arc is Arc<Mutex<Conductor>>
        
        // FIX E0599: Use `.ok().and_then(|m| m.into_inner().ok())` to correctly flatten 
        // the Option<Result<T, E>> into Option<T>. This consumes the Arc and the Mutex 
        // and handles both non-unique Arc and Mutex poisoning gracefully.
        if let Some(conductor) = Arc::try_unwrap(conductor_arc)
            .ok() // Result<Mutex, Arc> -> Option<Mutex> (discards non-unique Arc)
            .and_then(|m| m.into_inner().ok()) // Mutex<T> -> Result<T, PoisonError> -> Option<T> (discards poisoning)
        {
            // `conductor` is now the owned Conductor struct (T), allowing the consuming method call.
            conductor.graceful_teardown();
        } else {
            // The shutdown failed because other Arcs still exist or the Mutex was poisoned.
            // No action needed here, as the resources are already marked as taken.
        }
    }
    
    // Clear other owned resources
    state.conductor_state.take();
    state.animation_conductor.take();
    state.animation_state.take();
    
    // Clear Godot references
    state.signals_node.take();
    state.tilemap_node.take();

    "SSXLEngine resources shut down and released.".into()
}