use core::iter::{Product, Sum};
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::cmp::epsilon::EPSILON;
use crate::cmp::float::is_equal;
use crate::math;
use crate::prelude::Tuple4;

/// Creates a 3-dimensional vector.
#[inline(always)]
#[must_use]
pub fn vector<X, Y, Z>(x: X, y: Y, z: Z) -> Vec3
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    Vec3::new(x.into(), y.into(), z.into())
}

/// A 3-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3(f64, f64, f64);
impl Vec3 {
    /// Creates a new vector.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self(x, y, z) }

    /// Creates a vector with all elements set to `value`.
    #[must_use]
    #[inline(always)]
    pub const fn splat(value: f64) -> Self { Self(value, value, value) }
}

impl Vec3 {
    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    /// All `f64::INFINITY`.
    pub const INFINITY: Self = Self::splat(f64::INFINITY);
    /// All `f64::MAX`.
    pub const MAX: Self = Self::splat(f64::MAX);
    /// All `f64::MIN`.
    pub const MIN: Self = Self::splat(f64::MIN);
    /// All `f64::NAN`.
    pub const NAN: Self = Self::splat(f64::NAN);
    /// All `f64::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);
    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);
    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    /// All ones.
    pub const ONE: Self = Self::splat(1.0);
    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);
}

impl Vec3 {
    pub const fn x(&self) -> f64 { self.0 }

    pub const fn y(&self) -> f64 { self.1 }

    pub const fn z(&self) -> f64 { self.2 }

    pub const fn w(&self) -> f64 { 0.0 }
}

impl Vec3 {
    /// Returns a vector containing each element of `self` modified by a mapping
    /// function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self::new(f(self.x()), f(self.y()), f(self.z()))
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub const fn dot(self, rhs: Self) -> f64 {
        (self.x() * rhs.x()) + (self.y() * rhs.y()) + (self.z() * rhs.z()) + (self.w() * rhs.w())
    }

    /// Returns a vector where every component is the dot product of `self` and
    /// `rhs`.
    #[inline]
    #[must_use]
    pub const fn dot_to_vec(self, rhs: Self) -> Self { Self::splat(self.dot(rhs)) }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub const fn cross(self, rhs: Self) -> Self {
        Self(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f64 { math::sqrt(self.dot(self)) }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub const fn length_squared(self) -> f64 { self.dot(self) }

    /// Takes the reciprocal (inverse) of length, `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f64 { self.length().recip() }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f64 { (self - rhs).length() }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f64 { (self - rhs).length_squared() }

    /// Returns `self` normalized to length 1.0 (unit length)
    ///
    /// For valid results, `self` must be finite and _not_ of length zero,
    /// nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// # Panics
    ///
    /// Will panic if the resulting normalized vector is not finite
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self { self.mul(self.length_recip()) }

    /// Returns `self` normalized to length 1.0 if possible, else returns
    /// `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or
    /// non-finite, the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero()`].
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        (len.is_finite() && len >= EPSILON).then(|| self / len)
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns a
    /// fallback value.
    ///
    /// In particular, if the input is zero (or very close to zero), or
    /// non-finite, the result of this operation will be the fallback value.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        let len = self.length();
        if len.is_finite() && len >= EPSILON {
            self * len.recip()
        } else {
            fallback
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or
    /// non-finite, the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
    pub fn normalize_or_zero(self) -> Self { self.normalize_or(Self::ZERO) }

    /// Normalizes the vector to unit length, **without checking for zero
    /// magnitude**.
    ///
    /// # Safety
    ///
    /// - Calling this on a zero-length vector will result in division by zero,
    ///   producing NaN or infinite components.
    /// - The caller must ensure the vector is non-zero.
    ///
    /// This is an unsafe operation; prefer using [`normalize`] when possible.
    ///
    /// [`normalize`]: Vec3::normalize
    #[deprecated]
    pub unsafe fn normalize_unchecked(&self) -> Self { *self / self.length() }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool { math::abs(self.length_squared() - 1.0) <= EPSILON }

    /// Returns the reflection vector for a given incident vector `self` and
    /// surface normal `normal`.
    ///
    /// `normal` must be normalized for correct results.
    ///
    /// reflected ray direction of a ray v = v + 2b where v is the vector and b
    /// is height of v parallel to the normal n is unit vector of len 1 but
    /// v may not be. To get b, we scale normal vector by the length of the
    /// projection of v onto n basically perpendicular height of v as said
    /// before this is given by dot product of v and n
    ///
    /// *Formula: r = v - 2(v · n)n*
    ///
    /// if n were not a unit vector, we'd also need to divide this dot product
    /// by length of n i.e, normalize it now v point onto the surface and we
    /// want the reflection to point out of the surface so v + 2b becomes v - 2b
    ///
    /// # Panics
    ///
    /// Will panic if `normal` is not normalized when `assert` is enabled.
    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        debug_assert!(normal.is_normalized());
        self - normal * (2.0 * self.dot(normal))
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the
    /// value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`. sWhen `s` is
    /// `1.0`, the result will be equal to `rhs`. When `s` is outside of
    /// range `[0, 1]`, the result is linearly extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f64) -> Self { self * (1.0 - s) + rhs * s }
}

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.x(), rhs.x())
            && is_equal(self.y(), rhs.y())
            && is_equal(self.z(), rhs.z())
            && is_equal(self.w(), rhs.w())
    }
}

