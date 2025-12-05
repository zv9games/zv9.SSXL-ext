// ssxl_engine_ffi/src/lib.rs — SSXL-ext 9.1 — FINAL FIXED VERSION
// Works perfectly for both CLI (ssxl.exe) and Godot (ssxl_godot.dll)

use std::sync::atomic::{AtomicBool, Ordering};
// HIGH: Replaced external 'once_cell::sync::OnceCell' with standard library 'OnceLock' (Rust 1.70+)
use std::sync::{Mutex, OnceLock}; 
use std::panic;

// LOW: Assume Cargo.toml updated to bincode v2.x for security/features
use bincode; 
use tracing::{info, error, Level, span};
use ssxl_generate::{Conductor, ConductorProgressReceiver};
use ssxl_generate::task::task_queue::GenerationTask;
use ssxl_math::prelude::Vec2i;
use ssxl_shared::{initialize_shared_data, CHUNKS_COMPLETED_COUNT};
use tokio::sync::mpsc::error::TryRecvError;

// Cleanup: Replaced OnceCell with std::sync::OnceLock
static CONDUCTOR: OnceLock<Mutex<Conductor>> = OnceLock::new();
static PROGRESS_RECEIVER: OnceLock<Mutex<ConductorProgressReceiver>> = OnceLock::new();
static REQUEST_SENDER: OnceLock<tokio::sync::mpsc::UnboundedSender<GenerationTask>> = OnceLock::new();
static INIT_RUNNING: AtomicBool = AtomicBool::new(false);
static INIT_SUCCESSFUL: AtomicBool = AtomicBool::new(false);

const CHUNK_SIZE: i32 = 32;

// ──────────────────────────────────────────────────────────────────────────────
// Public FFI entry points (used by both CLI and Godot)
// ──────────────────────────────────────────────────────────────────────────────

// CRITICAL FFI SAFETY: panic::catch_unwind() guarantees that Rust panics 
// do not unwind across the C boundary, preventing Undefined Behavior.
#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    if INIT_SUCCESSFUL.load(Ordering::Relaxed) {
        return true;
    }
    // HIGH: Correctly handles concurrent initialization attempts
    if INIT_RUNNING.swap(true, Ordering::Acquire) {
        return INIT_SUCCESSFUL.load(Ordering::Relaxed);
    }

    let result = panic::catch_unwind(|| {
        let _span = span!(Level::INFO, "ssxl_start_runtime").entered();
        initialize_shared_data();

        match Conductor::new(None) {
            Ok((conductor, _state, request_sender, progress_receiver)) => {
                // Cleanup: OnceLock::set used.
                let _ = CONDUCTOR.set(Mutex::new(conductor));
                let _ = PROGRESS_RECEIVER.set(Mutex::new(ConductorProgressReceiver::new(progress_receiver)));
                let _ = REQUEST_SENDER.set(request_sender);
                info!("SSXL 9.1 FFI Bridge → Conductor ONLINE");
                INIT_SUCCESSFUL.store(true, Ordering::Release);
                true
            }
            Err(e) => {
                error!("Failed to start Conductor: {:?}", e);
                false
            }
        }
    });

    INIT_RUNNING.store(false, Ordering::Release);
    // Returns false if a panic occurred or if initialization failed.
    result.unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn ssxl_request_chunk(x: i32, y: i32) {
    if !INIT_SUCCESSFUL.load(Ordering::Relaxed) {
        return;
    }
    if let Some(sender) = REQUEST_SENDER.get() {
        let task = GenerationTask {
            chunk_coords: Vec2i::new(x as i64, y as i64),
            generator_id: "default".to_string(),
        };
        // NOTE: UnboundedSender::send only fails if the receiver is dropped (shutdown).
        let _ = sender.send(task);
    }
}

