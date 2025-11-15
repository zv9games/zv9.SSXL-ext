// ssxl_godot/src/generation_api.rs (CLEANED)

//! # GenerationAPI
//!
//! This module provides the high-level, exposed interface for the Godot runtime
//! (e.g., GDScript or C#) to control the **SSXL-ext Generation Conductor**.
//! It handles the necessary **synchronization (Arc/Mutex)** to safely pass commands
//! from the single-threaded Godot main loop to the background Conductor thread.

// --- Godot GDExtension Imports ---
use godot::classes::Node;
use godot::obj::Gd;
use godot::builtin::GString;

// --- Standard Library Imports (Synchronization & Logging) ---
use std::sync::{Arc, Mutex};
use tracing::{info, error, warn}; // Tracing/logging for debugging and tracking the tempo

// --- SSXL-ext Internal Crates Imports ---
use ssxl_generate::Conductor;
use ssxl_generate::GeneratorConfig;


// -----------------------------------------------------------------------------
// GenerationAPI Struct
// -----------------------------------------------------------------------------

/// # GenerationAPI
///
/// A wrapper that holds a reference to the **thread-safe Conductor**.
/// It is responsible for accepting Godot inputs, validating them, and forwarding
/// the commands to the core generation logic.
/// The lifetime parameter `'a` ensures the API wrapper doesn't outlive the engine context.
#[derive(Default)]
// FIX 1: Allows the struct to be constructed and used externally via FFI without warning.
#[allow(dead_code)] 
pub struct GenerationAPI<'a> {
    /// A potentially temporary, borrowed reference to the globally shared,
    /// mutex-protected `Conductor` instance.
    conductor: Option<&'a Arc<Mutex<Conductor>>>,
}

// FIX 2: Allows all methods, which are used externally, to exist without warning.
#[allow(dead_code)] 
impl<'a> GenerationAPI<'a> {
    /// Constructs a new `GenerationAPI`, injecting the `Conductor` reference.
    pub fn new(conductor: Option<&'a Arc<Mutex<Conductor>>>) -> Self {
        GenerationAPI { conductor }
    }

    // -------------------------------------------------------------------------
    // Core Method: Map Generation Dispatch
    // -------------------------------------------------------------------------

    /// Initiates a new map generation task on the worker threads.
    /// This is the primary method called from the Godot environment.
    ///
    /// # Arguments
    /// * `width`, `height`: Dimensions of the map (in tiles or chunks).
    /// * `seed`: The generation seed string, converted from Godot's `GString`.
    /// * `generator_name`: The ID of the generator (e.g., "CellularAutomata").
    pub fn build_map(
        &self,
        width: i32,
        height: i32,
        seed: GString,
        generator_name: GString,
        _signals_node: Option<&Gd<Node>>, // Retained for future signal use, currently unused
    ) {
        // FIX: Abort early if the dimensions are invalid (e.g., 0 or negative).
        if width <= 0 || height <= 0 {
            error!("GenerationAPI: Map dimensions must be positive (received {}x{}). Aborting command.", width, height);
            return;
        }

        // 1. Check for the Conductor and acquire the Mutex lock.
        if let Some(arc) = self.conductor {
            // Use `map` to perform the action only if the lock succeeds.
            let result = arc.lock().map(|mut conductor| {
                info!("GenerationAPI: Attempting to build map with width={} height={}...", width, height);

                // 2. Build the thread-safe configuration object.
                let config = GeneratorConfig {
                    width: width as usize,
                    height: height as usize,
                    seed: seed.to_string(),
                    generator_name: generator_name.to_string(),
                };

                // 3. Dispatch the command to the Conductor.
                // The Conductor handles placing this task on the worker queue.
                conductor.start_generation(config)
            });

            // 4. Handle results and synchronization errors.
            match result {
                Ok(Ok(())) => info!("Generation command successfully dispatched to Conductor."),
                Ok(Err(e)) => error!("Failed to start map generation: {}", e), // Conductor reported logic error
                Err(e) => error!("Failed to lock Conductor mutex: {}", e),         // Mutex poisoned/lock error
            }

        } else {
            warn!("GenerationAPI: Conductor is not initialized. Cannot build map.");
        }
    }

    // -------------------------------------------------------------------------
    // Utility Method: Active Generator Management
    // -------------------------------------------------------------------------

    /// Sets the active generator to be used for future generation requests.
    ///
    /// # Returns
    /// `true` if the generator was successfully set, `false` otherwise.
    pub fn set_generator(&self, id: GString) -> bool {
        let generator_id: String = id.to_string();

        if let Some(arc) = self.conductor {
            // Use `map_or_else` to handle both the Mutex lock error (`Err`) and the
            // internal Conductor's result (`Ok(Result)`).
            return arc.lock().map_or_else(
                |e| {
                    error!("Failed to lock Conductor mutex for set_generator: {}", e);
                    false
                },
                |mut conductor| {
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

    // -------------------------------------------------------------------------
    // Utility Method: Active Generator Query
    // -------------------------------------------------------------------------

    /// Retrieves the ID of the currently active generator.
    ///
    /// # Returns
    /// The generator ID as a Godot `GString`, or an error string if the lock fails.
    pub fn get_active_generator_id(&self) -> GString {
        if let Some(arc) = self.conductor {
            return arc.lock().map_or_else(
                |e| {
                    error!("Failed to lock Conductor mutex for get_active_generator_id: {}", e);
                    GString::from("Error: Mutex Lock Failed")
                },
                |conductor| {
                    let id_string = conductor.get_active_generator_id();
                    // Convert the internal Rust String back to Godot's GString type.
                    GString::from(&id_string)
                },
            );
        }
        GString::from("Error: Conductor Not Initialized")
    }
}