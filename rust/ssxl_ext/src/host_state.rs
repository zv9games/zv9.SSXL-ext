use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::config::GlobalConfig;
use crate::generate_conductor::GenerateConductor;
use crate::generate_anim_conductor::AnimConductor;
use crate::rhythm_manager::RhythmManager;
use crate::shared_error::SSXLCoreError;
use crate::{ssxl_error, ssxl_info};

pub type InstanceType = i64;

fn create_null_instance_id() -> InstanceType {
    0
}

pub static HOST_SINGLETON: OnceCell<Option<HostState>> = OnceCell::new();

/// Immutable access is no longer used by the engine, but we keep it for completeness.
/// Most systems now require mutable access (see get_host_state_mut).
pub fn get_host_state() -> Result<&'static mut HostState, SSXLCoreError> {
    unsafe {
        HOST_SINGLETON
            .get()
            .and_then(|opt| opt.as_ref())
            .ok_or_else(|| {
                ssxl_error!("Attempted to access HostState before it was initialized.");
                SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
            })?;

        let ptr = &HOST_SINGLETON as *const _ as *mut OnceCell<Option<HostState>>;
        (*ptr)
            .get_mut()
            .and_then(|opt| opt.as_mut())
            .ok_or_else(|| {
                ssxl_error!("Attempted to access HostState before it was initialized.");
                SSXLCoreError::InitializationError("HostState singleton not set.".to_string())
            })
    }
}

/// Explicit mutable accessor (same as get_host_state, but kept for clarity)
pub fn get_host_state_mut() -> Result<&'static mut HostState, SSXLCoreError> {
    get_host_state()
}

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

pub struct HostState {
    pub config: Arc<GlobalConfig>,
    pub conductor: GenerateConductor,
    pub anim_conductor: AnimConductor,
    pub rhythm_manager: RhythmManager,
    pub is_core_ready: bool,
    pub tilemap_id: InstanceType,
}

pub fn shutdown_host_state() -> Result<(), SSXLCoreError> {
    let taken = unsafe {
        let ptr = &HOST_SINGLETON as *const _ as *mut OnceCell<Option<HostState>>;
        (*ptr).take()
    };

    match taken {
        Some(_) => Ok(()),
        None => Err(SSXLCoreError::NotInitialized),
    }
}
