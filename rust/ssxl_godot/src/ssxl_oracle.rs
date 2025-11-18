use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use crate::ssxl_engine::SSXLEngine;

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
            tick_count: 0
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
                // Drives the engine. The engine itself emits the tick_complete signal 
                // after processing the work, ensuring quantum alignment.
                engine.bind_mut().tick(self.tick_count);
                self.tick_count += 1;
            }
            None => {
                // Should only be hit if the system fails to link.
                // Keeping this for safety, even if stripped of print
            }
        }
    }

    #[func]
    pub fn get_total_tiles_placed(&self) -> i64 {
        match self.engine.as_ref() {
            Some(engine) => {
                engine.bind().get_current_tile_count() as i64
            }
            None => {
                0
            }
        }
    }
    
    #[func]
    pub fn ping(&self) {
    }

    #[func]
    pub fn reset(&mut self) {
        self.tick_count = 0;
    }

    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}