// ============================================================================
// ⚡ SSXL Job Module ⚡
// File: ssxl_shared/src/job/mod.rs
// ----------------------------------------------------------------------------
// This module is the **command center** of the SSXL engine.
// It defines the packets of intent (jobs) and the packets of outcome (results)
// that flow between the engine core and its generation pipeline.
// Think of it as the **messenger guild**: jobs go in, results come out.
// ============================================================================

use crate::ChunkData; // 🎯 Core chunk payload imported from lib.rs

// -----------------------------------------------------------------------------
// 🚀 Instruction Packet: SSXLJob
// -----------------------------------------------------------------------------
// Represents the set of commands the engine can receive.
// Each variant is a ritual order, telling the generator what to do next.
pub enum SSXLJob {
    // 🗺️ BuildMap: Spin up a new world grid with given dimensions and seed.
    BuildMap {
        width: u32,
        height: u32,
        seed: u64,
        generator_id: String, // 🔑 Which generator to invoke
    },

    // 🔧 SetGenerator: Swap out the active generator configuration mid-flight.
    SetGenerator {
        generator_id: String,
    },

    // 🛑 StopGeneration: Halt all jobs, both running and queued.
    StopGeneration,

    // ... Future expansion: more job types can be added here.
}

// -----------------------------------------------------------------------------
// 🎯 Result Packet: JobResult
// -----------------------------------------------------------------------------
// Represents the outcomes returned back to the engine tick loop.
// Each variant is a signal of success, completion, or failure.
pub enum JobResult {
    // 🌟 ChunkGenerated: A new chunk is ready for integration into world state.
    ChunkGenerated {
        x: i32,          // Explicit coordinates for clarity
        y: i32,
        data: ChunkData, // 📦 The freshly minted chunk payload
    },

    // ✅ MapBuildComplete: The BuildMap job has finished successfully.
    MapBuildComplete,

    // 💥 Error: Something went wrong during job execution.
    Error(String),
}
