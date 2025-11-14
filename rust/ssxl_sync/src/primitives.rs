// ssxl_sync/src/primitives.rs

//! # Synchronization Primitives (`ssxl_sync::primitives`)
//!
//! This module defines the core thread-safe data structures and communication channels.

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};
use tracing::info; // Required for the start_sync_worker function

// --- 1. Thread-Safe Resource Wrapper ---

/// A generic, thread-safe wrapper for data shared between multiple threads.
/// It uses `parking_lot::RwLock` for fast, efficient concurrent read access.
#[derive(Debug, Clone)]
pub struct AtomicResource<T> {
    /// The actual data protected by an Arc (shared ownership) and RwLock (read/write access control).
    data: Arc<RwLock<T>>,
}

impl<T> AtomicResource<T> {
    /// Creates a new `AtomicResource` containing the provided data.
    pub fn new(data: T) -> Self {
        AtomicResource {
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Acquires a read lock on the data. Allows multiple concurrent readers.
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read()
    }

    /// Acquires an exclusive write lock on the data. Blocks all other readers/writers.
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}


// --- 2. Generic Synchronous Channel (Crossbeam) ---

/// Creates an unbounded, multi-producer, single-consumer channel for synchronous (blocking) communication.
/// This is used primarily for simple signalling or utility messaging.
pub fn create_sync_channel() -> (Sender<String>, Receiver<String>) {
    unbounded()
}

/// A placeholder function demonstrating where a synchronous worker thread might be started.
pub fn start_sync_worker() {
    info!("SSXL Synchronization Worker placeholder started.");
}