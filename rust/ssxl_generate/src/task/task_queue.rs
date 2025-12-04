// ssxl_generate/src/task/task_queue.rs

use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use tracing::{info, error, warn, debug};
use std::collections::HashMap;
use std::sync::Arc;

use ssxl_math::prelude::Vec2i;
use ssxl_math::coordinate_system::ChunkKey;
use glam::I64Vec3;

use ssxl_cache::ChunkCache;
use ssxl_shared::CHUNK_SIZE as SHARED_CHUNK_SIZE;

use crate::Generator;
use crate::conductor::conductor_state;

pub use ssxl_shared::message::generation_message::{GenerationMessage, GenerationTask};

type DynGenerator = Box<dyn Generator + Send + Sync>;
pub const CHUNK_SIZE: i64 = SHARED_CHUNK_SIZE as i64;

/// Handles a single chunk request: cache check → generate → cache → send result
pub fn handle_chunk_unit(
    chunk_coords: Vec2i,
    generator_name: &str,
    generators: &HashMap<String, Arc<DynGenerator>>,
    chunk_cache: &Arc<ChunkCache>,
    progress_sender: &mpsc::Sender<GenerationMessage>,
    conductor_state: &Arc<conductor_state::ConductorState>,
) {
    let chunk_key = ChunkKey(I64Vec3 {
        x: chunk_coords.x,
        y: chunk_coords.y,
        z: 0,
    });

    // 1. Cache HIT
    if let Some(chunk_data_arc) = chunk_cache.load_chunk(&chunk_key) {
        debug!(?chunk_coords, "Cache HIT");

        let msg = GenerationMessage::Generated(chunk_coords, chunk_data_arc);
        if progress_sender.try_send(msg).is_err() {
            warn!(?chunk_coords, "Failed to send cached chunk (channel full/closed)");
        }
        return;
    }

    // 2. Cache MISS → generate
    debug!(?chunk_coords, generator = %generator_name, "Cache MISS → generating");

    let generator = generators
        .get(generator_name)
        .expect("Generator must exist in map");

    // ChunkData is returned directly from generate_chunk()
    let chunk_data = generator.generate_chunk(chunk_coords);
    let tile_count = chunk_data.tiles.len() as u64;
    conductor_state.increment_tile_count(tile_count);

    let chunk_data_arc = Arc::new(chunk_data);

    // 3. Save to cache
    if chunk_cache.save_chunk(&chunk_key, chunk_data_arc.clone()).is_err() {
        error!(?chunk_coords, "Failed to save generated chunk to cache");
    }

    // 4. Send result
    let msg = GenerationMessage::Generated(chunk_coords, chunk_data_arc);
    if progress_sender.try_send(msg).is_err() {
        warn!(?chunk_coords, "Failed to send generated chunk (channel full/closed)");
    } else {
        debug!(?chunk_coords, "Sent newly generated chunk");
    }
}

/// Main async loop: receives tasks → spawns blocking generation
pub fn start_request_loop(
    rt_handle: Handle,
    mut request_rx: mpsc::UnboundedReceiver<GenerationTask>,
    progress_tx: mpsc::Sender<GenerationMessage>,
    generators: Arc<HashMap<String, Arc<DynGenerator>>>,
    chunk_cache: Arc<ChunkCache>,
    conductor_state: Arc<conductor_state::ConductorState>,
) {
    rt_handle.spawn(async move {
        info!("Generation Task Queue started");

        let mut active_tasks: Vec<JoinHandle<()>> = Vec::new();

        while let Some(task) = request_rx.recv().await {
            if !conductor_state.as_ref().is_active() {
                warn!(?task.chunk_coords, "Dropping task — Conductor not active");
                continue;
            }

            let progress_tx = progress_tx.clone();
            let generators = generators.clone();
            let cache = chunk_cache.clone();
            let state = conductor_state.clone();

            let handle = tokio::task::spawn_blocking(move || {
                handle_chunk_unit(
                    task.chunk_coords,
                    &task.generator_id,
                    &generators,
                    &cache,
                    &progress_tx,
                    &state,
                );
            });

            active_tasks.push(handle);
        }

        info!("Request channel closed. Draining {} active tasks...", active_tasks.len());

        for handle in active_tasks {
            if let Err(e) = handle.await {
                error!("Generation task panicked: {:?}", e);
            }
        }

        let _ = progress_tx.send(GenerationMessage::GenerationComplete).await;
        info!("Generation Task Queue shut down cleanly");
    });
}