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
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        return Err(SSXLCoreError::ConductorBusy);
    }

    if tilemap_id_raw == 0 {
        return Err(SSXLCoreError::InvalidTarget);
    }

    if !host_state.is_core_ready {
        return Err(SSXLCoreError::InitializationError("Core not ready.".to_string()));
    }

    let map_extent = 1;
    let chunk_size = 32;

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
    let chunk_size = 8;

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
