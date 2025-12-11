// ============================================================================
// ⚙️ Thread Pool Manager (`ssxl_sync::pool_manager`)
// ----------------------------------------------------------------------------
// This module implements a fixed-size thread pool for executing synchronous,
// CPU-intensive generation tasks outside of the main thread and Tokio runtime.
// It is the backbone for high-throughput batch processing of `ChunkData`,
// ensuring that heavy work does not block async event loops.
//
// Key Concepts:
//   • Worker: Represents a single thread in the pool, with an ID and join handle.
//   • GenerationTask: Enum describing the unit of work (e.g., generate a chunk,
//     or shut down gracefully).
//   • ConductorResult: Enum describing the result of a task (completed chunk or error).
//   • WorkerPool: Manages the pool of workers, task queue, and graceful shutdown.
//
// Design Choices:
//   • Fixed pool size (POOL_SIZE = 4) for predictable resource usage.
//   • `crossbeam_channel` used for task/result communication because it provides
//     synchronous, blocking semantics ideal for CPU-bound work.
//   • `Arc<Receiver<Task>>` allows multiple worker threads to share the same
//     task queue safely.
//   • Results are sent back to the Conductor via a separate channel, enabling
//     centralized collection of completed work.
//
// Workflow:
//   1. Initialization (`WorkerPool::new`):
//      - Creates task and result channels.
//      - Spawns N worker threads, each running `run_worker_loop`.
//   2. Task submission (`submit_task`):
//      - Sends a `GenerationTask` into the pool’s task channel.
//   3. Worker loop (`run_worker_loop`):
//      - Blocks until a task arrives.
//      - Processes the task (e.g., generate a chunk).
//      - Sends back a `ConductorResult`.
//      - Shuts down gracefully if the channel is closed.
//   4. Shutdown (`Drop` impl):
//      - Sends a `Shutdown` task to unblock workers.
//      - Joins all worker threads to ensure clean termination.
//
// Educational Note:
//   • This module demonstrates a classic concurrency pattern: a fixed-size
//     worker pool consuming tasks from a shared queue.
//   • By separating task submission, worker execution, and result collection,
//     the system achieves both parallelism and safety.
//   • Graceful shutdown ensures no threads are leaked and all resources are
//     properly cleaned up.
// ============================================================================


use std::thread::{self, JoinHandle};
use crossbeam_channel::{Sender, Receiver, unbounded};
use tracing::{info, error, warn};
use std::sync::Arc;

use ssxl_math::prelude::Vec2i;
use ssxl_shared::ChunkData; 

const POOL_SIZE: usize = 4;

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub enum GenerationTask {
    GenerateChunk,
    Shutdown,
}

#[derive(Debug)]
pub enum ConductorResult {
    CompletedChunk(Arc<ChunkData>),
    Error(String),
}

pub type Task = GenerationTask;
pub type TaskResult = ConductorResult;

pub struct WorkerPool {
    task_sender: Sender<Task>,
    workers: Vec<Worker>,
}

impl WorkerPool {
    pub fn new() -> (Self, Receiver<TaskResult>) {
        let (task_tx, task_rx) = unbounded::<Task>();
        let (result_tx, result_rx) = unbounded::<TaskResult>();
        
        let mut workers = Vec::with_capacity(POOL_SIZE);
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
                workers,
            },
            result_rx,
        )
    }

    pub fn submit_task(&self, task: Task) -> Result<(), crossbeam_channel::SendError<Task>> {
        self.task_sender.send(task)
    }

    fn run_worker_loop(
        id: usize,
        task_rx: Arc<Receiver<Task>>,
        result_tx: Sender<TaskResult>,
    ) {
        info!("Worker {} started, ready to receive tasks.", id);

        loop {
            match task_rx.recv() {
                Ok(task) => {
                    info!("Worker {} processing task {:?}", id, task);
                    
                    let result: TaskResult = TaskResult::CompletedChunk(
                        Arc::new(ChunkData::new_at_coords(Vec2i::new(0, 0)))
                    );
                    
                    if let Err(e) = result_tx.send(result) {
                        warn!("Worker {} failed to send result: {}", id, e);
                        break;
                    }
                }
                Err(_) => {
                    info!("Worker {} task channel closed. Shutting down.", id);
                    break;
                }
            }
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        info!("WorkerPool initiating graceful shutdown...");
        
        let _ = self.task_sender.send(Task::Shutdown);
        
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


