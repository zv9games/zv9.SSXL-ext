//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_data_build_options

use godot::prelude::*;
use godot::builtin::{GString, Vector2i};
use godot_macros::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(init)]
pub struct BuildOptions {
    #[base]
    base: Base<Node>, // Or Control, depending on context

    // ── Map Generation ──
    #[export]
    pub width: i32,
    #[export]
    pub height: i32,
    #[export]
    pub seed: i64,
    #[export]
    pub mode: GString,
    #[export]
    pub animate: bool,
    #[export]
    pub black: Vector2i,
    #[export]
    pub blue: Vector2i,

    // ── Noise Configuration ──
    #[export]
    pub fill_ratio: f64,
    #[export]
    pub steps: i32,
    #[export]
    pub birth_limit: i32,
    #[export]
    pub survival_limit: i32,

    // ── Pipeline Execution ──
    #[export]
    pub chunk_size: i32,
    #[export]
    pub delivery_interval_ms: i32,
    #[export]
    pub enable_logging: bool,
}

#[godot_api]
impl BuildOptions {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            width: 64,
            height: 64,
            seed: 42,
            mode: "automata".into(),
            animate: false,
            black: Vector2i::new(0, 0),
            blue: Vector2i::new(1, 1),
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
            chunk_size: 1000,
            delivery_interval_ms: 2,
            enable_logging: true,
        }
    }

    /// Converts to internal noise configuration.
    pub fn to_noise_config(&self) -> NoiseConfig {
        NoiseConfig {
            width: self.width as usize,
            height: self.height as usize,
            seed: self.seed as u64,
            fill_ratio: self.fill_ratio.clamp(0.0, 1.0),
            steps: self.steps.max(1),
            birth_limit: self.birth_limit.max(0),
            survival_limit: self.survival_limit.max(0),
        }
    }

    /// Converts mode string to internal enum.
    pub fn noise_type(&self) -> NoiseType {
        GodotNoiseType::from_str(&self.mode.to_string()).to_internal()
    }

    /// Returns tile atlas coordinates.
    pub fn tile_coords(&self) -> (Vector2i, Vector2i) {
        (self.black, self.blue)
    }

    /// Returns a preset configuration.
    pub fn preset(name: &str) -> Self {
        match name {
            "cave" => Self {
                fill_ratio: 0.45,
                steps: 5,
                birth_limit: 4,
                survival_limit: 3,
                ..Self::init(Base::default())
            },
            "island" => Self {
                fill_ratio: 0.35,
                steps: 3,
                birth_limit: 5,
                survival_limit: 2,
                ..Self::init(Base::default())
            },
            _ => Self::init(Base::default()),
        }
    }

    /// Returns a debug-friendly summary.
    pub fn describe(&self) -> String {
        format!(
            "BuildOptions {{ {}x{}, seed: {}, mode: {}, fill: {:.2}, steps: {}, birth: {}, survival: {}, chunk: {}, pacing: {}ms, animate: {}, logging: {} }}",
            self.width, self.height, self.seed, self.mode,
            self.fill_ratio, self.steps, self.birth_limit, self.survival_limit,
            self.chunk_size, self.delivery_interval_ms, self.animate, self.enable_logging
        )
    }

    /// Applies a preset from GDScript.
    #[func]
    fn apply_preset(&mut self, name: GString) {
        let preset = BuildOptions::preset(&name.to_string());
        *self = preset;
    }
}
// the end