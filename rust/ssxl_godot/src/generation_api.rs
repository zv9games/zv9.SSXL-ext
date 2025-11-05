// ssxl_godot/src/generation_api.rs

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::Gd;
use godot::builtin::GString;

use std::sync::{Arc, Mutex};
use tracing::{info, error, warn};

// Internal Crate Dependencies
use ssxl_generate::Conductor;
// FIX: Replace the unresolved ssxl_shared import with the correctly exposed GeneratorConfig.
use ssxl_generate::GeneratorConfig; 


/// Delegate struct responsible for handling all calls from Godot related to 
/// map generation and sending commands to the background Conductor thread.
// ✅ FIX 1: Removed `Debug` because `Conductor` doesn't implement it, which prevents the derive macro from working.
#[derive(Default)] 
pub struct GenerationAPI<'a> {
    conductor: Option<&'a Arc<Mutex<Conductor>>>,
}

impl<'a> GenerationAPI<'a> {
    pub fn new(conductor: Option<&'a Arc<Mutex<Conductor>>>) -> Self {
        GenerationAPI { conductor }
    }

    // --------------------------------------------------------------------------
    // API IMPLEMENTATION
    // --------------------------------------------------------------------------

    /// Sends a request to the Conductor to start building a map.
    pub fn build_map(
        &self, 
        width: i32, 
        height: i32, 
        seed: GString, 
        generator_name: GString,
        _signals_node: Option<&Gd<Node>>,
    ) {
        if let Some(arc) = self.conductor {
            let result = arc.lock().map(|mut conductor| {
                info!("GenerationAPI: Attempting to build map with width={} height={}...", width, height);

                // FIX: Construct GeneratorConfig directly, aligning with Conductor's API.
                let config = GeneratorConfig {
                    width: width as usize,
                    height: height as usize,
                    seed: seed.to_string(),
                    generator_name: generator_name.to_string(),
                };
                
                // Call the Conductor's core method, which now takes a single config struct.
                conductor.start_generation(config)
            });

            match result {
                Ok(Ok(())) => info!("Generation command successfully dispatched to Conductor."),
                Ok(Err(e)) => error!("Failed to start map generation: {}", e),
                Err(e) => error!("Failed to lock Conductor mutex: {}", e),
            }

        } else {
            warn!("GenerationAPI: Conductor is not initialized. Cannot build map.");
        }
    }

    /// Attempts to change the active generator used by the Conductor.
    pub fn set_generator(&self, id: GString) -> bool {
        let generator_id: String = id.to_string(); // FIX: Use String instead of the undefined GeneratorId
        
        if let Some(arc) = self.conductor {
            return arc.lock().map_or_else(
                |e| {
                    error!("Failed to lock Conductor mutex for set_generator: {}", e);
                    false
                },
                |mut conductor| {
                    // FIX: Pass a String slice (&str) to set_active_generator
                    match conductor.set_active_generator(&generator_id) { 
                        Ok(()) => {
                            info!("GenerationAPI: Active generator set to '{}'.", generator_id);
                            true
                        }
                        Err(e) => {
                            error!("GenerationAPI: Failed to set generator '{}': {}", generator_id, e);
                            false
                        }
                    }
                },
            );
        }
        warn!("GenerationAPI: Conductor is not initialized. Cannot set generator.");
        false
    }
    
    /// Gets the ID of the currently active generator.
    pub fn get_active_generator_id(&self) -> GString {
        if let Some(arc) = self.conductor {
            return arc.lock().map_or_else(
                |e| {
                    error!("Failed to lock Conductor mutex for get_active_generator_id: {}", e);
                    GString::from("Error: Mutex Lock Failed")
                },
                |conductor| {
                    let id_string = conductor.get_active_generator_id();
                    // ✅ FIX 2: Convert the returned String to &String (reference) 
                    // which satisfies the GString::from trait bound.
                    GString::from(&id_string) 
                },
            );
        }
        GString::from("Error: Conductor Not Initialized")
    }
}