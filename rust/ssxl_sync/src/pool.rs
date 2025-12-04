//! # Thread Pool Manager (`ssxl_sync::pool_manager`)
//!
//! Defines the generic, fixed-size thread pool used for executing synchronous,
//! CPU-intensive generation tasks off the main thread and the main Tokio runtime.
//! This pattern ensures high-throughput batch processing of `ChunkData`.

use std::thread::{self, JoinHandle};
use crossbeam_channel::{Sender, Receiver, unbounded};
use tracing::{info, error, warn};
use std::sync::Arc;

// --- Imports from sibling crates/modules ---
// NOTE: Imports from ssxl_generate removed to avoid cyclical dependency.
use ssxl_math::prelude::Vec2i;
// FIX: Import ChunkData directly from the root of ssxl_shared to resolve both E0433 errors.
use ssxl_shared::ChunkData; 

// --- Configuration ---
/// Defines the size of the worker pool.
const POOL_SIZE: usize = 4;


// --- 1. Worker Definition ---

/// Represents a single worker thread's state and handle.
struct Worker {
    /// The ID of the worker thread.
    id: usize,
    /// The handle to join the thread on shutdown.
    handle: Option<JoinHandle<()>>,
}

// --- Local Definitions to Break Dependency Cycle ---
/// The unit of work sent to the thread pool.
#[derive(Debug)]
pub enum GenerationTask {
    /// A command to begin generating a new chunk of data.
    GenerateChunk,
    /// A command to signal the worker thread to shut down gracefully.
    Shutdown,
}

/// The result returned from the completed work.
#[derive(Debug)]
pub enum ConductorResult {
    /// A successfully completed chunk of generated data.
    // FIX 1 (Line 47): Use the directly imported ChunkData.
    CompletedChunk(Arc<ChunkData>),
    /// An error that occurred during generation.
    Error(String),
}

/// The unit of work sent to the thread pool (Alias for local definition).
pub type Task = GenerationTask;

/// The result returned from the completed work (Alias for local definition).
pub type TaskResult = ConductorResult;


// --- 2. Pool Manager Structure ---

/// Manages the pool of worker threads and the task queue.
pub struct WorkerPool {
    /// The channel used to send tasks from the `Conductor` to the workers.
    task_sender: Sender<Task>,
    // Redundant `result_receiver` field removed.
    /// Collection of worker structs, primarily used to hold join handles for shutdown.
    workers: Vec<Worker>,
}

impl WorkerPool {
    /// Creates a new worker pool and starts all worker threads.
    pub fn new() -> (Self, Receiver<TaskResult>) {
        let (task_tx, task_rx) = unbounded::<Task>();
        let (result_tx, result_rx) = unbounded::<TaskResult>();
        
        let mut workers = Vec::with_capacity(POOL_SIZE);
        
        // Wrap the task receiver in an Arc to be shared by all worker threads.
        let shared_task_rx = Arc::new(task_rx);

        for id in 0..POOL_SIZE {
            let worker_task_rx = Arc::clone(&shared_task_rx);
            let worker_result_tx = result_tx.clone();
            
            let handle = thread::Builder::new()
                .name(format!("ssxl-worker-{}", id))
                .spawn(move || {
                    WorkerPool::run_worker_loop(id, worker_task_rx, worker_result_tx);
                })
                .expect(&format!("Failed to spawn worker thread {}", id));
            
            workers.push(Worker { id, handle: Some(handle) });
        }

        info!("WorkerPool initialized with {} threads.", POOL_SIZE);

        (
            WorkerPool {
                task_sender: task_tx,
                // Removed the unused `result_receiver` from initialization.
                workers,
            },
            result_rx, // The primary receiver is correctly returned for the Conductor to use.
        )
    }

    /// Submits a new generation task to the pool.
    pub fn submit_task(&self, task: Task) -> Result<(), crossbeam_channel::SendError<Task>> {
        self.task_sender.send(task)
    }

    /// The main loop executed by each worker thread.
    fn run_worker_loop(
        id: usize,
        task_rx: Arc<Receiver<Task>>,
        result_tx: Sender<TaskResult>,
    ) {
        info!("Worker {} started, ready to receive tasks.", id);

        loop {
            // Blocks until a task is available or the sender is dropped (shutdown).
            match task_rx.recv() {
                Ok(task) => {
                    // --- Perform CPU-intensive generation work here ---
                    info!("Worker {} processing task {:?}", id, task);
                    
                    // TODO: Execute the actual generation/batch function
                    let result: TaskResult = TaskResult::CompletedChunk(
                        // FIX 2 (Line 130): Use the directly imported ChunkData.
                        Arc::new(ChunkData::new_at_coords(Vec2i::new(0, 0)))
                    );
                    
                    // Send the result back to the Conductor
                    if let Err(e) = result_tx.send(result) {
                        warn!("Worker {} failed to send result: {}", id, e);
                        // The Conductor's receiver must have dropped. Exit.
                        break;
                    }
                }
                Err(_) => {
                    // Sender was dropped, time to shut down.
                    info!("Worker {} task channel closed. Shutting down.", id);
                    break;
                }
            }
        }
    }
}

// --- 3. Graceful Shutdown ---

impl Drop for WorkerPool {
    /// Gracefully shuts down all worker threads.
    fn drop(&mut self) {
        info!("WorkerPool initiating graceful shutdown...");
        
        // Attempt to send a shutdown command to any worker currently blocked on `recv()`.
        let _ = self.task_sender.send(Task::Shutdown);
        
        // Wait for all workers to finish.
        for mut worker in self.workers.drain(..) {
            if let Some(handle) = worker.handle.take() {
                if let Err(e) = handle.join() {
                    error!("Worker {} thread failed to join: {:?}", worker.id, e);
                } else {
                    info!("Worker {} successfully joined.", worker.id);
                }
            }
        }
        info!("WorkerPool shutdown complete.");
    }
}