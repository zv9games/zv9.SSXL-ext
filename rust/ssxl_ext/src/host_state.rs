// rust/SSXL-ext/src/host_state.rs

use godot::prelude::InstanceId;
use once_cell::sync::OnceCell; 
use std::sync::Arc;

use crate::config::GlobalConfig;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor; 
use crate::rhythm_manager::RhythmManager; 
use crate::shared_error::SSXLCoreError;

// FIX: Removed the import for Godot-dependent logging macros
// use crate::{ssxl_error, ssxl_info};

// --------------------------------------------------------------------------
// --- Singleton & Access Functions ---
// --------------------------------------------------------------------------

/// The global, thread-safe singleton holding the core state of the system.
pub static HOST_SINGLETON: OnceCell<Option<HostState>> = OnceCell::new();

/// Attempts to retrieve a reference to the global `HostState`.
/// This function is safe to call from the main thread during runtime.
pub fn get_host_state() -> Result<&'static HostState, SSXLCoreError> {
    HOST_SINGLETON.get()
        // Check if the OnceCell is set (the outer Option)
        .and_then(|host_option| host_option.as_ref()) 
        .ok_or_else(|| {
            // FIX: Replaced ssxl_error! with eprintln!
            eprintln!("ERROR: Attempted to access HostState before it was initialized (immutable access).");
            SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
        })
}

/// Attempts to retrieve a MUTABLE reference to the global `HostState`.
/// This function is ONLY safe to call from the Godot Main Thread during runtime.
pub fn get_host_state_mut() -> Result<&'static mut HostState, SSXLCoreError> {
    // FIX 5: Introduce the function needed for mutable access on the main thread.
    // This encapsulates the required unsafe pattern for accessing the static mutably.
    let host_state_mut = unsafe {
        // 1. Get a mutable pointer to the static OnceCell<Option<HostState>> container.
        let host_singleton_mut_ptr = 
            &HOST_SINGLETON as *const _ as *mut once_cell::sync::OnceCell<Option<HostState>>;

        // 2. Call get_mut() on the mutable container (requires dereferencing the pointer).
        // 3. Call as_mut() to get Option<&mut HostState>.
        (*host_singleton_mut_ptr)
            .get_mut()
            .and_then(|opt| opt.as_mut())
    };

    host_state_mut.ok_or_else(|| {
        // FIX: Replaced ssxl_error! with eprintln!
        eprintln!("ERROR: Attempted to access HostState before it was initialized (mutable access).");
        SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
    })
}

/// Initializes the global `HostState` singleton.
pub fn init_host_state(
    conductor: GenerateConductor, 
    anim_conductor: AnimConductor, // Assuming this is also initialized here
    config: Arc<GlobalConfig>
) -> Result<(), SSXLCoreError> {
    // FIX: Replaced ssxl_info! with eprintln!
    eprintln!("INFO: Initializing HostState...");

    let new_state = HostState {
        // Core components
        conductor,
        anim_conductor,
        config,
        rhythm_manager: RhythmManager::new(), // Initialize the rhythm manager

        // Volatile / Runtime fields
        is_core_ready: true, // Mark as ready immediately after init
        // FIX 3: InstanceId::from_i64(0) is the correct way to initialize a null ID.
        tilemap_id: InstanceId::from_i64(1),
    };
    
    // Try to set the static singleton instance to Some(new_state)
    HOST_SINGLETON.set(Some(new_state)).map_err(|_| {
        // FIX: Replaced ssxl_error! with eprintln!
        eprintln!("ERROR: HostState initialization failed: Already initialized.");
        // FIX 4: Using the missing InitializationError variant (Awaiting addition to SSXLCoreError)
        SSXLCoreError::InitializationError("HostState was already set.".to_string())
    })
}


// --------------------------------------------------------------------------
// --- HostState Structure ---
// --------------------------------------------------------------------------

/// The main structure containing all state required by the Rust core.
/// This struct is a main-thread singleton accessed via `get_host_state()`.
pub struct HostState {
    // --- Configuration & Conductor Components ---
    /// The global, immutable configuration settings.
    pub config: Arc<GlobalConfig>,
    /// The main procedural generation orchestrator.
    pub conductor: GenerateConductor,
    /// The manager for dynamic map/tile animations.
    pub anim_conductor: AnimConductor,
    /// The manager for low-frequency, synchronization-critical operations.
    pub rhythm_manager: RhythmManager,

    // --- Runtime Flags & Metadata ---
    /// Flag indicating if all core components are initialized and running.
    pub is_core_ready: bool,
    /// The Instance ID of the current target TileMap in Godot.
    pub tilemap_id: InstanceId,
}