macro_rules! impl_ops {
    ($Struct:ident, $t:ty, $Trait:ident, $func:ident, $op:tt) => {
        impl $Trait for $Struct {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: Self) -> Self::Output {
                Self(self.x() $op rhs.x(), self.y() $op rhs.y(), self.z() $op rhs.z())
            }
        }

        impl $Trait<&Self> for $Struct {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: &Self) -> Self::Output {
                self.$func(*rhs)
            }
        }

        impl $Trait<&$Struct> for &$Struct {
            type Output = $Struct;
            #[inline]
            fn $func(self, rhs: &$Struct) -> Self::Output {
                (*self).$func(*rhs)
            }
        }

        impl $Trait<$Struct> for &$Struct {
            type Output = $Struct;
            #[inline]
            fn $func(self, rhs: $Struct) -> Self::Output {
                (*self).$func(rhs)
            }
        }

        impl $Trait<$t> for $Struct {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: $t) -> Self::Output {
                Self(self.x() $op rhs, self.y() $op rhs, self.z() $op rhs)
            }
        }

        impl $Trait<&$t> for $Struct {
            type Output = $Struct;
            #[inline]
            fn $func(self, rhs: &$t) -> Self::Output {
                self.$func(*rhs)
            }
        }

        impl $Trait<&$t> for &$Struct {
            type Output = $Struct;
            #[inline]
            fn $func(self, rhs: &$t) -> Self::Output {
                (*self).$func(*rhs)
            }
        }

        impl $Trait<$t> for &$Struct {
            type Output = $Struct;
            #[inline]
            fn $func(self, rhs: $t) -> Self::Output {
                (*self).$func(rhs)
            }
        }
    };
}

impl_ops!(Vec3, f64, Mul, mul, *);
impl_ops!(Vec3, f64, Div, div, /);
impl_ops!(Vec3, f64, Add, add, +);
impl_ops!(Vec3, f64, Sub, sub, -);

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self { Self(self.x().neg(), self.y().neg(), self.z().neg()) }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output { (*self).neg() }
}

impl AsRef<[f64; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f64; 3] { unsafe { &*(self as *const Self as *const [f64; 3]) } }
}

impl AsMut<[f64; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f64; 3] { unsafe { &mut *(self as *mut Self as *mut [f64; 3]) } }
}

impl Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl Product for Vec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Vec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("index out of bounds"),
        }
    }
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.*}, {:.*}, {:.*}]", p, self.x(), p, self.y(), p, self.z())
    }
}

impl From<[f64; 3]> for Vec3 {
    #[inline]
    fn from(a: [f64; 3]) -> Self { Self::new(a[0], a[1], a[2]) }
}

impl From<Vec3> for [f64; 3] {
    #[inline]
    fn from(v: Vec3) -> Self { [v.0, v.1, v.2] }
}
impl From<[f64; 4]> for Vec3 {
    #[inline]
    fn from(a: [f64; 4]) -> Self { Self::new(a[0], a[1], a[2]) }
}

impl From<Vec3> for [f64; 4] {
    #[inline]
    fn from(v: Vec3) -> Self { [v.0, v.1, v.2, 0.0] }
}

impl From<(f64, f64, f64)> for Vec3 {
    #[inline]
    fn from(t: (f64, f64, f64)) -> Self { Self::new(t.0, t.1, t.2) }
}

impl From<Vec3> for (f64, f64, f64) {
    #[inline]
    fn from(v: Vec3) -> Self { (v.0, v.1, v.2) }
}
impl From<(f64, f64, f64, f64)> for Vec3 {
    #[inline]
    fn from(t: (f64, f64, f64, f64)) -> Self { Self::new(t.0, t.1, t.2) }
}

