use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct GenerationConfig {
    pub perlin: PerlinNoiseConfig,
    pub ca: CellularAutomataConfig,
    pub world_seed: u64,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            perlin: PerlinNoiseConfig::default(),
            ca: CellularAutomataConfig::default(),
            world_seed: 5011993,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PerlinNoiseConfig {
    pub scale: f64,
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
    pub threshold: f64,
}

impl Default for PerlinNoiseConfig {
    fn default() -> Self {
        Self {
            scale: 250.0,
            octaves: 3,
            persistence: 0.5,
            lacunarity: 2.0,
            threshold: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct CellularAutomataConfig {
    pub death_limit: u8,
    pub birth_limit: u8,
    pub steps: u8,
}

impl Default for CellularAutomataConfig {
    fn default() -> Self {
        Self {
            death_limit: 4,
            birth_limit: 5,
            steps: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ThreadingConfig {
    pub generation_worker_count: u32,
    pub animation_worker_count: u32,
    pub task_channel_capacity: usize,
}

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self {
            generation_worker_count: 4,
            animation_worker_count: 2,
            task_channel_capacity: 4096,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MapSettingsConfig {
    pub chunk_size: u32,
    pub map_extent_chunks: i32,
    pub tile_scale_factor: f32,
}

impl Default for MapSettingsConfig {
    fn default() -> Self {
        Self {
            chunk_size: 32,
            map_extent_chunks: 8,
            tile_scale_factor: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AnimationConfig {
    pub worker_count: usize,
    pub simulation_fps: u32,
    pub fluid_damping_factor: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            worker_count: 2,
            simulation_fps: 30,
            fluid_damping_factor: 0.95,
        }
    }
}
