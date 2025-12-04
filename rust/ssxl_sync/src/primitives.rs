// ssxl_sync/src/primitives.rs

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};
// DELETE: tracing::info removed (placeholder worker function removed)

// --------------------------------------------------------------------------------
// --- Thread-Safe Resource Management (AtomicResource) ---
// --------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AtomicResource<T> {
    data: Arc<RwLock<T>>,
}

impl<T> AtomicResource<T> {
    pub fn new(data: T) -> Self {
        AtomicResource {
            data: Arc::new(RwLock::new(data)),
        }
    }

    #[inline(always)] // O(1) Accessor: Force inlining
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read()
    }

    #[inline(always)] // O(1) Mutator: Force inlining
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}

impl<T: Default> Default for AtomicResource<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// --------------------------------------------------------------------------------
// --- Synchronization Primitives (Channels) ---
// --------------------------------------------------------------------------------

/// Creates an unbounded, multi-producer, single-consumer channel.
/// Generic over message type `M` (Zero Entropy).
pub fn create_unbounded_channel<M>() -> (Sender<M>, Receiver<M>) {
    unbounded()
}
// DELETE: start_sync_worker placeholder removed (non-primitive code).