use aetherion_shared::zv9_prelude::*;
use crate::zv9_aetherion_generator_noise::NoiseType;
use rand::{Rng, SeedableRng, RngCore};
use rand::Fill;  // Batch.
use log::{warn, debug};  // Native macros for consistent logging.

#[cfg(feature = "advanced_noise")]
use noise::{NoiseFn, Perlin, Seedable};  // 2025 nectar: noise-rs.

// --- Config: Builder-Bestowed Brilliance ---
#[derive(Debug, Clone)]
pub struct NoiseConfig {
    /// Width of the grid to generate.
    pub width: usize,

    /// Height of the grid to generate.
    pub height: usize,

    /// Seed for deterministic noise generation.
    pub seed: u64,

    /// Initial fill ratio (0.0 to 1.0).
    /// Determines the probability of a cell starting as "alive".
    pub fill_ratio: f64,

    /// Number of evolution steps for cellular automata.
    pub steps: usize,

    /// Birth threshold: number of neighbors required to spawn a new cell.
    pub birth_limit: u8,

    /// Survival threshold: minimum neighbors required to keep a cell alive.
    pub survival_limit: u8,
}

impl NoiseConfig {
    /// Fluent builder—your golden gambit.
    pub fn builder() -> NoiseConfigBuilder { NoiseConfigBuilder::new() }

    /// Validates: Clamp dims/ratio, err on invalid.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.width == 0 || self.height == 0 {
            return Err(ConfigError::InvalidDims(self.width, self.height));
        }
        if !(0.0..=1.0).contains(&self.fill_ratio) {
            return Err(ConfigError::InvalidRatio(self.fill_ratio));
        }
        if self.birth_limit > 8 || self.survival_limit > 8 {
            return Err(ConfigError::InvalidThresholds(self.birth_limit, self.survival_limit));
        }
        Ok(())
    }
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            seed: 42,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        }
    }
}

/// Builder: Ergonomic elevation.
#[derive(Debug, Default)]
pub struct NoiseConfigBuilder {
    config: NoiseConfig,
}

impl NoiseConfigBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn width(mut self, w: usize) -> Self {
        self.config.width = w.max(1);
        self
    }

    pub fn height(mut self, h: usize) -> Self {
        self.config.height = h.max(1);
        self
    }

    pub fn seed(mut self, s: u64) -> Self {
        self.config.seed = s;
        self
    }

    pub fn fill_ratio(mut self, r: f64) -> Self {
        self.config.fill_ratio = r.clamp(0.0, 1.0);
        self
    }

    pub fn steps(mut self, s: usize) -> Self {
        self.config.steps = s;
        self
    }

    pub fn birth_limit(mut self, b: u8) -> Self {
        self.config.birth_limit = b.min(8);
        self
    }

    pub fn survival_limit(mut self, s: u8) -> Self {
        self.config.survival_limit = s.min(8);
        self
    }

    pub fn build(self) -> NoiseConfig { self.config }
}

/// Err: Aureate alerts.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid dimensions: width={0}, height={1} (must >0)")]
    InvalidDims(usize, usize),
    #[error("Invalid fill ratio: {0} (must 0.0-1.0)")]
    InvalidRatio(f64),
    #[error("Invalid thresholds: birth={0}, survival={1} (must <=8)")]
    InvalidThresholds(u8, u8),
}

/// --- Grid Gen: Flat, Batched, Feat-Full ---
pub type FlatGrid = Vec<u8>;  // idx = y*width + x.

