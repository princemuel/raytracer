use float_cmp::approx_eq;

use crate::math::epsilon::Epsilon;

/// Floating-point approximate equality comparison with domain-specific
/// precision levels.
///
/// Provides three comparison modes tailored for different use cases in ray
/// tracing and geometric computation where exact floating-point equality is
/// unreliable.
///
/// # Examples
///
/// ```
/// use crate::math::ApproxEq;
///
/// let a = 0.1 + 0.2;
/// let b = 0.3;
///
/// // Default tight comparison for math operations
/// assert!(a.approx_eq(b));
///
/// // Loose comparison for geometric intersections
/// assert!(1.0001.approx_eq_low_precision(1.0002));
///
/// // Custom epsilon for specific requirements
/// assert!(1.5.approx_eq_epsilon(1.51, 0.02));
/// ```
pub trait ApproxEq<Rhs = Self> {
    /// High-precision comparison using tight epsilon.
    ///
    /// For mathematical operations, unit tests, sanity checks.
    fn approx_eq(self, other: Rhs) -> bool;

    /// Low-precision comparison using loose epsilon.
    ///
    /// For ray-surface intersections, shadow calculations,
    /// geometric transformations where small errors accumulate.
    fn approx_eq_low_precision(self, other: Rhs) -> bool;

    /// Custom epsilon comparison for specialized cases.
    ///
    /// Use when you need explicit control over precision tolerance.
    ///
    /// Special values:
    /// - `NaN` is never considered equal to anything.
    /// - Infinities (`±∞`) are **not** considered approximately equal, even to
    ///   themselves.
    ///
    /// Rationale:
    /// - In a **math library**, one might allow `∞ == ∞` as `true`.
    /// - In a **raytracer**, `∞` represents an error signal (e.g. a ray that
    ///   never intersects) rather than a valid position. Treating infinities as
    ///   equal would let invalid results pass as geometrically meaningful,
    ///   which is unsafe in rendering logic.
    ///
    /// # Examples
    /// ```
    /// use crate::math::ApproxEq;
    ///
    /// // Standard float comparisons with explicit epsilon
    /// assert!(1.000_000_001_f64.approx_eq_epsilon(1.0, 1e-6));
    /// assert!(!1.0001_f64.approx_eq_epsilon(1.0, 1e-6));
    ///
    /// // NaN is never equal
    /// assert!(!f64::NAN.approx_eq_epsilon(f64::NAN, 1e-6));
    ///
    /// // Infinities are never equal, even to themselves
    /// assert!(!f64::INFINITY.approx_eq_epsilon(f64::INFINITY, 1e-6));
    /// assert!(!f64::NEG_INFINITY.approx_eq_epsilon(f64::NEG_INFINITY, 1e-6));
    /// ```
    fn approx_eq_epsilon(self, other: Rhs, epsilon: f64) -> bool;
}

impl ApproxEq for f64 {
    #[inline]
    fn approx_eq(self, other: Self) -> bool { self.approx_eq_epsilon(other, Epsilon::TIGHT) }

    #[inline]
    fn approx_eq_low_precision(self, other: Self) -> bool {
        self.approx_eq_epsilon(other, Epsilon::INTERSECT)
    }

    #[inline]
    fn approx_eq_epsilon(self, other: Self, epsilon: f64) -> bool {
        if self.is_nan() || other.is_nan() {
            return false; // NaN never equals anything
        }

        if self.is_infinite() || other.is_infinite() {
            return false; // ban infinities
        }

        approx_eq!(f64, self, other, epsilon = epsilon)
    }
}

// Extend support for common numeric types
impl ApproxEq for f32 {
    #[inline]
    fn approx_eq(self, other: Self) -> bool {
        self.approx_eq_epsilon(other, (Epsilon::TIGHT as f32).into())
    }

    #[inline]
    fn approx_eq_low_precision(self, other: Self) -> bool {
        self.approx_eq_epsilon(other, (Epsilon::INTERSECT as f32).into())
    }

