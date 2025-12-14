// rust/SSXL-ext/src/host_state.rs

use godot::prelude::InstanceId;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::config::GlobalConfig;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor;
use crate::rhythm_manager::RhythmManager;
use crate::shared_error::SSXLCoreError;
use crate::{ssxl_error, ssxl_info};

// --------------------------------------------------------------------------
// --- Conditional InstanceId Creation ---
// --------------------------------------------------------------------------

/// Helper function to safely create a null InstanceId without triggering FFI in the CLI.
fn create_null_instance_id() -> InstanceId {
    // === CLI MOCK IMPLEMENTATION (Safe) ===
    #[cfg(feature = "ssxl_cli")]
    {
        // 🔥 CRITICAL FIX: InstanceId::default() does not exist.
        // We use InstanceId::from_i64(1) as a sentinel value. This satisfies the
        // Godot binding's internal 'non-zero' assertion check, allowing the CLI to proceed,
        // while the core logic knows that 1 is a mock/unassigned ID.
        InstanceId::from_i64(1) 
    }

    // === GDExtension IMPLEMENTATION ===
    #[cfg(not(feature = "ssxl_cli"))]
    {
        // In the GDExtension environment, we must return the actual null ID (0), 
        // which is only safe because the Godot engine is running and FFI is loaded.
        InstanceId::from_i64(0) 
    }
}

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
            // 🔥 FIX 2: Replaced eprintln! with ssxl_error!
            ssxl_error!("Attempted to access HostState before it was initialized (immutable access).");
            SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
        })
}

/// Attempts to retrieve a MUTABLE reference to the global `HostState`.
/// This function is ONLY safe to call from the Godot Main Thread during runtime.
pub fn get_host_state_mut() -> Result<&'static mut HostState, SSXLCoreError> {
    // This block is unsafe because it bypasses Rust's static mutability checks,
    // relying on the caller (Godot main thread) to ensure exclusive access.
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
        // 🔥 FIX 3: Replaced eprintln! with ssxl_error!
        ssxl_error!("Attempted to access HostState before it was initialized (mutable access).");
        SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
    })
}

/// Initializes the global `HostState` singleton.
pub fn init_host_state(
    conductor: GenerateConductor,
    anim_conductor: AnimConductor,
    config: Arc<GlobalConfig>
) -> Result<(), SSXLCoreError> {
    // 🔥 FIX 4: Replaced eprintln! with ssxl_info!
    ssxl_info!("Initializing HostState...");

    let new_state = HostState {
        // Core components
        conductor,
        anim_conductor,
        config,
        rhythm_manager: RhythmManager::new(), // Initialize the rhythm manager

        // Volatile / Runtime fields
        is_core_ready: true, // Mark as ready immediately after init
        // CRITICAL FIX: Use the conditional helper function.
        tilemap_id: create_null_instance_id(),
    };

    // Try to set the static singleton instance to Some(new_state)
    HOST_SINGLETON.set(Some(new_state)).map_err(|_| {
        // 🔥 FIX 5: Replaced eprintln! with ssxl_error!
        ssxl_error!("HostState initialization failed: Already initialized.");
        SSXLCoreError::InitializationError("HostState was already set.".to_string())
    })
}


// --------------------------------------------------------------------------
// --- HostState Structure ---
// --------------------------------------------------------------------------

/// The main structure containing all state required by the Rust core.
/// This struct is a main-thread singleton accessed via `get_host_state()` and `get_host_state_mut()`.
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
    /// Defaulted to 0 (null/unassigned) until a TileMap is provided by the Godot script.
    pub tilemap_id: InstanceId,
}