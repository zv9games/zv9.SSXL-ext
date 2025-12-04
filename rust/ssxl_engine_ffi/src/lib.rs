use std::os::raw::c_char;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Mutex;
use std::panic;

use once_cell::sync::OnceCell;
use bincode;

use ssxl_generate::{Conductor, start_runtime_placeholder, ConductorProgressReceiver};
use tokio::sync::mpsc::error::TryRecvError;
use ssxl_shared::{initialize_shared_data, CHUNKS_COMPLETED_COUNT};

use tracing::{info, error, Level, span};

static CONDUCTOR: OnceCell<Mutex<Conductor>> = OnceCell::new();
static PROGRESS_RECEIVER: OnceCell<Mutex<ConductorProgressReceiver>> = OnceCell::new();

static INIT_RUNNING: AtomicBool = AtomicBool::new(false);
static INIT_SUCCESSFUL: AtomicBool = AtomicBool::new(false);

const FFI_ERR_RUNTIME_NOT_INIT: isize = -1;
const FFI_ERR_LOCK_FAILED: isize = -2;
const FFI_ERR_CHANNEL_DISCONNECTED: isize = -3;
const FFI_ERR_SERIALIZATION_FAILED: isize = -4;
const FFI_ERR_BUFFER_TOO_SMALL: isize = -5;
const FFI_ERR_EMPTY_MESSAGE: isize = -6;

#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    if INIT_SUCCESSFUL.load(Ordering::Relaxed) {
        info!("FFI Bridge: Runtime already running.");
        return true;
    }

    if INIT_RUNNING.swap(true, Ordering::Acquire) {
        return INIT_SUCCESSFUL.load(Ordering::Relaxed);
    }

    let result = panic::catch_unwind(|| {
        let span = span!(Level::INFO, "ssxl_start_runtime_panic_guard");
        let _guard = span.enter();

        initialize_shared_data();

        if CONDUCTOR.get().is_some() {
            info!("FFI Bridge: Conductor already set. Setting success flag.");
            INIT_SUCCESSFUL.store(true, Ordering::Release);
            return true;
        }

        match Conductor::new(None) {
            Ok((conductor, _state, _request_sender, progress_receiver)) => {

                if CONDUCTOR.set(Mutex::new(conductor)).is_err() {
                    error!("FFI Bridge: Conductor::set() failed (Possible race condition).");
                    return false;
                }

                let progress_receiver_ffi = ConductorProgressReceiver::new(progress_receiver);

                if PROGRESS_RECEIVER.set(Mutex::new(progress_receiver_ffi)).is_err() {
                    error!("FFI Bridge: PROGRESS_RECEIVER::set() failed (Possible race condition).");
                    return false;
                }

                info!("FFI Bridge: Conductor Runtime started successfully.");
                INIT_SUCCESSFUL.store(true, Ordering::Release);
                true
            }
            Err(e) => {
                error!("FFI Bridge: Failed to initialize Conductor: {:?}", e);
                false
            }
        }
    });

    INIT_RUNNING.store(false, Ordering::Release);

    match result {
        Ok(success) => success,
        Err(_) => {
            error!("FFI BRIDGE FATAL: A panic occurred inside ssxl_start_runtime. Preventing crash.");
            INIT_SUCCESSFUL.store(false, Ordering::Release);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn ssxl_poll_progress_message(
    buffer: *mut u8,
    buffer_len: usize,
) -> isize {
    if buffer.is_null() || buffer_len == 0 {
        return FFI_ERR_BUFFER_TOO_SMALL;
    }

    let receiver_mutex = match PROGRESS_RECEIVER.get() {
        Some(m) => m,
        None => return FFI_ERR_RUNTIME_NOT_INIT,
    };

    let mut receiver_guard = match receiver_mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return FFI_ERR_LOCK_FAILED,
    };

    match receiver_guard.rx.try_recv() {
        Ok(message) => {

            let bytes = match bincode::serialize(&message) {
                Ok(b) => b,
                Err(e) => {
                    error!("FFI Bridge: Bincode serialization failed: {:?}", e);
                    return FFI_ERR_SERIALIZATION_FAILED;
                }
            };

            let write_len = bytes.len();

            if write_len > buffer_len {
                error!("FFI Bridge: Buffer too small for message ({} > {}).", write_len, buffer_len);
                return FFI_ERR_BUFFER_TOO_SMALL;
            }

            if write_len == 0 {
                return FFI_ERR_EMPTY_MESSAGE;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, write_len);
            }
            write_len as isize
        }
        Err(TryRecvError::Empty) => {
            0
        }
        Err(TryRecvError::Disconnected) => {
            error!("FFI Bridge: Progress channel disconnected.");
            FFI_ERR_CHANNEL_DISCONNECTED
        }
    }
}

#[no_mangle]
pub extern "C" fn ssxl_shutdown_runtime() {
    match panic::catch_unwind(|| {
        let span = span!(Level::INFO, "ssxl_shutdown_runtime_panic_guard");
        let _guard = span.enter();

        if let Some(mutex) = CONDUCTOR.get() {
            if let Ok(conductor) = mutex.lock() {
                conductor.signal_shutdown_graceful();
                info!("FFI Bridge: Conductor Runtime signalled for shutdown.");
            } else {
                error!("FFI Bridge: Failed to acquire lock for shutdown (Possible poisoned lock).");
            }
        }

        INIT_SUCCESSFUL.store(false, Ordering::Release);

    }) {
        Ok(_) => {},
        Err(_) => {
            error!("FFI BRIDGE FATAL: Panic during ssxl_shutdown_runtime.");
        }
    }
}

#[no_mangle]
pub extern "C" fn ssxl_is_runtime_ready() -> bool {
    INIT_SUCCESSFUL.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn ssxl_is_receiver_ready() -> bool {
    PROGRESS_RECEIVER.get().is_some()
}

#[no_mangle]
pub extern "C" fn ssxl_initialize_engine() -> bool {
    ssxl_start_runtime()
}

#[no_mangle]
pub extern "C" fn ssxl_trigger_runtime_test() {
    match panic::catch_unwind(|| {
        let span = span!(Level::INFO, "ssxl_trigger_runtime_test_panic_guard");
        let _guard = span.enter();
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

#[no_mangle]
pub extern "C" fn ssxl_write_status(
    buffer: *mut c_char,
    buffer_len: usize,
    id: u32,
) -> isize {
    if buffer.is_null() || buffer_len == 0 {
        return -1;
    }

    let status = format!(
        "Engine status for id {}: Runtime Running: {}",
        id,
        INIT_SUCCESSFUL.load(Ordering::Relaxed)
    );

    let bytes = status.as_bytes();
    let write_len = bytes.len().min(buffer_len.saturating_sub(1));

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, write_len);
        *buffer.add(write_len) = 0;
    }

    write_len as isize
}

#[no_mangle]
pub extern "C" fn ssxl_get_chunks_completed() -> u32 {
    CHUNKS_COMPLETED_COUNT.load(Ordering::Relaxed) as u32
}