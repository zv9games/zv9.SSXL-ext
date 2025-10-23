use godot::prelude::*;
#[allow(unused_imports)]
use aetherion_core::zv9_prelude::*;
use aetherion_shared::zv9_shared_logging::log_info;
use crate::AetherionEngine;

/// 🔮 AetherionOracle — Godot-facing node for manually driving the AetherionEngine.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionOracle {
    #[base]
    base: Base<Node>,
    engine: Option<Gd<AetherionEngine>>,
    tick_count: u64,
}

#[godot_api]
impl AetherionOracle {
    

    /// Called when the node enters the scene tree.
    #[func]
    fn _ready(&mut self) {
        godot_print!("🔮 Oracle is online. I await the ignition.");
        log_info("AetherionOracle", "Node for map oracle initialized");
        self.base_mut().set_process(true);

    }

    /// Links the Oracle to a target engine node.
    #[func]
    pub fn set_engine(&mut self, engine: Gd<AetherionEngine>) {
        self.engine = Some(engine);
        godot_print!("🔗 Oracle: Engine link established.");
    }

    /// Sends a tick to the linked engine.
    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                godot_print!("🔮 Oracle: Tick {} → Engine", self.tick_count);
                engine.call("tick", &[Variant::from(self.tick_count)]);
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
