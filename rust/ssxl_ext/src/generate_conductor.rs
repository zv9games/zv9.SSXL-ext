use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::collections::HashMap;

use flume::Receiver;

use crate::config::GlobalConfig;
use crate::sync_pool::SyncPool;

use crate::shared_message::GenerationDataMessage;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;

use crate::generate_conductor_state::{
    ConductorState,
    ConductorStateContainer,
    GenerationMetrics,
};

use crate::shared_math::ChunkCoords;
use crate::shared_chunk::Chunk;
use crate::shared_types::InstanceType;


pub struct GenerateConductor {
    state_container: ConductorStateContainer,
    thread_pool: SyncPool,
    chunk_receiver: Receiver<GenerationDataMessage>,

    /// NEW: stores generated chunks so CA can access neighbors.
    chunk_cache: HashMap<ChunkCoords, Chunk>,

    pub tilemap_target_id: InstanceType,   // ✅ now resolves correctly
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

            // ✅ NEW
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

		let mut submitted_count = 0;
		for job in initial_jobs {
			match self.thread_pool.submit_job(job) {
				Ok(_) => submitted_count += 1,
				Err(e) => {
					eprintln!("ERROR: Failed to submit job to worker pool: {:?}", e);
					return Err(SSXLCoreError::ChannelSendError(e.to_string()));
				}
			}
		}

		eprintln!("INFO: Generation started, submitted {} chunks.", submitted_count);
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
				let is_valid_id = self.tilemap_target_id != 0;

				if is_valid_id {
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
		eprintln!("INFO: GenerateConductor initiating shutdown (pool cleanup deferred to HostCleanup)...");
		eprintln!("INFO: GenerateConductor shut down successfully.");
	}

	pub fn get_state_container(&self) -> &ConductorStateContainer {
		&self.state_container
	}

    pub fn poll_chunks_and_render(&mut self) -> (u32, bool) {
        let mut chunks_rendered = 0;

        while let Ok(message) = self.chunk_receiver.try_recv() {
            match message {
                GenerationDataMessage::CompletedChunk(chunk) => {
                    let coords = chunk.position;

                    // ✅ NEW: store chunk for neighbor-aware CA
                    self.chunk_cache.insert(coords, chunk.clone());

                    // existing rendering logic
                    match crate::host_tilemap::render_chunk_direct(
                        self.tilemap_target_id,
                        chunk,
                    ) {
                        Ok(_) => {
                            chunks_rendered += 1;
                            self.state_container.increment_completed_chunks();
                        }
                        Err(e) => {
                            eprintln!("ERROR: Failed to directly render chunk: {:?}", e);
                        }
                    }
                }

                GenerationDataMessage::JobFailure(e) => {
                    eprintln!("ERROR: A worker job failed: {:?}", e);
                }

                GenerationDataMessage::Ack => {
                    eprintln!("WARN: Conductor received unexpected Ack message.");
                }
            }
        }

        let metrics = self.state_container.get_metrics();
        let is_finished =
            metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;

        if is_finished && self.state_container.get_state() == ConductorState::Generating {
            self.state_container.transition_to(ConductorState::Finished);

            // ✅ NEW: clear cache after generation is complete
            self.chunk_cache.clear();

            eprintln!("INFO: All chunks rendered. Generation finished.");
        }

        (chunks_rendered, is_finished)
    }

    /// NEW: expose neighbor lookup for CA
    pub fn get_chunk(&self, coords: ChunkCoords) -> Option<&Chunk> {
        self.chunk_cache.get(&coords)
    }
}
