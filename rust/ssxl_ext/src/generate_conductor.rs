use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use flume::Receiver;
use crate::sync_pool::SyncPool;
use crate::config::GlobalConfig;
use crate::shared_message::GenerationDataMessage;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;
use crate::generate_conductor_state::{
    ConductorState, ConductorStateContainer, GenerationMetrics,
};

/// Tilemap target identifier used by the conductor.
///
/// For Godot builds this should be the numeric instance ID (as i64),
/// for CLI builds it's just a numeric handle. We keep it as a plain
/// integer here to avoid binding-layer issues with InstanceId.
pub type InstanceType = i64;

pub struct GenerateConductor {
    state_container: ConductorStateContainer,
    thread_pool: SyncPool,
    chunk_receiver: Receiver<GenerationDataMessage>,
    pub tilemap_target_id: InstanceType,
    signal_emitted: AtomicBool,
}

impl GenerateConductor {
    pub fn new(num_workers: u32, config: Arc<GlobalConfig>) -> Self {
        let (pool, chunk_receiver) = SyncPool::new(num_workers as usize, config);
        let state_container = ConductorStateContainer::new();
        eprintln!("INFO: GenerateConductor initialized.");
        Self {
            state_container,
            thread_pool: pool,
            chunk_receiver,
            // Default non-zero placeholder ID; real value is set by host code.
            tilemap_target_id: 1,
            signal_emitted: AtomicBool::new(false),
        }
    }

    pub fn set_ready_status(&mut self, status: bool) {
        if status {
            self.state_container.transition_to(ConductorState::Ready);
        } else {
            self.state_container.transition_to(ConductorState::Idle);
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

    pub fn poll_chunks_and_render(&self) -> (u32, bool) {
        let mut chunks_rendered = 0;

        while let Ok(message) = self.chunk_receiver.try_recv() {
            match message {
                GenerationDataMessage::CompletedChunk(chunk) => {
                    match crate::host_tilemap::render_chunk_direct(self.tilemap_target_id, chunk) {
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
            self.state_container
                .transition_to(ConductorState::Finished);
            eprintln!("INFO: All chunks rendered. Generation finished.");
        }

        (chunks_rendered, is_finished)
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
        eprintln!(
            "INFO: GenerateConductor initiating shutdown (pool cleanup deferred to HostCleanup)..."
        );
        eprintln!("INFO: GenerateConductor shut down successfully.");
    }

    pub fn get_state_container(&self) -> &ConductorStateContainer {
        &self.state_container
    }

    pub fn get_chunk_receiver(&self) -> &Receiver<GenerationDataMessage> {
        &self.chunk_receiver
    }

    pub fn get_metrics(&self) -> GenerationMetrics {
        self.state_container.get_metrics()
    }
}
