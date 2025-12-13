use crate::shared_math::ChunkCoords;
use crate::ssxl_warn;
use flume::{Receiver, Sender};
use crate::host_state::get_host_state;
use crate::tools::Profiler;

#[derive(Debug, Clone)]
pub enum OracleQuery {
    GetChunkHardness(ChunkCoords),
    GetGlobalTimeOfDay,
    IsTileBlocked(i32, i32),
}

#[derive(Debug, Clone)]
pub enum OracleResponse {
    ChunkHardness(f64),
    GlobalTimeOfDay(f32),
    TileBlockedStatus(bool),
    Error(String),
}

pub struct OracleRequest {
    pub query: OracleQuery,
    pub response_sender: flume::Sender<OracleResponse>,
}

pub struct OracleConductor {
    pub request_receiver: Receiver<OracleRequest>,
    pub request_sender: Sender<OracleRequest>,
}

impl OracleConductor {
    pub fn new() -> Self {
        let (request_sender, request_receiver) = flume::unbounded();
        OracleConductor { request_receiver, request_sender }
    }

    pub fn poll_and_process(&self) {
        let mut processed_count = 0;
        let _p = Profiler::start("Oracle Poll");
        
        while processed_count < 10 {
            match self.request_receiver.try_recv() {
                Ok(request) => {
                    let response = self.handle_query(request.query);
                    
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

    fn handle_query(&self, query: OracleQuery) -> OracleResponse {
        let _host_state = get_host_state();

        match query {
            OracleQuery::GetGlobalTimeOfDay => {
                OracleResponse::GlobalTimeOfDay(0.5)
            }
            OracleQuery::IsTileBlocked(_x, _y) => {
                OracleResponse::TileBlockedStatus(false)
            }
            _ => OracleResponse::Error("Query not implemented".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct OracleClient {
    request_sender: Sender<OracleRequest>,
}

impl OracleClient {
    pub fn query_blocking(&self, query: OracleQuery) -> Result<OracleResponse, String> {
        let (response_sender, response_receiver) = flume::bounded(1);
        
        let request = OracleRequest { query, response_sender };
        self.request_sender.send(request).map_err(|e| format!("Failed to send query: {}", e))?;

        response_receiver.recv().map_err(|e| format!("Failed to receive response: {}", e))
    }
}