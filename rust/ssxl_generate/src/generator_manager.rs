//! Manages the registration, initialization, and selection of procedural
//! generation algorithms (Generators).

use tracing::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;

use ssxl_math::Vec2i;
use ssxl_shared::chunk_data::ChunkData;

// Generator trait and implementations
use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::CellularAutomataGenerator;

// CORRECTED: Import rule constants directly from their source module (ca::rule_set).
use crate::ca::rule_set::{
    RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD,
};

// Define a type alias for thread-safe dynamic generators
pub type DynGenerator = Box<dyn Generator + Send + Sync>;

// -----------------------------------------------------------------------------
// GENERATOR MANAGER
// -----------------------------------------------------------------------------

/// Responsible for initializing and providing access to all registered generator
/// algorithms.
pub struct GeneratorManager {
    generators: HashMap<String, Arc<DynGenerator>>,
    default_perlin_id: String,
}

impl GeneratorManager {
    /// Initializes all generator algorithms and registers them by their unique ID.
    pub fn new() -> Result<Self, String> {
        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();
        
        // Instantiate and register Perlin
        let perlin: DynGenerator = Box::new(PerlinGenerator::new(64.0));
        let default_perlin_id = perlin.id().to_string();
        generators.insert(default_perlin_id.clone(), Arc::new(perlin));
        
        // Instantiate and register Cellular Automata variants
        // The constants are now correctly in scope.
        let ca_generators = [RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD];
        for rule in ca_generators.iter().cloned() {
            let gen: DynGenerator = Box::new(CellularAutomataGenerator::new(rule));
            generators.insert(gen.id().to_string(), Arc::new(gen));
        }

        info!("Registered {} generator algorithms.", generators.len());

        Ok(GeneratorManager { generators, default_perlin_id })
    }

    /// Provides a reference to the internal map (for ID validation).
    pub fn get_map_ref(&self) -> &HashMap<String, Arc<DynGenerator>> {
        &self.generators
    }

    /// Provides a cloned map for passing to asynchronous worker loops.
    pub fn get_map_clone(&self) -> HashMap<String, Arc<DynGenerator>> {
        self.generators.clone()
    }
    
    /// Finds and runs a generator synchronously for a single chunk.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i, active_id: &str) -> ChunkData {
        let generator_arc = self.generators
            .get(active_id)
            .unwrap_or_else(|| panic!("Cannot find active generator with ID: {}", active_id));

        generator_arc.generate_chunk(chunk_coords)
    }

    /// Determines the initial generator ID based on configuration, falling back to Perlin.
    pub fn get_initial_id(&self, config_id: &str) -> String {
        if self.generators.contains_key(config_id) {
            config_id.to_string()
        } else {
            warn!("Config default generator ID '{}' not found. Falling back to Perlin: {}", config_id, self.default_perlin_id);
            self.default_perlin_id.clone()
        }
    }
}