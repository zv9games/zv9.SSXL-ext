// ssxl_shared/src/errors.rs
//! Defines the canonical error type for the entire Aetherion Engine workspace.
//!
//! All crates should return an 'AetherionError' to ensure clean, consistent
//! error propagation throughout the engine layers (Core, Generation, Godot Interface).

use thiserror::Error;

/// The primary result type used throughout the Aetherion Engine.
/// Aliases the standard Result using the engine's canonical error type.
pub type SSXLResult<T> = Result<T, SSXLError>;

/// Canonical error type for all Aetherion Engine components.
#[derive(Error, Debug)]
pub enum SSXLError {
    /// Errors related to file operations, configuration loading, or I/O failure.
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors related to data serialization or deserialization (e.g., failed bincode/serde operation).
    #[error("Serialization/Data Error: {0}")]
    Serialization(String),

    /// Errors caused by data structures being in an invalid or impossible state (e.g., malformed GridBounds).
    #[error("Data Consistency Error: {0}")]
    DataConsistency(String),

    /// Errors specific to the generation pipeline, often due to configuration or boundary failures.
    #[error("Generation Failure: {0}")]
    GenerationPipeline(String),

    /// Errors related to external FFI or GDExtension communication.
    #[error("Interface Bridge Error: {0}")]
    InterfaceBridge(String),

    /// An error that should never happen, indicating a fundamental bug in logic or state management.
    #[error("Mythic Core Logic Failure (BUG!): {0}")]
    CoreLogicBug(String),

    /// Catch-all for other external errors, often used when converting from third-party crates.
    /// This variant is primarily used to wrap `anyhow::Error`.
    #[error("External Crate Error: {0}")]
    External(String),
}

// --- Helper Conversion Implementations (for common libraries) ---

/// Example conversion for the 'bincode' serialization library.
impl From<bincode::Error> for SSXLError {
    fn from(err: bincode::Error) -> Self {
        SSXLError::Serialization(format!("Bincode failure: {}", err))
    }
}

/// Allows conversion from a generic `anyhow::Error` into the canonical `AetherionError`.
/// This is CRITICAL for using the `anyhow!` macro in dependent crates.
impl From<anyhow::Error> for SSXLError {
    fn from(err: anyhow::Error) -> Self {
        SSXLError::External(format!("General anyhow error: {}", err))
    }
}

// You can add more 'From' implementations here as you introduce new dependencies,
// like 'tokio::JoinError' or 'rand::Error'.