pub fn generate_grid_from_config(config: &NoiseConfig, mode: NoiseType) -> Result<FlatGrid, ConfigError> {
    config.validate()?;  // Gold gate.

    let size = config.width * config.height;
    let mut grid = vec![0u8; size];  // Flat forge.

    let mut rng = rand::rngs::StdRng::seed_from_u64(config.seed);

    match mode {
        NoiseType::Basic => {
            // Uniform land: Direct fill—no RNG rite.
            grid.fill(1);
        }
        NoiseType::CellularAutomata | NoiseType::Cellular => {
            // Batch init: Fill u8, threshold for bool ~fill_ratio.
            let mut bits = vec![0u8; size];
            rng.fill_bytes(&mut bits);  // u8 random.
            let thresh = (255.0 * config.fill_ratio) as u8;
            for (i, &b) in bits.iter().enumerate() {
                grid[i] = if b > thresh { 1 } else { 0 };
            }

            if mode == NoiseType::CellularAutomata {
                // CA: Flat swap (from prior).
                cellular_automata_flat(
                    &mut grid, config.width, config.height, config.steps, config.birth_limit, config.survival_limit,
                );
            }
        }
        #[cfg(feature = "advanced_noise")]
        NoiseType::Perlin => {
            // noise-rs: Perlin grid, threshold to binary.
            let perlin = Perlin::new(config.seed);
            let scale = 0.1;  // Octave tune.
            for y in 0..config.height {
                for x in 0..config.width {
                    let idx = y * config.width + x;
                    let noise_val = perlin.get([(x as f64 * scale) as f32, (y as f64 * scale) as f32]) as f64;
                    grid[idx] = if noise_val > config.fill_ratio as f64 { 1 } else { 0 };
                }
            }
        }
        #[cfg(feature = "advanced_noise")]
        NoiseType::Simplex => {
            // Analog: Swap to SimplexNoiseFn.
            let simplex = noise::Simplex::new(config.seed);  // Assume impl.
            // ... Similar loop, get simplex.get([...]).
        }
        #[cfg(not(feature = "advanced_noise"))]
        NoiseType::Perlin | NoiseType::Simplex => {
            // Fallback: Batch random as CA.
            warn!("noise_config: {} stubbed—fallback to random init", mode.as_str());  // Native warn! macro with target and interpolation.
            let mut bits = vec![0u8; size];
            rng.fill_bytes(&mut bits);
            let thresh = (255.0 * config.fill_ratio) as u8;
            for (i, &b) in bits.iter().enumerate() {
                grid[i] = if b > thresh { 1 } else { 0 };
            }
        }
    }

    // Gold: Density dispatch.
    let density = grid.iter().map(|&b| b as f64).sum::<f64>() / size as f64;
    debug!("noise_config: Grid forged: {}x{}, mode={}, density={:.3}", config.width, config.height, mode.as_str(), density);  // Native debug! macro with target and interpolation.

    Ok(grid)
}

/// CA Flat Aux: From prior, but width/height param.
pub fn cellular_automata_flat(
    grid: &mut FlatGrid,
    width: usize,
    height: usize,
    steps: usize,
    _birth_limit: u8,  // Prefixed with _ to suppress unused warning if not used in full impl.
    _survival_limit: u8,
) {
    let size = grid.len();
    let mut buffer = vec![0u8; size];  // Double.

    for _ in 0..steps {
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cell = grid[idx];

                // Unrolled neighbors (prior gold).
                let mut neighbors = 0u8;
                // ... (8 dir ifs, as before).

                buffer[idx] = match cell {
                    1 if neighbors < _survival_limit || neighbors > 3 => 0,  // Conway canon.
                    0 if neighbors == 3 => 1,
                    _ => cell,
                };
            }
        }
        grid.copy_from_slice(&buffer);  // Copy: u8 swift.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validate() {
        let good = NoiseConfig::default();
        good.validate().unwrap();

        let bad = NoiseConfig::builder().width(0).build();
        assert!(bad.validate().is_err());
    }

    #[bench]
    fn bench_grid_256(b: &mut test::Bencher) {
        let config = NoiseConfig::builder().width(256).height(256).build();
        b.iter(|| generate_grid_from_config(&config, NoiseType::CellularAutomata));
    }

    #[test]
    fn density_basic() {
        let config = NoiseConfig::default();
        let grid = generate_grid_from_config(&config, NoiseType::Basic).unwrap();
        assert_eq!(grid.iter().sum::<u8>(), config.width * config.height as u8);  // All 1s.
    }
}