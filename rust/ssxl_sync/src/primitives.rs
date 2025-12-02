use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};
use tracing::info;

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

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read()
    }

    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}

impl<T: Default> Default for AtomicResource<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub fn create_sync_channel() -> (Sender<String>, Receiver<String>) {
    unbounded()
}

pub fn start_sync_worker() {
    info!("SSXL Synchronization Worker placeholder started.");
}