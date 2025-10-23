//C:/ZV9/zv9.aetherion/rust/src/shared/math.rs

#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🔁 Full circle constant (τ = 2π).
pub const TAU: f64 = std::f64::consts::PI * 2.0;

/// 🔒 Clamps a value between a minimum and maximum bound.
///
/// # Examples
/// ```
/// fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
///     if val < min {
///         min
///     } else if val > max {
///         max
///     } else {
///         val
///     }
/// }
///
/// let x = clamp(5, 0, 10);    // returns 5
/// let y = clamp(-3, 0, 10);   // returns 0
/// let z = clamp(42, 0, 10);   // returns 10
/// ```
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_clamp_integers() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-3, 0, 10), 0);
        assert_eq!(clamp(42, 0, 10), 10);
    }

    #[test]
    fn stress_clamp_floats() {
        assert_eq!(clamp(3.5_f64, 1.0, 4.0), 3.5);
        assert_eq!(clamp(-1.2_f64, 0.0, 2.0), 0.0);
        assert_eq!(clamp(9.9_f64, 0.0, 5.0), 5.0);
    }

    #[test]
    fn stress_clamp_edge_equal_bounds() {
        assert_eq!(clamp(5, 5, 5), 5);
        assert_eq!(clamp(0.0_f64, 0.0, 0.0), 0.0);
    }

    #[test]
    fn stress_clamp_strings() {
        assert_eq!(clamp("b", "a", "c"), "b");
        assert_eq!(clamp("z", "a", "m"), "m");
        assert_eq!(clamp("a", "b", "c"), "b");
    }

    #[test]
    fn stress_tau_constant_precision() {
        let expected = std::f64::consts::PI * 2.0;
        assert!((TAU - expected).abs() < 1e-12);
    }
}


//end math.rs