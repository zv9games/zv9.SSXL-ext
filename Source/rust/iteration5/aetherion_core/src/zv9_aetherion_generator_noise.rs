use rand::{Rng, SeedableRng};
use rand::prelude::SliceRandom;
use std::str::FromStr;
use log::warn;  // Native warn! macro for consistent logging.

#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;

#[cfg(feature = "advanced_noise")]
use fastnoise2::{FastNoise2, NoiseType as FnNoiseType};

/// 🔊 Basic: Inline trig—ns-nimble.
#[inline(always)]
pub fn basic_noise(x: f32, y: f32) -> f32 {
    (x.sin() + y.cos()) * 0.5
}

/// 🎛 NoiseType: Unified, feat-aware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseType {
    Basic,
    Perlin,
    Simplex,
    Cellular,
    CellularAutomata,
}

impl NoiseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Perlin => "perlin",
            Self::Simplex => "simplex",
            Self::Cellular => "cellular",
            Self::CellularAutomata => "automata",
        }
    }

    pub fn is_available(&self) -> bool {
        match self {
            Self::Basic | Self::CellularAutomata => true,
            #[cfg(feature = "advanced_noise")]
            Self::Perlin | Self::Simplex | Self::Cellular => true,
            #[cfg(not(feature = "advanced_noise"))]
            _ => false,
        }
    }

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        Self::from_str(s)
    }
}

pub type GodotNoiseType = NoiseType;

/// Local error type for parsing NoiseType—self-contained and precise.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown noise type: {0}")]
    UnknownType(String),
}

impl FromStr for NoiseType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Self::Basic),
            "perlin" => Ok(Self::Perlin),
            "simplex" => Ok(Self::Simplex),
            "cellular" => Ok(Self::Cellular),
            "automata" | "cellularautomata" => Ok(Self::CellularAutomata),
            _ => Err(ParseError::UnknownType(s.to_string())),
        }
    }
}

/// 🧪 Dispatcher: Feat-forked for velocity.
pub fn generate_noise(x: f32, y: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Basic => basic_noise(x, y),
        #[cfg(feature = "advanced_noise")]
        NoiseType::Perlin => {
            static PERLIN: std::sync::OnceLock<FastNoise2> = std::sync::OnceLock::new();
            let noise = PERLIN.get_or_init(|| {
                let mut fn2 = FastNoise2::new();
                fn2.set_noise_type(FnNoiseType::Perlin);
                fn2.set_seed(42);
                fn2
            });
            noise.get_noise_2d(x as f64, y as f64) as f32
        }
        #[cfg(feature = "advanced_noise")]
        NoiseType::Simplex => {
            static SIMPLEX: std::sync::OnceLock<FastNoise2> = std::sync::OnceLock::new();
            let noise = SIMPLEX.get_or_init(|| {
                let mut fn2 = FastNoise2::new();
                fn2.set_noise_type(FnNoiseType::Simplex);
                fn2.set_seed(42);
                fn2
            });
            noise.get_noise_2d(x as f64, y as f64) as f32
        }
        #[cfg(feature = "advanced_noise")]
        NoiseType::Cellular => {
            static CELL: std::sync::OnceLock<FastNoise2> = std::sync::OnceLock::new();
            let noise = CELL.get_or_init(|| {
                let mut fn2 = FastNoise2::new();
                fn2.set_noise_type(FnNoiseType::Cellular);
                fn2.set_seed(42);
                fn2
            });
            noise.get_noise_2d(x as f64, y as f64) as f32
        }
        #[cfg(not(feature = "advanced_noise"))]
        NoiseType::Perlin | NoiseType::Simplex | NoiseType::Cellular => {
            warn!("noise: {} not implemented—fallback to basic", noise_type.as_str());  // Native warn! macro with target and interpolation.
            basic_noise(x, y)
        }
        NoiseType::CellularAutomata => 0.0,
    }
}

/// 🧱 Grid: Flat u8, batch RNG—cache caress.
pub type Flat = Vec<Vec<u8>>;

pub fn generate_grid_noise(
    width: usize,
    height: usize,
    noise_type: NoiseType,
    seed: u64,
) -> Flat {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut grid = vec![vec![0u8; width]; height];

    for y in 0..height {
        for x in 0..width {
            let xf = x as f32 / width as f32;
            let yf = y as f32 / height as f32;
            let noise_val = generate_noise(xf * 10.0, yf * 10.0, noise_type);
            let threshold = 0.5;
            grid[y][x] = if noise_val > threshold { 1 } else { 0 };
        }
    }

    grid
}