// ssxl_generate/generator_manager.rs

//! The GeneratorManager acts as a registry for all available procedural generation algorithms.
//!
//! It initializes concrete generator types (Perlin, Cellular Automata) and stores them
//! as trait objects, enabling the Conductor to select and execute any generator by ID.

use tracing::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;

use ssxl_math::Vec2i;
use ssxl_shared::chunk_data::ChunkData;

use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::CellularAutomataGenerator;

use crate::ca::rule_set::{
    RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD,
};

/// Type alias for a thread-safe, dynamically dispatched Generator trait object.
/// This allows the HashMap to store different generator types uniformly.
pub type DynGenerator = Box<dyn Generator + Send + Sync>;

// --- 1. Manager Structure ---

/// Manages and provides access to all initialized generation algorithms.
pub struct GeneratorManager {
    /// The core registry: Maps generator ID strings to thread-safe generator instances.
    generators: HashMap<String, Arc<DynGenerator>>,
    /// The ID of the default Perlin noise generator, used as a fallback.
    default_perlin_id: String,
}

impl GeneratorManager {
    /// Initializes all generator algorithms and registers them in the HashMap.
    ///
    /// This is the "Dimension Registry" initialization, setting up all available
    /// generation **experiments**.
    pub fn new() -> Result<Self, String> {
        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();
        
        // 1. Register Perlin Generator (The base noise layer)
        let perlin: DynGenerator = Box::new(PerlinGenerator::new(64.0));
        let default_perlin_id = perlin.id().to_string();
        // Wrap in Arc for thread-safe sharing with worker tasks.
        generators.insert(default_perlin_id.clone(), Arc::new(perlin));
        
        // 2. Register Cellular Automata Generators (The structured content layers)
        let ca_generators = [RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD];
        for rule in ca_generators.iter().cloned() {
            let gen: DynGenerator = Box::new(CellularAutomataGenerator::new(rule));
            generators.insert(gen.id().to_string(), Arc::new(gen));
        }

        info!("Registered {} generator algorithms.", generators.len());

        Ok(GeneratorManager { generators, default_perlin_id })
    }

    /// Returns a reference to the internal generator map.
    /// Used by the Conductor for immediate, local lookups.
    pub fn get_map_ref(&self) -> &HashMap<String, Arc<DynGenerator>> {
        &self.generators
    }

    /// Returns a clone of the internal generator map.
    /// Used when spawning new background tasks that need to own a map copy (e.g., the Request Loop).
    pub fn get_map_clone(&self) -> HashMap<String, Arc<DynGenerator>> {
        self.generators.clone()
    }
    
    /// Executes the generation process for a single chunk using the specified active generator ID.
    ///
    /// Used primarily by the Conductor for synchronous, single-chunk requests.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i, active_id: &str) -> ChunkData {
        let generator_arc = self.generators
            .get(active_id)
            // Panic is appropriate here as the generator ID should have been validated previously.
            .unwrap_or_else(|| panic!("Cannot find active generator with ID: {}", active_id));

        // Dereference the Arc<DynGenerator> and call the trait method.
        generator_arc.generate_chunk(chunk_coords)
    }

    /// Determines the initial active generator ID, prioritizing the config ID and falling back to default Perlin.
    pub fn get_initial_id(&self, config_id: &str) -> String {
        if self.generators.contains_key(config_id) {
            config_id.to_string()
        } else {
            warn!("Config default generator ID '{}' not found. Falling back to Perlin: {}", config_id, self.default_perlin_id);
            self.default_perlin_id.clone()
        }
    }
}