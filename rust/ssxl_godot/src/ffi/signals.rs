// ssxl_godot/src/ffi/signals.rs

use godot::prelude::*;
// FIX: Removed the unused glob import `godot::classes::*` and replaced it with an explicit import for `Node`.
use godot::classes::Node; 
// FIX: Removed unnecessary import for `godot::obj::Base` as it is covered by `godot::prelude::*`.

#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLSignals {
    #[base]
    base: Base<Node>,
}

// FIX 2: Define the required constructor in a standard impl block.
impl SSXLSignals {
    pub fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl SSXLSignals {
    // The previous #[func] pub fn init is removed from here.

    // --- Generation Lifecycle Signals ---
    
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

    // --- Animation & Utility Signals ---
    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);

    #[signal]
    fn animation_update(percent_done: f32, new_atlas_coords: Vector2i);

    #[signal]
    fn animation_state_changed(enabled: bool);

    #[signal]
    fn engine_status_updated(status_message: GString);
}