
#[allow(unused_imports)]
use aetherion_core::zv9_prelude::*;


pub const SIGNALS: &[&str] = &[
    // ✅ Core generation signals
    "build_map_start",
    "generation_progress",
    "generation_complete",
    "map_building_status",

    // 🧠 Lifecycle & diagnostics
    "tick_started",
    "tick_completed",
    "frame_budget_exceeded",
    "engine_init_start",
    "engine_initialized",
    "pipeline_start",
    "pipeline_complete",
    "sync_required",
    "rust_error",

    // 🔁 Tilemap & map events
    "map_chunk_ready",
    "tilemap_updated",
    "tilemap_sync_complete",
    "map_build_cancelled",
    "map_build_failed",
    "map_build_duration",
    "rust_extension_ready",
];

//end definitions.rs