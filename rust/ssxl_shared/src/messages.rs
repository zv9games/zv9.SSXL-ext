// ssxl_shared/src/messages.rs

use serde::{Serialize, Deserialize};
use crate::chunk_data::ChunkData;

// --------------------------------------------------------------------------------
// MESSAGE DATA STRUCTURES
// --------------------------------------------------------------------------------

/// A message wrapper sent from the generation worker thread (Conductor) to the
/// main thread (ChannelHandler) containing the result of a completed chunk task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkMessage {
    /// A chunk was successfully generated and is ready for presentation.
    Generated(ChunkData),
    /// A request was completed but returned no data (e.g., a known missing chunk).
    NoData,
    /// An error occurred during the generation process.
    Error(String),
}