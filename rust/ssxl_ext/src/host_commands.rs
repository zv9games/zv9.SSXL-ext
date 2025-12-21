use crate::host_state::HostState;
use crate::generate_conductor_state::ConductorState;
use crate::generate_task_queue::GenerationTask;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;

use std::sync::Arc;

#[cfg(feature = "godot-binding")]
use once_cell::sync::Lazy;

#[cfg(feature = "godot-binding")]
static TEST_TILEMAP_ID: Lazy<i64> = Lazy::new(|| 1337000);

#[cfg(not(feature = "godot-binding"))]
const TEST_TILEMAP_ID: i64 = 1337000;

pub fn handle_start_command(
    host_state: &mut HostState,
    tilemap_id_raw: i64,
) -> Result<(), SSXLCoreError> {
    // ----------------------------------------------------
    // ✅ Safety checks
    // ----------------------------------------------------
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        return Err(SSXLCoreError::ConductorBusy);
    }

    if tilemap_id_raw == 0 {
        return Err(SSXLCoreError::InvalidTarget);
    }

    if !host_state.is_core_ready {
        return Err(SSXLCoreError::InitializationError("Core not ready.".to_string()));
    }

    // ----------------------------------------------------
    // ✅ Dynamic world size (set by api_build_map)
    // ----------------------------------------------------
    let world_w = host_state.world_width.max(1);
    let world_h = host_state.world_height.max(1);

    // ✅ Chunk size from map_settings (correct location)
    let chunk_size_i32 = host_state.config.map_settings.chunk_size as i32;
    let chunk_size_u32 = chunk_size_i32 as u32;

    // Compute number of chunks needed
    let chunks_x = (world_w + chunk_size_i32 - 1) / chunk_size_i32;
    let chunks_y = (world_h + chunk_size_i32 - 1) / chunk_size_i32;

    // ----------------------------------------------------
    // ✅ Build jobs dynamically
    // ----------------------------------------------------
    let seed = host_state.config.generation.world_seed;
    let config = Arc::new(host_state.config.generation.clone());

    let mut jobs = Vec::new();

    for cx in 0..chunks_x {
        for cy in 0..chunks_y {
            let task = GenerationTask::new(
                (cx, cy),
                chunk_size_u32,
                seed,
                config.clone(),
            );
            jobs.push(GenerationJob::new(task));
        }
    }

    // ----------------------------------------------------
    // ✅ Start generation
    // ----------------------------------------------------
    let tilemap_id = tilemap_id_raw;
    host_state.conductor.start_generation(tilemap_id, jobs)
}

pub fn trigger_structural_test_job(
    host_state: &mut HostState,
) -> Result<(), SSXLCoreError> {
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        return Err(SSXLCoreError::ConductorBusy);
    }

    if !host_state.is_core_ready {
        return Err(SSXLCoreError::InitializationError(
            "Core not ready for test.".to_string(),
        ));
    }

    let map_extent = 0;
    let chunk_size = 8u32;

    let tilemap_id = {
        #[cfg(feature = "godot-binding")]
        { *TEST_TILEMAP_ID }

        #[cfg(not(feature = "godot-binding"))]
        { TEST_TILEMAP_ID }
    };

    let seed = host_state.config.generation.world_seed;
    let config = Arc::new(host_state.config.generation.clone());

    let mut jobs = Vec::new();
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new(
                (chunk_x, chunk_y),
                chunk_size,
                seed,
                config.clone(),
            );
            jobs.push(GenerationJob::new(task));
        }
    }

    host_state.conductor.start_generation(tilemap_id, jobs)
}
