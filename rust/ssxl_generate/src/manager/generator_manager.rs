// ============================================================================
// 🎼 Generator Manager (`crate::manager::generator_manager`)
// ----------------------------------------------------------------------------
// This module defines the `GeneratorManager`, the central registry and access
// point for all procedural generation algorithms in the SSXL engine. It ensures
// that multiple generator types (e.g., Perlin noise, cellular automata) can be
// initialized, stored, and retrieved in a uniform way.
//
// Purpose:
//   • Maintain a registry of all available generator implementations.
//   • Provide safe, shared access to generators across async tasks.
//   • Allow dynamic lookup of generators by ID.
//   • Execute chunk generation requests using the active generator.
//
// Key Components:
//   • DynGenerator (type alias)
//       - Represents a dynamically dispatched generator trait object.
//       - Box<dyn Generator + Send + Sync> ensures uniform storage and thread safety.
//       - Wrapped in Arc for shared ownership across tasks.
//
//   • GeneratorManager (struct)
//       - Holds:
//           • generators: HashMap mapping generator IDs → Arc<DynGenerator>.
//           • default_perlin_id: fallback ID for Perlin generator.
//       - Acts as the registry and dispatcher for all generators.
//
// Implementation Methods:
//   • new
//       - Initializes registry with:
//           • Perlin generator (base noise layer).
//           • Multiple cellular automata generators (different rule sets).
//       - Logs how many generators were registered.
//       - Returns a fully constructed `GeneratorManager`.
//
//   • get_map_ref
//       - Returns a reference to the internal generator map.
//       - Useful for quick lookups without cloning.
//
//   • get_map_clone
//       - Returns a cloned copy of the generator map.
//       - Useful for background tasks requiring ownership.
//
//   • generate_single_chunk
//       - Executes chunk generation for given coordinates using specified generator ID.
//       - Panics if ID is invalid (should be validated earlier).
//       - Returns fully generated `ChunkData`.
//
//   • get_initial_id
//       - Determines initial active generator ID based on configuration.
//       - If config ID exists → return it.
//       - If not → log warning and return default Perlin ID.
//
// Workflow:
//   1. Registry is initialized with Perlin + CA generators.
//   2. Conductor queries `GeneratorManager` for active generator ID.
//   3. Chunk generation requests are dispatched to the correct generator.
//   4. Fallback logic ensures system stability if config ID is invalid.
//
// Design Choices:
//   • Arc + Box<dyn Generator> ensures safe concurrent access and polymorphism.
//   • HashMap provides O(1) lookup for generator IDs.
//   • Logging improves visibility into registration and fallback events.
//   • Default Perlin fallback ensures system always has a valid generator.
//
// Educational Note:
//   • This module demonstrates how Rust can combine trait objects, smart pointers,
//     and collections to build a flexible plugin-like architecture.
//   • By centralizing generator management, the engine remains modular, extensible,
//     and resilient to configuration errors.
// ============================================================================


use tracing::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use ssxl_math::prelude::Vec2i;
use ssxl_shared::ChunkData;
use crate::Generator;
use crate::perlin::perlin_generator::PerlinGenerator;
use crate::ca::cellular_automata_generator::CellularAutomataGenerator;
use crate::ca::rule_set::{
    RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD,
};

pub type DynGenerator = Box<dyn Generator + Send + Sync>;

pub struct GeneratorManager {
    generators: HashMap<String, Arc<DynGenerator>>,
    default_perlin_id: String,
}

impl GeneratorManager {
    pub fn new() -> Result<Self, String> {
        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();
        
        let perlin: DynGenerator = Box::new(PerlinGenerator::new(64.0));
        let default_perlin_id = perlin.id().to_string();
        generators.insert(default_perlin_id.clone(), Arc::new(perlin));
        
        let ca_generators = [RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD];
        for rule in ca_generators.iter().cloned() {
            let gen: DynGenerator = Box::new(CellularAutomataGenerator::new(rule));
            generators.insert(gen.id().to_string(), Arc::new(gen));
        }

        info!("Registered {} generator algorithms.", generators.len());

        Ok(GeneratorManager { generators, default_perlin_id })
    }

    pub fn get_map_ref(&self) -> &HashMap<String, Arc<DynGenerator>> {
        &self.generators
    }

    pub fn get_map_clone(&self) -> HashMap<String, Arc<DynGenerator>> {
        self.generators.clone()
    }
    
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i, active_id: &str) -> ChunkData {
        let generator_arc = self.generators
            .get(active_id)
            .unwrap_or_else(|| panic!("Cannot find active generator with ID: {}", active_id));

        generator_arc.generate_chunk(chunk_coords)
    }

    pub fn get_initial_id(&self, config_id: &str) -> String {
        if self.generators.contains_key(config_id) {
            config_id.to_string()
        } else {
            warn!(
                "Config default generator ID '{}' not found. Falling back to Perlin: {}",
                config_id, self.default_perlin_id
            );
            self.default_perlin_id.clone()
        }
    }
}
