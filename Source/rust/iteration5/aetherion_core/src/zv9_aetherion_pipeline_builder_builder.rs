use aetherion_shared::zv9_prelude::*;
use crate::zv9_aetherion_generator_noise_config::{generate_grid_from_config, NoiseConfig};
use crate::zv9_aetherion_generator_noise::NoiseType;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};
use crate::zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;  // Fixed: Direct import from correct module path.
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use aetherion_shared::shared::SerializableVector2i;
use aetherion_shared::shared::EngineMessage;
use aetherion_shared::zv9_shared_pipeline_data_tile::TileInfo;

use std::thread;
use std::time::{Duration, Instant};
use anyhow::{Context, Result};  // Gold: Err elegy.
use log::{info, error};  // Native macros for consistent logging.

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MapBuilderConfig {
    batch_size: usize,
    progress_interval: usize,
    animate: bool,
}

impl MapBuilderConfig {
    pub fn new() -> Self { Self::default() }

    pub fn batch_size(mut self, bs: usize) -> Self {
        self.batch_size = bs.max(100).min(100_000);
        self
    }

    pub fn progress_interval(mut self, pi: usize) -> Self {
        self.progress_interval = pi.max(1).min(50);
        self
    }

    pub fn animate(mut self, a: bool) -> Self {
        self.animate = a;
        self
    }

    pub fn build<D>(self) -> impl Fn(
        &mut ChunkStreamer<D>,
        &NoiseConfig,
        NoiseType,
        SerializableVector2i,
        SerializableVector2i,
    ) -> Result<()>
    where
        D: ChunkDelivery + Send + Clone + 'static,
    {
        move |streamer: &mut ChunkStreamer<D>,
              config: &NoiseConfig,
              mode: NoiseType,
              black: SerializableVector2i,
              blue: SerializableVector2i| {
            spawn_map_builder_with(
                streamer,
                config,
                mode,
                self.animate,
                black,
                blue,
                self.batch_size,
                self.progress_interval,
            )
        }
    }
}

/// 🧵 Spawns: Lazy iter, batch extend—velocity vow.
pub fn spawn_map_builder<D: ChunkDelivery + Send + Clone + 'static>(
    streamer: &mut ChunkStreamer<D>,
    config: &NoiseConfig,
    mode: NoiseType,
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
) -> Result<()> {
    spawn_map_builder_with(streamer, config, mode, animate, black, blue, 10_000, 5)
}

