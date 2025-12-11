// rust/SSXL-ext/src/lib.rs

use godot::prelude::*;
// FIX: Remove the unnecessary and conflicting import of InitHandle and DeinitHandle,
// as they are not used in the older trait methods (on_level_init/deinit).
use godot::init::{ExtensionLibrary, InitLevel}; 
// All attribute macro calls have been replaced by the trait implementation.

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

/// Placeholder struct required for the ExtensionLibrary trait.
struct SSXLExtension;

/// The GDExtension entry point, using the robust trait implementation.
#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    
    /// Called when Godot reaches a new initialization stage. 
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Core {
            // Class registration would happen here or rely on the build system's setup.
        }

        if level == InitLevel::Scene {
             // Call custom initialization routine.
            // This is equivalent to the body of the original #[init] function.
            match host_init::initialize_ssxl_core() {
                Ok(_) => ssxl_info!("Core resources successfully initialized and workers started."),
                Err(e) => {
                    // Logging macros are available directly (godot_error!).
                    godot_error!("FATAL: SSXL Core failed to initialize. Reason: {}", e);
                }
            }
        }
    }

    /// Called when the GDExtension library is about to be unloaded.
    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Call custom thread-safe cleanup routine.
            // This is equivalent to the body of the original #[deinit] function.
            host_cleanup::cleanup_ssxl_core();
            // Logging macros are available directly (godot_print!).
            godot_print!("SSXL GDExtension terminated successfully.");
        }
    }
}