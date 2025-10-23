use std::str::FromStr;
use std::fmt;
use serde::{Serialize, Deserialize};
use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_pipeline_data_tile::{TileInfo, tile_flags};
use crate::zv9_aetherion_generator_noise::NoiseType;
use crate::zv9_aetherion_generator_noise_config::NoiseConfig;

//
// ─── External Noise Type ───────────────────────────────────────────────────────
//

/// 🧠 ExternalNoiseType — editor-safe wrapper for exposing noise types to external engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalNoiseType {
    Basic,
    Cellular,
    CellularAutomata,
    Perlin,
    Simplex,
}

impl ExternalNoiseType {
    /// Converts to internal NoiseType used by the generator.
    pub fn to_internal(self) -> NoiseType {
        match self {
            Self::Basic => NoiseType::Basic,
            Self::Cellular => NoiseType::Cellular,
            Self::CellularAutomata => NoiseType::CellularAutomata,
            Self::Perlin => NoiseType::Perlin,
            Self::Simplex => NoiseType::Simplex,
        }
    }
}

impl fmt::Display for ExternalNoiseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Basic => "basic",
            Self::Cellular => "cellular",
            Self::CellularAutomata => "automata",
            Self::Perlin => "perlin",
            Self::Simplex => "simplex",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for ExternalNoiseType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Self::Basic),
            "cellular" => Ok(Self::Cellular),
            "automata" => Ok(Self::CellularAutomata),
            "perlin" => Ok(Self::Perlin),
            "simplex" => Ok(Self::Simplex),
            _ => Err(()),
        }
    }
}

impl From<ExternalNoiseType> for NoiseType {
    fn from(value: ExternalNoiseType) -> Self {
        value.to_internal()
    }
}

//
// ─── Map Build Options ─────────────────────────────────────────────────────────
//

/// 🗺️ MapBuildOptions — configuration for procedural map generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapBuildOptions {
    pub width: i32,
    pub height: i32,
    pub seed: u64,
    pub mode: ExternalNoiseType,
    pub animate: bool,
    pub fill_ratio: f32,
    pub steps: usize,
    pub birth_limit: u8,
    pub survival_limit: u8,
    pub black: SerializableVector2i,
    pub blue: SerializableVector2i,
    pub delivery_interval_ms: Option<u32>,
}

impl MapBuildOptions {
    /// Creates a default configuration with clamped dimensions.
    pub fn default(width: i32, height: i32, seed: u64) -> Self {
        Self {
            width: width.clamp(1, 4096),
            height: height.clamp(1, 4096),
            seed,
            mode: ExternalNoiseType::CellularAutomata,
            animate: false,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
            black: SerializableVector2i { x: 0, y: 0 },
            blue: SerializableVector2i { x: 1, y: 1 },
            delivery_interval_ms: Some(2),
        }
    }

    /// Converts to internal NoiseConfig used by the generator.
    pub fn to_noise_config(&self) -> NoiseConfig {
        NoiseConfig {
            width: self.width.max(1) as usize,
            height: self.height.max(1) as usize,
            seed: self.seed,
            fill_ratio: self.fill_ratio.clamp(0.0, 1.0) as f64,
            steps: self.steps,
            birth_limit: self.birth_limit,
            survival_limit: self.survival_limit,
        }
    }

    /// Returns the internal noise type.
    pub fn noise_type(&self) -> NoiseType {
        self.mode.to_internal()
    }

    /// Returns total tile count.
    pub fn total_tiles(&self) -> usize {
        (self.width * self.height).max(1) as usize
    }

    /// Returns true if animation is enabled.
    pub fn is_animated(&self) -> bool {
        self.animate
    }

    /// Returns a human-readable summary.
    pub fn describe(&self) -> String {
        format!(
            "MapBuildOptions: {}x{}, mode={}, seed={}, animated={}, fill={}, steps={}, birth={}, survival={}",
            self.width,
            self.height,
            self.mode,
            self.seed,
            self.animate,
            self.fill_ratio,
            self.steps,
            self.birth_limit,
            self.survival_limit
        )
    }
}

//
// ─── Procedural Tile Generator ─────────────────────────────────────────────────
//

/// Generates a tile at the given position using a hash-based pattern.
pub fn tile_at(x: u64, y: u64, seed: u64) -> TileInfo {
    let hash = x.wrapping_mul(31)
        .wrapping_add(y.wrapping_mul(17))
        .wrapping_add(seed);

    TileInfo {
        rotation: (hash % 4) as u8,
        variant_id: Some((hash % 7) as i32),
        flags: tile_flags::VISIBLE | tile_flags::COLLIDABLE,
        ..Default::default()
    }
}

/// Generates a virtual field of tiles for benchmarking or preview.
pub fn generate_virtual_field(
    width: u64,
    height: u64,
    seed: u64,
) -> impl Iterator<Item = (u64, u64, TileInfo)> {
    (0..height).flat_map(move |y| {
        (0..width).map(move |x| (x, y, tile_at(x, y, seed)))
    })
}
