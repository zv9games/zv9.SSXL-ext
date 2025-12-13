use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel};

// ----------------------------------------------------
// 1. PURE RUST CORE MODULES
// ----------------------------------------------------

pub mod tools;
pub mod math;
pub mod config;

// Concurrency and Caching
pub mod cache;
pub mod sync_pool;
pub mod sync_rhythm;

// ----------------------------------------------------
// 2. SHARED DATA CONTRACTS
// ----------------------------------------------------

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

// ----------------------------------------------------
// 3. GENERATION SUBSYSTEM
// ----------------------------------------------------

pub mod generate_perlin;
pub mod generate_ca;
pub mod generate_ca_simulation;
pub mod generate_conductor;
pub mod generate_conductor_state;
// ✅ CRITICAL FIX: Add the file containing the GenerationManager struct and FFI exports 1 & 2.
pub mod generate_manager; 
// ✅ CRITICAL FIX: Add the file containing the final bitmask FFI export (ssxl_ext_bitmask_to_id).
// Assuming you placed it in a file named generate_utils.rs, which is common for such utilities.
pub mod generate_utils; 

// ----------------------------------------------------
// 4. ANIMATION / SIMULATION SUBSYSTEM
// ----------------------------------------------------

pub mod animate_events;
pub mod animate_worker;
pub mod animate_conductor;

// ----------------------------------------------------
// 5. THE HOST / BRIDGE LAYER (GDExtension Interface)
// ----------------------------------------------------

pub mod bridge_ffi;
pub mod bridge_signals;
pub mod bridge_oracle;

// Lifecycle and State
pub mod host_state;
pub mod host_init;
pub mod host_cleanup;

// Godot Loop Integration
pub mod host_tick;
pub mod host_poller;
pub mod host_render;

// Godot TileMap and API Interaction
pub mod host_commands;
pub mod host_tilemap;
pub mod host_tilemap_status;

// ----------------------------------------------------
// 6. GDExtension ENTRY POINTS
// ----------------------------------------------------

struct SSXLExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Core {
        }

        if level == InitLevel::Scene {
            match host_init::initialize_ssxl_core() {
                Ok(_) => ssxl_info!("Core resources successfully initialized and workers started."),
                Err(e) => {
                    godot_error!("FATAL: SSXL Core failed to initialize. Reason: {}", e);
                }
            }
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            host_cleanup::cleanup_ssxl_core();
            godot_print!("SSXL GDExtension terminated successfully.");
        }
    }
}