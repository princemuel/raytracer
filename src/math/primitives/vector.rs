use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::epsilon::{EPSILON, is_equal_float};
use crate::math::primitives::components::Coordinate3D;

/// Create a 3D vector
#[inline]
pub fn vector<T>(x: T, y: T, z: T) -> Vec3
where
    T: core::convert::Into<f64>,
{
    Vec3::new(x.into(), y.into(), z.into())
}

/// 3D Vector - represents direction and magnitude
#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3(f64, f64, f64);
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self(x, y, z) }

    pub const fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
}

impl Vec3 {
    #[inline]
    pub const fn unit_x() -> Self { Self::new(1.0, 0.0, 0.0) }

    #[inline]
    pub const fn unit_y() -> Self { Self::new(0.0, 1.0, 0.0) }

    #[inline]
    pub const fn unit_z() -> Self { Self::new(0.0, 0.0, 1.0) }
}

impl Vec3 {
    /// Calculate magnitude (length) of vector
    #[inline]
    pub fn magnitude(&self) -> f64 { (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt() }

    /// Calculate squared magnitude (avoids sqrt for comparisons)
    #[inline]
    pub fn magnitude_sqrd(&self) -> f64 { self.0 * self.0 + self.1 * self.1 + self.2 * self.2 }

    /// Normalizes the vector to unit length.
    ///
    /// Returns `None` if the vector is effectively zero (magnitude < `EPSILON`)
    /// to avoid producing NaN values.
    pub fn normalize(self) -> Option<Self> {
        let magnitude = self.magnitude();
        (magnitude >= EPSILON).then(|| self / magnitude)
    }

    /// Normalizes the vector to unit length, **without checking for zero
    /// magnitude**.
    ///
    /// # Safety
    ///
    /// - Calling this on a zero-length vector will result in division by zero,
    ///   producing NaN or infinite components.
    /// - The caller must ensure the vector is non-zero.
    ///
    /// This is a low-level operation; prefer using [`normalize`] when possible.
    #[inline]
    pub unsafe fn normalize_unchecked(self) -> Self { self / self.magnitude() }

    /// The dot product of a vector
    #[inline]
    pub fn dot(self, rhs: Self) -> f64 { self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 }

    /// The cross product of a vector
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    /// Reflect vector across a normal.
    /// Normal should be normalized for correct results
    #[inline]
    pub fn reflect(self, normal: Self) -> Self { self - 2.0 * self.dot(normal) * normal }

    /// Linear interpolation between two vectors
    #[inline]
    pub fn lerp(self, other: Self, t: f64) -> Self { self + t * (other - self) }
}

impl Coordinate3D for Vec3 {
    fn x(&self) -> f64 { self.0 }

    fn y(&self) -> f64 { self.1 }

    fn z(&self) -> f64 { self.2 }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal_float(self.0, rhs.0) && is_equal_float(self.1, rhs.1) && is_equal_float(self.2, rhs.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2) }
}
// Scalar multiplication
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs) }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { rhs * self }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let inverse = 1.0 / rhs;
        Self::new(self.0 * inverse, self.1 * inverse, self.2 * inverse)
    }
}

impl From<Vec3> for [f64; 4] {
    fn from(v: Vec3) -> Self { [v.0, v.1, v.2, 0.0] }
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec3({:.3}, {:.3}, {:.3})", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_equality_is_exact() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn vec3_equality_is_inside_epsilon_range() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0000005, 2.0000001, 2.9999999);
        assert_eq!(t1, t2);
    }

    #[test]
    fn vec3_inequality_is_outside_epsilon_range() {
        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0001, 2.0, 3.0);
        assert_ne!(t1, t2);
    }

    #[test]
    fn vec3_equality_with_epsilon_edge_case() {
        let epsilon = 1e-5;

        let t1 = Vec3::new(1.0, 2.0, 3.0);
        let t2 = Vec3::new(1.0 + epsilon * 0.9, 2.0, 3.0);
        assert_eq!(t1, t2);

        let t3 = Vec3::new(1.0 + epsilon * 1.1, 2.0, 3.0);
        assert_ne!(t1, t3);
    }

    #[test]
    fn vec3_magnitude_squared_avoids_sqrt() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude_sqrd(), 25.0);
        // Cucumber doesn't test this optimization
    }

    #[test]
    fn vec3_zero_magnitude_iz_zero() {
        let v = vector(0.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn vec3_normalize_zero_vector_returns_none() {
        let zero = Vec3::zero();
        assert_eq!(zero.normalize(), None);
    }

    #[test]
    fn vec3_reflection_approaching_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn vec3_reflection_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let sqrt2_div2 = (2.0_f64).sqrt() / 2.0;
        let n = vector(sqrt2_div2, sqrt2_div2, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
