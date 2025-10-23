// ─── Prelude & Delivery ────────────────────────────────────────────────────────
//use aetherion_shared::zv9_prelude::*;
pub mod zv9_aetherion_pipeline_builder_dummy_delivery;
pub use zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;
pub use aetherion_shared::zv9_prelude as zv9_prelude;

// ─── Codegen ───────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_codegen_config;
pub mod zv9_aetherion_codegen_dsl;
pub mod zv9_aetherion_codegen_emitter;
pub mod zv9_aetherion_codegen_parser;

pub mod codegen {
    pub use crate::zv9_aetherion_codegen_config::*;
    pub use crate::zv9_aetherion_codegen_dsl::*;
    pub use crate::zv9_aetherion_codegen_emitter::*;
    pub use crate::zv9_aetherion_codegen_parser::*;
}

// ─── Core Engine ───────────────────────────────────────────────────────────────
pub mod zv9_aetherion_core_conductor;
pub mod zv9_aetherion_core_dimension;
pub mod zv9_aetherion_core_lifecycle;
pub mod zv9_aetherion_core_runtime;

pub mod core {
    pub use crate::zv9_aetherion_core_conductor::*;
    pub use crate::zv9_aetherion_core_dimension::*;
    pub use crate::zv9_aetherion_core_lifecycle::*;
    pub use crate::zv9_aetherion_core_runtime::*;
}

// ─── Generator ─────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_generator_noise;
pub mod zv9_aetherion_generator_noise_config;
pub mod zv9_aetherion_generator_patterns;
pub mod zv9_aetherion_generator_pattern_type;

pub mod generator {
    pub use crate::zv9_aetherion_generator_noise::*;
    pub use crate::zv9_aetherion_generator_noise_config::*;
    pub use crate::zv9_aetherion_generator_patterns::*;
    pub use crate::zv9_aetherion_generator_pattern_type::*;
}

// ─── Interaction ──────────────────────────────────────────────────────────────
pub mod zv9_aetherion_interaction_modifiers;
pub mod zv9_aetherion_interaction_tools;

pub mod interaction {
    pub use crate::zv9_aetherion_interaction_modifiers::*;
    pub use crate::zv9_aetherion_interaction_tools::*;
}

// ─── Pipeline ──────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_pipeline_builder_bitmask;
pub mod zv9_aetherion_pipeline_builder_builder;
pub mod zv9_aetherion_pipeline_builder_streamer;
pub mod zv9_aetherion_pipeline_builder_threaded;


pub mod pipeline {
    pub mod builder {
        pub use crate::zv9_aetherion_pipeline_builder_bitmask::*;
        pub use crate::zv9_aetherion_pipeline_builder_builder::*;
        pub use crate::zv9_aetherion_pipeline_builder_streamer::*;
        pub use crate::zv9_aetherion_pipeline_builder_streamer::{SyncBridge, ChunkDelivery};
        pub use crate::zv9_aetherion_pipeline_builder_threaded::*;
    }

    pub mod data {

        pub use aetherion_shared::zv9_shared_pipeline_data_chunk::*;
        pub use aetherion_shared::zv9_shared_pipeline_data_data::*;
        //pub use aetherion_shared::zv9_shared_pipeline_data_grid::*;
        //pub use aetherion_shared::zv9_shared_pipeline_data_tile::*;
    }
}


// ─── Structure ─────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_structure_generation;
pub mod zv9_aetherion_structure_placement;

pub mod structure {
    pub use crate::zv9_aetherion_structure_generation::*;
    pub use crate::zv9_aetherion_structure_placement::*;
}

// ─── Shared ────────────────────────────────────────────────────────────────────


// ─── Trailkeeper ───────────────────────────────────────────────────────────────
// trailkeeper is now in it's own crate. 

// ─── General Utilities ─────────────────────────────────────────────────────────
pub mod zv9_util_config;
pub mod zv9_util_direction;
pub mod zv9_util_position;
pub mod zv9_util_profiling;
pub mod zv9_util_random;
pub mod zv9_util_time;
pub mod zv9_util_timer;
pub mod zv9_util_velocity;

pub mod util {
    pub use crate::zv9_util_config::*;
    pub use crate::zv9_util_direction::*;
    pub use crate::zv9_util_position::*;
    pub use crate::zv9_util_profiling::*;
    pub use crate::zv9_util_random::*;
    pub use crate::zv9_util_time::*;
    pub use crate::zv9_util_timer::*;
    pub use crate::zv9_util_velocity::*;
}
