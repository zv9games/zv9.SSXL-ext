#[cfg(feature = "godot-binding")]
use godot::prelude::*;

#[cfg(feature = "godot-binding")]
use godot::init::{ExtensionLibrary, InitLevel};


pub mod tools;
pub mod math;
pub mod config;

pub mod cache;
pub mod sync_pool;
pub mod sync_rhythm;

pub mod shared_tile;
pub mod shared_chunk;
pub mod shared_config;
pub mod shared_error;
pub mod shared_math;
pub mod shared_message;
pub mod generate_task_queue;
pub mod shared_job;
pub mod generate_runtime;
pub mod generate_batch_processor;
pub mod generate_anim_conductor;
pub mod rhythm_manager;

pub mod generate_perlin;
pub mod generate_ca;
pub mod generate_ca_simulation;
pub mod generate_conductor;
pub mod generate_conductor_state;
pub mod generate_manager;

pub mod animate_events;
pub mod animate_worker;
pub mod animate_conductor;

pub mod bridge_ffi;
pub mod bridge_signals;
pub mod bridge_oracle;
pub mod host_conductor;
pub mod host_state;
pub mod host_init;
pub mod host_cleanup;
pub mod host_poller;
pub mod host_render;
pub mod host_commands;
pub mod host_tilemap;
pub mod host_tilemap_status;
pub mod tile_conversion;


#[cfg(feature = "godot-binding")]
struct SSXLExtension;

#[cfg(feature = "godot-binding")]
#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            match host_init::initialize_ssxl_core() {
                Ok(_) => crate::ssxl_info!("Core resources successfully initialized and workers started."),
                Err(e) => crate::ssxl_error!("FATAL: SSXL Core failed to initialize. Reason: {}", e),
            }
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            host_cleanup::cleanup_ssxl_core();
            crate::ssxl_info!("SSXL GDExtension terminated successfully.");
        }
    }
}
