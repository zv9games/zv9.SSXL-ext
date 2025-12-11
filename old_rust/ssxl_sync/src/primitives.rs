// ============================================================================
// 🔒 Synchronization Primitives (`ssxl_sync::primitives`)
// ----------------------------------------------------------------------------
// This module defines two foundational building blocks for safe, concurrent
// programming in the SSXL engine:
//
// 1. AtomicResource<T>
//    • A thread-safe wrapper around shared data.
//    • Internally uses `Arc<RwLock<T>>` from the `parking_lot` crate.
//    • Provides cheap cloning (`Arc`) and fine-grained read/write access
//      (`RwLock`), allowing multiple readers or a single writer at a time.
//    • Exposes ergonomic `read()` and `write()` methods that return guards,
//      ensuring Rust’s borrow checker enforces safe access.
//    • Implements `Default` so resources can be initialized predictably.
//
//    Educational note:
//    • `RwLock` (read–write lock) is ideal when reads are frequent and writes
//      are rare, as it allows concurrent readers but exclusive writers.
//    • Wrapping in `Arc` makes the resource clonable and shareable across
//      threads without copying the underlying data.
//    • This pattern is common in game engines and async runtimes where shared
//      state (e.g., configuration, animation state) must be accessed safely
//      by multiple worker threads.
//
// 2. create_unbounded_channel<M>
//    • A helper function that creates an unbounded multi-producer,
//      single-consumer channel using `crossbeam_channel`.
//    • Generic over message type `M`, so it can be used for any kind of
//      communication payload (commands, updates, events).
//    • Unbounded channels are useful for high-throughput pipelines where
//      backpressure is not desired (e.g., logging, fire-and-forget updates).
//
//    Educational note:
//    • Channels are a core concurrency primitive in Rust, enabling safe
//      message passing between threads without shared mutable state.
//    • `crossbeam_channel` provides blocking semantics, which are well-suited
//      for CPU-bound worker pools (as opposed to async channels in Tokio).
//
// Why this matters:
// • Together, `AtomicResource` and `create_unbounded_channel` provide the
//   foundation for building higher-level concurrency abstractions in SSXL.
// • They encapsulate common patterns (shared state + message passing) in a
//   reusable, ergonomic way.
// • By centralizing these primitives, the engine ensures consistency and
//   reduces boilerplate across subsystems (generation, animation, networking).
// ============================================================================



use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};

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

    #[inline(always)]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read()
    }

    #[inline(always)]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}

impl<T: Default> Default for AtomicResource<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub fn create_unbounded_channel<M>() -> (Sender<M>, Receiver<M>) {
    unbounded()
}
