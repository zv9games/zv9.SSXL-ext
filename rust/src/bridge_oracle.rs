// rust/SSXL-ext/src/bridge_oracle.rs

use crate::shared_math::ChunkCoords;
use crate::ssxl_warn;

/// Queries that worker threads can send to the main thread.
#[derive(Debug, Clone)]
pub enum OracleQuery {
    /// Request the current "hardness" value for a specific chunk coordinate.
    GetChunkHardness(ChunkCoords),
    /// Request the global time of day (affects lighting/spawning).
    GetGlobalTimeOfDay,
    /// Query if a specific world tile is blocked by a non-map asset (e.g., a tree, a player).
    IsTileBlocked(i32, i32), 
}

/// The corresponding response the main thread sends back.
#[derive(Debug, Clone)]
pub enum OracleResponse {
    ChunkHardness(f64),
    GlobalTimeOfDay(f32),
    TileBlockedStatus(bool),
    Error(String),
}

/// A structured request containing the query and a return channel.
pub struct OracleRequest {
    pub query: OracleQuery,
    // The one-time SPSC sender for the response to go back to the worker thread.
    pub response_sender: flume::Sender<OracleResponse>,
}

// rust/SSXL-ext/src/bridge_oracle.rs

use flume::{Receiver, Sender};
use crate::host_state::get_host_state;
use crate::tools::Profiler;

/// Manages the queue of queries sent from worker threads to the Godot main thread.
pub struct OracleConductor {
    // Receiver for queries from the worker threads (MPSC)
    pub request_receiver: Receiver<OracleRequest>,
    // Sender used by the workers to submit requests (cloned for each worker)
    pub request_sender: Sender<OracleRequest>,
}

impl OracleConductor {
    /// Creates the conductor and the communication channel.
    pub fn new() -> Self {
        let (request_sender, request_receiver) = flume::unbounded();
        OracleConductor { request_receiver, request_sender }
    }

    /// Processes a limited number of incoming queries per frame.
    /// This is called by host_tick.rs to process worker requests safely.
    pub fn poll_and_process(&self) {
        // Use a budget to prevent stalling the main thread
        let mut processed_count = 0;
        let _p = Profiler::start("Oracle Poll");
        
        while processed_count < 10 { // Max 10 oracle queries per frame
            match self.request_receiver.try_recv() {
                Ok(request) => {
                    // 1. Process the Query
                    let response = self.handle_query(request.query);
                    
                    // 2. Send the Response back to the requesting worker
                    // This is a single-time send on the response_sender carried by the request.
                    let _ = request.response_sender.send(response);
                    
                    processed_count += 1;
                },
                Err(flume::TryRecvError::Empty) => break,
                Err(flume::TryRecvError::Disconnected) => {
                    ssxl_warn!("Oracle Conductor: Request channel disconnected.");
                    break;
                }
            }
        }
    }

    /// Executes the Godot API call or snapshot query based on the request.
    fn handle_query(&self, query: OracleQuery) -> OracleResponse {
        // Access Godot state safely on the main thread
        let host_state = get_host_state(); 

        match query {
            OracleQuery::GetGlobalTimeOfDay => {
                // Example: Query the Godot engine's time
                // This call would involve FFI or Gd<Node>::call()
                // let time = host_state.get_time_of_day_snapshot(); 
                OracleResponse::GlobalTimeOfDay(0.5) // Example return
            }
            OracleQuery::IsTileBlocked(x, y) => {
                // Example: Check if the cache layer (host_state) has an entity at (x,y)
                // let is_blocked = host_state.entity_block_cache.is_blocked(x, y);
                OracleResponse::TileBlockedStatus(false)
            }
            _ => OracleResponse::Error("Query not implemented".to_string()),
        }
    }
}

// rust/SSXL-ext/src/bridge_oracle.rs

/// The interface used by worker threads (generate_ca.rs, animate_worker.rs) 
/// to interact with the Oracle.
#[derive(Clone)]
pub struct OracleClient {
    request_sender: Sender<OracleRequest>,
}

impl OracleClient {
    /// Submits a query to the main thread and blocks waiting for the response.
    /// WARNING: This should be used sparingly, as it blocks the worker thread 
    /// until the main thread processes the query. Non-blocking alternatives are preferred.
    pub fn query_blocking(&self, query: OracleQuery) -> Result<OracleResponse, String> {
        // 1. Create a temporary SPSC channel for the response
        let (response_sender, response_receiver) = flume::bounded(1); 
        
        // 2. Build the request and send it
        let request = OracleRequest { query, response_sender };
        self.request_sender.send(request).map_err(|e| format!("Failed to send query: {}", e))?;

        // 3. Block and wait for the response from the main thread
        response_receiver.recv().map_err(|e| format!("Failed to receive response: {}", e))
    }
    
    // Non-blocking query method would exist for complex chains.
}