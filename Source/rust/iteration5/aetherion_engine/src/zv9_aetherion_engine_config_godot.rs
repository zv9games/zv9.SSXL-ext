use godot::prelude::*;
use aetherion_shared::zv9_shared_config::EngineConfig;

/// 🌉 Godot-facing wrapper for EngineConfig.
/// Used to expose engine configuration to GDScript and Godot runtime.
#[derive(GodotClass)]
#[class(init, base = Object)]
pub struct EngineConfigGodot {
    #[var] pub tile_size: i32,
    #[var] pub chunk_width: i32,
    #[var] pub chunk_height: i32,
    #[var] pub seed: i64,
    #[var] pub enable_voxel_mode: bool,
}

#[godot_api]
impl EngineConfigGodot {
    /// Default constructor required by Godot.
    fn init(_base: Base<Object>) -> Self {
        Self {
            tile_size: 32,
            chunk_width: 8,
            chunk_height: 8,
            seed: 0,
            enable_voxel_mode: false,
        }
    }

    /// Converts this Godot-facing config into a pure Rust EngineConfig.
    pub fn to_engine_config(&self) -> EngineConfig {
        EngineConfig {
            tile_size: self.tile_size,
            chunk_width: self.chunk_width,
            chunk_height: self.chunk_height,
            seed: self.seed,
            enable_voxel_mode: self.enable_voxel_mode,
        }
    }

    /// Creates a Godot-facing wrapper from a pure Rust EngineConfig.
    pub fn from_engine_config(config: &EngineConfig) -> Gd<Self> {
        Gd::from_init_fn(|_base| Self {
            tile_size: config.tile_size,
            chunk_width: config.chunk_width,
            chunk_height: config.chunk_height,
            seed: config.seed,
            enable_voxel_mode: config.enable_voxel_mode,
        })
    }
}
