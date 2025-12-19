#[cfg(feature = "godot-binding")]
use crate::host_state::{HostState, HOST_SINGLETON};
use crate::shared_error::SSXLCoreError;

#[cfg(feature = "godot-binding")]
pub fn shutdown_ssxl_runtime() -> Result<(), SSXLCoreError> {
    cleanup_ssxl_core();
    Ok(())
}

#[cfg(feature = "godot-binding")]
pub fn cleanup_ssxl_core() {
    crate::ssxl_info!("SSXL-ext Core: Starting cleanup procedure.");

    let taken_state = unsafe {
        let host_singleton_mut: *mut once_cell::sync::OnceCell<Option<HostState>> =
            &HOST_SINGLETON as *const _ as *mut _;
        (*host_singleton_mut).take()
    };

    match taken_state {
        Some(Some(host_state)) => {
            let HostState {
                conductor,
                anim_conductor: _,
                ..
            } = host_state;

            crate::ssxl_info!("Cleanup: Instructing Conductor to shut down worker threads...");
            conductor.shutdown();

            crate::ssxl_info!("SSXL-ext Core: Cleanup complete. Resources released.");
        }
        Some(None) => {
            crate::ssxl_warn!("Cleanup called but HostState was already cleaned up.");
        }
        None => {
            crate::ssxl_warn!("Cleanup called but SSXL-ext was never initialized.");
        }
    }
}

#[cfg(not(feature = "godot-binding"))]
pub fn shutdown_ssxl_runtime() -> Result<(), SSXLCoreError> {
    Ok(())
}

#[cfg(not(feature = "godot-binding"))]
pub fn cleanup_ssxl_core() {}
