// rust/SSXL-ext/src/sync_pool.rs

use std::thread;
use std::sync::Arc;
use flume::{Receiver, Sender};

// Shared types used for concurrent operations
use crate::shared_job::GenerationJob; 
use crate::shared_message::GenerationDataMessage;
use crate::config::GlobalConfig;
use crate::generate_runtime;

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
        let sender_clone = chunk_sender.clone();
        
        let handle = thread::spawn(move || {
            eprintln!("[worker {}] spawned and entering loop.", id);

            // Loop until the channel is disconnected (which signals shutdown)
            while let Ok(job) = task_receiver.recv() {
                eprintln!(
                    "[worker {}] received job for chunk {:?}, step {:?}",
                    id,
                    job.id,
                    job.current_step
                );

                // --- EXECUTE FULL GENERATION RUNTIME ---
                eprintln!(
                    "[worker {}] starting run_generation_job for chunk {:?}",
                    id,
                    job.id
                );
                let final_result = generate_runtime::run_generation_job(job, &config.generation);
                eprintln!(
                    "[worker {}] run_generation_job finished with result: {:?}",
                    id,
                    final_result.as_ref().map(|c| c.position)
                );

                // --- FINISHER DELIVERY ---
                let message = match final_result {
                    Ok(completed_chunk) => {
                        eprintln!(
                            "[worker {}] completed job for chunk {:?}",
                            id,
                            completed_chunk.position
                        );
                        GenerationDataMessage::CompletedChunk(completed_chunk)
                    }
                    Err(e) => {
                        eprintln!("[worker {}] failed job: {:?}", id, e);
                        GenerationDataMessage::JobFailure(e)
                    }
                };

                // Non-blocking delivery of the result back to the Conductor's main thread poller.
                if let Err(e) = sender_clone.send(message) {
                    eprintln!(
                        "[worker {}] failed to send result: {}. Conductor likely shut down.",
                        id,
                        e
                    );
                    break; 
                }

                eprintln!("[worker {}] finished cycle and waiting for next job.", id);
            }

            eprintln!("[worker {}] shutting down (task channel closed).", id);
        });

        Worker { id, handle: Some(handle) }
    }
}


// --------------------------------------------------------------------------
// --- ThreadPool Structure ---
// --------------------------------------------------------------------------

/// Manages the pool of worker threads dedicated to procedural generation.
pub struct SyncPool {
    workers: Vec<Worker>,
    // The pool holds the sender end of the channel for task submission
    task_sender: Sender<GenerationJob>,
    // The pool sends completed messages back to the Conductor
    chunk_sender: Sender<GenerationDataMessage>,
    // This handle ensures the Receiver stays alive while the workers are running.
    _task_receiver_final_handle: Receiver<GenerationJob>,
}

impl SyncPool {
    /// Creates a new SyncPool and spawns the specified number of workers.
    /// Returns the Conductor's receiving channel for completed work.
    ///
    /// Returns: (SyncPool instance, Receiver for completed chunks)
    pub fn new(num_workers: usize, config: Arc<GlobalConfig>) -> (Self, Receiver<GenerationDataMessage>) {
        // --- 1. Setup Channels ---
        let (task_sender, task_receiver_final_handle) = flume::unbounded();
        let task_receiver_arc = Arc::new(task_receiver_final_handle.clone());
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

        // This one runs on the main thread, so SSXL logging is safe if you want:
        eprintln!(
            "INFO: Started SyncPool with {} dedicated workers.",
            num_workers
        );
        
        let pool = SyncPool { 
            workers, 
            task_sender, 
            chunk_sender,
            _task_receiver_final_handle: task_receiver_final_handle,
        };
        
        (pool, chunk_receiver)
    }
    
    /// Submits a new generation job to the worker pool.
    pub fn submit_job(&self, job: GenerationJob) -> Result<(), flume::SendError<GenerationJob>> {
        eprintln!(
            "DEBUG: Submitting job for chunk {:?} at step {:?} to worker pool.",
            job.id,
            job.current_step
        );
        self.task_sender.send(job)
    }
    
    /// Retrieves the current status of the worker pool (worker count and queue size).
    pub fn get_status(&self) -> (usize, usize) {
        let worker_count = self.workers.len();
        let queue_size = self.task_sender.len();
        (worker_count, queue_size)
    }
}

impl SyncPool {
    /// Signals workers to stop and waits for all threads to finish (clean shutdown).
    /// This method takes ownership and is intended to be called when the pool is 
    /// no longer needed by the owning structure.
    pub fn cleanup(self) {
        eprintln!("INFO: SyncPool.cleanup() called. Dropping channels and joining workers...");

        // Drop all senders and the receiver handle. This signals to workers to exit their loops.
        drop(self.task_sender);
        drop(self._task_receiver_final_handle);
        drop(self.chunk_sender);
        
        // Wait for all worker threads to finish.
        for mut worker in self.workers.into_iter() {
            if let Some(handle) = worker.handle.take() {
                match handle.join() {
                    Ok(()) => {
                        eprintln!("INFO: Worker {} thread joined successfully.", worker.id);
                    }
                    Err(e) => {
                        eprintln!("ERROR: Worker {} thread join failed: {:?}", worker.id, e);
                    }
                }
            }
        }
        eprintln!("INFO: SyncPool shut down successfully.");
    }
}
