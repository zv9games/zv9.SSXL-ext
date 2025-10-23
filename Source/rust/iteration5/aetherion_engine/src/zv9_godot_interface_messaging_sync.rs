use godot::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use aetherion_shared::shared::EngineMessage;
use aetherion_core::pipeline::builder::ChunkDelivery;
use aetherion_core::pipeline::data::MapDataChunk;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::SyncBridge;

//
// ─── GodotSync ────────────────────────────────────────────────────────────────
//

/// 🧵 GodotSync — thread-safe queue for chunk and signal delivery between Rust and Godot.
#[derive(Clone, Debug)]
pub struct GodotSync {
    inner: Arc<Mutex<GodotSyncInner>>,
    id: usize,
}

#[derive(Default, Debug)]
struct GodotSyncInner {
    chunks: Vec<MapDataChunk>,
    signals: Vec<EngineMessage>,
}

impl Default for GodotSync {
    fn default() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
            id,
        }
    }
}

impl GodotSync {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn sync_id(&self) -> usize {
        self.id
    }

    pub fn add_chunk(&self, chunk: MapDataChunk) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.chunks.push(chunk);
        }
    }

    pub fn drain_chunks(&self) -> Vec<MapDataChunk> {
        self.inner
            .lock()
            .map(|mut inner| inner.chunks.drain(..).collect())
            .unwrap_or_default()
    }

    pub fn add_signal(&self, signal: EngineMessage) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.signals.push(signal);
        }
    }

    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        self.inner
            .lock()
            .map(|mut inner| inner.signals.drain(..).collect())
            .unwrap_or_default()
    }

    pub fn has_pending(&self) -> bool {
        self.inner
            .lock()
            .map(|inner| !inner.chunks.is_empty() || !inner.signals.is_empty())
            .unwrap_or(false)
    }

    pub fn push_status(&self, status: &str) {
        self.add_signal(EngineMessage::Status(status.to_string()));
    }
}

//
// ─── GodotDelivery ────────────────────────────────────────────────────────────
//

/// 🚀 GodotDelivery — wrapper for GodotSync that satisfies ChunkDelivery trait.
#[derive(Clone, Debug)]
pub struct GodotDelivery {
    pub sync: GodotSync,
    pub bridge: SyncBridge,
}

impl GodotDelivery {
    pub fn sync_id(&self) -> usize {
        self.sync.sync_id()
    }

    /// Should only be called on the main thread.
    pub fn sync_mut(&mut self) -> &mut GodotSync {
        self.forward_bridge_signals();
        &mut self.sync
    }

    pub fn push_status(&self, status: &str) {
        self.sync.push_status(status);
    }

    /// Forward bridge signals into the main sync queue.
    pub fn forward_bridge_signals(&mut self) {
        for signal in self.bridge.drain_signals() {
            self.sync.add_signal(signal);
        }
    }
}

impl ChunkDelivery for GodotDelivery {
    fn deliver(&mut self, chunk: MapDataChunk) -> Result<()> {
        // Called by worker threads: safe, no Godot API calls.
        self.sync.add_chunk(chunk);
        Ok(())
    }

    fn sync(&mut self) -> &mut SyncBridge {
        &mut self.bridge
    }

    fn sync_id(&self) -> usize {
        self.sync.sync_id()
    }
}
