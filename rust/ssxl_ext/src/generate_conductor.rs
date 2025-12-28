use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use flume::Receiver;

use crate::config::GlobalConfig;
use crate::generate_conductor_state::{
    ConductorState, ConductorStateContainer, GenerationMetrics,
};
use crate::shared_chunk::Chunk;
use crate::shared_error::SSXLCoreError;
use crate::shared_job::GenerationJob;
use crate::shared_math::ChunkCoords;
use crate::shared_message::GenerationDataMessage;
use crate::shared_types::InstanceType;
use crate::sync_pool::SyncPool;

pub struct GenerateConductor {
    state_container: ConductorStateContainer,
    thread_pool: SyncPool,
    chunk_receiver: Receiver<GenerationDataMessage>,

    /// Stores generated chunks so CA or other systems can access neighbors.
    chunk_cache: HashMap<ChunkCoords, Chunk>,

    /// Target TileMap/renderer ID.
    pub tilemap_target_id: InstanceType,

    signal_emitted: AtomicBool,
}

impl GenerateConductor {
    pub fn new(num_workers: u32, config: Arc<GlobalConfig>) -> Self {
        let (pool, chunk_receiver) = SyncPool::new(num_workers as usize, config);
        let state_container = ConductorStateContainer::new();

        Self {
            state_container,
            thread_pool: pool,
            chunk_receiver,
            chunk_cache: HashMap::new(),
            tilemap_target_id: 1,
            signal_emitted: AtomicBool::new(false),
        }
    }

    pub fn start_generation(
        &mut self,
        tilemap_id: InstanceType,
        initial_jobs: Vec<GenerationJob>,
    ) -> Result<(), SSXLCoreError> {
        if self.state_container.get_state() != ConductorState::Ready {
            eprintln!("WARN: Conductor is not ready to start generation.");
            return Err(SSXLCoreError::ConductorBusy);
        }

        self.signal_emitted.store(false, Ordering::SeqCst);
        self.tilemap_target_id = tilemap_id;

        let total_chunks = initial_jobs.len() as u32;
        self.state_container.set_total_chunks(total_chunks);
        self.state_container.transition_to(ConductorState::Generating);

        for job in initial_jobs {
            if let Err(e) = self.thread_pool.submit_job(job) {
                eprintln!("ERROR: Failed to submit job to worker pool: {:?}", e);
                return Err(SSXLCoreError::ChannelSendError(e.to_string()));
            }
        }

        // One-time lifecycle log
        eprintln!("INFO: Generation started.");
        Ok(())
    }

    pub fn get_metrics(&self) -> GenerationMetrics {
        self.state_container.get_metrics()
    }

    pub fn try_finalize_and_get_target_id(&self) -> Option<InstanceType> {
        if self.state_container.get_state() != ConductorState::Finished {
            return None;
        }

        match self.signal_emitted.swap(true, Ordering::SeqCst) {
            false => {
                if self.tilemap_target_id != 0 {
                    Some(self.tilemap_target_id)
                } else {
                    eprintln!("ERROR: Conductor cannot finalize: TileMap ID is invalid.");
                    None
                }
            }
            true => None,
        }
    }

    pub fn shutdown(self) {
        eprintln!("INFO: GenerateConductor shutting down.");
    }

    pub fn get_state_container(&self) -> &ConductorStateContainer {
        &self.state_container
    }

    // GODOT-BINDING VERSION (direct tilemap writes)
    #[cfg(feature = "godot-binding")]
    pub fn poll_chunks_and_render(&mut self) -> (u32, bool) {
        use crate::host_render::render_available_chunks;

        let (chunks_rendered, is_finished) = render_available_chunks(
            self.tilemap_target_id,
            &self.chunk_receiver,
            &self.state_container,
        );

        if is_finished && self.state_container.get_state() == ConductorState::Generating {
            self.state_container.transition_to(ConductorState::Finished);
            self.chunk_cache.clear();
            eprintln!("INFO: All generation jobs completed.");
        }

        (chunks_rendered, is_finished)
    }

    // NON-GODOT VERSION (cache only)
    #[cfg(not(feature = "godot-binding"))]
    pub fn poll_chunks_and_render(&mut self) -> (u32, bool) {
        let mut chunks_rendered = 0;

        while let Ok(message) = self.chunk_receiver.try_recv() {
            match message {
                GenerationDataMessage::CompletedChunk(chunk) => {
                    self.chunk_cache.insert(chunk.position, chunk);
                    chunks_rendered += 1;
                    self.state_container.increment_completed_chunks();
                }
                GenerationDataMessage::JobFailure(e) => {
                    eprintln!("ERROR: Worker job failed: {:?}", e);
                }
                GenerationDataMessage::Ack => {
                    // No spam — silently ignore
                }
            }
        }

        let metrics = self.state_container.get_metrics();
        let is_finished =
            metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;

        if is_finished && self.state_container.get_state() == ConductorState::Generating {
            self.state_container.transition_to(ConductorState::Finished);
            self.chunk_cache.clear();
            eprintln!("INFO: All generation jobs completed.");
        }

        (chunks_rendered, is_finished)
    }

    pub fn get_chunk(&self, coords: ChunkCoords) -> Option<&Chunk> {
        self.chunk_cache.get(&coords)
    }
}
