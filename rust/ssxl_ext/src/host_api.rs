// src/host_api.rs

#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::builtin::{GString, VarDictionary};
#[cfg(feature = "godot-binding")]
use godot::classes::Node;

#[cfg(feature = "godot-binding")]
use crate::export_api;
#[cfg(feature = "godot-binding")]
use crate::host_state::{get_host_state, get_host_state_mut};
#[cfg(feature = "godot-binding")]
use crate::host_commands;
#[cfg(feature = "godot-binding")]
use crate::host_tilemap_status;
#[cfg(feature = "godot-binding")]
use crate::host_conductor::SSXLConductor;
#[cfg(feature = "godot-binding")]
use crate::generate_conductor_state::ConductorState;
#[cfg(feature = "godot-binding")]
use crate::ssxl_tilemap::SSXLTileMap;

#[cfg(feature = "godot-binding")]
impl SSXLConductor {
    // ----------------------------------------------------
    // Internal API helpers (called by #[func] wrappers)
    // ----------------------------------------------------

    pub fn api_set_tilemap(&mut self, tilemap: Gd<Node>) {
        export_api!("set_tilemap(tilemap: Node)");

        if let Ok(_ssxl_tm) = tilemap.clone().try_cast::<SSXLTileMap>() {
            self.tilemap_target = Some(tilemap.clone());
            crate::ssxl_info!(
                "TileMap target successfully registered: {:?}",
                self.tilemap_target
                    .as_ref()
                    .unwrap()
                    .instance_id()
            );
        } else {
            crate::ssxl_error!(
                "api_set_tilemap: Expected SSXLTileMap, but received a different node type."
            );
            self.emit_generation_error(
                "set_tilemap failed: target must be an SSXLTileMap node."
            );
        }
    }

    pub fn api_initialize_runtime_shell(&mut self, signal_receiver: Gd<Node>) {
        export_api!("initialize_runtime_shell(signal_receiver: Node)");
        self.signal_target = Some(signal_receiver);
        crate::ssxl_info!("Runtime shell initialization requested. Signal target registered.");
    }

    pub fn api_set_generator(&mut self, id: GString) {
        export_api!("set_generator(id: String)");
        self.active_generator_id = id.to_string();
        crate::ssxl_info!("Generator set to: {}", self.active_generator_id);
    }

    pub fn api_build_map(&mut self, config: VarDictionary) -> bool {
        export_api!("build_map(config: Dictionary)");
        crate::ssxl_info!(
            "Received request to build map with config: {:?}",
            config
        );

        if self.tilemap_target.is_none() {
            crate::ssxl_error!("FATAL: Cannot build map. TileMap target is missing.");
            self.emit_generation_error("Cannot build map: TileMap target is missing.");
            return false;
        }

        // ✅ Extract width/height from Godot config and store in HostState
        match get_host_state_mut() {
            Ok(host_state) => {
                if let Some(w) = config.get("width").and_then(|v| v.try_to::<i64>().ok()) {
                    host_state.world_width = w as i32;
                }
                if let Some(h) = config.get("height").and_then(|v| v.try_to::<i64>().ok()) {
                    host_state.world_height = h as i32;
                }

                crate::ssxl_info!(
                    "World size set from Godot: {} x {}",
                    host_state.world_width,
                    host_state.world_height
                );

                // ✅ Mark conductor as Ready
                let conductor = &host_state.conductor;
                let state_container = conductor.get_state_container();
                state_container.transition_to(ConductorState::Ready);

                crate::ssxl_info!("Conductor state set to Ready after build_map().");
            }
            Err(e) => {
                crate::ssxl_error!(
                    "HostState not available during build_map; cannot set conductor Ready: {:?}",
                    e
                );
                self.emit_generation_error(&format!(
                    "HostState not available during build_map: {:?}",
                    e
                ));
                return false;
            }
        }

        true
    }

    // ----------------------------------------------------
    // ✅ Real conductor state returned to Godot
    // ----------------------------------------------------

    pub fn api_get_status(&self) -> GString {
        export_api!("get_status() -> String");

        match get_host_state() {
            Ok(state) => {
                let conductor = &state.conductor;
                let real_state = conductor.get_state_container().get_state();
                GString::from(&format!("{}", real_state))
            }
            Err(e) => {
                crate::ssxl_error!(
                    "Failed to retrieve HostState in get_status(): {:?}",
                    e
                );
                GString::from("Error")
            }
        }
    }

    pub fn api_get_active_generator_id(&self) -> GString {
        export_api!("get_active_generator_id() -> String");
        (&self.active_generator_id).into()
    }

    pub fn api_oracle_tick(&self, _delta: f32) {
        export_api!("oracle_tick(delta: float)");
        if self.tilemap_target.is_some() {
            // Placeholder for future oracle_tick logic
        }
    }

    pub fn api_get_metrics(&self) -> VarDictionary {
        export_api!("get_metrics() -> Dictionary");
        VarDictionary::new()
    }

    pub fn api_start_generation(&mut self, target_tilemap: Gd<Node>) -> bool {
        export_api!("start_generation(target_tilemap: Node) -> bool");

        let host_state_mut = match get_host_state_mut() {
            Ok(state) => state,
            Err(e) => {
                crate::ssxl_error!(
                    "HostState not ready for start_generation command: {:?}",
                    e
                );
                self.emit_generation_error(&format!(
                    "HostState not ready for start_generation: {:?}",
                    e
                ));
                return false;
            }
        };

        let tilemap_id = target_tilemap.instance_id().to_i64();

        match host_commands::handle_start_command(host_state_mut, tilemap_id) {
            Ok(_) => {
                self.emit_generation_started(tilemap_id, 0);
                true
            }
            Err(e) => {
                crate::ssxl_error!("Failed to start generation: {:?}", e);
                self.emit_generation_error(&format!(
                    "Failed to start generation: {:?}",
                    e
                ));
                false
            }
        }
    }

    pub fn api_get_status_report(&self) -> VarDictionary {
        export_api!("get_status_report() -> Dictionary");

        match get_host_state() {
            Ok(state) => host_tilemap_status::get_status_report_dict(state),
            Err(e) => {
                crate::ssxl_error!("Failed to get status report: {:?}", e);
                VarDictionary::new()
            }
        }
    }
}
