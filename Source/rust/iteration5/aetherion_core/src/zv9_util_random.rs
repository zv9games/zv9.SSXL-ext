#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use rand::{Rng, SeedableRng};
use rand::distributions::uniform::SampleUniform;
use log::info; // Explicit import for logging

#[cfg(feature = "fast_rng")]
use rand_pcg::Pcg32;

#[cfg(not(feature = "fast_rng"))]
use rand::rngs::StdRng;

/// Alias for whichever RNG backend is active
#[cfg(feature = "fast_rng")]
type EngineRng = Pcg32;

#[cfg(not(feature = "fast_rng"))]
type EngineRng = StdRng;

/// 🎲 Seeded random number generator wrapper for reproducible procedural logic.
pub struct Randomizer {
    rng: EngineRng,
}

impl Randomizer {
    /// Creates a new `Randomizer` from a given seed.
    pub fn new(seed: u64) -> Self {
        #[cfg(feature = "fast_rng")]
        info!("🎲 RNG backend active: PCG32 (fast)");

        #[cfg(not(feature = "fast_rng"))]
        info!("🎲 RNG backend active: StdRng (fallback)");

        Self {
            rng: EngineRng::seed_from_u64(seed),
        }
    }

    pub fn gen_range<T>(&mut self, min: T, max: T) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.gen_range(min..max)
    }

    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }

    pub fn gen_unit_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    pub fn gen_unit_f64(&mut self) -> f64 {
        self.rng.gen()
    }
}

/// 🔁 Returns either -1 or 1 randomly.
pub fn random_sign(rng: &mut impl Rng) -> i32 {
    if rng.gen_bool(0.5) { 1 } else { -1 }
}

/// 🧭 Returns a random 2D direction vector (±1, ±1).
pub fn random_direction_2d(rng: &mut impl Rng) -> (i32, i32) {
    (random_sign(rng), random_sign(rng))
}