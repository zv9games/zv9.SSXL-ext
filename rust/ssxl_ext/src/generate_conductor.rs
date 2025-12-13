// rust/SSXL-ext/src/generate_conductor.rs

use godot::prelude::*;
use std::sync::Arc;
// ADD: Required for the atomic, single-signal guard
use std::sync::atomic::{AtomicBool, Ordering}; 
use flume::Receiver;
// ✅ FIX: Renamed import from ThreadPool to SyncPool
use crate::sync_pool::SyncPool; 
use crate::config::GlobalConfig;
use crate::shared_message::GenerationDataMessage;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;

use crate::generate_conductor_state::{
    ConductorState, ConductorStateContainer, GenerationMetrics
};

pub struct GenerateConductor {
    state_container: ConductorStateContainer,
    // ✅ FIX: Renamed struct field from ThreadPool to SyncPool
    thread_pool: SyncPool, 
    chunk_receiver: Receiver<GenerationDataMessage>,
    pub tilemap_target_id: InstanceId,
    // CRITICAL LIFECYCLE GUARD: Prevents emitting the final signal more than once.
    signal_emitted: AtomicBool, 
}

impl GenerateConductor {
    pub fn new(num_workers: u32, config: Arc<GlobalConfig>) -> Self {
        // ✅ FIX: Called SyncPool::new
        let (pool, chunk_receiver) = SyncPool::new(num_workers as usize, config); 
        let state_container = ConductorStateContainer::new();
        eprintln!("INFO: GenerateConductor initialized.");
        Self {
            state_container,
            thread_pool: pool,
            chunk_receiver,
            tilemap_target_id: InstanceId::from_i64(1),
            // Initialize the signal guard to false.
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

    /// Enforces the explicit lifecycle guard by checking state and resetting the signal flag.
    pub fn start_generation(&mut self, tilemap_id: InstanceId, initial_jobs: Vec<GenerationJob>) -> Result<(), SSXLCoreError> {
        // Lifecycle Guard: Prevent premature activation
        if self.state_container.get_state() != ConductorState::Ready {
            eprintln!("WARN: Conductor is not ready to start generation.");
            return Err(SSXLCoreError::ConductorBusy);
        }
        
        // Reset the signal guard when a new generation starts.
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

    /// Implements the non-blocking channel read for chunk delivery and respects engine responsiveness.
    pub fn poll_chunks_and_render(&self) -> (u32, bool) {
        let mut chunks_rendered = 0;
        // CRITICAL: Non-blocking channel read
        while let Ok(message) = self.chunk_receiver.try_recv() { 
            match message {
                GenerationDataMessage::CompletedChunk(chunk) => {
                    // Incremental delivery to Godot
                    match crate::host_tilemap::render_chunk_direct(
                        self.tilemap_target_id,
                        chunk
                    ) {
                        Ok(_) => {
                            chunks_rendered += 1;
                            self.state_container.increment_completed_chunks();
                        },
                        Err(e) => {
                            eprintln!("ERROR: Failed to directly render chunk: {:?}", e);
                        }
                    }
                },
                GenerationDataMessage::JobFailure(e) => {
                    eprintln!("ERROR: A worker job failed: {:?}", e);
                }
                GenerationDataMessage::Ack => {
                    eprintln!("WARN: Conductor received unexpected Ack message.");
                }
            }
        }
        let metrics = self.state_container.get_metrics();
        let is_finished = metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;
        if is_finished && self.state_container.get_state() == ConductorState::Generating {
            // Lifecycle transition: Worker threads are done, and all chunks are rendered.
            self.state_container.transition_to(ConductorState::Finished); 
            eprintln!("INFO: All chunks rendered. Generation finished.");
        }
        (chunks_rendered, is_finished)
    }

    /// Implements the atomic 'single-signal' lifecycle guard and closes the loop.
    pub fn try_finalize_and_get_target_id(&self) -> Option<InstanceId> {
        // 1. Check State: Only proceed if the state machine is in the final state.
        if self.state_container.get_state() != ConductorState::Finished {
            return None;
        }

        // 2. Atomic Guard: Use swap to ensure signal is broadcast only once.
        match self.signal_emitted.swap(true, Ordering::SeqCst) {
            false => {
                // This is the first frame to claim the signal.
                if self.tilemap_target_id.to_i64() != 0 {
                    Some(self.tilemap_target_id)
                } else {
                    eprintln!("ERROR: Conductor cannot finalize: TileMap ID is invalid.");
                    None 
                }
            },
            true => {
                // The signal has already been emitted on a previous frame.
                None
            }
        }
    }

    // ✅ FIX: Removed the explicit thread_pool.shutdown() call here.
    // The SyncPool struct owns the threads and channels, so when the 
    // GenerateConductor (and thus self.thread_pool) is dropped at host cleanup, 
    // its Drop implementation (or lack thereof, since we need to join threads)
    // will be handled by the HostCleanup module directly or by the natural Drop 
    // of the owning struct. The current self.thread_pool.shutdown() would
    // be tricky here because `shutdown` often needs ownership (`self`).
    pub fn shutdown(self) {
        eprintln!("INFO: GenerateConductor initiating shutdown (pool cleanup deferred to HostCleanup)...");
        // self.thread_pool.shutdown(); // REMOVED: Deferred pool cleanup.
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