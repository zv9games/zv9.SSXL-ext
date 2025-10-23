// ssxl_godot/src/aetherion_oracle.rs (Cleaned and updated for v8.2)

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use godot::builtin::Variant;
use crate::ssxl_engine::SSXLEngine; 

/// 🔮 AetherionOracle — Godot-facing node for manually driving the AetherionEngine 
/// and providing complex query/lookup logic (future state).
#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLOracle {
    #[base]
    base: Base<Node>,
    /// Link to the SSXL instance. Stored as Gd<SSXLEngine> for safety.
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

    /// Called when the node enters the scene tree.
    #[func]
    fn _ready(&mut self) {
        godot_print!("🔮 SSXLOracle (v8.2) is online. I await the ignition.");
        self.base_mut().set_process(true);
    }

    /// Links the Oracle to a target engine node.
    #[func]
    pub fn set_engine(&mut self, engine: Gd<SSXLEngine>) {
        self.engine = Some(engine);
        godot_print!("🔗 Oracle: Engine link established.");
    }

    /// Sends a tick to the linked engine (via an explicit call to the engine's tick function).
    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                // Assuming SSXLEngine will have a #[func] called 'tick'
                let args = [Variant::from(self.tick_count)];
                engine.call("tick", &args); 
                
                godot_print!("🔮 Oracle: Tick {} → Engine", self.tick_count);
                self.tick_count += 1;
            }
            None => {
                godot_warn!("⚠️ Oracle: No engine linked. Tick aborted.");
            }
        }
    }

    /// Responds to a ping from external systems.
    #[func]
    pub fn ping(&self) {
        godot_print!("🔮 Oracle: Ping received. I am awake.");
    }

    /// Resets the internal tick counter.
    #[func]
    pub fn reset(&mut self) {
        self.tick_count = 0;
        godot_print!("🔄 Oracle: Tick counter reset.");
    }

    /// Returns the current tick count.
    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}