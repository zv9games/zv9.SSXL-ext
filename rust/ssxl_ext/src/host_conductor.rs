// rust/SSXL-ext/src/host_conductor.rs (UNIFIED AND FIXED)

// 🎯 CRITICAL FIX: Gate all Godot imports
#[cfg(feature = "godot-binding")]
use godot::prelude::*;

#[cfg(feature = "godot-binding")]
use godot::classes::{Node, TileMap};

#[cfg(feature = "godot-binding")]
use godot::builtin::{GString, VarDictionary};

// --- Imports from host_tick.rs ---
// 🎯 FIX 2: Gate core logic imports that are only used within the Godot-bound SSXLConductor implementation.
#[cfg(feature = "godot-binding")]
use crate::host_poller::poll_conductor_status;
#[cfg(feature = "godot-binding")]
use crate::host_state::{get_host_state, get_host_state_mut, HostState};
#[cfg(feature = "godot-binding")]
use crate::generate_conductor::GenerateConductor;
#[cfg(feature = "godot-binding")]
use crate::host_commands;
#[cfg(feature = "godot-binding")]
use crate::host_tilemap_status;
// ---------------------------------

// ----------------------------------------------------
// 🎯 GDExtension Implementation
// ----------------------------------------------------

// 🎯 CRITICAL FIX: Add a non-gated struct/impl block to satisfy dependencies 
// in other files when godot-binding is disabled.
#[cfg(not(feature = "godot-binding"))]
pub struct SSXLConductor {}
#[cfg(not(feature = "godot-binding"))]
impl SSXLConductor {}

#[cfg(feature = "godot-binding")]
#[derive(GodotClass)]
#[class(base = Node)]
pub struct SSXLConductor {
    // Fields from the old host_conductor.rs (API State)
    tilemap_target: Option<Gd<TileMap>>,
    signal_target: Option<Gd<Node>>,
    active_generator_id: String,

    #[base]
    base: Base<Node>,
}

#[cfg(feature = "godot-binding")]
#[godot_api]
impl INode for SSXLConductor {
    fn init(base: Base<Node>) -> Self {
        crate::ssxl_info!("SSXLConductor initialized.");
        Self {
            tilemap_target: None,
            signal_target: None,
            active_generator_id: "none".to_owned(),
            base,
        }
    }
    
    fn ready(&mut self) {
        self.base_mut().set_process(true);
    }

    /// The Godot engine's main loop update function (from host_tick.rs).
    fn process(&mut self, _delta: f64) {
        let host_state: &HostState = match get_host_state() {
            Ok(state) => state,
            Err(e) => {
                crate::ssxl_error!(
                    "SSXL Process Error: HostState not initialized in _process: {:?}",
                    e
                );
                return;
            }
        };
        
        if host_state.is_core_ready {
            let conductor: &GenerateConductor = &host_state.conductor;
            poll_conductor_status(conductor);
        }
    }
}

#[cfg(feature = "godot-binding")]
#[godot_api]
impl SSXLConductor {
    // --- API Functions from host_conductor.rs ---
    
    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.tilemap_target = Some(tilemap);
        crate::ssxl_info!(
            "TileMap target successfully registered: {:?}",
            self.tilemap_target.as_ref().unwrap().instance_id()
        );
    }

    #[func]
    pub fn initialize_runtime_shell(&mut self, signal_receiver: Gd<Node>) {
        self.signal_target = Some(signal_receiver);
        crate::ssxl_info!("Runtime shell initialization requested. Signal target registered.");
    }

    #[func]
    pub fn set_generator(&mut self, id: GString) {
        self.active_generator_id = id.to_string();
        crate::ssxl_info!("Generator set to: {}", self.active_generator_id);
    }

    #[func]
    pub fn build_map(&mut self, config: VarDictionary) -> bool {
        crate::ssxl_info!("Received request to build map with config: {:?}", config);

        if self.tilemap_target.is_none() {
            crate::ssxl_error!("FATAL: Cannot build map. TileMap target is missing.");
            return false;
        }
        true
    }

    #[func]
    pub fn get_status(&self) -> GString {
        "Running - Waiting for build_map".into()
    }

    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        (&self.active_generator_id).into() 
    }

    #[func]
    pub fn oracle_tick(&self, _delta: f32) {
        if let Some(_tilemap) = &self.tilemap_target {
            // Placeholder for future oracle_tick logic
        }
    }

    #[func]
    pub fn get_metrics(&self) -> VarDictionary {
        VarDictionary::new()
    }

    // --- API Functions from host_tick.rs ---
    
    /// Public method called by GDScript to kick off generation.
    /// Uses a unified i64 ID model: InstanceId is converted to i64 and passed through.
    #[func]
    fn start_generation(&mut self, target_tilemap: Gd<Node>) -> bool {
        let host_state_mut = match get_host_state_mut() {
            Ok(state) => state,
            Err(e) => {
                crate::ssxl_error!(
                    "HostState not ready for start_generation command: {:?}",
                    e
                );
                return false;
            }
        };

        let tilemap_id = target_tilemap.instance_id();

        // Unified ID model: use i64 at the boundary, not u64.
        let tilemap_id_i64 = tilemap_id.to_i64();

        match host_commands::handle_start_command(host_state_mut, tilemap_id_i64) {
            Ok(_) => true,
            Err(e) => {
                crate::ssxl_error!("Failed to start generation: {:?}", e);
                false
            }
        }
    }

    /// Public method called by GDScript to get the current generation status.
    #[func]
    fn get_status_report(&self) -> VarDictionary {
        match get_host_state() {
            Ok(state) => host_tilemap_status::get_status_report_dict(state),
            Err(e) => {
                crate::ssxl_error!("Failed to get status report: {:?}", e);
                VarDictionary::new()
            }
        }
    }
}
