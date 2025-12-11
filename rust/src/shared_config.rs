// rust/SSXL-ext/src/shared_config.rs

use serde::{Deserialize, Serialize};

/// Combines all settings relevant for the procedural generation workers.
/// This struct is passed via the GenerationTask to the sync_pool.
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

/// Configuration for the Fractal Brownian Motion (FBM) noise function.
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

/// Configuration for the Cellular Automata simulation rules.
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

// rust/SSXL-ext/src/shared_config.rs

/// Configuration for all thread pools and concurrency limits.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ThreadingConfig {
    // Number of dedicated workers for one-time generation (sync_pool.rs)
    pub generation_worker_count: u32,
    // Number of dedicated workers for continuous animation/simulation (animate_worker.rs)
    pub animation_worker_count: u32,
    // Max number of tasks/chunks allowed in the main generation queue
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

// rust/SSXL-ext/src/shared_config.rs

/// Configuration defining the physical layout of the world chunks.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MapSettingsConfig {
    // The size (edge length) of a single chunk (e.g., 32 -> 32x32 tiles)
    pub chunk_size: u32,
    // The initial number of chunks to generate from the center (0,0) outward (e.g., 8 -> 17x17 grid)
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

// rust/SSXL-ext/src/shared_config.rs

/// Configuration for the continuous simulation and animation workers.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AnimationConfig {
    // The desired speed of the simulation loop, decoupled from Godot's FPS.
    pub simulation_fps: u32,
    pub fluid_damping_factor: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            simulation_fps: 30, // 30Hz simulation loop
            fluid_damping_factor: 0.95,
        }
    }
}