use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum SSXLCoreError {
    #[error("Core state uninitialized: HostState singleton is not yet set.")]
    UninitializedState,

    #[error("Core state uninitialized: HostState singleton is not yet set.")]
    NotInitialized,

    #[error("Core initialization failed: {0}")]
    InitializationError(String),

    #[error("Conductor lifecycle error: Conductor is currently busy and cannot accept new jobs.")]
    ConductorBusy,

    #[error("Conductor lifecycle error: System is currently in state '{0}'.")]
    InvalidConductorState(String),

    #[error("Configuration error: Invalid value for '{0}'.")]
    InvalidConfig(String),

    #[error("Channel sending failed: {0}")]
    ChannelSendError(String),

    #[error("Channel receiving failed: {0}")]
    ChannelRecvError(String),

    #[error("Thread management error: Worker thread join failed.")]
    ThreadJoinError,

    #[error("Generation data error: {0}")]
    GenerationDataError(String),

    #[error("Mathematical boundary error: {0}")]
    MathError(String),

    #[error("Target TileMap ID is invalid (0).")]
    InvalidTarget,

    #[error("TileMap ID '{0}' is invalid or null.")]
    InvalidInstance(i64),

    #[error("FFI Bridge error: Direct memory write failed: {0}")]
    FFIWriteError(String),

    #[error("Godot API failure on '{0}': {1}")]
    GodotAPIFailure(String, String),
}

impl SSXLCoreError {
    pub fn to_ffi_code(&self) -> isize {
        match self {
            SSXLCoreError::UninitializedState => -2,
            SSXLCoreError::NotInitialized => -2,
            SSXLCoreError::InvalidTarget => -5,
            SSXLCoreError::ConductorBusy => -6,
            _ => {
                crate::ssxl_warn!(
                    "Unhandled critical error in SSXLCoreError::to_ffi_code. \
                     Mapping to FFI -3 (ConductorStopped): {:?}",
                    self
                );
                -3
            }
        }
    }
}
