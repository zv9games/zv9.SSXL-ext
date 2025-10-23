use crate::zv9_aetherion_pipeline_builder_streamer::{ChunkDelivery, SyncBridge};
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use anyhow::{Result, Context}; // Gold: Err elegy.
use aetherion_shared::shared::EngineMessage;

#[cfg(feature = "verbose")]
use tracing::info; // Feat: Trace-taming.

use rand::Rng; // For ID feat.

#[cfg(feature = "rand_id")]
use rand::rngs::StdRng;

/// Builder: Fluent forge—your golden gambit.
#[derive(Debug, Clone, Default)]
pub struct DummyBuilder {
    verbose: bool,
    rand_id: bool,
}

impl DummyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    #[cfg(feature = "rand_id")]
    pub fn rand_id(mut self, r: bool) -> Self {
        self.rand_id = r;
        self
    }

    pub fn build(self) -> DummyDelivery {
        DummyDelivery::new_with(self.verbose, self.rand_id)
    }
}

/// 🧪 Dummy: Batch bliss, feat-forked.
#[derive(Clone)]
pub struct DummyDelivery {
    bridge: SyncBridge,
    verbose: bool,
    #[cfg(feature = "rand_id")]
    rng: StdRng,
    #[cfg(feature = "rand_id")]
    sync_id: usize,
}

impl DummyDelivery {
    pub fn new() -> Self {
        Self::new_with(true, false)
    }

    fn new_with(verbose: bool, rand_id: bool) -> Self {
        #[cfg(feature = "rand_id")]
        let mut rng = StdRng::from_entropy();
        #[cfg(feature = "rand_id")]
        let sync_id = rng.gen();
        #[cfg(not(feature = "rand_id"))]
        let _ = rand_id; // Unused.

        Self {
            bridge: SyncBridge::new(1024),
            verbose,
            #[cfg(feature = "rand_id")]
            rng,
            #[cfg(feature = "rand_id")]
            sync_id,
        }
    }

    /// Batch deliver: Velocity vow—fmt once.
    pub fn deliver_batch(&mut self, chunks: Vec<MapDataChunk>) -> Result<()> {
        if chunks.is_empty() {
            return Ok(());
        }

        let total = chunks.iter().map(|c| c.len()).sum::<usize>();
        if self.verbose {
            #[cfg(feature = "verbose")]
            info!("🧪 Dummy batch: {} chunks, {} tiles total", chunks.len(), total);
            #[cfg(not(feature = "verbose"))]
            println!("🧪 Dummy batch: {} chunks, {} tiles", chunks.len(), total);
        }

        for chunk in chunks {
            self.deliver(chunk).context("Batch deliver fail")?;
        }

        Ok(())
    }

    /// Drain signals: Gold harvest for tests.
    pub fn drain_signals(&mut self) -> Vec<EngineMessage> {
        self.bridge.drain_signals()
    }
}

impl ChunkDelivery for DummyDelivery {
    fn deliver(&mut self, chunk: MapDataChunk) -> Result<()> {
        if self.verbose {
            #[cfg(feature = "verbose")]
            info!("🧪 Dummy received: {} tiles", chunk.len());
            #[cfg(not(feature = "verbose"))]
            println!("🧪 Dummy received: {} tiles", chunk.len());
        }

        Ok(())
    }

    fn sync(&mut self) -> &mut SyncBridge {
        &mut self.bridge
    }

    fn sync_id(&self) -> usize {
        #[cfg(feature = "rand_id")]
        {
            self.sync_id
        }

        #[cfg(not(feature = "rand_id"))]
        {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aetherion_shared::shared::EngineMessage;

    #[test]
    fn dummy_new_and_deliver() {
        let mut dummy = DummyDelivery::new();
        let chunk = MapDataChunk::default(); // Mock.
        dummy.deliver(chunk).unwrap();
        assert_eq!(dummy.sync_id(), 0);
    }

    #[test]
    fn batch_deliver() {
        let mut dummy = DummyDelivery::new();
        let chunks = vec![MapDataChunk::default(); 10];
        dummy.deliver_batch(chunks).unwrap();
    }

    #[test]
    fn drain_signals() {
        let mut dummy = DummyDelivery::new();
        dummy.sync().add_signal(EngineMessage::Status("test".into()));
        let sigs = dummy.drain_signals();
        assert_eq!(sigs.len(), 1);
    }

    #[bench]
    fn bench_deliver_1k(b: &mut test::Bencher) {
        let mut dummy = DummyDelivery::new();
        let chunk = MapDataChunk::default();
        b.iter(|| dummy.deliver(chunk.clone()).unwrap());
    }
}
