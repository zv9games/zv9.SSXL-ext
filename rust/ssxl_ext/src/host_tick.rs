use godot::prelude::*;
use crate::host_poller::poll_conductor_status;
use crate::host_state::{get_host_state, get_host_state_mut, HostState};
use crate::generate_conductor::GenerateConductor;
use crate::host_commands;
use crate::host_tilemap_status;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct SSXLConductor {
    base: Base<Node>,
}

#[godot_api]
impl INode for SSXLConductor {
    fn init(base: Base<Node>) -> Self {
        godot_print!("SSXLConductor Node initialized in Godot.");

        Self {
            base,
        }
    }
    
    fn ready(&mut self) {
        self.base_mut().set_process(true);
    }

    /// The Godot engine's main loop update function.
    fn process(&mut self, _delta: f64) {
        let host_state: &HostState = get_host_state().expect("HostState not initialized in _process");
        
        if host_state.is_core_ready {
            let conductor: &GenerateConductor = &host_state.conductor;
            
            // Call the poller to check the channel and perform Direct Write if data exists.
            poll_conductor_status(conductor);
        }
    }
}

#[godot_api]
impl SSXLConductor {
    /// Public method called by GDScript to kick off generation.
    #[func]
    fn start_generation(&mut self, target_tilemap: Gd<Node>) -> bool {
        let host_state_mut = match get_host_state_mut() {
            Ok(state) => state,
            Err(e) => {
                godot_error!("HostState not ready for start_generation command: {:?}", e);
                return false;
            }
        };
        let tilemap_id = target_tilemap.instance_id();

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
    fn get_status_report(&self) -> VarDictionary {
        match get_host_state() {
            Ok(state) => host_tilemap_status::get_status_report_dict(state),
            Err(_) => VarDictionary::new(),
        }
    }
}