// rust/SSXL-ext/src/sync_pool.rs

use std::thread;
use std::sync::Arc;
use flume::{Receiver, Sender};

// Shared types used for concurrent operations
use crate::shared_job::GenerationJob; 
use crate::shared_message::GenerationDataMessage;
use crate::config::GlobalConfig;
use crate::generate_runtime;

// --- FIX: Removed the import for Godot-dependent logging macros
// use crate::{ssxl_info, ssxl_error};

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
                let final_result = generate_runtime::run_generation_job(job, &config.generation);

                // --- FINISHER DELIVERY ---
                // Send the final result (Completed Chunk or Failure Message) back to the Conductor.
                let message = match final_result {
                    Ok(completed_chunk) => {
                        // Using eprintln! for standard thread logging
                        eprintln!("INFO: Worker {} completed job {:?}", id, completed_chunk.position);
                        GenerationDataMessage::CompletedChunk(completed_chunk)
                    },
                    Err(e) => {
                        // Using eprintln! for standard thread logging
                        eprintln!("ERROR: Worker {} failed job: {:?}", id, e);
                        GenerationDataMessage::JobFailure(e)
                    }
                };

                // Non-blocking delivery of the result back to the Conductor's main thread poller.
                if let Err(e) = sender_clone.send(message) {
                    // The main thread is no longer listening; break the loop and shut down.
                    // Using eprintln! for standard thread logging
                    eprintln!("ERROR: Worker {} failed to send result: {}. Conductor likely shut down.", id, e);
                    break; 
                }
            }
            // Using eprintln! for standard thread logging
            eprintln!("INFO: Worker {} shutting down.", id);
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

        eprintln!("INFO: Started SyncPool with {} dedicated workers.", num_workers);
        
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
        self.task_sender.send(job)
    }
    
    // ✅ FIX E0599: Implemented the method required by GenerationManager.
    /// Retrieves the current status of the worker pool (worker count and queue size).
    pub fn get_status(&self) -> (usize, usize) {
        let worker_count = self.workers.len();
        // flume::Sender::len() reports the number of messages currently waiting in the queue.
        let queue_size = self.task_sender.len();
        (worker_count, queue_size)
    }

    // ❌ REMOVED: The shutdown method that takes `self` (ownership) is removed here 
    // to resolve E0507 in GenerationManager. Cleanup will be done by the owner 
    // when the struct is dropped, or via a public, static cleanup method.
    /*
    pub fn shutdown(self) {
        // ... shutdown logic ...
    }
    */
}

// NOTE: To ensure threads are properly joined upon cleanup (which is essential
// for clean shutdown), we re-introduce the shutdown logic using a static method 
// or implement the Drop trait. Since `GenerationManager` is fixed to no longer 
// call `shutdown(&self)`, we defer the joining/cleanup to the host layer 
// responsible for owning and dropping the SyncPool instance.
// However, since the original code contained the join logic, we will define a 
// static `cleanup` function that takes ownership for the host to call.

impl SyncPool {
    /// Signals workers to stop and waits for all threads to finish (clean shutdown).
    /// This method takes ownership and is intended to be called when the pool is 
    /// no longer needed by the owning structure.
    pub fn cleanup(self) {
        // Drop all senders and the receiver handle. This signals to workers to exit their loops.
        drop(self.task_sender);
        drop(self._task_receiver_final_handle);
        drop(self.chunk_sender);
        
        // Wait for all worker threads to finish.
        for mut worker in self.workers.into_iter() {
            if let Some(handle) = worker.handle.take() {
                if let Err(e) = handle.join() {
                    eprintln!("ERROR: Worker {} thread join failed: {:?}", worker.id, e);
                }
            }
        }
        eprintln!("INFO: SyncPool shut down successfully.");
    }
}