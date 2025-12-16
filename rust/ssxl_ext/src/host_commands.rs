use crate::host_state::HostState;
use crate::generate_conductor_state::ConductorState;
use crate::generate_task_queue::GenerationTask;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;


#[cfg(feature = "godot-binding")]
use once_cell::sync::Lazy;

/// TEST TILEMAP ID — now stored as a plain integer.
/// Godot conversion happens *inside* the function, never at API boundary.
#[cfg(feature = "godot-binding")]
static TEST_TILEMAP_ID: Lazy<i64> = Lazy::new(|| 1337000);

#[cfg(not(feature = "godot-binding"))]
const TEST_TILEMAP_ID: i64 = 1337000;

/// Handles the "start generation" command from Godot or CLI.
/// `tilemap_id_raw` is always a plain integer — safe for both builds.
pub fn handle_start_command(
    host_state: &mut HostState,
    tilemap_id_raw: i64,
) -> Result<(), SSXLCoreError> {
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        crate::ssxl_warn!("Command: Attempted to start generation while already Running. (FFI -6)");
        return Err(SSXLCoreError::ConductorBusy);
    }

    if tilemap_id_raw == 0 {
        crate::ssxl_error!("Command: Invalid TileMap ID (0) provided. Cannot proceed. (FFI -5)");
        return Err(SSXLCoreError::InvalidTarget);
    }

    if !host_state.is_core_ready {
        crate::ssxl_warn!("Command: Core not ready. Initialization failed or is pending.");
        return Err(SSXLCoreError::InitializationError(
            "Core not ready.".to_string(),
        ));
    }

    let map_extent = 1;
    let chunk_size = 32;

    let mut jobs = Vec::new();
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new((chunk_x, chunk_y), chunk_size);
            jobs.push(GenerationJob::new(task));
        }
    }

    let total_jobs = jobs.len();

    // ✅ Godot conversion happens *inside* the function, not at API boundary.
    #[cfg(feature = "godot-binding")]
    let tilemap_id = tilemap_id_raw;

    #[cfg(not(feature = "godot-binding"))]
    let tilemap_id = tilemap_id_raw;

    match host_state.conductor.start_generation(tilemap_id, jobs) {
        Ok(_) => {
            crate::ssxl_info!(
                "Command: Successfully initiated {} generation tasks.",
                total_jobs
            );
            Ok(())
        }
        Err(e) => {
            crate::ssxl_error!("Command: Conductor failed to start generation: {:?}", e);
            Err(e)
        }
    }
}

/// Runs a 1×1 structural test job.
pub fn trigger_structural_test_job(
    host_state: &mut HostState,
) -> Result<(), SSXLCoreError> {
    crate::ssxl_info!("Structural Test: Initiating 1x1 conductor lifecycle test...");

    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        crate::ssxl_warn!("Structural Test: Conductor is already busy. Cannot run test. (FFI -6)");
        return Err(SSXLCoreError::ConductorBusy);
    }

    if !host_state.is_core_ready {
        crate::ssxl_warn!("Structural Test: Core not ready. Initialization failed or is pending.");
        return Err(SSXLCoreError::InitializationError(
            "Core not ready for test.".to_string(),
        ));
    }

    let map_extent = 0;
    let chunk_size = 8;

    // ✅ TEST TILEMAP ID is now a plain integer
    let tilemap_id = {
        #[cfg(feature = "godot-binding")]
        { *TEST_TILEMAP_ID }

        #[cfg(not(feature = "godot-binding"))]
        { TEST_TILEMAP_ID }
    };

    // ✅ You forgot this block — now restored
    let mut jobs = Vec::new();
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new((chunk_x, chunk_y), chunk_size);
            jobs.push(GenerationJob::new(task));
        }
    }

    let total_jobs = jobs.len();

    match host_state.conductor.start_generation(tilemap_id, jobs) {
        Ok(_) => {
            crate::ssxl_info!(
                "Structural Test: Successfully started {} test task(s) with ID: {}. \
                 Result will be reported via FFI poll.",
                total_jobs,
                tilemap_id
            );
            Ok(())
        }
        Err(e) => {
            crate::ssxl_error!("Structural Test: Conductor failed to start: {:?}.", e);
            Err(e)
        }
    }
}
