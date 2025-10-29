// ssxl_engine_ffi/src/lib.rs (Functional Fix)

//! Low-level C FFI Bridge for Godot Communication.
//!
//! This module exposes functions to C/Godot that manage the SSXL runtime lifecycle
//! and facilitate the transfer of generated chunk data.

use std::ffi::CString;
use std::sync::OnceLock;

// --- EXTERNAL CRATE DEPENDENCIES ---
use ssxl_generate::{Conductor, start_runtime_placeholder};
// FIX: Removed unused import `ssxl_math::Vec2i`
use ssxl_shared::initialize_shared_data;
use tracing::{info, error};

// --- 1. Conductor Management (Singleton State) ---

/// A thread-safe, lazily initialized global instance of the Conductor.
static CONDUCTOR: OnceLock<Conductor> = OnceLock::new();

/// Initializes the Aetherion Runtime (Conductor) and stores it globally.
#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    // ... [Function Body Remains Unchanged]
    initialize_shared_data(); 

    if CONDUCTOR.get().is_some() {
        info!("FFI Bridge: Runtime already running.");
        return true;
    }

    match Conductor::new(None) {
        Ok((conductor, _state, _receiver)) => {
            if CONDUCTOR.set(conductor).is_err() {
                error!("FFI Bridge: Conductor::set() failed (Possible race condition).");
                return false;
            }
            info!("FFI Bridge: Conductor Runtime started successfully.");
            true
        }
        Err(e) => {
            tracing::error!("FFI Bridge: Failed to initialize Conductor: {:?}", e);
            false
        }
    }
}

/// Gracefully shuts down the Conductor by signaling its internal state.
#[no_mangle]
pub extern "C" fn ssxl_shutdown_runtime() {
    if let Some(conductor) = CONDUCTOR.get() {
        conductor.signal_shutdown_graceful();
        info!("FFI Bridge: Conductor Runtime signalled for shutdown.");
    }
}

/// Checks if the engine's asynchronous core has been successfully initialized.
#[no_mangle]
pub extern "C" fn ssxl_is_runtime_ready() -> bool {
    CONDUCTOR.get().is_some()
}

// --- 2. Generation Bridge (If applicable, call internal logic here) ---

// --- 3. Compatibility and Utility Functions (Kept for CLI) ---

/// Triggers the structural test of the Conductor (used by CLI menu).
#[no_mangle]
pub extern "C" fn ssxl_trigger_runtime_test() {
    info!("FFI Bridge: Received command to trigger Conductor structural test.");
    start_runtime_placeholder();
    info!("FFI Bridge: Conductor test sequence complete.");
}

#[no_mangle]
pub extern "C" fn ssxl_initialize_engine() -> bool {
    ssxl_start_runtime()
}

#[no_mangle]
pub extern "C" fn ssxl_get_status(id: u32) -> *mut std::os::raw::c_char {
    let status = format!(
        "Engine status for id {}: Runtime Running: {}",
        id,
        CONDUCTOR.get().is_some()
    );
    match CString::new(status) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("Error: Invalid Status String").unwrap().into_raw(),
    }
}

/// FFI function to free strings allocated by Rust.
#[no_mangle]
pub extern "C" fn ssxl_free_string(s: *mut std::os::raw::c_char) {
    unsafe {
        if s.is_null() { return }
        let _ = CString::from_raw(s);
    }
}