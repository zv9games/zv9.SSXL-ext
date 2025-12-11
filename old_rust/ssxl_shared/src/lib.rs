// ============================================================================
// 📦 SSXL Shared Crate Root
// File: ssxl_shared/src/lib.rs (or equivalent entry point)
// ----------------------------------------------------------------------------
// This file defines the top-level structure of the `ssxl_shared` crate.
// It organizes submodules, re-exports key types, and provides FFI entry points
// for external runtimes (CLI, Godot, etc.).
//
// Educational notes:
//   - In Rust, the crate root (`lib.rs` or `main.rs`) is the starting point
//     for module resolution. Declaring `pub mod ...` here makes submodules
//     available to the rest of the crate.
//   - Re-exports (`pub use ...`) provide a curated public API, so downstream
//     code doesn’t need to know the internal file layout.
//   - FFI functions (`#[no_mangle] extern "C" fn ...`) allow this crate to be
//     called from non-Rust environments (e.g., C, Godot).
// ============================================================================

use serde::{Deserialize, Serialize}; // Serialization traits for data persistence and networking
use tracing;                         // Structured logging for runtime diagnostics

// -----------------------------------------------------------------------------
// 📂 Submodule Declarations
// -----------------------------------------------------------------------------
// Each `pub mod` corresponds to a directory or file under `src/`.
// Declaring them here makes their contents available throughout the crate.
// -----------------------------------------------------------------------------
pub mod chunk;    // Chunk-related data structures (ChunkData, GridBounds, etc.)
pub mod tile;     // Tile-related data structures (TileData, TileType, etc.)
pub mod error;    // Error handling utilities (SSXLError, SSXLResult)
pub mod config;   // Configuration loading and parsing (SSXLConfig)
pub mod message;  // Messaging system for animation/generation commands
pub mod math;     // Math primitives and helpers (serialization, vectors, etc.)
pub mod job;      // Job definitions for generation tasks

// -----------------------------------------------------------------------------
// 🔗 Public Re-exports
// -----------------------------------------------------------------------------
// These re-exports expose commonly used types at the crate root.
// This simplifies imports for downstream crates (e.g., `ssxl_godot`).
// -----------------------------------------------------------------------------
pub use config::config::{get_config_from_path, SSXLConfig}; // Config loader + struct
pub use ssxl_math::primitives::{ChunkId, TileCoord};        // Core math identifiers

pub use chunk::chunk_data::{ChunkData, CHUNK_SIZE};         // Chunk struct + size constant
pub use tile::tile_data::TileData;                          // Tile struct
pub use chunk::grid_bounds::GridBounds;                     // Chunk boundary struct
pub use tile::tile_type::TileType;                          // Tile type enum

pub use message::messages::{
    AnimationCommand,           // Commands sent to animation subsystem
    AnimationType,              // Types of animations (flip, tween, etc.)
    AnimationPayload,           // Payload data for animation updates
    UpdateSender,               // Channel type for sending updates
    AnimationConductorHandle,   // Channel type for sending commands
    AnimationState,             // Global animation state struct
    CommandResult,              // Standardized command result type
    AnimationUpdate,            // Struct for animation update messages
};

pub use error::errors::{SSXLError, SSXLResult}; // Error type + result alias
pub use anyhow;                                // General-purpose error handling crate

// -----------------------------------------------------------------------------
// 🌐 FFI Export Function
// -----------------------------------------------------------------------------
// Provides an entry point for external runtimes (e.g., CLI).
// `#[no_mangle]` ensures the function name is preserved for linking.
// `extern "C"` makes it callable from C or other languages.
// -----------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn ssxl_start_runtime() -> bool {
    initialize_shared_data();
    tracing::info!("FFI Function `ssxl_start_runtime` called and shared initialization completed.");
    true 
}

// -----------------------------------------------------------------------------
// 📊 Shared Data Structures
// -----------------------------------------------------------------------------
// Defines simple data primitives used across the engine.
// These can be serialized for persistence or transmitted across FFI boundaries.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SSXLData {
    pub id: u64,        // Unique identifier
    pub timestamp: u64, // Timestamp for creation/update
    pub value: String,  // Arbitrary string payload
}

pub fn initialize_shared_data() {
    tracing::info!("SSXL Shared Data Primitives initialized (Priority 1 complete).");
}

// -----------------------------------------------------------------------------
// 📦 Prelude Module
// -----------------------------------------------------------------------------
// The `prelude` re-exports commonly used types so that downstream code can
// import them with a single `use ssxl_shared::prelude::*;`.
// This is a Rust convention for convenience and readability.
// -----------------------------------------------------------------------------
pub mod prelude {
    pub use super::chunk::chunk_data::{ChunkData, CHUNK_SIZE};
    pub use super::tile::tile_data::TileData;
    pub use super::chunk::grid_bounds::GridBounds;
    pub use super::tile::tile_type::TileType;
    
    pub use super::{ChunkId, TileCoord};
    
    pub use super::error::errors::{SSXLError, SSXLResult};
    
    // Animation-related types
    pub use super::message::messages::{AnimationCommand, AnimationType, AnimationState};
    pub use super::message::messages::AnimationUpdate;

    // Config access
    pub use super::config::config::SSXLConfig;
}

// -----------------------------------------------------------------------------
// 📈 Global Atomic Counter
// -----------------------------------------------------------------------------
// Tracks the number of chunks completed during generation.
// `AtomicUsize` allows safe concurrent updates across threads.
// -----------------------------------------------------------------------------
use std::sync::atomic::AtomicUsize;

pub static CHUNKS_COMPLETED_COUNT: AtomicUsize = AtomicUsize::new(0);
