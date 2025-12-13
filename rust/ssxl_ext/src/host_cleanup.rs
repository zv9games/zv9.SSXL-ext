// rust/SSXL-ext/src/host_cleanup.rs

use godot::prelude::*;
use crate::host_state::{HostState, HOST_SINGLETON}; // Combined import

/// The primary cleanup function called by Godot when the GDExtension is unloaded.
/// This ensures a clean exit by shutting down all background workers and state.
pub fn cleanup_ssxl_core() {
    godot_print!("SSXL-ext Core: Starting cleanup procedure.");

    // 1. Attempt to take ownership of the global state
    let taken_state = unsafe {
        // FIX 1: Provide explicit type annotation for the mutable raw pointer to resolve E0282.
        // We cast the immutable static reference to a mutable pointer of its known type.
        let host_singleton_mut: *mut once_cell::sync::OnceCell<Option<HostState>> = 
             &HOST_SINGLETON as *const _ as *mut _;
        
        // Dereference the mutable pointer and call take() on the mutable reference.
        (*host_singleton_mut).take()
    };
    
    // `HOST_SINGLETON.take()` returns Option<Option<HostState>> because of the definition in host_state.rs.
    match taken_state {
        Some(host_state_option) => {
            // FIX 2: Pattern match the inner Option to extract the HostState value.
            if let Some(host_state) = host_state_option {
                
                // 2. Safely dismantle the Conductor and its resources
                // FIX 3: Destructure HostState to take ownership of 'conductor' and ignore other non-Copy fields (Partial Move fix).
                let HostState { 
                    conductor, 
                    anim_conductor: _, // Take ownership but ignore the value if cleanup isn't done here
                    .. // Ignore all other fields
                } = host_state;

                // 3. Initiate Thread Pool Shutdown (The most critical step)
                godot_print!("Cleanup: Instructing Conductor to shut down worker threads...");
                conductor.shutdown(); // Assume a shutdown method exists on the Conductor

                // 4. Cleanup other Godot-related resources (e.g., cached InstanceIds, etc.)
                // Since the HostState struct is dropped here, all its contained fields 
                // (like Configs, TileMap IDs, etc.) are properly destroyed.
                
                godot_print!("SSXL-ext Core: Cleanup complete. Resources released.");
            } else {
                // If the inner Option was None (i.e., someone already cleaned up the value)
                godot_warn!("Cleanup called but HostState was already cleaned up (Inner Option::None).");
            }
        }
        None => {
            // If the outer Option was None (i.e., OnceCell was never set)
            godot_warn!("Cleanup called but SSXL-ext was not initialized or already cleaned up.");
        }
    }
}