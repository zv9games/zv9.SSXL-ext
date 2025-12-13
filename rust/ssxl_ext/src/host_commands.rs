use godot::prelude::*;
use crate::host_state::HostState;
use crate::generate_conductor_state::ConductorState;
use crate::generate_task_queue::GenerationTask;
use crate::shared_job::GenerationJob;
use crate::shared_error::SSXLCoreError;
// FIX 1: Import the Lazy type for non-const static initialization (requires 'once_cell' in Cargo.toml)
use once_cell::sync::Lazy; 

// FIX 2: Change to Lazy initialization pattern.
static TEST_TILEMAP_ID: Lazy<InstanceId> = Lazy::new(|| InstanceId::from_i64(1337000));

/// Handles the high-level 'start_generation' command received from GDScript.
/// This is the entry point for work into the Rust core.
pub fn handle_start_command(host_state: &mut HostState, tilemap_id: InstanceId) -> Result<(), SSXLCoreError> {
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        godot_warn!("Command: Attempted to start generation while already Running. (FFI -6)");
        return Err(SSXLCoreError::ConductorBusy);
    }

    if tilemap_id.to_i64() == 0 {
        godot_error!("Command: Invalid TileMap InstanceId (0) provided. Cannot proceed. (FFI -5)");
        return Err(SSXLCoreError::InvalidTarget);
    }

    if !host_state.is_core_ready {
        godot_warn!("Command: Core not ready. Initialization failed or is pending.");
        return Err(SSXLCoreError::InitializationError("Core not ready.".to_string()));
    }
    
    let map_extent = 1;
    let chunk_size = 32;
    
    let mut jobs = Vec::new();
    
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new(
                (chunk_x, chunk_y),
                chunk_size,
            );
            
            let job = GenerationJob::new(task);
            jobs.push(job);
        }
    }
    
    let total_jobs = jobs.len();
    
    match host_state.conductor.start_generation(tilemap_id, jobs) {
        Ok(_) => {
            godot_print!("Command: Successfully initiated {} generation tasks.", total_jobs);
            Ok(())
        },
        Err(e) => {
            godot_error!("Command: Conductor failed to start generation: {:?}", e);
            Err(e)
        }
    }
}

/// Performs a structural integrity test of the Conductor, running a minimal generation job.
/// This checks the full Conductor lifecycle: scheduling, multithreaded execution, 
/// and signaling, without relying on a live TileMap.
pub fn trigger_structural_test_job(host_state: &mut HostState) -> Result<(), SSXLCoreError> {
    godot_print!("Structural Test: Initiating 1x1 conductor lifecycle test...");

    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        godot_warn!("Structural Test: Conductor is already busy. Cannot run test. (FFI -6)");
        return Err(SSXLCoreError::ConductorBusy);
    }

    if !host_state.is_core_ready {
        godot_warn!("Structural Test: Core not ready. Initialization failed or is pending.");
        return Err(SSXLCoreError::InitializationError("Core not ready for test.".to_string()));
    }
    
    let map_extent = 0;
    let chunk_size = 8;
    // FIX 3: Dereference the Lazy static variable to get the InstanceId value.
    let tilemap_id = *TEST_TILEMAP_ID;
    
    let mut jobs = Vec::new();
    
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new(
                (chunk_x, chunk_y),
                chunk_size,
            );
            
            let job = GenerationJob::new(task); 
            jobs.push(job);
        }
    }
    
    let total_jobs = jobs.len();
    
    match host_state.conductor.start_generation(tilemap_id, jobs) {
        Ok(_) => {
            godot_print!("Structural Test: Successfully started {} test task(s) with ID: {}. Result will be reported via FFI poll.", 
                total_jobs, tilemap_id.to_i64());
            Ok(())
        },
        Err(e) => {
            godot_error!("Structural Test: Conductor failed to start: {:?}.", e);
            Err(e)
        }
    }
}