    #[inline]
    fn approx_eq_epsilon(self, other: Self, epsilon: f64) -> bool {
        (self as f64).approx_eq_epsilon(other as f64, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};

    use super::*;

    #[test]
    fn test_tight_precision_comparisons() {
        // Should pass with tight epsilon (1e-9)
        assert!(1.0.approx_eq(1.0));
        assert!(0.0.approx_eq(0.0));
        assert!(1.0.approx_eq(1.0 + 1e-10));
        assert!((-1.0).approx_eq(-1.0 + 1e-10));

        // Should fail with tight epsilon
        assert!(!1.0.approx_eq(1.0 + 1e-8));
        assert!(!0.0.approx_eq(1e-8));
        assert!(!1.0.approx_eq(1.1));
    }

    #[test]
    fn test_low_precision_comparisons() {
        // Should pass with loose epsilon (1e-4)
        assert!(1.0.approx_eq_low_precision(1.0001));
        assert!(0.0.approx_eq_low_precision(1e-5));
        assert!((0.1 + 0.2).approx_eq_low_precision(0.3));

        // Geometric calculation scenarios
        #[allow(clippy::approx_constant)]
        let value = 3.14159;
        assert!(PI.approx_eq_low_precision(value));
        #[allow(clippy::approx_constant)]
        let value = 2.71828;
        assert!(E.approx_eq_low_precision(value));

        // Should still fail for significant differences
        assert!(!1.0.approx_eq_low_precision(1.001));
        assert!(!0.0.approx_eq_low_precision(0.001));
    }

    #[test]
    fn test_custom_epsilon() {
        // Very strict
        assert!(1.0.approx_eq_epsilon(1.0 + 1e-12, 1e-11));
        assert!(!1.0.approx_eq_epsilon(1.0 + 1e-10, 1e-11));

        // Very loose
        assert!(1.0.approx_eq_epsilon(1.5, 1.0));
        assert!(PI.approx_eq_epsilon(22.0 / 7.0, 0.01));

        // Zero epsilon (exact equality)
        assert!(1.0.approx_eq_epsilon(1.0, 0.0));
        assert!(!1.0.approx_eq_epsilon(1.0 + f64::EPSILON, 0.0));
    }

    #[test]
    fn test_special_float_values() {
        // Infinities
        assert!(!f64::INFINITY.approx_eq_epsilon(f64::INFINITY, Epsilon::STANDARD));
        assert!(!f64::INFINITY.approx_eq_epsilon(f64::NEG_INFINITY, Epsilon::STANDARD));
        assert!(!f64::INFINITY.approx_eq_epsilon(1e100, Epsilon::STANDARD));

        // NaN - should never be equal
        assert!(!f64::NAN.approx_eq(f64::NAN));
        assert!(!f64::NAN.approx_eq_low_precision(f64::NAN));
        assert!(!f64::NAN.approx_eq_epsilon(f64::NAN, 1.0));
        assert!(!f64::NAN.approx_eq_epsilon(1.0, 1.0));

        // Zero variants
        assert!(0.0.approx_eq(-0.0));
        assert!((-0.0).approx_eq(0.0));
    }

    // TODO: Fix logic, precision and test for large numbers
    #[test]
    #[ignore]
    fn test_large_numbers() {
        let large1 = 1e100;
        let large2 = 1e100 + 1e90;

        // Should work with appropriate epsilon
        assert!(!large1.approx_eq(large2)); // Tight fails
        assert!(large1.approx_eq_low_precision(large2)); // Loose passes

        // Very large differences
        let large3 = 1e100 + 1e95;
        assert!(!large1.approx_eq_low_precision(large3));
    }

    #[test]
    fn test_small_numbers() {
        let small1 = 1e-100;
        let small2 = 1e-100 + 1e-110;

        assert!(small1.approx_eq(small2));

        // Near zero
        assert!(1e-15.approx_eq(0.0));
        assert!((-1e-15).approx_eq(0.0));
        assert!(!1e-5.approx_eq(0.0)); // Should fail for larger values
    }

    #[test]
    fn test_mathematical_constants() {
        // Test conversions and calculations with known constants
        assert!(PI.approx_eq(std::f64::consts::PI));
        assert!((2.0 * PI).approx_eq(std::f64::consts::TAU));
        #[allow(clippy::approx_constant)]
        let value = 1.5707963;
        assert!((PI / 2.0).approx_eq_epsilon(value, Epsilon::STANDARD));

        // Calculated vs constant
        let calculated_e = (1.0 + 1.0 / 1e6_f64).powf(1e6);
        assert!(calculated_e.approx_eq_low_precision(E));
    }

    // TODO: Fix logic, precision and test for accumulated errors
    #[test]
    #[ignore]
    fn test_accumulated_errors() {
        // Simulate accumulated floating-point errors
        let mut sum = 0.0;
        for _ in 0..1000 {
            sum += 0.1;
        }

        // Should be 100.0, but has accumulated error
        assert!(!sum.approx_eq(100.0)); // Tight precision fails
        assert!(sum.approx_eq_low_precision(100.0)); // Loose precision passes

        // Verify the error is actually there
        assert!((sum - 100.0f64).abs() > Epsilon::TIGHT);
        assert!((sum - 100.0f64).abs() < Epsilon::INTERSECT);
    }

    #[test]
    fn test_trigonometric_identities() {
        let angle = PI / 6.0; // 30 degrees

        // sin²θ + cos²θ = 1
        let sin_sq = angle.sin().powi(2);
        let cos_sq = angle.cos().powi(2);
        assert!((sin_sq + cos_sq).approx_eq(1.0));

        // tan(θ) = sin(θ)/cos(θ)
        let tan_calc = angle.sin() / angle.cos();
        assert!(tan_calc.approx_eq(angle.tan()));
    }

    // TODO: Fix logic, precision and test for f32 support
    #[test]
    #[ignore]
    fn test_f32_support() {
        let a: f32 = 1.0;
        let b: f32 = 1.0 + 1e-8;

        assert!(a.approx_eq_low_precision(b));
        assert!(!a.approx_eq(b)); // Should fail with tight precision

        // Test f32 precision limits
        let c: f32 = 1.0;
        let d: f32 = 1.0 + f32::EPSILON;
        assert!(c.approx_eq_epsilon(d, f32::EPSILON as f64 * 2.0));
    }

    #[test]
    fn test_ray_tracing_scenarios() {
        // Surface intersection tolerance
        let ray_hit = 0.9999999;
        let surface = 1.0;
        assert!(ray_hit.approx_eq_low_precision(surface));

        // Shadow ray epsilon (avoiding self-intersection)
        let shadow_epsilon = 1e-4;
        let surface_point = 5.0;
        let shadow_start = surface_point + shadow_epsilon;
        assert!(!surface_point.approx_eq_epsilon(shadow_start, shadow_epsilon / 2.0));
        assert!(surface_point.approx_eq_epsilon(shadow_start, shadow_epsilon * 2.0));

        // Normal vector normalization check
        let length_sq = 0.999999999; // Almost normalized
        assert!(length_sq.approx_eq_low_precision(1.0));
    }

    // TODO: Fix logic, precision and test for perfomance critical paths
    #[test]
    #[ignore]
    fn test_performance_critical_paths() {
        // Ensure the trait methods inline properly for hot paths
        let a = 1.23456789;
        let b = 1.23456790;

        // These should compile to efficient comparisons
        let _tight = a.approx_eq(b);
        let _loose = a.approx_eq_low_precision(b);
        let _custom = a.approx_eq_epsilon(b, 1e-7);

        // Test batch operations
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = [1.0001, 2.0001, 3.0001, 4.0001, 5.0001];

        let all_close = values
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a.approx_eq_low_precision(*b));

        assert!(all_close);
    }
}
