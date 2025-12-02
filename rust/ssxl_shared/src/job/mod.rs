// File: ssxl_shared/src/job/mod.rs (Optimized Imports)

// FIX: Removed unused imports: `ChunkId` and `std::fmt::Display`.
use crate::ChunkData; // Import public re-exports from lib.rs

// --- The Instruction Packet ---
pub enum SSXLJob {
    /// Command to initialize map generation.
    BuildMap {
        width: u32,
        height: u32,
        seed: u64, // Use parsed seed
        // FIX: Replaced unknown GString with standard String
        generator_id: String
    },
    /// Command to change the active generator configuration.
    SetGenerator {
        // FIX: Replaced unknown GString with standard String
        generator_id: String
    },
    /// Command to stop all running and pending jobs.
    StopGeneration,
    // ...
}

// --- The Result Packet (Sent back to the Engine::tick) ---
// FIX: Renamed to JobResult to avoid collision with SSXLResult alias
pub enum JobResult {
    /// A chunk has been successfully generated and is ready for world state update.
    ChunkGenerated {
        // NOTE: x/y coordinates are often implicitly contained in ChunkData's bounds,
        // but keeping them here for explicit communication.
        x: i32,
        y: i32,
        // FIX: ChunkData is now imported and recognized
        data: ChunkData
    },
    /// The BuildMap job is finished.
    MapBuildComplete,
    /// An error occurred during processing.
    // FIX: Replaced unknown GString with standard String
    Error(String),
}