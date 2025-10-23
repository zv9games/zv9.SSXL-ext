

use crate::zv9_prelude::*;
use crate::zv9_shared_pipeline_data_chunk::MapDataChunk;


/// 📨 EngineMessage — messages sent from Rust to Godot for signal dispatch.
/// These drive procedural feedback, status updates, and runtime events.
#[derive(Clone, Debug)]
pub enum EngineMessage {
    // ✅ Lifecycle
    Start,
    Cancelled,
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },

    // 📊 Feedback
    Progress(i32),
    Status(String),
    Warning(String),
    Error(String),

    // 🧩 Chunk Delivery
    MapChunkReady,
	ChunkReady(MapDataChunk),
	Chunk(MapDataChunk), // ✅ Add this


    // 🧠 Runtime Signals
    Paused,
    Resumed,
    Retry,
    Diagnostics {
        memory_usage: u64,
        thread_count: usize,
        tick_rate: f32,
    },

    // 🧪 Custom Event
    Custom {
        name: String,
        payload: serde_json::Value,
    },
}
