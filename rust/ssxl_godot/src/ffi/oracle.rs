// ssxl_godot/src/ffi/oracle.rs

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
                // FIX: Use the correct method name `process_engine_tick` and remove the argument,
                // as confirmed by the compiler's suggestion.
                engine.bind_mut().process_engine_tick();
                self.tick_count += 1;
            }
            None => {
                // Engine not ready. Silent failure is acceptable during initialization.
            }
        }
    }

    #[func]
    pub fn get_current_tile_count(&self) -> u64 {
        match self.engine.as_ref() {
            Some(engine) => {
                engine.bind().get_current_tile_count()
            }
            None => {
                0
            }
        }
    }

    #[func]
    pub fn get_status(&self) -> GString {
        match self.engine.as_ref() {
            Some(engine) => {
                engine.bind().get_status()
            }
            None => {
                GString::from("Engine not bound.")
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