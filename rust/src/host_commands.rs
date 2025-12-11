// rust/SSXL-ext/src/host_commands.rs

use godot::prelude::*;
use crate::host_state::HostState;
use crate::generate_conductor_state::ConductorState;
use crate::generate_task_queue::GenerationTask; // GenerationTask is likely here
use crate::shared_job::GenerationJob;           // FIX 1: GenerationJob is moved to shared_job
use crate::shared_error::SSXLCoreError; 
use crate::generate_conductor::GenerateConductor; 

/// Handles the high-level 'start_generation' command received from GDScript.
/// This is the entry point for work into the Rust core.
// FIX 1: Change to mutable reference (&mut HostState) because conductor.start_generation modifies state.
pub fn handle_start_command(host_state: &mut HostState, tilemap_id: InstanceId) -> Result<(), SSXLCoreError> {
    
    // 1. LIFECYCLE GUARD: Check for premature activation
    if host_state.conductor.get_state_container().get_state() == ConductorState::Generating {
        godot_warn!("Command: Attempted to start generation while already Running.");
        return Err(SSXLCoreError::ConductorBusy); 
    }

    // 2. STATE GUARD: Ensure core resources are ready
    if !host_state.is_core_ready {
        godot_warn!("Command: Core not ready. Initialization failed or is pending.");
        return Err(SSXLCoreError::InitializationError("Core not ready.".to_string()));
    }
    
    // 3. Command Payload Construction
    
    // This part involves calculating *what* to generate.
    let map_settings = &host_state.config.map_settings;
    let chunk_size = map_settings.chunk_size;
    let map_extent = map_settings.map_extent_chunks; // e.g., 8 chunks in each direction

    // FIX 2: Change vector type initialization to match the expected type
    let mut jobs = Vec::new(); // `jobs` will now hold Vec<GenerationJob>
    
    // Generate a task for every chunk in the defined extent
    for chunk_x in -map_extent..=map_extent {
        for chunk_y in -map_extent..=map_extent {
            let task = GenerationTask::new(
                (chunk_x, chunk_y),
                chunk_size,
                // Include necessary config/seeds for the workers
            );
            
            // FIX 3: Convert the GenerationTask into a GenerationJob before pushing.
            // Assuming GenerationJob::new(task) is the constructor.
            let job = GenerationJob::new(task); 
            jobs.push(job);
        }
    }
    
    // Capture the length BEFORE the vector is moved.
    let total_jobs = jobs.len();
    
    // 4. Conductor Initiation
    // Pass the calculated jobs and the target TileMap ID to the Conductor
    match host_state.conductor.start_generation(tilemap_id, jobs) { // 'jobs' is moved here
        Ok(_) => {
            // FIX 5: Use the captured length (total_jobs) instead of the moved 'jobs' vector.
            godot_print!("Command: Successfully initiated {} generation tasks.", total_jobs); 
            Ok(())
        },
        Err(e) => {
            godot_error!("Command: Conductor failed to start generation: {:?}", e);
            // Return the specific error from the Conductor
            Err(e) 
        }
    }
}