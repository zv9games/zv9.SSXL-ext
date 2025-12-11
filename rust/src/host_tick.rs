// rust/SSXL-ext/src/host_tick.rs

use godot::prelude::*;
use crate::host_poller::poll_conductor_status;
// Assuming these are all correct and publicly available:
// FIX 1: Import the new get_host_state_mut function
use crate::host_state::{get_host_state, get_host_state_mut, HostState}; 
use crate::generate_conductor::GenerateConductor;
// The incorrect import is REMOVED: use crate::host_commands::handle_godot_command;
use crate::host_commands; // Keeping this for the later function call

// This macro registers the class with Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct SSXLConductor {
    base: Base<Node>,
    // Local reference to the HostState data structure. 
}

#[godot_api]
impl INode for SSXLConductor {
    // Constructor (called when Godot creates the node)
    fn init(base: Base<Node>) -> Self {
        godot_print!("SSXLConductor Node initialized in Godot.");

        Self {
            base,
        }
    }
    
    // The main execution point called on every frame.
    fn ready(&mut self) {
        self.base_mut().set_process(true); // Ensure _process is called
    }

    /// The Godot engine's main loop update function.
    /// This is where we execute the non-blocking polling logic.
    fn process(&mut self, _delta: f64) {
        // SAFETY: We assume 'get_host_state()' is initialized and only accessed 
        // on the main thread (Godot's promise for _process).
        // NOTE: Error handling for get_host_state should be implemented here for production code
        let host_state: &HostState = get_host_state().expect("HostState not initialized in _process");
        
        // Ensure the conductor is actually running tasks before polling
        if host_state.is_core_ready {
            let conductor: &GenerateConductor = &host_state.conductor;
            
            // --- The Ticking Heartbeat ---
            // Call the poller to check the channel and perform Direct Write if data exists.
            poll_conductor_status(conductor);
            
            // We could optionally also run the animation/sync rhythm here if needed.
            // crate::sync_rhythm::run_sync_check(); 
        }
    }
}

// rust/SSXL-ext/src/host_tick.rs (continued)

#[godot_api]
impl SSXLConductor {
    /// Public method called by GDScript to kick off generation.
    /// This wraps the host_commands::handle_start_command logic.
    #[func]
    fn start_generation(&mut self, target_tilemap: Gd<Node>) -> bool {
        // FIX 2: Use the new get_host_state_mut() function to safely obtain &mut HostState, 
        // cleaning up the previous UB-causing unsafe block.
        let host_state_mut = match get_host_state_mut() {
            Ok(state) => state,
            Err(e) => {
                godot_error!("HostState not ready for start_generation command: {:?}", e);
                return false;
            }
        };
        let tilemap_id = target_tilemap.instance_id();

        // The correct function call:
        // Pass the mutable reference obtained from get_host_state_mut()
        match host_commands::handle_start_command(host_state_mut, tilemap_id) {
            Ok(_) => true,
            Err(e) => {
                godot_error!("Failed to start generation: {:?}", e);
                false
            }
        }
    }

    /// Public method called by GDScript to get the current generation status.
    #[func]
    fn get_status_report(&self) -> Dictionary {
        // This will call into host_tilemap_status.rs
        // NOTE: We wrap the host_state access in a match/unwrap for safety
        match get_host_state() {
            Ok(state) => crate::host_tilemap_status::get_status_report_dict(state),
            Err(_) => Dictionary::new(),
        }
    }
}