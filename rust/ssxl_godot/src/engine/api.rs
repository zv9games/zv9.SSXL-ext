// ============================================================================
// 🎼 Engine Initialization and API Layer (`crate::engine::initializer`)
// ----------------------------------------------------------------------------
// This module provides the glue between the SSXL engine core and the Godot
// game engine. It defines initialization routines and public-facing APIs that
// allow Godot scripts to interact with the procedural generation system.
//
// Purpose:
//   • Bootstrap the SSXL engine inside Godot.
//   • Set up conductor state, communication channels, and runtime orchestration.
//   • Provide a safe, thread-aware interface for spawning and managing the Conductor.
//   • Expose a public API for chunk data retrieval compatible with Godot.
//
// Key Components:
//   • EngineInitializer (struct)
//       - Acts as a bootstrapper for the engine.
//       - Provides methods to:
//           • Create a new initializer (`new`).
//           • Execute channel and state setup (`execute_channel_and_state_setup`).
//           • Wrap and spawn the conductor (`execute_conductor_setup_and_spawn`).
//       - Ensures conductor is safely shared across threads using `Arc<Mutex>`.
//       - Extracts and clones `AnimationConductorHandle` for animation orchestration.
//
//   • GenerationAPI (struct)
//       - Provides a public-facing API for chunk data retrieval.
//       - Intended to be called from Godot scripts.
//       - Currently a stub implementation returning an empty `Dictionary`.
//       - Future extension point for exposing generated chunk data to Godot.
//
// Workflow:
//   1. Godot calls `EngineInitializer::new()` to create an initializer.
//   2. `execute_channel_and_state_setup` wires channels and conductor state.
//   3. `execute_conductor_setup_and_spawn` wraps conductor in `Arc<Mutex>`
//      and provides an animation handle.
//   4. Godot scripts can call `GenerationAPI::fetch_chunk_data` to retrieve
//      chunk data (stubbed for now).
//
// Design Choices:
//   • Separation of initialization (EngineInitializer) from data access (GenerationAPI).
//   • Use of `Arc<Mutex>` ensures safe concurrent access to conductor across threads.
//   • Returning `Dictionary` aligns with Godot’s native data structures for FFI.
//   • Stubbed `fetch_chunk_data` provides a placeholder for future expansion.
//
// Educational Note:
//   • This module demonstrates how Rust can integrate with external engines
//     (like Godot) via FFI. By combining safe concurrency primitives, modular
//     initialization, and Godot-compatible data types, it creates a bridge
//     between procedural generation logic and game engine scripting.
// ============================================================================

use godot::prelude::GString;
use godot::builtin::Dictionary;
use std::error::Error;
use std::sync::{Arc, Mutex};
use ssxl_generate::Conductor;
use ssxl_shared::AnimationConductorHandle;
use crate::engine::api_initializers::{execute_channel_and_state_setup, GenesisHandles};

#[derive(Default)]
pub struct EngineInitializer {}

impl EngineInitializer {
    pub fn new() -> Self { Self::default() }

    pub fn execute_channel_and_state_setup(
        &self,
        config_path: Option<&str>,
    ) -> Result<GenesisHandles, Box<dyn Error>> {
        execute_channel_and_state_setup(config_path)
    }

    pub fn execute_conductor_setup_and_spawn(
        &self,
        handles: GenesisHandles,
    ) -> (Option<Arc<Mutex<Conductor>>>, AnimationConductorHandle) {
        let conductor_arc = Arc::new(Mutex::new(handles._gen_conductor));
        let anim_handle: AnimationConductorHandle = handles.anim_command_tx.clone();
        (Some(conductor_arc), anim_handle)
    }
}

#[derive(Debug, Clone)]
pub enum GenerationAPI {
    Uninitialized,
    Active,
    Halted,
    Error(String),
}

impl Default for GenerationAPI {
    fn default() -> Self {
        GenerationAPI::Uninitialized
    }
}

impl GenerationAPI {
    /// Return a human-readable status string for Godot
    pub fn as_status(&self) -> GString {
        match self {
            GenerationAPI::Uninitialized => GString::from("Uninitialized"),
            GenerationAPI::Active => GString::from("Active"),
            GenerationAPI::Halted => GString::from("Halted"),
            GenerationAPI::Error(msg) => GString::from(format!("Error: {}", msg).as_str()),
        }
    }

    /// Example stub for chunk data retrieval
    pub fn fetch_chunk_data(&self, _x: i32, _y: i32) -> Dictionary {
        Dictionary::new()
    }
}

