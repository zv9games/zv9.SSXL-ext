// ssxl_godot/src/SSXL_oracle.rs

//! # SSXLOracle
//!
//! This module defines the `SSXLOracle` Godot class, an optional, **decoupled manager**
//! responsible for driving the main `SSXLEngine` update loop. It provides an explicit
//! clock (`tick` method) for precise control, which is essential for deterministic
//! simulation or fixed-timestep game loops. It serves as a query endpoint for engine state.

// --- Godot GDExtension Imports ---
use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};

// --- Local Crate Imports ---
use crate::ssxl_engine::SSXLEngine;


// -----------------------------------------------------------------------------
// SSXLOracle Struct Definition
// -----------------------------------------------------------------------------

/// # SSXLOracle
///
/// A Godot class that acts as the external driver and observer of the SSXL system.
#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct SSXLOracle {
    /// The base field required for the Godot GDExtension class implementation.
    #[base]
    base: Base<Node>,

    /// A reference to the core `SSXLEngine` instance it controls.
    /// This connection allows the Oracle to explicitly call the engine's `tick` function.
    engine: Option<Gd<SSXLEngine>>,

    /// Internal counter for the number of times the engine has been updated.
    tick_count: u64,
}

impl SSXLOracle {
    /// Custom constructor logic called when the Godot node is created.
    pub fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
            tick_count: 0
        }
    }
}


// -----------------------------------------------------------------------------
// Exposed Godot API (Methods callable from GDScript/C#)
// -----------------------------------------------------------------------------

#[godot_api]
impl SSXLOracle {

    /// **Godot `_ready` Hook:** Initializes the Oracle upon entering the scene tree.
    #[func]
    fn _ready(&mut self) {
        godot_print!("🔮 SSXLOracle (v8.2) is online. I await the ignition.");
        self.base_mut().set_process(true);
    }

    /// Establishes the link between the Oracle and the `SSXLEngine` instance.
    /// This is mandatory before calling `tick`.
    #[func]
    pub fn set_engine(&mut self, engine: Gd<SSXLEngine>) {
        self.engine = Some(engine);
        godot_print!("🔗 Oracle: Engine link established.");
    }

    /// Drives the core game loop for the `SSXLEngine`, sending the current
    /// tick count as the time delta/update indicator. This dictates the system's **tempo**.
    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                // CLEANUP: Use direct method call for type-safety and performance.
                engine.bind_mut().tick(self.tick_count);

                godot_print!("🔮 Oracle: Tick {} → Engine", self.tick_count);
                self.tick_count += 1;
            }
            None => {
                godot_warn!("⚠️ Oracle: No engine linked. Tick aborted.");
            }
        }
    }

    /// ⭐ **FIX FOR ANIMATION:** Retrieves the total number of tiles placed by the engine.
    /// This is called by the GDScript animation timer to poll generation progress.
    #[func]
    pub fn get_total_tiles_placed(&self) -> i64 {
        match self.engine.as_ref() {
            Some(engine) => {
                // CRITICAL: Call the SSXLEngine FFI method to safely read the atomic/shared progress state.
                // NOTE: SSXLEngine MUST implement `get_current_tile_count` to fetch this data.
                engine.bind().get_current_tile_count() as i64
            }
            None => {
                0
            }
        }
    }
    
    /// Simple diagnostics function for checking if the Oracle is reachable.
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

    /// Retrieves the current tick count.
    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}