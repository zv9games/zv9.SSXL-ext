#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use log::{warn, debug, info};  // Native macros—unify the choir.

use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use aetherion_shared::shared::SerializableVector2i;
use aetherion_shared::shared::TileInfo;  // Fixed: Import TileInfo from shared.
use crate::structure::tile_at;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};

#[cfg(feature = "profile")]
use tracing::{span, Level};

#[cfg(feature = "parallel")]
use rayon::prelude::*;  // Added: For par_iter in generate_terrain.

/// 🎼 ProcCommand — now with builders for golden fluency.
pub enum ProcCommand {  // Fixed: Remove derive(Debug) to avoid conflict with manual impl.
    GenerateTerrain { seed: u64 },  // Parametrized gold.
    OverlayStructure,
    ApplyModifier(Box<dyn Fn(&mut MapDataChunk) + Send + Sync>),  // +Sync for par safety.
    EmitSignal(String),
    WaitTicks(u64),
}

impl ProcCommand {
    /// Builder chain—ergonomic alchemy.
    pub fn builder() -> ProcCommandBuilder { ProcCommandBuilder::new() }
}

/// Fluent builder for commands.
#[derive(Debug, Default)]
pub struct ProcCommandBuilder;

impl ProcCommandBuilder {
    pub fn new() -> Self { Self }

    pub fn generate_terrain(mut self, seed: u64) -> ProcCommand {
        ProcCommand::GenerateTerrain { seed }
    }

    pub fn overlay_structure(self) -> ProcCommand { ProcCommand::OverlayStructure }

    pub fn apply_modifier<F>(self, f: F) -> ProcCommand
    where F: Fn(&mut MapDataChunk) + Send + Sync + 'static {
        ProcCommand::ApplyModifier(Box::new(f))
    }

    pub fn emit_signal(self, msg: impl Into<String>) -> ProcCommand {
        ProcCommand::EmitSignal(msg.into())
    }

    pub fn wait_ticks(self, n: u64) -> ProcCommand { ProcCommand::WaitTicks(n) }
}

impl std::fmt::Debug for ProcCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcCommand::GenerateTerrain { seed } => write!(f, "GenerateTerrain(seed={})", seed),
            ProcCommand::OverlayStructure => write!(f, "OverlayStructure"),
            ProcCommand::ApplyModifier(_) => write!(f, "ApplyModifier(<fn>)"),
            ProcCommand::EmitSignal(msg) => write!(f, "EmitSignal({})", msg),
            ProcCommand::WaitTicks(n) => write!(f, "WaitTicks({})", n),
        }
    }
}

/// 🎛 Conductor — now with metrics & err handling.
#[derive(Debug)]
pub struct Conductor<D: ChunkDelivery + Clone> {
    queue: VecDeque<ProcCommand>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
    #[cfg(feature = "profile")]
    span: Option<tracing::Span>,
}

#[derive(Debug, thiserror::Error)]  // Gold: Rich errs.
pub enum ConductorError {
    #[error("Terrain gen timed out after {0:.2?}")]
    Timeout(Duration),
    #[error("Merge failed: {0}")]
    Merge(String),
    #[error("Queue overflow (max {0})")]
    Overflow(usize),
}

impl<D: ChunkDelivery + Clone> Conductor<D> {
    #[cfg(feature = "profile")]
    pub fn new(streamer: ChunkStreamer<D>) -> Self {
        let span = Some(span!(Level::INFO, "conductor").entered());
        Self { queue: VecDeque::new(), ticks_waiting: 0, streamer, span }
    }

    #[cfg(not(feature = "profile"))]
    pub fn new(streamer: ChunkStreamer<D>) -> Self {  // Fixed: Conditional new() to handle span scope.
        Self { queue: VecDeque::new(), ticks_waiting: 0, streamer }
    }

    pub fn enqueue(&mut self, cmd: ProcCommand) {
        if self.queue.len() > 1024 {  // Gold: Bound queue.
            warn!("conductor: Queue near overflow—dropping cmd");  // Native warn! macro, with target for structure.
            return;
        }
        self.queue.push_back(cmd);
    }

    /// Batch enqueue for speed.
    pub fn enqueue_batch<I>(&mut self, cmds: I)
    where I: IntoIterator<Item = ProcCommand> {
        self.queue.extend(cmds);
    }

    pub fn tick(&mut self, tick: u64, chunk: &mut MapDataChunk) -> Result<(), ConductorError> {
        #[cfg(feature = "profile")]
        let _guard = self.span.as_ref().unwrap().span().entered();

        if cfg!(feature = "profile") {
            debug!("conductor: 🕒 Tick {} started", tick);  // Native debug! macro—remove ! from custom, embrace log:: with target & args.
        }

        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            return Ok(());  // Early out—no log spam.
        }

        if let Some(cmd) = self.queue.pop_front() {
            match cmd {
                ProcCommand::GenerateTerrain { seed } => {
                    self.generate_terrain(seed, chunk)?;
                }
                ProcCommand::OverlayStructure => {
                    info!("conductor: 🏗 Overlaying structure...");  // Fixed: Native info! macro—structured target.
                    self.push_status("Structure overlay complete");
                }
                ProcCommand::ApplyModifier(f) => {
                    info!("conductor: 🖌 Applying modifier...");  // Fixed: Native info! macro.
                    f(chunk);
                    self.push_status("Modifier applied");
                }
                ProcCommand::EmitSignal(msg) => {
                    self.push_status(&msg);
                }
                ProcCommand::WaitTicks(n) => {
                    self.ticks_waiting = n;
                    self.push_status(&format!("Waiting {} ticks", n));
                }
            }
        }

