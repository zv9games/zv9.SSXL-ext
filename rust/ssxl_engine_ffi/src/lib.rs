// ssxl_engine_ffi/src/lib.rs

//! Foreign Function Interface (FFI) Bridge for the SSXL Engine.
//!
//! This module exposes C-compatible functions to external host environments (like Godot's
//! GDExtension), allowing them to start, stop, and query the SSXL procedural generation
//! runtime, which is managed by the thread-safe `Conductor` singleton.

use std::ffi::CString;
use std::sync::OnceLock;
use std::os::raw::c_char; // Explicitly import c_char for FFI clarity

use ssxl_generate::{Conductor, start_runtime_placeholder};
use ssxl_shared::initialize_shared_data;
use tracing::{info, error};

// --- 1. Thread-Safe Singleton for the Conductor ---

/// A thread-safe, one-time initialization container for the core Conductor runtime.
/// Using OnceLock ensures that the Conductor is initialized exactly once, protecting
/// against initialization race conditions across different FFI calls.
static CONDUCTOR: OnceLock<Conductor> = OnceLock::new();

// --- 2. FFI Functions for Runtime Management ---

/// Starts the SSXL Conductor runtime if it is not already running.
///
/// This function is idempotent: calling it multiple times will only initialize the
/// runtime once. It is the primary entry point for the external engine.
///
/// # Safety/FFI
/// Marked `extern "C"` and `#[no_mangle]` for C-ABI compatibility.
///
/// # Returns
/// `true` if the runtime is running (either started now or was already running),
/// `false` if initialization failed.
#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    // Ensure core engine configuration data is initialized first.
    initialize_shared_data();

    if CONDUCTOR.get().is_some() {
        info!("FFI Bridge: Runtime already running.");
        return true;
    }

    // Attempt to initialize and set the Conductor singleton.
    match Conductor::new(None) {
        Ok((conductor, _state, _progress_receiver, _request_sender)) => {
            if CONDUCTOR.set(conductor).is_err() {
                // Should only happen in a severe race condition during first initialization.
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

/// Signals the Conductor to begin a graceful shutdown sequence.
///
/// The FFI consumer should call this before unloading the DLL/shared library.
/// This prevents systemic entropy by allowing worker threads to finish.
#[no_mangle]
pub extern "C" fn ssxl_shutdown_runtime() {
    if let Some(conductor) = CONDUCTOR.get() {
        // The conductor's destructor will handle waiting for threads if necessary.
        conductor.signal_shutdown_graceful();
        info!("FFI Bridge: Conductor Runtime signalled for shutdown.");
    }
}

/// Checks if the Conductor runtime has been successfully initialized.
///
/// # Returns
/// `true` if the Conductor is initialized and accessible, `false` otherwise.
#[no_mangle]
pub extern "C" fn ssxl_is_runtime_ready() -> bool {
    CONDUCTOR.get().is_some()
}

/// A convenience alias for the primary initialization function.
#[no_mangle]
pub extern "C" fn ssxl_initialize_engine() -> bool {
    ssxl_start_runtime()
}

// --- 3. FFI Functions for Diagnostics and Debugging ---

/// Triggers a structural test sequence within the Conductor's runtime manager.
/// This is typically used for integration testing or initial smoke checks.
#[no_mangle]
pub extern "C" fn ssxl_trigger_runtime_test() {
    info!("FFI Bridge: Received command to trigger Conductor structural test.");
    start_runtime_placeholder();
    info!("FFI Bridge: Conductor test sequence complete.");
}

/// Retrieves a formatted status string from the engine.
///
/// # Safety/FFI Contract
/// The calling environment is responsible for calling `ssxl_free_string` on the
/// returned raw pointer to prevent a memory leak, as the string is allocated
/// on the Rust heap.
///
/// # Returns
/// A raw C-style string pointer (`*mut c_char`).
#[no_mangle]
pub extern "C" fn ssxl_get_status(id: u32) -> *mut c_char {
    let status = format!(
        "Engine status for id {}: Runtime Running: {}",
        id,
        CONDUCTOR.get().is_some()
    );
    match CString::new(status) {
        // Consume the CString, transferring ownership of the raw pointer to C.
        Ok(c_string) => c_string.into_raw(),
        // Return a safe error string on failure (e.g., status contained a null byte).
        Err(_) => CString::new("Error: Invalid Status String").unwrap().into_raw(),
    }
}

/// Frees a string pointer that was allocated by Rust and passed to C.
///
/// This function fulfills the FFI memory contract. It takes ownership of the
/// raw pointer and reconstructs the CString, which then frees the memory
/// when it goes out of scope.
///
/// # Safety
/// The caller must ensure that `s` is a valid pointer that was originally
/// returned by a Rust FFI function (like `ssxl_get_status`).
#[no_mangle]
pub extern "C" fn ssxl_free_string(s: *mut c_char) {
    unsafe {
        // Check for null pointer defensively.
        if s.is_null() { return }
        // Reconstruct CString from the raw pointer, taking ownership.
        // When `_` drops, the memory is safely deallocated.
        let _ = CString::from_raw(s);
    }
}