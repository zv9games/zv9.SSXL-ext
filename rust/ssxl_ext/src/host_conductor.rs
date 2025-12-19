// ----------------------------------------------------
// Godot binding imports
// ----------------------------------------------------
#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::classes::{Node, TileMap};
#[cfg(feature = "godot-binding")]
use godot::builtin::{GString, VarDictionary};

// ----------------------------------------------------
// Internal engine imports
// ----------------------------------------------------
#[cfg(feature = "godot-binding")]
use crate::host_poller::{poll_conductor_status, ConductorEvents};
#[cfg(feature = "godot-binding")]
use crate::host_state::{get_host_state, HostState};
#[cfg(feature = "godot-binding")]
use crate::generate_conductor::GenerateConductor;

// ----------------------------------------------------
// Public re-export for other modules
// ----------------------------------------------------
#[cfg(feature = "godot-binding")]
pub use self::ssxl_conductor_impl::SSXLConductor;

// ----------------------------------------------------
// Stub for non-Godot builds
// ----------------------------------------------------
#[cfg(not(feature = "godot-binding"))]
pub struct SSXLConductor {}

#[cfg(not(feature = "godot-binding"))]
impl SSXLConductor {}

// ----------------------------------------------------
// Godot-facing conductor implementation
// ----------------------------------------------------
#[cfg(feature = "godot-binding")]
mod ssxl_conductor_impl {
    use super::*;

    #[derive(GodotClass)]
    #[class(base = Node)]
    pub struct SSXLConductor {
        pub tilemap_target: Option<Gd<TileMap>>,
        pub signal_target: Option<Gd<Node>>,
        pub active_generator_id: String,

        #[base]
        pub base: Base<Node>,
    }

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

        fn exit_tree(&mut self) {
            if let Err(e) = crate::host_cleanup::shutdown_ssxl_runtime() {
                crate::ssxl_error!("CRITICAL: Runtime cleanup failed during exit_tree: {:?}", e);
            }
            self.base_mut().set_process(false);
            crate::ssxl_info!("SSXLConductor terminated.");
        }

        fn process(&mut self, _delta: f64) {
            let host_state: &mut HostState = match get_host_state() {
                Ok(state) => state,
                Err(e) => {
                    crate::ssxl_error!(
                        "SSXL Process Error: HostState not initialized in _process: {:?}",
                        e
                    );

                    self.emit_generation_error(&format!(
                        "HostState not initialized in _process: {:?}",
                        e
                    ));
                    return;
                }
            };

            if host_state.is_core_ready {
                let conductor: &mut GenerateConductor = &mut host_state.conductor;
                let events: ConductorEvents = poll_conductor_status(conductor);
                self.poll_and_emit_signals(conductor, &events);
            } else {
                self.emit_conductor_idle();
                self.base_mut().set_process(false);
            }
        }
    }

    #[godot_api]
    impl SSXLConductor {
        // ----------------------------------------------------
        // ✅ Existing signals
        // ----------------------------------------------------
        #[signal]
        fn conductor_ready();

        #[signal]
        fn conductor_idle();

        #[signal]
        fn generation_started(tilemap_id: i64, total_chunks: i32);

        #[signal]
        fn chunk_rendered(completed: i32, total: i32);

        #[signal]
        fn chunk_failed(error: GString);

        #[signal]
        fn generation_progress(
            completed: i32,
            total: i32,
            metrics: VarDictionary,
        );

        #[signal]
        fn generation_finished(tilemap_id: i64);

        #[signal]
        fn generation_error(message: GString);

        #[signal]
        fn debug_event(message: GString);

        // ----------------------------------------------------
        // ✅ NEW: SSXL universal event bus signal (fixed)
        // ----------------------------------------------------
        #[signal]
        fn ssxl_event(event: VarDictionary);

        // ----------------------------------------------------
        // ✅ NEW: Test method for headless Godot integration (fixed)
        // ----------------------------------------------------
        #[func]
        pub fn test_emit_event(&mut self) {
            let mut d = VarDictionary::new();
            let _ = d.insert("type", "rust_test_event");
            let _ = d.insert("ok", true);

            self.base_mut().emit_signal("ssxl_event", &[d.to_variant()]);
        }

        // ----------------------------------------------------
        // Existing API methods
        // ----------------------------------------------------
        #[func]
        pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
            self.api_set_tilemap(tilemap);
        }

        #[func]
        pub fn initialize_runtime_shell(&mut self, signal_receiver: Gd<Node>) {
            self.api_initialize_runtime_shell(signal_receiver);
        }

        #[func]
        pub fn set_generator(&mut self, id: GString) {
            self.api_set_generator(id);
        }

        #[func]
        pub fn build_map(&mut self, config: VarDictionary) -> bool {
            self.api_build_map(config)
        }

        #[func]
        pub fn get_status(&self) -> GString {
            self.api_get_status()
        }

        #[func]
        pub fn get_active_generator_id(&self) -> GString {
            self.api_get_active_generator_id()
        }

        #[func]
        pub fn oracle_tick(&self, delta: f32) {
            self.api_oracle_tick(delta);
        }

        #[func]
        pub fn get_metrics(&self) -> VarDictionary {
            self.api_get_metrics()
        }

        #[func]
        pub fn start_generation(&mut self, target_tilemap: Gd<Node>) -> bool {
            self.api_start_generation(target_tilemap)
        }

        #[func]
        pub fn get_status_report(&self) -> VarDictionary {
            self.api_get_status_report()
        }
    }
}
