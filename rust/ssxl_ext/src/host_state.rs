// --------------------------------------------------------------------------
// --- CONDITIONAL IMPORTS AND TYPE ALIASES (CLEANED & FIXED) ---
// --------------------------------------------------------------------------

use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::config::GlobalConfig;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor;
use crate::rhythm_manager::RhythmManager;
use crate::shared_error::SSXLCoreError;
use crate::{ssxl_error, ssxl_info};

/// ✅ Public-facing ID type is ALWAYS a plain integer.
/// Godot conversion happens *inside* Godot-only functions.
pub type InstanceType = i64;


// --------------------------------------------------------------------------
// --- Null ID Creation (SAFE) ---
// --------------------------------------------------------------------------

/// ✅ Always return a plain integer.
/// Godot conversion happens later, never here.
fn create_null_instance_id() -> InstanceType {
    0
}

// --------------------------------------------------------------------------
// --- Singleton & Access Functions ---
// --------------------------------------------------------------------------

pub static HOST_SINGLETON: OnceCell<Option<HostState>> = OnceCell::new();

pub fn get_host_state() -> Result<&'static HostState, SSXLCoreError> {
    HOST_SINGLETON
        .get()
        .and_then(|host_option| host_option.as_ref())
        .ok_or_else(|| {
            ssxl_error!("Attempted to access HostState before it was initialized (immutable access).");
            SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
        })
}

pub fn get_host_state_mut() -> Result<&'static mut HostState, SSXLCoreError> {
    let host_state_mut = unsafe {
        let host_singleton_mut_ptr =
            &HOST_SINGLETON as *const _ as *mut OnceCell<Option<HostState>>;

        (*host_singleton_mut_ptr)
            .get_mut()
            .and_then(|opt| opt.as_mut())
    };

    host_state_mut.ok_or_else(|| {
        ssxl_error!("Attempted to access HostState before it was initialized (mutable access).");
        SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
    })
}

// --------------------------------------------------------------------------
// --- HostState Initialization ---
// --------------------------------------------------------------------------

pub fn init_host_state(
    conductor: GenerateConductor,
    anim_conductor: AnimConductor,
    config: Arc<GlobalConfig>,
) -> Result<(), SSXLCoreError> {
    ssxl_info!("Initializing HostState...");

    let new_state = HostState {
        config,
        conductor,
        anim_conductor,
        rhythm_manager: RhythmManager::new(),
        is_core_ready: true,
        tilemap_id: create_null_instance_id(),
    };

    HOST_SINGLETON.set(Some(new_state)).map_err(|_| {
        ssxl_error!("HostState initialization failed: Already initialized.");
        SSXLCoreError::InitializationError("HostState was already set.".to_string())
    })
}

// --------------------------------------------------------------------------
// --- HostState Structure ---
// --------------------------------------------------------------------------

pub struct HostState {
    pub config: Arc<GlobalConfig>,
    pub conductor: GenerateConductor,
    pub anim_conductor: AnimConductor,
    pub rhythm_manager: RhythmManager,

    pub is_core_ready: bool,

    /// ✅ Now always a plain integer.
    /// Godot conversion happens inside Godot-only code.
    pub tilemap_id: InstanceType,
}
