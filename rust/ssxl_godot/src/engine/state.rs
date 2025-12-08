// ============================================================================
// 🎼 SSXL Engine State & Godot Integration (`crate::engine::state`)
// ----------------------------------------------------------------------------
// This module defines the internal state management and Godot-facing class
// for the SSXL engine. It provides the bridge between Rust’s procedural
// generation system and Godot’s scripting layer.
//
// Purpose:
//   • Encapsulate all runtime state of the SSXL engine in `InternalState`.
//   • Provide safe access to internal state via the `state!` macro.
//   • Expose `SSXLEngine` as a Godot class, enabling direct interaction
//     from GDScript while maintaining Rust’s safety guarantees.
//
// Key Components:
//   • state! macro
//       - Provides safe access to the `_internal_state` field inside `SSXLEngine`.
//       - Two forms:
//           • Expression form: `state!(self)` → returns mutable reference.
//           • Statement form: `state!(self, name)` → binds reference to local variable.
//       - Required because `_internal_state` is wrapped in `UnsafeCell`.
//       - Ensures interior mutability while respecting Godot’s object ownership.
//
//   • InternalState (struct)
//       - Holds all non-FFI-safe runtime state for the engine.
//       - Fields:
//           • conductor: shared `Conductor` instance (Arc<Mutex<Conductor>>).
//           • conductor_state: snapshot of conductor status.
//           • animation_conductor: handle for animation commands.
//           • animation_state: snapshot of animation status.
//           • signals_node: Godot Node for emitting signals.
//           • tilemap_node: Godot TileMap for rendering.
//           • initializer: bootstraps conductor threads and channels.
//           • poller: drains async channels (generation + animation).
//           • genesis_handles: optional initialization handles.
//           • generation_api: API for chunk data queries.
//           • tick_count: frame counter for engine ticks.
//       - Implements `Default` to provide an empty, safe initial state.
//
//   • SSXLEngine (struct)
//       - The Godot-facing class exposed to GDScript.
//       - Attributes:
//           • #[derive(GodotClass)] + #[class(tool, base = Node, init)]
//             - Marks SSXLEngine as a Godot class.
//             - `tool`: usable in Godot editor.
//             - `base = Node`: inherits from Godot’s Node.
//             - `init`: ensures proper initialization.
//       - Fields:
//           • base: Godot Node base.
//           • _internal_state: UnsafeCell<InternalState>, hidden from Godot.
//       - Wraps `InternalState` inside `UnsafeCell` to allow interior mutability,
//         since Godot owns object lifetime and Rust must adapt.
//
// Design Choices:
//   • Separation of `InternalState` from `SSXLEngine` keeps FFI-safe and
//     non-FFI-safe concerns distinct.
//   • Use of `Arc<Mutex>` ensures safe concurrent access to conductor state.
//   • `UnsafeCell` provides controlled interior mutability required by Godot.
//   • Macro-based access (`state!`) ensures consistent and safe state handling.
//
// Educational Note:
//   • This module demonstrates how Rust can integrate deeply with Godot by
//     exposing classes via `GodotClass` while maintaining strong safety
//     guarantees. By combining concurrency primitives, interior mutability,
//     and macro-based access, it creates a robust foundation for bridging
//     procedural generation logic with Godot’s scripting environment.
// ============================================================================


use godot::prelude::*;
use godot::classes::{TileMap, Node};
use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use ssxl_shared::AnimationConductorHandle;
use ssxl_generate::{Conductor, conductor::ConductorState}; 
use ssxl_shared::AnimationState;
use ssxl_shared::message::GenerationCommand;
use crate::tilemap::async_poll::AsyncPoller; 
use crate::engine::api_initializers::GenesisHandles as FfiGenesisHandles; 
use crate::engine::api::{GenerationAPI, EngineInitializer};
// FIX: ADDED IMPORTS FOR SENDER
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
// REMOVED: use ssxl_generate::task::task_queue::GenerationTask; // Unused after E0599 fix

#[macro_export]
macro_rules! state {
    ($self:ident) => { unsafe { &mut *$self._internal_state.get() } };
    ($self:ident, $name:ident) => { let $name = unsafe { &mut *$self._internal_state.get() }; };
}

pub use state; 

pub struct InternalState {
    pub conductor: Option<Arc<Mutex<Conductor>>>,
    pub conductor_state: Option<ConductorState>,
    pub animation_conductor: Option<AnimationConductorHandle>,
    pub animation_state: Option<AnimationState>,
    pub signals_node: Option<Gd<Node>>,
    pub tilemap_node: Option<Gd<TileMap>>,

    // FIX: ADDED MISSING FIELD for map generation requests
    pub request_sender: UnboundedSender<GenerationCommand>, 

    pub initializer: EngineInitializer,
    pub poller: AsyncPoller,
    
    pub genesis_handles: Option<FfiGenesisHandles>,
    pub generation_api: GenerationAPI,
    
    pub tick_count: u64,
}

impl Default for InternalState {
    fn default() -> Self {
        // FIX: Initialize the sender (unconnected at this stage)
        let (request_sender, _redundant_gen_rx) = unbounded_channel(); 
        
        Self {
            conductor: None,
            conductor_state: None,
            animation_conductor: None,
            animation_state: None,
            signals_node: None,
            tilemap_node: None,

            // FIX: Assign the initialized sender
            request_sender,

            initializer: EngineInitializer::new(),
            poller: AsyncPoller::new(),
            
            genesis_handles: None,
            generation_api: GenerationAPI::default(),
            
            tick_count: 0,
        }
    }
}

#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLEngine {
    #[base]
    pub base: Base<Node>,
    pub(crate) _internal_state: UnsafeCell<InternalState>,
}