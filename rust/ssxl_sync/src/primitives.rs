use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use std::sync::Mutex; 
use crossbeam_channel::{unbounded, Receiver, Sender};
use tracing::{info, error}; 
use ssxl_shared::chunk_data::ChunkData;
use ssxl_shared::tile_data::AnimationUpdate;
use tokio::sync::mpsc;
// 🛑 CRITICAL FIX: Add Godot imports required for AnimationCommand variants
// NOTE: These imports are left here, but are no longer needed for the fixed AnimationCommand
use godot::obj::Gd;         
use godot::classes::Node;   


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

// ------------------------------------------------------------------
// 3. ANIMATION CONDUCTION (Exposed Types)
// ------------------------------------------------------------------

/// Represents a command sent from the main thread to the animation worker.
#[derive(Debug, Clone)]
pub enum AnimationCommand {
    Start,
    Stop,
    UpdateFramerate(f32),
    /// Command to register a generated chunk's data for the animator to track and update.
    RegisterChunk(Arc<ChunkData>),
    Complete,   
    
    // ✅ CRITICAL FIX APPLIED: Removed the Gd<Node> field entirely, as it violates the Send bound required by mpsc channel for cross-thread communication.
    StartTestAnimation, // <--- REVERTED TO UNIT VARIANT
    StopTestAnimation,  
}

/// The manager for the animation worker thread.
/// It holds the sender for issuing commands to the worker.
/// The `new()` and `send_command()` methods are implemented in `pool_manager.rs`.
#[derive(Debug)]
pub struct AnimationConductor {
    // This private field is public only to other modules in the crate (`pub(crate)`)
    // so `pool_manager.rs` can access it for implementation.
    pub(crate) command_sender: mpsc::UnboundedSender<AnimationCommand>,
}

// ✅ FIX: Added a public getter method for the private command_sender field.
impl AnimationConductor {
    /// Public getter for the command sender handle.
    /// Returns a clone of the sender handle, which is cheap for MPSC Senders.
    pub fn get_command_sender(&self) -> AnimationConductorHandle {
        self.command_sender.clone()
    }
}

// FIX: Add the missing type aliases needed for re-export in lib.rs
/// The type alias for the sender handle to the animation worker.
pub type AnimationConductorHandle = mpsc::UnboundedSender<AnimationCommand>;

/// The type alias for the receiver of animation updates from the worker.
pub type AnimationReceiver = mpsc::UnboundedReceiver<AnimationUpdate>;


// ------------------------------------------------------------------
// 4. ANIMATION STATUS AND STATE (NEW SECTION)
// ------------------------------------------------------------------

/// Represents the operational state of the Animation Conductor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationStatus {
    // Maps to "Animation Init Pending" status
    Initializing,
    Running,
    Paused,
    ShuttingDown,
    Error,
}

/// Shared, thread-safe state exposed to the FFI consumer for animation monitoring.
/// This fixes the "Animation Init Pending" log by providing a status mechanism.
#[derive(Clone)]
pub struct AnimationState {
    status: Arc<Mutex<AnimationStatus>>,
}

impl AnimationState {
    pub fn new() -> Self {
        AnimationState {
            status: Arc::new(Mutex::new(AnimationStatus::Initializing)),
        }
    }

    /// Public Getter (Read Access for FFI/CLI)
    pub fn get_status(&self) -> AnimationStatus {
        match self.status.lock() {
            Ok(guard) => *guard,
            Err(e) => {
                error!("Mutex poisoned when reading animation status: {}", e);
                AnimationStatus::Error
            }
        }
    }

    /// Internal Mutator (For Animation Worker state changes)
    pub(crate) fn set_status(&self, new_status: AnimationStatus) {
        // Use a simple unwrap here, as a panic indicates a poisoned worker thread anyway.
        *self.status.lock().unwrap() = new_status;
    }
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