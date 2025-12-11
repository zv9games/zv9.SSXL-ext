// ============================================================================
// 🔮 SSXL Oracle (`crate::ffi::oracle`)
// ----------------------------------------------------------------------------
// This module defines the `SSXLOracle` class, a Godot-facing node that acts
// as a lightweight interface to the SSXL engine. It provides query and control
// methods that can be called directly from GDScript, serving as an "oracle"
// for engine status, ticks, and tile counts.
//
// Purpose:
//   • Expose a simple Godot node (`SSXLOracle`) that delegates work to `SSXLEngine`.
//   • Provide lifecycle hooks and query methods accessible from GDScript.
//   • Track tick progression and allow external scripts to reset or monitor it.
//   • Offer a clean bridge between Rust engine logic and Godot’s scripting layer.
//
// Key Components:
//   • SSXLOracle (struct)
//       - Attributes:
//           • #[derive(GodotClass)] + #[class(tool, base = Node, init)]
//             - Marks SSXLOracle as a Godot class.
//             - `tool`: usable in the Godot editor.
//             - `base = Node`: inherits from Godot’s Node.
//             - `init`: ensures proper initialization.
//       - Fields:
//           • base: underlying Godot Node.
//           • engine: optional reference to `SSXLEngine`.
//           • tick_count: counter for processed ticks.
//
//   • init (method)
//       - Constructor for SSXLOracle.
//       - Initializes with no engine bound and tick_count = 0.
//
//   • Godot API Methods (#[godot_api])
//       - _ready
//           • Lifecycle hook called when node enters the scene tree.
//           • Enables per-frame processing.
//       - set_engine
//           • Binds an `SSXLEngine` instance to this oracle.
//           • Allows delegation of tick and query methods.
//       - tick
//           • Advances the engine by one tick via `process_engine_tick`.
//           • Increments tick_count.
//       - get_current_tile_count
//           • Queries engine for total tiles generated.
//           • Returns 0 if engine not bound.
//       - get_status
//           • Queries engine for human-readable status string.
//           • Returns "Engine not bound." if no engine attached.
//       - ping
//           • Simple test function; placeholder for connectivity checks.
//       - reset
//           • Resets tick_count to 0.
//       - get_tick
//           • Returns current tick_count.
//
// Design Choices:
//   • Oracle pattern provides a lightweight façade over the engine.
//   • Optional engine reference allows flexible binding/unbinding at runtime.
//   • Tick counter enables monitoring of engine progression from scripts.
//   • Minimal methods keep the API surface simple and script-friendly.
//
// Educational Note:
//   • This module demonstrates how Rust can expose custom Godot nodes that
//     act as façades over complex engine logic. By delegating to `SSXLEngine`,
//     `SSXLOracle` provides a clean, script-accessible interface for queries
//     and ticks, while maintaining Rust’s safety and Godot’s usability.
// ============================================================================


use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use crate::engine::SSXLEngine;

#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLOracle {
    #[base]
    base: Base<Node>,
    engine: Option<Gd<SSXLEngine>>,
    tick_count: u64,
}

impl SSXLOracle {
    pub fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
            tick_count: 0,
        }
    }
}

#[godot_api]
impl SSXLOracle {
    #[func]
    fn _ready(&mut self) {
        self.base_mut().set_process(true);
    }

    #[func]
    pub fn set_engine(&mut self, engine: Gd<SSXLEngine>) {
        self.engine = Some(engine);
    }

    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                engine.bind_mut().process_engine_tick();
                self.tick_count += 1;
            }
            None => {}
        }
    }

    #[func]
    pub fn get_current_tile_count(&self) -> u64 {
        match self.engine.as_ref() {
            Some(engine) => engine.bind().get_current_tile_count(),
            None => 0,
        }
    }

    #[func]
    pub fn get_status(&self) -> GString {
        match self.engine.as_ref() {
            Some(engine) => engine.bind().get_status(),
            None => GString::from("Engine not bound."),
        }
    }

    #[func]
    pub fn ping(&self) {}

    #[func]
    pub fn reset(&mut self) {
        self.tick_count = 0;
    }

    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}
