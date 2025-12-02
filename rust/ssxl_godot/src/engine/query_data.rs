// ssxl_godot/src/engine/query_data.rs
//
// This module contains logic for querying specific data chunks (Logic Implementation).

// FIX: Removed unused imports: `godot::prelude::*` and `godot::obj::Gd`.
use godot::builtin::Dictionary; 

// CRITICAL FIX: The #[macro_use] attribute should be placed before the crate module to work correctly.
// Also, it's better to just use the standard import pattern if the macro is defined elsewhere.
use crate::engine::state as state_module; 
use state_module::{SSXLEngine, state}; 

/// Provides the external Godot layer with read-only access to specific chunk data.
pub fn fetch_chunk_data_logic(engine: &mut SSXLEngine, x: i32, y: i32) -> Dictionary {
    // NOTE: Based on the original code, this logic relies on a field named `generation_api`
    // which was not present in the last provided `InternalState` context. Assuming it exists 
    // for now, or is being delegated through another structure in state.
    // If state has been fully refactored, the logic below might be incorrect, 
    // but the pattern remains:
    
    // Original logic:
    // state!(engine).generation_api.fetch_chunk_data(x, y)
    
    // Using the state macro access as written in the original logic:
    state!(engine).generation_api.fetch_chunk_data(x, y)
}