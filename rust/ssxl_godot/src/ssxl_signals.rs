// ssxl_godot/src/ssxl_signals.rs

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Base;

#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct SSXLSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl SSXLSignals {
    pub fn init(base: Base<Node>) -> Self {
        SSXLSignals { base }
    }

    // --- Generation Lifecycle Signals ---

    #[signal]
    fn build_map_start();

    #[signal]
    fn chunk_generated(x: i32, y: i32);

    #[signal]
    fn build_map_complete();
    
    // NEW: Generation Stop
    #[signal]
    fn build_map_stopped();

    #[signal]
    fn generation_error(error_message: godot::prelude::GString);
    
    // NEW: Chunk Data Ready
    // NOTE: Argument names match the original `chunk_generated` signal structure
    #[signal]
    fn chunk_data_ready(x: i32, y: i32); 

    // NEW: Tick Complete
    #[signal]
    fn tick_complete(current_tick: u64);

    // --- Animation & Utility Signals ---

    #[signal]
    fn tile_flip_updated(tile_id: i32, flip_frame: i32);

    #[signal]
    fn animation_update(percent_done: f32, new_atlas_coords: godot::builtin::Vector2i);

    // NEW: Animation State
    #[signal]
    fn animation_state_changed(enabled: bool);

    #[signal]
    fn engine_status_updated(status_message: godot::prelude::GString);
}