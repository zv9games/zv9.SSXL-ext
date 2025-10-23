#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use aetherion_shared::zv9_shared_messages::EngineMessage;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use anyhow::{Result, Context};  // Gold: Err elegy.

#[cfg(feature = "trace")]
use tracing::info;  // Feat: Trace-taming, compile-out release.

// Import for mocks—gold key to test harmony.
use crate::zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;

//
// ─── Chunk Delivery Trait ──────────────────────────────────────────────────────
//

/// 📦 ChunkDelivery — trait for delivering chunks to an external system.
pub trait ChunkDelivery: Send + Clone {
    fn deliver(&mut self, chunk: MapDataChunk) -> Result<()>;

    fn sync(&mut self) -> &mut SyncBridge;

    /// 💬 Pushes a status message into the signal stream.
    fn push_status(&mut self, msg: &str) -> Result<()> {
        let signal = EngineMessage::Status(msg.to_string());
        #[cfg(feature = "trace")]
        info!("📤 ChunkDelivery pushing status: {:?}", signal);
        self.sync().add_signal(signal);
        Ok(())
    }

    /// 🧵 Returns the sync ID for tracing.
    fn sync_id(&self) -> usize;
}

//
// ─── Conductor ─────────────────────────────────────────────────────────────────
//

/// 🎛 Conductor — orchestrates procedural flow and coordinates delivery pacing.
#[derive(Debug)]
pub struct Conductor<D: ChunkDelivery + Clone> {
    queue: VecDeque<MapDataChunk>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
    est_queue_cap: usize,  // Reserve hint.
}

impl<D: ChunkDelivery + Clone> Conductor<D> {
    pub fn new(streamer: ChunkStreamer<D>, est_cap: usize) -> Self {
        let mut queue = VecDeque::new();
        queue.reserve(est_cap);  // Velocity: Pre-alloc.
        Self {
            queue,
            ticks_waiting: 0,
            streamer,
            est_queue_cap: est_cap,  // Fixed: Assign est_cap to est_queue_cap.
        }
    }

    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) -> Result<()> {
        if self.queue.len() > self.est_queue_cap * 2 {  // Gold: Overflow guard.
            anyhow::bail!("Queue overflow: {} > cap {}", self.queue.len(), self.est_queue_cap);
        }
        #[cfg(feature = "trace")]
        info!("📦 Conductor enqueued: {} tiles", chunk.len());
        self.queue.push_back(chunk);
        Ok(())
    }

    /// Batch: Extend harmony.
    pub fn enqueue_batch(&mut self, chunks: impl IntoIterator<Item = MapDataChunk>) -> Result<()> {
        let iter = chunks.into_iter();
        let added = iter.size_hint().0;
        if self.queue.len() + added > self.est_queue_cap * 2 {
            anyhow::bail!("Batch overflow risk");
        }
        #[cfg(feature = "trace")]
        info!("📦 Conductor batch enqueued: +{}", added);
        self.queue.extend(iter);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        #[cfg(feature = "trace")]
        info!("⏱️ Conductor tick");

        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            return Ok(());  // Early, no log.
        }

        if let Some(chunk) = self.queue.pop_front() {
            #[cfg(feature = "trace")]
            info!("📤 Conductor delivering: {} tiles", chunk.len());
            self.streamer.sync().add_signal(EngineMessage::ChunkReady(chunk.clone()));
            self.streamer.enqueue_chunk(chunk)?;
        }

        self.streamer.try_deliver()?;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        #[cfg(feature = "trace")]
        info!("⏸️ Conductor paused");
        self.streamer.pause();
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        #[cfg(feature = "trace")]
        info!("▶️ Conductor resumed");
        self.streamer.resume();
        Ok(())
    }

    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.streamer.has_pending()
    }

    pub fn queue_len(&self) -> usize { self.queue.len() }

    pub fn streamer_mut(&mut self) -> &mut ChunkStreamer<D> { &mut self.streamer }
}

//
// ─── Chunk Streamer ────────────────────────────────────────────────────────────
//

/// 🚚 ChunkStreamer — manages pacing and delivery of chunks.
#[derive(Debug, Clone)]
pub struct ChunkStreamer<D: ChunkDelivery + Clone> {
    queue: VecDeque<MapDataChunk>,
    delivery: D,
    delivery_interval: Duration,
    last_delivery: Instant,
    paused: bool,
    est_queue_cap: usize,  // Reserve hint.
}

