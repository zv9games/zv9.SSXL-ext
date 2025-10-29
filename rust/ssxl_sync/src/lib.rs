// ssxl_sync/src/lib.rs
//!
//! Provides the core synchronization and communication channels for the engine,
//! allowing data exchange between the main thread, Godot, and worker threads.

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
// We use the tracing framework (configured in Cargo.toml) for logging.
use tracing::info;

// NOTE: We are using String as a temporary placeholder for the complex
// AetherionData struct, which will be defined later in the project.

// --- 1. Thread-Safe Resource Wrapper ---

/// A thread-safe wrapper for resources that allows multiple readers
/// or a single writer at any time.
#[derive(Debug, Clone)]
pub struct AtomicResource<T> {
    data: Arc<RwLock<T>>,
}

impl<T> AtomicResource<T> {
    /// Creates a new AtomicResource instance.
    pub fn new(data: T) -> Self {
        AtomicResource {
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Acquires a read lock on the resource.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read()
    }

    /// Acquires a write lock on the resource.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}

// --- 2. Channel Utilities ---

/// Creates an unbounded crossbeam channel pair.
pub fn create_sync_channel() -> (Sender<String>, Receiver<String>) {
    unbounded()
}

/// Starts the main synchronization worker task.
pub fn start_sync_worker() {
    info!("SSXL Synchronization Worker placeholder started.");
}

// ---------------------------
// IMPL: Unit Tests (Phase 1 Validation)
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // --- Channel Tests ---

    #[test]
    fn test_channel_functionality() {
        let (sender, receiver) = create_sync_channel();
        let test_message = "Test data for Phase 1 sync check".to_string();

        sender.send(test_message.clone()).expect("Failed to send test message");
        let received_message = receiver.recv().expect("Failed to receive test message");

        assert_eq!(received_message, test_message, "Received message did not match sent message.");
    }

    #[test]
    fn test_sync_worker_placeholder_runs() {
        start_sync_worker();
    }
    
    // --- AtomicResource Tests ---

    /// Tests that the AtomicResource correctly handles concurrent reads and a single write.
    /// FIX: Explicitly join reader threads to eliminate the race condition.
    #[test]
    fn test_concurrent_read_write() {
        let resource = AtomicResource::new(0);

        // 1. Concurrent Reads (Must all read 0)
        let mut handles = vec![];
        for _ in 0..5 {
            let res_clone = resource.clone();
            handles.push(thread::spawn(move || {
                let value = *res_clone.read();
                assert_eq!(value, 0);
            }));
        }

        // CRITICAL FIX: Wait for all initial readers to complete before allowing the writer to run.
        for h in handles {
            h.join().unwrap();
        }

        // 2. Single Write (Set to 42)
        let mut writer_guard = resource.write();
        *writer_guard = 42;
        drop(writer_guard); // Lock released

        // 3. New Concurrent Reads (Must all read 42)
        let mut handles = vec![];
        for _ in 0..5 {
            let res_clone = resource.clone();
            handles.push(thread::spawn(move || {
                let value = *res_clone.read();
                assert_eq!(value, 42); // Verify the change
            }));
        }
        
        // Wait for all subsequent reader threads to complete.
        for h in handles {
            h.join().unwrap();
        }
    }

    /// Tests basic read/write access without concurrency.
    #[test]
    fn test_basic_read_write() {
        let resource = AtomicResource::new(10);
        
        {
            let value = *resource.read();
            assert_eq!(value, 10);
        }
        
        {
            let mut writer = resource.write();
            *writer = 20;
        }

        assert_eq!(*resource.read(), 20, "Basic write failed.");
    }
}