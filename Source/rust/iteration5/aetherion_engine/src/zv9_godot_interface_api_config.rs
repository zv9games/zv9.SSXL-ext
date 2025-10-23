use godot::prelude::*;
use rand::Rng;

use aetherion_shared::zv9_shared_config::EngineConfig;
use aetherion_shared::zv9_shared_logging::log_info;
use crate::zv9_aetherion_engine_config_godot::EngineConfigGodot;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionConfig {
    #[export]
    pub tile_size: i32,
    #[export]
    pub chunk_width: i32,
    #[export]
    pub chunk_height: i32,
    #[export]
    pub seed: i64,
    #[export]
    pub enable_voxel_mode: bool,
}

#[godot_api]
impl AetherionConfig {
    #[func]
    fn _ready(&self) {
        godot_print!("⚙️ AetherionConfig loaded.");
        log_info("AetherionConfig", "Configuration node initialized");
    }

    #[func]
    fn get_chunk_area(&self) -> i32 {
        let area = self.chunk_width * self.chunk_height;
        godot_print!(
            "📐 Chunk area: {} tiles ({}×{})",
            area,
            self.chunk_width,
            self.chunk_height
        );
        area
    }

    #[func]
    fn regenerate_seed(&mut self) {
        let mut rng = rand::thread_rng();
        self.seed = rng.gen_range(0..=i64::MAX);
        godot_print!("🌱 Seed regenerated → {}", self.seed);
        log_info("AetherionConfig", &format!("Seed regenerated: {}", self.seed));
    }

    #[func]
    fn to_engine_config(&self) -> Gd<EngineConfigGodot> {
        Gd::from_init_fn(|_base| EngineConfigGodot {
            tile_size: self.tile_size,
            chunk_width: self.chunk_width,
            chunk_height: self.chunk_height,
            seed: self.seed,
            enable_voxel_mode: self.enable_voxel_mode,
        })
    }

    #[func]
    fn apply_engine_config(&mut self, config: Gd<EngineConfigGodot>) {
        let config = config.bind();
        self.tile_size = config.tile_size;
        self.chunk_width = config.chunk_width;
        self.chunk_height = config.chunk_height;
        self.seed = config.seed;
        self.enable_voxel_mode = config.enable_voxel_mode;
    }
}