impl<D: ChunkDelivery + Clone> ChunkStreamer<D> {
    pub fn new(delivery: D, interval_ms: u64, est_cap: usize) -> Self {
        let mut queue = VecDeque::new();
        queue.reserve(est_cap);
        Self {
            queue,
            delivery,
            delivery_interval: Duration::from_millis(interval_ms),
            last_delivery: Instant::now(),
            paused: false,
            est_queue_cap: est_cap,  // Fixed: Assign est_cap to est_queue_cap.
        }
    }

    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) -> Result<()> {
        #[cfg(feature = "trace")]
        info!("📦 Streamer enqueued: {} tiles", chunk.len());
        self.queue.push_back(chunk);
        Ok(())
    }

    /// Batch: Extend for velocity.
    pub fn enqueue_batch(&mut self, chunks: impl IntoIterator<Item = MapDataChunk>) -> Result<()> {
        let iter = chunks.into_iter();
        let added = iter.size_hint().0;
        if self.queue.len() + added > self.est_queue_cap * 2 {
            anyhow::bail!("Streamer queue overflow");
        }
        #[cfg(feature = "trace")]
        info!("📦 Streamer batch: +{}", added);
        self.queue.extend(iter);
        Ok(())
    }

    #[inline(always)]
    pub fn try_deliver(&mut self) -> Result<()> {
        if self.paused { return Ok(()); }  // Inline early.

        if self.queue.is_empty() { return Ok(()); }

        let now = Instant::now();
        if now.duration_since(self.last_delivery) >= self.delivery_interval {
            if let Some(chunk) = self.queue.pop_front() {
                #[cfg(feature = "trace")]
                info!("🚚 Streamer delivering: {} tiles", chunk.len());
                self.delivery.deliver(chunk)?;
                self.last_delivery = now;
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        self.paused = true;
        #[cfg(feature = "trace")]
        info!("⏸️ Streamer paused");
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.paused = false;
        #[cfg(feature = "trace")]
        info!("▶️ Streamer resumed");
        Ok(())
    }

    pub fn has_pending(&self) -> bool { !self.queue.is_empty() }

    pub fn queue_len(&self) -> usize { self.queue.len() }

    /// Batch add: Vec→extend for sync harmony.
    pub fn add_signals_batch(&mut self, signals: Vec<EngineMessage>) -> Result<()> {
        self.delivery.sync().add_signals_batch(signals);
        Ok(())
    }

    pub fn sync(&mut self) -> &mut SyncBridge { self.delivery.sync() }

    pub fn delivery_mut(&mut self) -> &mut D { &mut self.delivery }
}

//
// ─── Sync Bridge ───────────────────────────────────────────────────────────────
//

/// 🔗 SyncBridge — allows delivery backends to emit signals and coordinate with the engine.
#[derive(Default, Clone, Debug)]
pub struct SyncBridge {
    signals: Vec<EngineMessage>,
    est_cap: usize,  // Reserve hint.
}

impl SyncBridge {
    pub fn new(est_cap: usize) -> Self {
        let mut signals = Vec::new();
        signals.reserve(est_cap);
        Self { signals, est_cap }
    }

    /// Single: Push with reserve.
    pub fn add_signal(&mut self, signal: EngineMessage) -> Result<()> {
        if self.signals.len() > self.est_cap * 2 {
            anyhow::bail!("Sync signals overflow");
        }
        #[cfg(feature = "trace")]
        info!("📥 SyncBridge signal: {:?}", signal);
        self.signals.push(signal);
        Ok(())
    }

    /// Batch: Extend for velocity.
    pub fn add_signals_batch(&mut self, signals: Vec<EngineMessage>) -> Result<()> {
        let added = signals.len();
        if self.signals.len() + added > self.est_cap * 2 {
            anyhow::bail!("Batch signals overflow");
        }
        #[cfg(feature = "trace")]
        info!("📥 SyncBridge batch: +{}", added);
        self.signals.extend(signals);
        Ok(())
    }

    pub fn drain_signals(&mut self) -> Vec<EngineMessage> {
        let drained = std::mem::take(&mut self.signals);
        #[cfg(feature = "trace")]
        info!("🧹 SyncBridge drained: {} signals", drained.len());
        drained
    }

    pub fn has_signals(&self) -> bool { !self.signals.is_empty() }

    /// Avg len metric: Gold gauge.
    pub fn avg_pending(&self) -> f64 {
        let cap = self.est_cap.max(1) as f64;
        self.signals.len() as f64 / cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Fixed: Import test::Bencher for benches; already in scope, but explicit for clarity.
    use std::prelude::v1::test;

    #[test]
    fn conductor_enqueue_tick() {
        // Fixed: Instantiate proper mock—DummyDelivery with defaults (est_cap=100, verbose=false, etc.).
        let delivery = DummyDelivery::new(100, false, None, 0);  // Tune args as per your DummyDelivery sig.
        let streamer = ChunkStreamer::new(delivery, 16, 100);
        let mut conductor = Conductor::new(streamer, 100);
        let chunk = MapDataChunk::default();
        conductor.enqueue_chunk(chunk).unwrap();

        assert!(conductor.has_pending());
        conductor.tick().unwrap();
        assert!(!conductor.has_pending());
    }

    #[test]
    fn streamer_try_deliver() {
        // Fixed: Same mock instantiation.
        let delivery = DummyDelivery::new(100, false, None, 0);
        let mut streamer = ChunkStreamer::new(delivery, 1, 100);  // 1ms interval.
        let chunk = MapDataChunk::default();
        streamer.enqueue_chunk(chunk).unwrap();

        std::thread::sleep(Duration::from_millis(2));  // >interval.
        streamer.try_deliver().unwrap();
        assert!(!streamer.has_pending());
    }

    #[test]
    fn sync_bridge_batch() {
        let mut bridge = SyncBridge::new(10);
        let sigs = vec![EngineMessage::Status("test".into()); 5];
        bridge.add_signals_batch(sigs).unwrap();

        let drained = bridge.drain_signals();
        assert_eq!(drained.len(), 5);
    }

    // Fixed: Use test::Bencher; mock as above. (Bench sig assumes std::test.)
    #[bench]
    fn bench_conductor_1k_ticks(b: &mut test::Bencher) {
        let delivery = DummyDelivery::new(1000, false, None, 0);  // Scaled for bench.
        let streamer = ChunkStreamer::new(delivery, 16, 1000);
        let mut conductor = Conductor::new(streamer, 1000);
        let chunk = MapDataChunk::default();

        b.iter(|| {
            for _ in 0..1000 {
                conductor.enqueue_chunk(chunk.clone()).unwrap();
                conductor.tick().unwrap();
            }
        });
    }
}