impl From<Vec3> for (f64, f64, f64, f64) {
    #[inline]
    fn from(v: Vec3) -> Self { (v.0, v.1, v.2, 0.0) }
}

impl TryFrom<Tuple4> for Vec3 {
    type Error = &'static str;

    fn try_from(t: Tuple4) -> Result<Self, Self::Error> {
        if !is_equal(t.w(), 0.0) {
            return Err("Invalid w component for Vec3");
        }
        Ok(Self(t.x(), t.y(), t.z()))
    }
}

impl TryFrom<&Tuple4> for Vec3 {
    type Error = &'static str;

    fn try_from(t: &Tuple4) -> Result<Self, Self::Error> { Vec3::try_from(*t) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_is_exact() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_equality_is_inside_epsilon_range() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0000005, 2.0000001, 2.9999999);
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_inequality_is_outside_epsilon_range() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0001, 2.0, 3.0);
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_equality_with_epsilon_edge_case() {
        let epsilon = 1e-5;

        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0 + epsilon * 0.9, 2.0, 3.0);
        assert_eq!(t1, t2);

        let t3 = Vec3::new(1.0 + epsilon * 1.1, 2.0, 3.0);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_magnitude_positive_nonunit() {
        let t1 = vector(1.0, 2.0, 3.0);
        assert_eq!(t1.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_magnitude_negative_nonunit() {
        let t1 = vector(-1.0, -2.0, -3.0);
        assert_eq!(t1.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_magnitude_squared_avoids_sqrt() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length_squared(), 25.0);
        // Cucumber doesn't test this optimization
    }

    #[test]
    fn test_zero_magnitude_is_zero() {
        let v = vector(0.0, 0.0, 0.0);
        assert_eq!(v.length(), 0.0);
    }

    #[test]
    fn test_normalize_simple_vector() {
        let t1 = vector(4.0, 0.0, 0.0);
        assert_eq!(t1.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normalize_non_simple_vector() {
        let t1 = vector(1.0, 2.0, 3.0);
        let expected_vector = vector(1.0 / 14_f64.sqrt(), 2.0 / 14_f64.sqrt(), 3.0 / 14_f64.sqrt());
        assert_eq!(t1.try_normalize(), Some(expected_vector));
    }

    #[test]
    fn test_try_normalize_zero_vector_is_none() {
        assert_eq!(Vec3::ZERO.try_normalize(), None);
    }

    #[test]
    fn test_try_normalize_tiny_vector_is_none() {
        // Very small nonzero vector — should also be treated as zero for normalization
        // purposes.
        let t1 = Vec3::new(1e-40, 1e-40, 1e-40);
        assert_eq!(t1.try_normalize(), None);
    }

    #[test]
    fn test_try_normalize_unit_vector_is_some() {
        let t1 = Vec3::new(1.0, 1.0, 1.0);
        assert!(t1.try_normalize().is_some());
    }

    #[test]
    fn test_normalize_or_with_nonzero_vector() {
        let t1 = Vec3::new(3.0, 4.0, 0.0);
        let normalized = t1.normalize_or(Vec3::ZERO);
        // length should now be approximately 1.0
        assert!((normalized.length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_normalize_or_with_zero_vector_returns_fallback() {
        let t1 = Vec3::ZERO;
        let fallback = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(t1.normalize_or(fallback), fallback);
    }

    #[test]
    fn test_normalize_or_with_tiny_vector_returns_fallback() {
        let t1 = Vec3::new(1e-40, 1e-40, 1e-40);
        let fallback = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(t1.normalize_or(fallback), fallback);
    }

    #[test]
    fn test_normalize_or_does_not_panic_on_zero() {
        let t1 = Vec3::ZERO;
        let fallback = Vec3::splat(42.0);
        let result = std::panic::catch_unwind(|| t1.normalize_or(fallback));
        assert!(result.is_ok());
    }

    #[test]
    fn test_dot_product_of_two_vectors() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(t1.dot(t2), 20.0);
    }

    #[test]
    fn test_cross_product_of_two_vectors() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(t1.cross(t2), Vec3::new(-1.0, 2.0, -1.0));
        assert_eq!(t2.cross(t1), Vec3::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_reflecting_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let normal = vector(0.0, 1.0, 0.0);
        let r = v.reflect(normal);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflecting_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let sqrt2_div2 = (2.0_f64).sqrt() / 2.0;

        let normal = vector(sqrt2_div2, sqrt2_div2, 0.0);
        let reflected = v.reflect(normal);
        assert_eq!(reflected, vector(1.0, 0.0, 0.0));
    }
}
