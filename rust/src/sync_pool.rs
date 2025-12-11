// rust/SSXL-ext/src/sync_pool.rs

use std::thread;
use std::sync::Arc;
use flume::{Receiver, Sender};

// Shared types used for concurrent operations
use crate::shared_job::GenerationJob; 
use crate::shared_message::GenerationDataMessage;
use crate::config::GlobalConfig;
use crate::generate_runtime;

// --- FIX: Import logging macros from the crate root ---
use crate::{ssxl_info, ssxl_error};
use crate::tools; // Keeping the tools module for future utilities if needed


// --------------------------------------------------------------------------
// --- Worker Structures ---
// --------------------------------------------------------------------------

/// A wrapper around a dedicated worker thread handle.
struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Spawns a new thread and starts the worker's execution loop.
    fn new(
        id: usize, 
        task_receiver: Arc<Receiver<GenerationJob>>, 
        chunk_sender: Sender<GenerationDataMessage>,
        // The worker needs a reference to the global configuration
        config: Arc<GlobalConfig>,
    ) -> Worker {
        // Clone the necessary handles for the new thread
        let sender_clone = chunk_sender.clone();
        
        let handle = thread::spawn(move || {
            // Loop until the channel is disconnected (which signals shutdown)
            while let Ok(job) = task_receiver.recv() {
                
                // --- EXECUTE FULL GENERATION RUNTIME ---
                // The dedicated runtime orchestrator handles all steps (Perlin, CA, etc.)
                let final_result = generate_runtime::run_generation_job(job, &config.generation);

                // --- FINISHER DELIVERY ---
                // Send the final result (Completed Chunk or Failure Message) back to the Conductor.
                let message = match final_result {
                    Ok(completed_chunk) => {
                        ssxl_info!("Worker {} completed job {:?}", id, completed_chunk.position);
                        GenerationDataMessage::CompletedChunk(completed_chunk)
                    },
                    Err(e) => {
                        ssxl_error!("Worker {} failed job: {:?}", id, e);
                        GenerationDataMessage::JobFailure(e)
                    }
                };

                // Non-blocking delivery of the result back to the Conductor's main thread poller.
                if let Err(e) = sender_clone.send(message) {
                    // The main thread is no longer listening; break the loop and shut down.
                    ssxl_error!("Worker {} failed to send result: {}. Conductor likely shut down.", id, e);
                    break; 
                }
            }
            ssxl_info!("Worker {} shutting down.", id);
        });

        Worker { id, handle: Some(handle) }
    }
}


// --------------------------------------------------------------------------
// --- ThreadPool Structure ---
// --------------------------------------------------------------------------

/// Manages the pool of worker threads dedicated to procedural generation.
pub struct ThreadPool {
    workers: Vec<Worker>,
    // The pool holds the sender end of the channel for task submission
    task_sender: Sender<GenerationJob>,
    // The pool sends completed messages back to the Conductor
    chunk_sender: Sender<GenerationDataMessage>,
    // This handle ensures the Receiver stays alive while the workers are running.
    _task_receiver_final_handle: Receiver<GenerationJob>,
}

impl ThreadPool {
    /// Creates a new ThreadPool and spawns the specified number of workers.
    /// Returns the Conductor's receiving channel for completed work.
    /// 
    /// Returns: (ThreadPool instance, Receiver for completed chunks)
    pub fn new(num_workers: usize, config: Arc<GlobalConfig>) -> (Self, Receiver<GenerationDataMessage>) {
        
        // --- 1. Setup Channels ---
        // (MPSC) Worker Submission Channel
        let (task_sender, task_receiver_final_handle) = flume::unbounded();
        // The workers will share ownership of the receiver end
        let task_receiver_arc = Arc::new(task_receiver_final_handle.clone());
        
        // (SPSC) Finisher Delivery Channel (Chunk/Error results back to Conductor)
        let (chunk_sender, chunk_receiver) = flume::unbounded();

        // --- 2. Spawn Workers ---
        let mut workers = Vec::with_capacity(num_workers);
        for id in 0..num_workers {
            workers.push(Worker::new(
                id,
                Arc::clone(&task_receiver_arc),
                chunk_sender.clone(),
                Arc::clone(&config),
            ));
        }

        ssxl_info!("Started ThreadPool with {} dedicated workers.", num_workers);
        
        let pool = ThreadPool { 
            workers, 
            task_sender, 
            chunk_sender,
            _task_receiver_final_handle: task_receiver_final_handle,
        };
        
        // Return the pool instance and the channel the Conductor will listen on
        (pool, chunk_receiver)
    }
    
    /// Submits a new generation job to the worker pool.
    pub fn submit_job(&self, job: GenerationJob) -> Result<(), flume::SendError<GenerationJob>> {
        self.task_sender.send(job)
    }

    /// Signals workers to stop and waits for all threads to finish (clean shutdown).
    pub fn shutdown(self) {
        // Drop all senders and the receiver handle. This signals to workers to exit their loops.
        drop(self.task_sender);
        drop(self._task_receiver_final_handle);
        drop(self.chunk_sender);
        
        // Wait for all worker threads to finish.
        for mut worker in self.workers.into_iter() {
            if let Some(handle) = worker.handle.take() {
                if let Err(e) = handle.join() {
                    ssxl_error!("Worker {} thread join failed: {:?}", worker.id, e);
                }
            }
        }
        ssxl_info!("ThreadPool shut down successfully.");
    }
}