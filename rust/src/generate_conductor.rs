// rust/SSXL-ext/src/generate_conductor.rs

use godot::prelude::*;
use std::sync::Arc;
use flume::{Receiver, Sender}; // For safe, multi-threaded communication

use crate::sync_pool::ThreadPool; // The worker pool manager
// --- FIX 1: Import GlobalConfig from the correct module 'config' ---
use crate::config::GlobalConfig; 
use crate::shared_message::GenerationDataMessage; 
use crate::shared_chunk::Chunk; // Note: Chunk is not directly used in the code shown, but may be used in render_chunk_direct.
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;

// --- FIX: Import logging macros from the crate root ---
use crate::{ssxl_info, ssxl_warn, ssxl_error};
use crate::tools; // Keep tools module use if other utilities reside there

use crate::generate_conductor_state::{
    ConductorState, ConductorStateContainer, GenerationMetrics
};

// --------------------------------------------------------------------------
// --- GenerateConductor Structure ---
// --------------------------------------------------------------------------

/// The main orchestrator for procedural generation.
/// Runs on the Godot main thread and handles communication with the background workers.
pub struct GenerateConductor {
    // Shared, thread-safe access to the state machine
    state_container: ConductorStateContainer, 
    
    // The worker pool manager, responsible for task submission and shutdown
    thread_pool: ThreadPool,

    // The channel where completed Chunks are delivered from worker threads.
    // This is the SPSC receiver the Conductor polls every tick.
    chunk_receiver: Receiver<GenerationDataMessage>, 
    
    // The InstanceID of the TileMap we are targeting (critical for host_tilemap.rs)
    pub tilemap_target_id: InstanceId,
}

impl GenerateConductor {
    /// Initializes the Conductor, starting the worker thread pool.
    /// 
    /// NOTE: The `ThreadPool::new` handles the creation of the necessary channels
    /// for job submission (sender) and chunk delivery (receiver).
    // FIX 2: Use the imported GlobalConfig directly.
    pub fn new(num_workers: u32, config: Arc<GlobalConfig>) -> Self {
        
        // 1. Initialize the ThreadPool (which gives us the chunk receiver)
        let (pool, chunk_receiver) = ThreadPool::new(num_workers as usize, config);

        // 2. Initialize the Conductor State
        let state_container = ConductorStateContainer::new();

        ssxl_info!("GenerateConductor initialized.");
        
        Self { 
            state_container,
            thread_pool: pool,
            chunk_receiver,
            // FIX 1: InstanceId::default() is not found. Use InstanceId::from_i64(0)
            // ID is zero/invalid until generation starts
            tilemap_target_id: InstanceId::from_i64(0), 
        }
    }

    /// Starts the generation process by delegating to the thread pool.
    pub fn start_generation(&mut self, tilemap_id: InstanceId, initial_jobs: Vec<GenerationJob>) -> Result<(), SSXLCoreError> {
        // FIX 2 & 3: ConductorState::Ready (fixed in last step) and SSXLCoreError::ConductorBusy must exist.
        if self.state_container.get_state() != ConductorState::Ready {
            ssxl_warn!("Conductor is not ready to start generation.");
            return Err(SSXLCoreError::ConductorBusy);
        }

        self.tilemap_target_id = tilemap_id;

        // 1. Update State and Metrics
        let total_chunks = initial_jobs.len() as u32;
        self.state_container.set_total_chunks(total_chunks);
        self.state_container.transition_to(ConductorState::Generating);

        // 2. Queue Tasks
        let mut submitted_count = 0;
        for job in initial_jobs {
            match self.thread_pool.submit_job(job) {
                Ok(_) => submitted_count += 1,
                Err(e) => {
                    ssxl_error!("Failed to submit job to worker pool: {:?}", e);
                    return Err(SSXLCoreError::ChannelSendError(e.to_string()));
                }
            }
        }
        
        ssxl_info!("Generation started, submitted {} chunks.", submitted_count);
        Ok(())
    }
    
    /// Polls the completed chunk channel and initiates the direct write.
    /// This function MUST be called only on the Godot main thread.
    /// 
    /// Returns: (chunks_rendered_this_frame, is_generation_finished)
    pub fn poll_chunks_and_render(&self) -> (u32, bool) {
        let mut chunks_rendered = 0;

        // Use a non-blocking try_recv loop to process all available chunks 
        while let Ok(message) = self.chunk_receiver.try_recv() {
            match message {
                GenerationDataMessage::CompletedChunk(chunk) => {
                    // 1. Execute the Direct Write to the Godot TileMap
                    match crate::host_tilemap::render_chunk_direct(
                        self.tilemap_target_id, 
                        chunk
                    ) {
                        Ok(_) => {
                            chunks_rendered += 1;
                            // 2. Update Metrics
                            self.state_container.increment_completed_chunks();
                        },
                        Err(e) => {
                            ssxl_error!("Failed to directly render chunk: {:?}", e);
                        }
                    }
                },
                GenerationDataMessage::JobFailure(e) => {
                    ssxl_error!("A worker job failed: {:?}", e);
                }
                // FIX: Handle the missing Ack variant. Log a warning and continue.
                GenerationDataMessage::Ack => {
                    ssxl_warn!("Conductor received unexpected Ack message.");
                }
            }
        }
        
        // 3. Check for Completion
        let metrics = self.state_container.get_metrics();
        let is_finished = metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;
        
        if is_finished && self.state_container.get_state() == ConductorState::Generating {
            // Final state flip
            self.state_container.transition_to(ConductorState::Finished);
            ssxl_info!("All chunks rendered. Generation finished.");
        }

        (chunks_rendered, is_finished)
    }

    /// Gracefully closes all channels and shuts down the thread pool.
    pub fn shutdown(self) {
        ssxl_info!("GenerateConductor initiating shutdown...");
        
        // The Conductor owns the ThreadPool, so calling its shutdown method 
        // handles channel closing and thread joining.
        self.thread_pool.shutdown();
        
        ssxl_info!("GenerateConductor shut down successfully.");
    }

    // --- Accessors for Poller/HostState ---

    /// Gets a thread-safe reference to the state container.
    pub fn get_state_container(&self) -> &ConductorStateContainer {
        &self.state_container
    }

    /// Gets a reference to the chunk receiver (for polling logic in host_poller).
    pub fn get_chunk_receiver(&self) -> &Receiver<GenerationDataMessage> {
        &self.chunk_receiver
    }

    /// Gets the current generation metrics.
    pub fn get_metrics(&self) -> GenerationMetrics {
        self.state_container.get_metrics()
    }
}