// ============================================================================
// 📡 SSXL Signals (`crate::ffi::signals`)
// ----------------------------------------------------------------------------
// This module defines the `SSXLSignals` class, a Godot-facing broadcaster node
// that emits signals from the Rust-based SSXL engine into the Godot scene graph.
// Signals are the primary communication bridge between Rust logic and Godot
// scripts, enabling real-time updates, event handling, and synchronization.
//
// Purpose:
//   • Provide a dedicated Godot node (`SSXLSignals`) for emitting engine signals.
//   • Allow GDScript or C# scripts to connect to engine events without directly
//     accessing Rust internals.
//   • Broadcast generation lifecycle events, animation updates, and engine status.
//
// Key Components:
//   • SSXLSignals (struct)
//       - Attributes:
//           • #[derive(GodotClass)] + #[class(tool, base = Node, init)]
//             - Marks SSXLSignals as a Godot class.
//             - `tool`: usable in the Godot editor.
//             - `base = Node`: inherits from Godot’s Node.
//             - `init`: ensures proper initialization.
//       - Fields:
//           • base: underlying Godot Node wrapped by this broadcaster.
//
//   • init (method)
//       - Constructor for SSXLSignals.
//       - Wraps the provided Node base inside the struct.
//
//   • Godot API Implementation (#[godot_api])
//       - Defines all signals exposed to Godot via #[signal] attributes.
//       - These signals can be connected to GDScript or C# methods in the editor.
//
// Signals Overview:
//   • Generation Lifecycle
//       - build_map_start: emitted when map generation begins.
//       - chunk_data_updated(x, y): emitted when chunk data is updated.
//       - chunk_generated_batch(batch): emitted when a chunk is fully generated
//         and converted into a render batch.
//       - build_map_complete: emitted when map generation completes successfully.
//       - build_map_stopped: emitted when generation is stopped prematurely.
//       - generation_error(error_message): emitted when a generation error occurs.
//       - chunk_data_ready(x, y): emitted when chunk data is ready for use.
//       - tick_complete(current_tick): emitted at the end of each tick (frame).
//
//   • Animation & Utility
//       - tile_flip_updated(tile_id, flip_frame): emitted when a tile’s flip frame updates.
//       - animation_update(percent_done, new_atlas_coords): emitted during animation updates.
//       - animation_state_changed(enabled): emitted when animation state changes.
//       - engine_status_updated(status_message): emitted when engine status changes.
//
// Design Choices:
//   • Separation of concerns: SSXLSignals contains no engine state, only broadcasts.
//   • Signal-based communication ensures loose coupling between Rust and Godot scripts.
//   • Rich set of signals provides granular visibility into engine lifecycle and animation.
//
// Educational Note:
//   • This module demonstrates how Rust can expose event-driven communication
//     to Godot via signals. By centralizing all engine broadcasts in `SSXLSignals`,
//     the design ensures that external scripts can react to engine events in a
//     clean, idiomatic Godot way, while Rust maintains control of core logic.
// ============================================================================


use godot::prelude::*;
use godot::classes::Node; 

#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLSignals {
    #[base]
    base: Base<Node>,
}

impl SSXLSignals {
    pub fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl SSXLSignals {
    #[signal]
    fn build_map_start();

    #[signal]
    fn chunk_data_updated(x: i32, y: i32);

    #[signal]
    fn chunk_generated_batch(batch: Dictionary);

    #[signal]
    fn build_map_complete();

    #[signal]
    fn build_map_stopped();

    #[signal]
    fn generation_error(error_message: GString);

    #[signal]
    fn chunk_data_ready(x: i32, y: i32);

    #[signal]
    fn tick_complete(current_tick: u64);

    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);

    #[signal]
    fn animation_update(percent_done: f32, new_atlas_coords: Vector2i);

    #[signal]
    fn animation_state_changed(enabled: bool);

    #[signal]
    fn engine_status_updated(status_message: GString);
	
	#[signal]
    fn chunk_applied(key_x: i64, key_y: i64);
}
