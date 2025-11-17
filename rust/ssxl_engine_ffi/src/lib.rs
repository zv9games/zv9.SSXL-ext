// ssxl_engine_ffi/src/lib.rs

//! Foreign Function Interface (FFI) Bridge for the SSXL Engine.
//!
//! This module exposes C-compatible functions to external host environments (like Godot's
//! GDExtension), allowing them to start, stop, and query the SSXL procedural generation
//! runtime, which is managed by the thread-safe `Conductor` singleton.

// REMOVED: use std::ffi::CString; // FIX: Unused import removed (no longer needed for ssxl_get_status pattern)
use std::os::raw::c_char; // FIX: c_void removed as it is not used in public FFI signatures.
use std::sync::OnceLock;
// FIX: Removed the redundant import 'use std::os::raw::c_char;' (and the non-breaking space).

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

/// **[NEW SAFE FUNCTION]** Writes a formatted status string directly into a C-owned buffer.
///
/// This eliminates the memory leak risk by having the calling environment (Godot)
/// allocate and own the memory, removing the need for an explicit Rust deallocation FFI call.
///
/// # Safety/FFI Contract
/// 1. The caller must ensure `buffer` is a valid, non-null pointer.
/// 2. The caller must ensure `buffer_len` accurately reflects the allocated size.
///
/// # Arguments
/// * `buffer`: A pointer to the C-allocated buffer where the string will be written.
/// * `buffer_len`: The maximum size (in bytes) of the buffer.
/// * `id`: A simple identifier for the status query.
///
/// # Returns
/// The number of bytes written to the buffer (excluding the null terminator),
/// or a negative value (`-1`) on failure (e.g., null buffer).
#[no_mangle]
pub extern "C" fn ssxl_write_status(
    buffer: *mut c_char,
    buffer_len: usize,
    id: u32,
) -> isize {
    let status = format!(
        "Engine status for id {}: Runtime Running: {}",
        id,
        CONDUCTOR.get().is_some()
    );

    // Critical safety checks before touching the raw pointer
    if buffer.is_null() || buffer_len == 0 {
        return -1;
    }

    let bytes = status.as_bytes();
    // Calculate the maximum number of bytes we can copy, leaving 1 for the null terminator.
    let write_len = bytes.len().min(buffer_len.saturating_sub(1));

    unsafe {
        // 1. Copy the status bytes into the C-owned memory.
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, write_len);
        // 2. CRITICAL: Null-terminate the string for C/GDScript compatibility.
        *buffer.add(write_len) = 0;
    }

    write_len as isize
}

// **[REMOVED]** ssxl_get_status (Used CString::into_raw(), requiring external free)
// **[REMOVED]** ssxl_free_string (The required cleanup function for ssxl_get_status)