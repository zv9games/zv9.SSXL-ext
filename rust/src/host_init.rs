// rust/SSXL-ext/src/host_init.rs

use godot::prelude::*;
use crate::sync_pool::ThreadPool;
use crate::host_state::init_host_state;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor; 
use crate::config::GlobalConfig;
use flume::{self, Receiver, Sender}; 
use std::sync::Arc;

// FIX: Remove the non-existent GenerationTaskMessage from the import list.
use crate::shared_message::GenerationDataMessage; 


/// The primary initialization function called by Godot when the GDExtension loads.
/// This function is responsible for setting up the entire SSXL-ext core.
pub fn initialize_ssxl_core() -> Result<(), String> {
    godot_print!("SSXL-ext Core: Starting initialization (v9.1.seed).");
    
    // 1. Load Global Configuration
    let config = match GlobalConfig::load_or_default() {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to load configuration: {}", e)),
    };
    let config = Arc::new(config); // Wrap config in Arc now for cloning

    // 2. Initialize Communication Channels 
    
    // FIX: Add type annotation GenerationDataMessage to the chunk channel
    // NOTE: This channel is immediately overwritten by the one created in ThreadPool::new
    let (chunk_sender, _chunk_receiver): (Sender<GenerationDataMessage>, Receiver<GenerationDataMessage>) = flume::unbounded(); // For completed chunks
    
    // NOTE: This block is now unused because GenerationTaskMessage import was removed.
    // It is also unnecessary because the task channel is managed internally by ThreadPool.
    // let (_task_sender, _task_receiver): (Sender<GenerationTaskMessage>, Receiver<GenerationTaskMessage>) = flume::unbounded(); // For new tasks
    
    // 3. Initialize the Thread Pool (`sync_pool.rs`)
    // The pool returns itself and the chunk_receiver (the results channel).
    let num_workers = config.threading.generation_worker_count; // FIX 1: Use correct field
    
    // FIX 2: Correct arguments for ThreadPool::new, and capture its return values.
    let (thread_pool, chunk_receiver) = ThreadPool::new(
        num_workers as usize,
        Arc::clone(&config), // Pass clone of the Arc<GlobalConfig>
    );
    // NOTE: The `chunk_sender` created earlier is now redundant and unused, 
    // and the new `chunk_receiver` is the correct one to pass to the conductor.
    
    // 4. Initialize the Conductor (`generate_conductor.rs`)
    // Conductor constructor expects num_workers and config.
    let conductor = GenerateConductor::new(
        num_workers, // FIX 3: Pass num_workers (u32)
        Arc::clone(&config), // FIX 3: Pass config
    );

    // 5. Initialize the Animation Conductor (`generate_anim_conductor.rs`)
    // FIX 4: Call AnimConductor::new() with NO arguments as per the function signature.
    let anim_conductor = AnimConductor::new();
    
    // 6. Initialize the Host State Singleton (`host_state.rs`)
    // FIX 5: Pass the missing anim_conductor argument.
    match init_host_state(conductor, anim_conductor, Arc::clone(&config)) { 
        Ok(_) => {
            godot_print!("SSXL-ext Core: Initialization complete. Ready for work.");
            Ok(())
        },
        Err(e) => {
            Err(format!("HostState initialization failed: {}", e))
        }
    }
}