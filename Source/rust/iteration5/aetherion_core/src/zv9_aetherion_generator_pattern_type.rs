#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Checkerboard,
    Radial { radius: f32 },  // Param gold: Tunable ripple.
    XorFractal { modulus: i32 },
    VerticalStripes { width: i32 },
    HorizontalStripes { height: i32 },
}

impl PatternType {
    /// Fluent forge—your golden gambit.
    pub fn builder() -> PatternBuilder { PatternBuilder::new() }

    /// Eval at (x,y): Inline oracle.
    #[inline(always)]
    pub fn eval(&self, x: i32, y: i32) -> bool {
        match self {
            Self::Checkerboard => (x + y) % 2 == 0,
            Self::Radial { radius } => {
                let dist_sq = (x as f32 * x as f32 + y as f32 * y as f32);
                dist_sq <= *radius * *radius  // Sqrt-free: Velocity vow.
            }
            Self::XorFractal { modulus } => (x ^ y) % *modulus == 0,
            Self::VerticalStripes { width } => (x / *width) % 2 == 0,
            Self::HorizontalStripes { height } => (y / *height) % 2 == 0,
        }
    }

    /// 3D stub: Feat for agnostic aura.
    #[cfg(feature = "3d")]
    #[inline(always)]
    pub fn eval_3d(&self, x: i32, y: i32, z: i32) -> bool {
        match self {
            Self::Checkerboard => (x + y + z) % 2 == 0,
            Self::Radial { radius } => {
                let dist_sq = (x as f32 * x as f32 + y as f32 * y as f32 + z as f32 * z as f32);
                dist_sq <= *radius * *radius
            }
            // ... Extend others.
            _ => self.eval(x, y),  // 2D fallback.
        }
    }
}

/// BlendMode: Enum emulsion—customize cue consummated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    And,
    Or,
    Xor,
    Lerp { weight: f32 },  // Weighted: Noise * w + pattern * (1-w).
}

impl Default for BlendMode {
    fn default() -> Self { Self::And }
}

/// Blend: Dispatch deft, inline interp.
#[inline(always)]
pub fn blend_noise_and_pattern(noise: bool, pattern: bool, mode: BlendMode) -> bool {
    match mode {
        BlendMode::And => noise && pattern,
        BlendMode::Or => noise || pattern,
        BlendMode::Xor => noise != pattern,
        BlendMode::Lerp { weight } => {
            let n = noise as u8 as f32;
			let p = pattern as u8 as f32;

            (n * weight + p * (1.0 - weight)) > 0.5  // Threshold: Bool-ish.
        }
    }
}

/// Builder: Ergonomic elevation.
#[derive(Debug, Default)]
pub struct PatternBuilder;

impl PatternBuilder {
    pub fn new() -> Self { Self }

    pub fn checkerboard(self) -> PatternType { PatternType::Checkerboard }

    pub fn radial(self, radius: f32) -> PatternType { PatternType::Radial { radius: radius.max(0.0) } }

    pub fn xor_fractal(self, modulus: i32) -> PatternType { PatternType::XorFractal { modulus: modulus.max(1) } }

    pub fn vertical_stripes(self, width: i32) -> PatternType { PatternType::VerticalStripes { width: width.max(1) } }

    pub fn horizontal_stripes(self, height: i32) -> PatternType { PatternType::HorizontalStripes { height: height.max(1) } }
}

impl FromStr for PatternType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "checkerboard" => Ok(Self::Checkerboard),
            "radial" => Ok(Self::Radial { radius: 10.0 }),  // Default r.
            "xor_fractal" => Ok(Self::XorFractal { modulus: 7 }),
            "vertical_stripes" => Ok(Self::VerticalStripes { width: 10 }),
            "horizontal_stripes" => Ok(Self::HorizontalStripes { height: 10 }),
            _ => Err(ParseError::UnknownPattern(s.to_string())),
        }
    }
}

/// IntoIterator: All patterns—gold gallery.
impl IntoIterator for PatternType {
    type Item = PatternType;
    type IntoIter = std::array::IntoIter<Self, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [Self::Checkerboard, Self::Radial { radius: 10.0 }, Self::XorFractal { modulus: 7 },
         Self::VerticalStripes { width: 10 }, Self::HorizontalStripes { height: 10 }].into_iter()
    }
}

/// Err: Aureate alerts.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown pattern: {0}")]
    UnknownPattern(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pattern() {
        assert_eq!(PatternType::from_str("radial").unwrap(), PatternType::Radial { radius: 10.0 });
        assert!(PatternType::from_str("foo").is_err());
    }

    #[test]
    fn eval_checkerboard() {
        assert!(PatternType::Checkerboard.eval(0, 0));
        assert!(!PatternType::Checkerboard.eval(1, 0));
    }

    #[bench]
    fn bench_radial_1k(b: &mut test::Bencher) {
        let pat = PatternType::Radial { radius: 50.0 };
        b.iter(|| {
            for y in 0..1000 {
                for x in 0..1000 {
                    let _ = pat.eval(x as i32, y as i32);
                }
            }
        });
    }

    #[test]
    fn blend_modes() {
        assert!(blend_noise_and_pattern(true, true, BlendMode::And));
        assert!(!blend_noise_and_pattern(true, false, BlendMode::And));
        assert!(blend_noise_and_pattern(true, false, BlendMode::Xor));
    }
}