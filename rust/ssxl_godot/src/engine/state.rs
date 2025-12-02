use godot::prelude::*;
use godot::classes::{TileMap, Node};
use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};

// CRITICAL FIX 1: Import AnimationConductorHandle directly from ssxl_sync.
use ssxl_sync::AnimationConductorHandle; 

// FIX: Corrected ssxl_generate path.
use ssxl_generate::{Conductor, conductor::ConductorState}; 
use ssxl_shared::AnimationState;
// FIX: Removed unused imports.

// FIX 2: Path changed to reflect module moved to src/tilemap/
use crate::tilemap::async_poll::AsyncPoller; 

// FIX 3: EngineInitializer moved to api.rs. Keep only GenesisHandles here.
use crate::engine::api_initializers::GenesisHandles as FfiGenesisHandles; 

// FIX 4: Path changed to reflect module assimilated into engine::api.
// NOW includes EngineInitializer to resolve the E0432 error.
use crate::engine::api::{GenerationAPI, EngineInitializer};

/// Helper macro — the only way to safely touch the UnsafeCell
/// Required because Godot owns the object lifetime, not Rust.
// CRITICAL FIX: Add #[macro_export] to make the macro visible outside the module.
#[macro_export]
macro_rules! state {
    // This is the expression form (for chaining)
    ($self:ident) => { unsafe { &mut *$self._internal_state.get() } };
    // This is the statement form (for local variable binding)
    ($self:ident, $name:ident) => { let $name = unsafe { &mut *$self._internal_state.get() }; };
}

// Re-export the macro for use by other files within the 'engine' module.
pub use state; 

// --- InternalState Definition ---

/// All non-FFI-safe state lives here.
pub struct InternalState {
    pub conductor: Option<Arc<Mutex<Conductor>>>,
    pub conductor_state: Option<ConductorState>,
    pub animation_conductor: Option<AnimationConductorHandle>,
    pub animation_state: Option<AnimationState>,
    pub signals_node: Option<Gd<Node>>,
    pub tilemap_node: Option<Gd<TileMap>>,

    pub initializer: EngineInitializer,
    pub poller: AsyncPoller,
    
    // Uses aliased name FfiGenesisHandles
    pub genesis_handles: Option<FfiGenesisHandles>,
    pub generation_api: GenerationAPI,
    
    // FIX 7: Added missing tick counter
    pub tick_count: u64,
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            conductor: None,
            conductor_state: None,
            animation_conductor: None,
            animation_state: None,
            signals_node: None,
            tilemap_node: None,

            initializer: EngineInitializer::new(),
            poller: AsyncPoller::new(),
            
            genesis_handles: None,
            generation_api: GenerationAPI::default(),
            
            // FIX 7: Initialize tick_count
            tick_count: 0,
        }
    }
}

// --- SSXLEngine Class Definition ---

/// The Godot-facing class — the only thing that exists in GDScript land.
#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLEngine {
    #[base]
    pub base: Base<Node>,

    // FIX: Changed from private to pub(crate) to allow access via the state! macro across modules.
    // Hidden from Godot — contains all real state
    pub(crate) _internal_state: UnsafeCell<InternalState>,
}