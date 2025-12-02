use std::sync::{Arc, Mutex};
use tracing::{info, error, warn};

use ssxl_generate::Conductor;
use ssxl_sync::AnimationConductorHandle;

// NOTE: All Godot and ssxl_shared imports (`Node`, `TileMap`, `Vec2i`, `AnimationCommand`) 
// have been removed as they were only required by the synchronous FFI methods we just deleted.

#[derive(Default)]
#[allow(dead_code)]
pub struct AnimationAPI<'a> {
    // These handles are kept because the Rust FFI layer needs to store them
    // to manage the lifecycle of the worker thread and potentially use them
    // for internal Rust-to-Rust communication or cleanup.
    animation_conductor: Option<&'a AnimationConductorHandle>,
    _conductor: Option<&'a Arc<Mutex<Conductor>>>,
}

// All external FFI control methods have been removed to eliminate synchronous entropy.
// The Animation Conductor must now be driven by internal Rust events or data availability,
// not by explicit Godot calls.
#[allow(dead_code)]
impl<'a> AnimationAPI<'a> {
    pub fn new(
        animation_conductor: Option<&'a AnimationConductorHandle>,
        conductor: Option<&'a Arc<Mutex<Conductor>>>,
    ) -> Self {
        AnimationAPI {
            animation_conductor,
            _conductor: conductor,
        }
    }
}