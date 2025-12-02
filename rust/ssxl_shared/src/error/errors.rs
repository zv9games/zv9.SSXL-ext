// ssxl_shared/src/errors.rs

//! # SSXL Error Definitions (`ssxl_shared::errors`)
//!
//! This module defines the global, canonical error type for the entire SSXL-ext
//! procedural generation engine. Centralizing error handling ensures that failures
//! across different crates (math, generate, cache, godot) can be consistently
//! reported, managed, and debugged, particularly across FFI boundaries.

use thiserror::Error;

/// A specialized `Result` type for the SSXL-ext project.
///
/// All function calls that can fail within the SSXL ecosystem should return this
/// type, wrapping the concrete `SSXLError` enum.
pub type SSXLResult<T> = Result<T, SSXLError>;

/// The comprehensive enumeration of all possible errors within the SSXL-ext engine.
#[derive(Error, Debug)]
pub enum SSXLError {
    /// Wrapper for standard I/O errors (e.g., file system access, network issues).
    /// This variant automatically handles conversion from `std::io::Error`.
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors encountered during data serialization (e.g., using Bincode or Serde).
    /// Indicates problems with converting data to or from a byte stream.
    #[error("Serialization/Data Error: {0}")]
    Serialization(String),

    /// Errors related to unexpected or invalid data states, such as a generator
    /// returning a tile array of the wrong size or an invalid chunk key.
    #[error("Data Consistency Error: {0}")]
    DataConsistency(String),

    /// Errors specific to the procedural generation pipeline (e.g., a generator
    /// failing to find a valid output after too many iterations).
    #[error("Generation Failure: {0}")]
    GenerationPipeline(String),

    /// Errors occurring in the Godot GDExtension bridge or FFI layer.
    /// Crucial for debugging communication issues between Rust and the Godot runtime.
    #[error("Interface Bridge Error: {0}")]
    InterfaceBridge(String),

    /// A critical, unexpected error indicating a bug in the core logic that
    /// should never occur under normal execution (a **"BUG!"**).
    #[error("Mythic Core Logic Failure (BUG!): {0}")]
    CoreLogicBug(String),

    /// Generic wrapper for errors originating from third-party libraries or crates
    /// that are not covered by other specific variants.
    #[error("External Crate Error: {0}")]
    External(String),
}


// --- Error Conversion Implementations ---

/// Implements conversion from the `bincode::Error` type into the SSXL `Serialization` error.
impl From<bincode::Error> for SSXLError {
    fn from(err: bincode::Error) -> Self {
        SSXLError::Serialization(format!("Bincode failure: {}", err))
    }
}

/// Implements conversion from the generic `anyhow::Error` type into the SSXL `External` error.
/// This provides a convenient way to integrate external library errors into the SSXL system.
impl From<anyhow::Error> for SSXLError {
    fn from(err: anyhow::Error) -> Self {
        // Use the debug message of the anyhow error to retain the source chain.
        SSXLError::External(format!("General anyhow error: {:?}", err))
    }
}