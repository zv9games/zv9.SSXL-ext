#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::str::FromStr;

use crate::generator::BlendMode;
use crate::generator::blend_noise_and_pattern;  // Fixed: Import the missing function for blending.

/// 🟫 Enum: Patterns as pantheon—param-prolific.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    Checkerboard,
    Radial { radius: f32 },
    XorFractal { modulus: i32 },
    VerticalStripes { stripe_width: i32 },
    HorizontalStripes { stripe_height: i32 },
}

impl Pattern {
    /// Builder: Fluent filigree.
    pub fn builder() -> PatternBuilder { PatternBuilder::new() }

    /// Eval: Inline invocation—velocity vow.
    #[inline(always)]
    pub fn eval(&self, x: i32, y: i32) -> bool {
        match self {
            Self::Checkerboard => (x + y) % 2 == 0,
            Self::Radial { radius } => {
                let dist_sq = x as f32 * x as f32 + y as f32 * y as f32;
                dist_sq <= *radius * *radius
            }
            Self::XorFractal { modulus } => (x ^ y) % *modulus == 0,
            Self::VerticalStripes { stripe_width } => (x / *stripe_width) % 2 == 0,
            Self::HorizontalStripes { stripe_height } => (y / *stripe_height) % 2 == 0,
        }
    }

    /// 3D: Agnostic aureate—feat-forked.
    #[cfg(feature = "3d")]
    #[inline(always)]
    pub fn eval_3d(&self, x: i32, y: i32, z: i32) -> bool {
        match self {
            Self::Checkerboard => (x + y + z) % 2 == 0,
            Self::Radial { radius } => {
                let dist_sq = x as f32 * x as f32 + y as f32 * y as f32 + z as f32 * z as f32;
                dist_sq <= *radius * *radius
            }
            Self::XorFractal { modulus } => ((x ^ y ^ z) % *modulus == 0),
            Self::VerticalStripes { stripe_width } => (x / *stripe_width) % 2 == 0,  // Z-ignore.
            Self::HorizontalStripes { stripe_height } => (y / *stripe_height) % 2 == 0,
        }
    }

    /// Density est: Grid sim stub—gold gauge.
    pub fn estimated_density(&self, width: usize, height: usize) -> f64 {
        let mut count = 0;
        for y in 0..height.min(100) {  // Sample for speed.
            for x in 0..width.min(100) {
                if self.eval(x as i32, y as i32) { count += 1; }
            }
        }
        count as f64 / ((height.min(100) * width.min(100)) as f64)
    }
}

/// Builder: Ergonomic elevation.
#[derive(Debug, Default)]
pub struct PatternBuilder;

impl PatternBuilder {
    pub fn new() -> Self { Self }

    pub fn checkerboard(self) -> Pattern { Pattern::Checkerboard }

    pub fn radial(self, radius: f32) -> Pattern { Pattern::Radial { radius: radius.max(0.0) } }

    pub fn xor_fractal(self, modulus: i32) -> Pattern { Pattern::XorFractal { modulus: modulus.max(1) } }

    pub fn vertical_stripes(self, stripe_width: i32) -> Pattern { Pattern::VerticalStripes { stripe_width: stripe_width.max(1) } }

    pub fn horizontal_stripes(self, stripe_height: i32) -> Pattern { Pattern::HorizontalStripes { stripe_height: stripe_height.max(1) } }
}

impl FromStr for Pattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('(').collect();
        match parts[0].trim() {
            "checkerboard" => Ok(Self::Checkerboard),
            "radial" => {
                let r = parts.get(1).and_then(|p| p.trim_end_matches(')').parse().ok()).unwrap_or(10.0);
                Ok(Self::Radial { radius: r })
            }
            "xor_fractal" => {
                let m = parts.get(1).and_then(|p| p.trim_end_matches(')').parse().ok()).unwrap_or(7);
                Ok(Self::XorFractal { modulus: m })
            }
            "vertical_stripes" => {
                let w = parts.get(1).and_then(|p| p.trim_end_matches(')').parse().ok()).unwrap_or(10);
                Ok(Self::VerticalStripes { stripe_width: w })
            }
            "horizontal_stripes" => {
                let h = parts.get(1).and_then(|p| p.trim_end_matches(')').parse().ok()).unwrap_or(10);
                Ok(Self::HorizontalStripes { stripe_height: h })
            }
            _ => Err(ParseError::UnknownPattern(s.to_string())),
        }
    }
}

/// IntoIter: Gallery gold.
impl IntoIterator for Pattern {
    type Item = Pattern;
    type IntoIter = std::vec::IntoIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            Self::Checkerboard,
            Self::Radial { radius: 10.0 },
            Self::XorFractal { modulus: 7 },
            Self::VerticalStripes { stripe_width: 10 },
            Self::HorizontalStripes { stripe_height: 10 },
        ].into_iter()
    }
}

/// Blend tie: From prior—pattern-noise pas de deux.
pub fn blend_with_noise(pattern: &Pattern, noise: bool, x: i32, y: i32, mode: BlendMode) -> bool {
    let pat = pattern.eval(x, y);
    blend_noise_and_pattern(noise, pat, mode)
}

/// Err: Aureate alerts.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown pattern: {0}")]
    UnknownPattern(String),
    #[error("Parse error in params: {0}")]
    ParamError(String),
}

#[cfg(test)]
mod stress_tests {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main};

    #[test]
    fn stress_checkerboard_grid() {
        let mut count = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if Pattern::Checkerboard.eval(x as i32, y as i32) {
                    count += 1;
                }
            }
        }

        let density = count as f64 / 1_000_000.0;
        println!("Checkerboard density over 1M tiles: {:.3}", density);
        assert!(density > 0.49 && density < 0.51); // Expect ~50%
    }
}