fn spawn_map_builder_with<D: ChunkDelivery + Send + Clone + 'static>(
    streamer: &mut ChunkStreamer<D>,
    config: &NoiseConfig,
    mode: NoiseType,
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
    batch_size: usize,
    progress_interval: usize,
) -> Result<()> {
    if config.width == 0 || config.height == 0 {
        anyhow::bail!("Invalid config: Dims must >0");
    }

    let width = config.width;
    let height = config.height;
    let grid = generate_grid_from_config(config, mode).context("Grid gen failed")?;
    let total_tiles = (width * height) as usize;
    let use_parallel = total_tiles > 1000 && cfg!(feature = "parallel");

    let mut streamer = streamer.clone();
    let width_i32 = width as i32;
    let height_i32 = height as i32;
    let mode_str = mode.as_str().to_string();

    thread::spawn(move || -> Result<(), anyhow::Error> {
        let start_time = Instant::now();
        info!("builder: Spawning: {}x{}, mode={}", width_i32, height_i32, mode_str);

        let sync = streamer.sync();
        sync.add_signal(EngineMessage::Start);
        sync.add_signal(EngineMessage::Status("🧬 Building terrain...".into()));

        let positions_iter = (0..height_i32)
            .flat_map(move |y| (0..width_i32).map(move |x| SerializableVector2i { x, y }));

        let tiles: Vec<(SerializableVector2i, TileInfo)> = match std::panic::catch_unwind(|| -> Result<Vec<_>, anyhow::Error> {
            if use_parallel {
                #[cfg(feature = "parallel")]
                {
                    use rayon::iter::ParallelIterator;
                    let v = positions_iter
                        .par_chunks(batch_size)
                        .flat_map(|batch| {
                            let mut tiles_batch = Vec::with_capacity(batch.len());
                            for pos in batch {
                                let ux = pos.x as usize;
                                let uy = pos.y as usize;
                                let idx = uy * width + ux;
                                if idx >= grid.len() { continue; }

                                let is_land = grid[idx] == 1;
                                let atlas = if is_land { black.clone() } else { blue.clone() };
                                let tile = TileInfo {
                                    source_id: 0,
                                    atlas_coords: atlas,
                                    alternate_id: 0,
                                    flags: 0,
                                    layer: 0,
                                    rotation: 0,
                                    variant_id: None,
                                    frame_count: None,
                                    animation_speed: None,
                                };
                                tiles_batch.push((pos.clone(), tile));
                            }
                            tiles_batch
                        })
                        .collect::<Vec<_>>();
                    Ok(v)
                }
                #[cfg(not(feature = "parallel"))]
                {
                    anyhow::bail!("Parallel disabled—enable 'parallel' feature");
                }
            } else {
                let positions: Vec<_> = positions_iter.collect();
                let mut all_tiles = Vec::with_capacity(total_tiles);
                for batch in positions.chunks(batch_size) {
                    let mut tiles_batch = Vec::with_capacity(batch.len());
                    for pos in batch {
                        let ux = pos.x as usize;
                        let uy = pos.y as usize;
                        let idx = uy * width + ux;
                        if idx >= grid.len() { continue; }

                        let is_land = grid[idx] == 1;
                        let atlas = if is_land { black.clone() } else { blue.clone() };
                        let tile = TileInfo {
                            source_id: 0,
                            atlas_coords: atlas,
                            alternate_id: 0,
                            flags: 0,
                            layer: 0,
                            rotation: 0,
                            variant_id: None,
                            frame_count: None,
                            animation_speed: None,
                        };
                        tiles_batch.push((pos.clone(), tile));
                    }
                    all_tiles.extend(tiles_batch);
                }
                Ok(all_tiles)
            }
        }) {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => return Err(e),
            Err(panic) => {
                error!("builder: Panic in chunk gen: {:?}", panic);
                let sync = streamer.sync();
                sync.add_signal(EngineMessage::Status("❌ Builder panic—check logs.".into()));
                return Err(anyhow::anyhow!("Builder panic: {:?}", panic));
            }
        };

        let mut chunk = MapDataChunk::new();
        chunk.tiles.extend(tiles);

        let mut progress_emitted = 0;
        let num_batches = (total_tiles + batch_size - 1) / batch_size;
        for i in 0..num_batches {
            let done = ((i + 1) * batch_size).min(total_tiles);
            let percent = (done * 100 / total_tiles) as i32;
            if percent >= progress_emitted + progress_interval as i32 {
                progress_emitted = percent;
                let sync = streamer.sync();
                sync.add_signal(EngineMessage::Progress(percent));
            }

            let sync = streamer.sync();
            sync.add_signal(EngineMessage::Chunk(chunk.clone()));

            streamer.enqueue_chunk(chunk.clone());
            if streamer.has_pending() {
                thread::sleep(Duration::from_millis(1));
            }
        }

        let duration = start_time.elapsed().as_secs_f64();
        let tiles_per_sec = total_tiles as f64 / duration;
        let sync = streamer.sync();
        sync.add_signal(EngineMessage::Progress(100));
        sync.add_signal(EngineMessage::Status("✅ Terrain generation complete.".into()));
        sync.add_signal(EngineMessage::Complete {
            width: width_i32,
            height: height_i32,
            mode: mode_str,
            animate,
            duration,
        });
        sync.add_signal(EngineMessage::Status(format!("⚡ {} tiles/sec", tiles_per_sec as i32).into()));

        info!("builder: Done: {:.3}s, {} tiles/sec", duration, tiles_per_sec);
        Ok(())
    });

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::test::Bencher;

    #[test]
    fn small_spawn() {
        let config = NoiseConfig::default();
        let delivery = DummyDelivery::new();  // Use DummyDelivery for mocking.
        let streamer = ChunkStreamer::new(delivery, 16, 1024);  // Provide required args.
        let mut streamer_ref = streamer;
        spawn_map_builder(
            &mut streamer_ref,
            &config,
            NoiseType::Basic,
            false,
            SerializableVector2i::default(),
            SerializableVector2i::default(),
        )
        .unwrap();
    }

    #[bench]
    fn bench_spawn_256(b: &mut Bencher) {
        let config = NoiseConfig::builder().width(256).height(256).build();
        let delivery = DummyDelivery::new();
        let streamer = ChunkStreamer::new(delivery, 16, 1024);
        let mut streamer_ref = streamer;
        let black = SerializableVector2i { x: 0, y: 0 };
        let blue = SerializableVector2i { x: 1, y: 0 };

        b.iter(|| {
            spawn_map_builder(
                &mut streamer_ref,
                &config,
                NoiseType::CellularAutomata,
                false,
                black,
                blue,
            )
            .unwrap();
        });
    }
}