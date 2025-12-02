use std::os::raw::c_char;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::panic;

use ssxl_generate::{Conductor, start_runtime_placeholder, ConductorProgressReceiver};
// This import will now resolve after fixing Cargo.toml
use tokio::sync::mpsc::error::TryRecvError;
use ssxl_shared::{initialize_shared_data, CHUNKS_COMPLETED_COUNT};
use tracing::{info, error};

static CONDUCTOR: OnceLock<Mutex<Conductor>> = OnceLock::new();
static PROGRESS_RECEIVER: OnceLock<Mutex<ConductorProgressReceiver>> = OnceLock::new();

// ------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    // Panic guard applied to maintain FFI boundary stability
    match panic::catch_unwind(|| {
        initialize_shared_data();

        if CONDUCTOR.get().is_some() {
            info!("FFI Bridge: Runtime already running.");
            return true;
        }

        match Conductor::new(None) {
            Ok((conductor, _state, _request_sender, progress_receiver)) => {
                
                // 1. Store Conductor
                if CONDUCTOR.set(Mutex::new(conductor)).is_err() {
                    error!("FFI Bridge: Conductor::set() failed (Possible race condition).");
                    return false;
                }
                
                // 2. Store the Channel Receiver, keeping the channel open.
                if PROGRESS_RECEIVER.set(Mutex::new(progress_receiver)).is_err() {
                    error!("FFI Bridge: PROGRESS_RECEIVER::set() failed (Possible race condition).");
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
    }) {
        Ok(result) => result,
        Err(_) => {
            error!("FFI BRIDGE FATAL: A panic occurred inside ssxl_start_runtime. Preventing crash.");
            false
        }
    }
}

// ------------------------------------------------------------------

// CORE FIX: The non-blocking polling function that Godot must call.
#[no_mangle]
pub extern "C" fn ssxl_poll_progress_message(
    buffer: *mut c_char,
    buffer_len: usize,
) -> isize {
    let receiver_mutex = match PROGRESS_RECEIVER.get() {
        Some(m) => m,
        None => return -1, // Runtime not initialized
    };

    let mut receiver_guard = match receiver_mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return -2, // Lock failed
    };
    
    // Use try_recv() for a non-blocking poll.
    match receiver_guard.try_recv() {
        Ok(message) => {
            // Using Debug formatting for message serialization.
            let status = format!("{:?}", message); 

            let bytes = status.as_bytes();
            let write_len = bytes.len().min(buffer_len.saturating_sub(1));

            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, write_len);
                *buffer.add(write_len) = 0; // Null-terminate the string
            }
            // Return the written length (positive)
            write_len as isize
        }
        Err(TryRecvError::Empty) => {
            0 // Return 0 to signal no message was received this tick
        }
        Err(TryRecvError::Disconnected) => {
            error!("FFI Bridge: Progress channel disconnected. Clearing static.");
            
            // Channel is permanently dead. Clear the static to reset the state.
            unsafe {
                let ptr = &PROGRESS_RECEIVER as *const _ as *mut OnceLock<Mutex<ConductorProgressReceiver>>;
                (*ptr).take();
            }
            -3 // Return a negative error code for permanent failure
        }
    }
}

// ------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn ssxl_shutdown_runtime() {
    // Panic guard applied to maintain FFI boundary stability
    match panic::catch_unwind(|| {
        // 1. Signal graceful shutdown to the Conductor thread
        if let Some(mutex) = CONDUCTOR.get() {
            if let Ok(conductor) = mutex.lock() {
                conductor.signal_shutdown_graceful();
                info!("FFI Bridge: Conductor Runtime signalled for shutdown.");
            } else {
                error!("FFI Bridge: Failed to acquire lock for shutdown (Possible poisoned lock).");
            }
        }
        
        // 2. Explicitly drop the receiver side (closes the channel for the workers)
        unsafe {
            let ptr = &PROGRESS_RECEIVER as *const _ as *mut OnceLock<Mutex<ConductorProgressReceiver>>;
            (*ptr).take();
        }
        info!("FFI Bridge: Progress Receiver dropped.");
    }) {
        Ok(_) => {},
        Err(_) => {
            error!("FFI BRIDGE FATAL: Panic during ssxl_shutdown_runtime.");
        }
    }
}

// ------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn ssxl_is_runtime_ready() -> bool {
    CONDUCTOR.get().is_some()
}

#[no_mangle]
pub extern "C" fn ssxl_is_receiver_ready() -> bool {
    PROGRESS_RECEIVER.get().is_some()
}

#[no_mangle]
pub extern "C" fn ssxl_initialize_engine() -> bool {
    ssxl_start_runtime()
}

// ------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn ssxl_trigger_runtime_test() {
    match panic::catch_unwind(|| {
        info!("FFI Bridge: Received command to trigger Conductor structural test.");
        start_runtime_placeholder();
        info!("FFI Bridge: Conductor test sequence complete.");
    }) {
        Ok(_) => {},
        Err(_) => {
            error!("FFI BRIDGE FATAL: Panic during runtime test. Check start_runtime_placeholder.");
        }
    }
}

// ------------------------------------------------------------------

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

    if buffer.is_null() || buffer_len == 0 {
        return -1;
    }

    let bytes = status.as_bytes();
    let write_len = bytes.len().min(buffer_len.saturating_sub(1));

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, write_len);
        *buffer.add(write_len) = 0;
    }

    write_len as isize
}

// ------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn ssxl_get_chunks_completed() -> u32 {
    CHUNKS_COMPLETED_COUNT.load(Ordering::Relaxed) as u32
}