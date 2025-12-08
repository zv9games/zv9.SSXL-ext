// ============================================================================
// 🎼 SSXL Godot Extension Entry (`lib.rs` or equivalent)
// ----------------------------------------------------------------------------
// This module defines the entry point and lifecycle bindings for integrating
// the SSXL engine with Godot via GDExtension. It ensures that the Rust-based
// `SSXLEngine` struct is properly registered as a Godot class and participates
// in the engine’s scene graph lifecycle.
//
// Purpose:
//   • Register `SSXLEngine` as a Godot class so it can be instantiated in GDScript.
//   • Bind Rust implementations of lifecycle methods (_init, _process, _ready)
//     to Godot’s Node lifecycle.
//   • Provide the mandatory extension entry point (`ssxl_godot_init`) for Godot.
//
// Key Components:
//   • #[gdextension] ssxl_godot_init
//       - Mandatory entry point for Godot’s GDExtension system.
//       - Called automatically when the dynamic library is loaded.
//       - Registers `SSXLEngine` with Godot via `builder.add_class::<SSXLEngine>()`.
//       - Declared `unsafe` because it interacts directly with Godot’s C/C++ runtime.
//
//   • ExtensionLibrary Implementation for SSXLEngine
//       - Maps Godot’s virtual methods to Rust implementations:
//           • _init: Constructor, runs when SSXLEngine is instantiated.
//             Delegates to `SSXLEngine::init`.
//           • _process: Called every frame, passes delta time to engine tick.
//             Delegates to `SSXLEngine::tick`, casting delta to `u64`.
//           • _ready: Called once when the node enters the scene tree.
//             Delegates to `SSXLEngine::on_ready` for startup logic.
//       - Ensures SSXLEngine participates fully in Godot’s scene lifecycle.
//
// Design Choices:
//   • Using `#[gdextension]` provides seamless integration with Godot’s extension system.
//   • Delegating lifecycle hooks to SSXLEngine methods keeps responsibilities clear:
//       - Initialization → `init`
//       - Frame updates → `tick`
//       - Scene setup → `on_ready`
//   • Unsafe block is minimized to the extension entry point, isolating FFI concerns.
//
// Educational Note:
//   • This module demonstrates how Rust can extend Godot by exposing custom
//     classes via GDExtension. By binding lifecycle methods, it ensures that
//     Rust logic runs in sync with Godot’s scene graph, enabling powerful
//     procedural generation and engine orchestration directly from GDScript.
// ============================================================================


use godot::prelude::*;
use godot::classes::Node;
use crate::ssxl_engine::SSXLEngine;

#[gdextension]
unsafe fn ssxl_godot_init(builder: &mut InitHandle) {
    builder.add_class::<SSXLEngine>();
}

impl ExtensionLibrary for SSXLEngine {
    fn _init(base: Base<Node>) -> Self {
        SSXLEngine::init(base)
    }

    fn _process(&mut self, delta: f64) {
        self.tick(delta as u64);
    }

    fn _ready(&mut self) {
        self.on_ready();
    }
}
