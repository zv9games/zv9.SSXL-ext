use thiserror::Error;

// -----------------------------------------------------------------------------
// SSXLResult
// -----------------------------------------------------------------------------
// A project-wide Result type that standardizes error handling.
// All fallible functions in the SSXL engine should return this type.
pub type SSXLResult<T> = Result<T, SSXLError>;

// -----------------------------------------------------------------------------
// SSXLError
// -----------------------------------------------------------------------------
// Central error enumeration for the SSXL engine.
// Each variant represents a distinct failure category, ensuring consistent
// reporting across subsystems and FFI boundaries.
#[derive(Error, Debug)]
pub enum SSXLError {
    // I/O failures such as file system or network errors.
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    // Serialization or deserialization problems (e.g., Bincode, Serde).
    #[error("Serialization/Data Error: {0}")]
    Serialization(String),

    // Invalid or inconsistent data states (e.g., wrong tile array size).
    #[error("Data Consistency Error: {0}")]
    DataConsistency(String),

    // Failures in the procedural generation pipeline.
    #[error("Generation Failure: {0}")]
    GenerationPipeline(String),

    // Errors in the Godot bridge or FFI layer.
    #[error("Interface Bridge Error: {0}")]
    InterfaceBridge(String),

    // Critical, unexpected bug in core logic.
    #[error("Mythic Core Logic Failure (BUG!): {0}")]
    CoreLogicBug(String),

    // Wrapper for errors from external crates not covered elsewhere.
    #[error("External Crate Error: {0}")]
    External(String),
}

// -----------------------------------------------------------------------------
// Error Conversions
// -----------------------------------------------------------------------------
// Provides convenient conversions from common external error types into SSXLError.
// Ensures external libraries integrate smoothly with the SSXL error system.
impl From<bincode::Error> for SSXLError {
    fn from(err: bincode::Error) -> Self {
        SSXLError::Serialization(format!("Bincode failure: {}", err))
    }
}

impl From<anyhow::Error> for SSXLError {
    fn from(err: anyhow::Error) -> Self {
        SSXLError::External(format!("General anyhow error: {:?}", err))
    }
}