// NOTE: Standard FFI error codes: 0=Empty, >0=Length, <0=Error.
#[no_mangle]
pub extern "C" fn ssxl_poll_progress_message(buffer: *mut u8, buffer_len: usize) -> isize {
    if buffer.is_null() || buffer_len == 0 {
        return -5; // Error: Invalid buffer
    }

    let receiver_mutex = match PROGRESS_RECEIVER.get() {
        Some(r) => r,
        None => return -1, // Error: Not initialized
    };

    let mut receiver = match receiver_mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return -2, // Error: Mutex poisoned
    };

    match receiver.rx.try_recv() {
        Ok(message) => {
            let bytes = match bincode::serialize(&message) {
                Ok(b) => b,
                Err(_) => return -4, // Error: Serialization failed
            };
            let len = bytes.len();
            
            if len > buffer_len { return -5; }
            if len == 0 { return -6; }

            unsafe { 
                // CRITICAL: Unsafe FFI operation: copy data into C-owned memory
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, len); 
            }
            len as isize
        }
        Err(TryRecvError::Empty) => 0, // No message available
        Err(TryRecvError::Disconnected) => -3, // Error: Channel closed
    }
}

// NOTE: Graceful shutdown includes panic::catch_unwind for safety
#[no_mangle]
pub extern "C" fn ssxl_shutdown_runtime() {
    let _ = panic::catch_unwind(|| {
        if let Some(c) = CONDUCTOR.get() {
            if let Ok(guard) = c.lock() {
                guard.signal_shutdown_graceful();
            }
        }
        INIT_SUCCESSFUL.store(false, Ordering::Release);
    });
}

#[no_mangle]
pub extern "C" fn ssxl_is_runtime_ready() -> bool {
    INIT_SUCCESSFUL.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn ssxl_get_chunks_completed() -> u32 {
    CHUNKS_COMPLETED_COUNT.load(Ordering::Relaxed) as u32
}

// ──────────────────────────────────────────────────────────────────────────────
// Godot-specific FFI functions — only linked when building for Godot
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "godot")]
extern "C" {
    fn ssxl_set_cell(x: i32, y: i32, tile_id: i32);
    fn ssxl_notify_tilemap_update();
}

#[cfg(not(feature = "godot"))]
unsafe extern "C" fn ssxl_set_cell(_x: i32, _y: i32, _tile_id: i32) {
    // No-op in standalone CLI
}

#[cfg(not(feature = "godot"))]
unsafe extern "C" fn ssxl_notify_tilemap_update() {
    // No-op in standalone CLI
}

pub unsafe fn ssxl_set_cell_safe(x: i32, y: i32, tile_id: i32) {
    ssxl_set_cell(x, y, tile_id)
}

pub unsafe fn ssxl_notify_tilemap_update_safe() {
    ssxl_notify_tilemap_update()
}

// ──────────────────────────────────────────────────────────────────────────────
// Direct chunk apply — used by generators
// ──────────────────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ssxl_apply_chunk_direct(
    key_x: i32,
    key_y: i32,
    tile_count: usize,
    local_x: *const i32,
    local_y: *const i32,
    tile_id: *const i32,
) -> bool {
    // CRITICAL: Input validation for FFI pointers
    if tile_count == 0 || local_x.is_null() || local_y.is_null() || tile_id.is_null() {
        return false;
    }

    unsafe {
        // CRITICAL: Unsafe FFI operation: creating slices from raw pointers.
        let lx = std::slice::from_raw_parts(local_x, tile_count);
        let ly = std::slice::from_raw_parts(local_y, tile_count);
        let ids = std::slice::from_raw_parts(tile_id, tile_count);
        let origin_x = key_x * CHUNK_SIZE;
        let origin_y = key_y * CHUNK_SIZE;

        for i in 0..tile_count {
            let world_x = origin_x + lx[i];
            let world_y = origin_y + ly[i];
            ssxl_set_cell_safe(world_x, world_y, ids[i]);
        }
        ssxl_notify_tilemap_update_safe();
    }
    true
}

pub fn dispatch_chunk_direct(key_x: i32, key_y: i32, tiles: &[(i32, i32, i32)]) {
    if tiles.is_empty() { return; }

    // NOTE: Requires temporary Vec allocations to convert the slice of tuples 
    // into three separate C-compatible arrays of raw pointers. This is a copy.
    let lx: Vec<i32> = tiles.iter().map(|t| t.0).collect();
    let ly: Vec<i32> = tiles.iter().map(|t| t.1).collect();
    let ids: Vec<i32> = tiles.iter().map(|t| t.2).collect();

    let ok = ssxl_apply_chunk_direct(
        key_x, key_y,
        tiles.len(),
        lx.as_ptr(),
        ly.as_ptr(),
        ids.as_ptr(),
    );

    if !ok {
        error!("dispatch_chunk_direct failed for ({key_x}, {key_y})");
    }
}