use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::Gd;
use godot::builtin::GString;
use std::sync::{Arc, Mutex};
use ssxl_generate::Conductor;
use ssxl_generate::conductor_state::ConductorState;
use ssxl_sync::{AnimationConductorHandle, AnimationState, Receiver};
use crate::api_initializers::EngineInitializer;
use crate::generation_api::GenerationAPI;
use crate::animation_api::AnimationAPI;
use crate::ssxl_engine::SSXLEngine; // Import the host struct

// -----------------------------------------------------------------------------
// 1. Core Setup Logic (Moves complexity out of SSXLEngine::initialize_core)
// -----------------------------------------------------------------------------

/// Assumption: We are adding this method to the existing EngineInitializer impl.
/// This method encapsulates all thread spawning and handle creation, which used
/// to take up ~15 lines in SSXLEngine::initialize_core.
impl EngineInitializer {
    /// Executes the full multi-threaded core setup, returning all necessary handles.
    pub fn execute_core_setup(
    ) -> (
        Option<Arc<Mutex<Conductor>>>, Option<Receiver>, Option<ConductorState>, 
        Option<AnimationConductorHandle>, Option<Receiver>, Option<AnimationState>
    ) {
        // The complex logic (info! logs, thread spawning, result unwrapping)
        // for ensure_conductor() and ensure_animation_conductor() lives here now.
        let (conductor_arc, gen_rx, gen_state) = EngineInitializer::ensure_conductor();
        let (anim_handle, anim_rx, anim_state) = EngineInitializer::ensure_animation_conductor();
        
        (conductor_arc, gen_rx, gen_state, anim_handle, anim_rx, anim_state)
    }
}


// -----------------------------------------------------------------------------
// 2. Trait Definition (The boilerplate reducer)
// -----------------------------------------------------------------------------

/// Trait implemented on SSXLEngine to provide access to API delegation methods
/// without cluttering the main SSXLEngine impl block.
pub trait EngineApiExtension {
    // This trait is empty but used to implement the methods below.
}

// -----------------------------------------------------------------------------
// 3. Trait Implementation (The bulk of the logic moved from ssxl_engine.rs)
// -----------------------------------------------------------------------------

/// Implements all public Godot API methods that are simple delegates.
/// This drastically reduces the LOC count in ssxl_engine.rs.
#[godot_api]
impl EngineApiExtension for SSXLEngine {
    
    // --- Generation API Delegation (REMOVED FROM ssxl_engine.rs) ---

    #[func]
    fn build_map(&mut self, width: i32, height: i32, seed: GString, generator_name: GString) {
        GenerationAPI::new(self.conductor.as_ref()).build_map(width, height, seed, generator_name, self.signals_node.as_ref());
    }

    #[func]
    fn set_generator(&mut self, id: GString) -> bool {
        GenerationAPI::new(self.conductor.as_ref()).set_generator(id)
    }

    #[func]
    fn get_active_generator_id(&self) -> GString {
        GenerationAPI::new(self.conductor.as_ref()).get_active_generator_id()
    }

    // --- Animation API Delegation (REMOVED FROM ssxl_engine.rs) ---

    #[func]
    fn start_loading_animation(&mut self, framerate: f32) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).start_loading_animation(framerate, self.signals_node.as_ref());
    }

    #[func]
    fn register_chunk_for_animation(&mut self, chunk_x: i32, chunk_y: i32) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).register_chunk_for_animation(chunk_x, chunk_y);
    }

    #[func]
    fn stop_loading_animation(&mut self) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref()).stop_loading_animation(self.signals_node.as_ref());
    }

    #[func]
    fn send_animation_command(&mut self, command_name: GString) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .send_command_by_name(command_name.to_string());
    }

    #[func]
    fn start_test_animation(&mut self) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .start_test_animation(self.tilemap_node.as_ref());
    }

    #[func]
    fn stop_test_animation(&mut self) {
        AnimationAPI::new(self.animation_conductor.as_ref(), self.conductor.as_ref())
            .stop_test_animation(self.signals_node.as_ref());
    }
}