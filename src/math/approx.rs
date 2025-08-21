use float_cmp::approx_eq;

use crate::math::epsilon::Epsilon;

/// floating-point approximate equality comparison with domain-specific
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
        approx_eq!(f64, self, other, epsilon = epsilon)
    }
}