        if cfg!(feature = "profile") {
            debug!("conductor: ✅ Tick {} complete", tick);  // Native debug! macro—elegant, macro-fluent.
        }
        Ok(())
    }

    #[inline(always)]  // Hot: Inline push.
    fn push_status(&mut self, msg: &str) {
        self.streamer.delivery_mut().push_status(msg);
    }

    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.ticks_waiting > 0
    }

    pub fn queue_len(&self) -> usize { self.queue.len() }

    pub fn streamer_mut(&mut self) -> &mut ChunkStreamer<D> { &mut self.streamer }
}

/// Optimized terrain: Row-par, flat tiles, batch insert.
impl<D: ChunkDelivery + Clone> Conductor<D> {
    fn generate_terrain(&mut self, seed: u64, chunk: &mut MapDataChunk) -> Result<(), ConductorError> {
        #[cfg(feature = "profile")]
        let _gen_span = span!(Level::INFO, "generate_terrain").entered();

        info!("conductor: 🌍 Generating terrain...");  // Fixed: Native info! for consistency.

        let start = Instant::now();
        let opt_timeout = Duration::from_secs(30);  // Opt: Cfg off?
        #[cfg(not(feature = "profile"))]
        let opt_timeout = Duration::MAX;  // No timeout in release.

        let width = 10_000u64;
        let height = 100_000u64;
        let total_tiles = width * height;  // ~1e9—cap? But batch.

        // Flat tiles: Vec<(pos, tile)>—no mid-hash.
        let mut tiles: Vec<(SerializableVector2i, TileInfo)> = Vec::with_capacity(total_tiles.min(1_000_000) as usize);  // Fixed: Use imported TileInfo, cap mem.

        let processed = Arc::new(AtomicUsize::new(0));
        let log_threshold = (total_tiles / 100_000).max(1) as usize;  // Adaptive batch log.

        // Par over rows: Finer grains, auto-balance (Rayon tip , ).
        #[cfg(feature = "parallel")]
        {
            (0..height as usize)
                .into_par_iter()  // Rows ~100k tasks, but Rayon fuses small.
                .map(|y| {
                    let mut row_tiles = Vec::with_capacity(width as usize);
                    for x in 0..width {
                        if Instant::now().duration_since(start) >= opt_timeout {
                            return row_tiles;  // Early per-row—less waste.
                        }

                        let tile = tile_at(x, y as u64, seed);
                        let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                        row_tiles.push((pos, tile));
                    }
                    row_tiles
                })
                .collect_into_vec(&mut tiles);  // Drain to flat—no intermediate Vec<Vec>.
        }
        #[cfg(not(feature = "parallel"))]
        {
            for y in 0..height as usize {
                let mut row_tiles = Vec::with_capacity(width as usize);
                for x in 0..width {
                    if Instant::now().duration_since(start) >= opt_timeout {
                        break;  // Early per-row—less waste.
                    }

                    let tile = tile_at(x, y as u64, seed);
                    let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                    row_tiles.push((pos, tile));
                }
                tiles.extend(row_tiles);
            }
        }

        // Batch insert: O(n) extend vs per-insert hash.
        for (pos, tile) in tiles {
            chunk.insert(pos, tile);
        }

        let elapsed = start.elapsed();
        if elapsed >= opt_timeout { return Err(ConductorError::Timeout(elapsed)); }

        let total = chunk.len();  // Post-insert count.
        info!("conductor: 🧨 Generated {} tiles in {:.2?} (~{} tiles/sec)", total, elapsed, total as f64 / elapsed.as_secs_f64());  // Fixed: Native info!, with direct args—no &format! bloat.

        // Batch status: Metrics gold.
        self.push_status(&format!("Terrain gen: {} tiles @ {:.1} tiles/sec", total, total as f64 / elapsed.as_secs_f64()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::builder::DummyDelivery;  // Assume.

    #[test]
    fn enqueue_and_tick_basic() {
        let delivery = DummyDelivery::new();  // Fixed: Use default new()—no args.
        let streamer = ChunkStreamer::new(delivery, 16, 1024);  // Add est_cap for harmony.
        let mut conductor = Conductor::new(streamer);
        conductor.enqueue_batch(vec![
            ProcCommand::builder().emit_signal("test"),
            ProcCommand::builder().wait_ticks(1),
        ]);  // Builder returns ProcCommand directly—no .build() needed.

        let mut chunk = MapDataChunk::default();
        assert!(conductor.has_pending());
        conductor.tick(1, &mut chunk).unwrap();
        assert!(conductor.has_pending());  // Wait enqueued.
    }

    #[bench]
    fn bench_terrain_small(b: &mut test::Bencher) {  // Criterion later.
        let delivery = DummyDelivery::new();
        let streamer = ChunkStreamer::new(delivery, 16, 1024);
        let mut conductor = Conductor::new(streamer);
        let mut chunk = MapDataChunk::default();

        b.iter(|| {
            conductor.tick(0, &mut chunk).unwrap();  // Stub gen.
        });
    }
}