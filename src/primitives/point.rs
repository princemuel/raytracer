use core::ops::{Add, Sub};

use crate::cmp::float::is_equal;
use crate::primitives::tuple::Tuple4;
use crate::primitives::vector::Vec3;

/// Create a 3-dimensional point
#[inline]
pub fn point<X, Y, Z>(x: X, y: Y, z: Z) -> Point3
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    Point3::new(x.into(), y.into(), z.into())
}

/// A 3-dimensional point representing a position in space
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Point3(f64, f64, f64);
impl Point3 {
    /// Creates a new point.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self(x, y, z) }

    /// Creates a vector with all elements set to `value`.
    #[inline]
    #[must_use]
    pub const fn splat(value: f64) -> Self { Self(value, value, value) }
}

impl Point3 {
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
    /// A unit point along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    /// A unit point along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    /// A unit point along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    /// All ones.
    pub const ONE: Self = Self::splat(1.0);
    /// A unit point along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    /// A unit point along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit point along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);
}

impl Point3 {
    pub const fn x(&self) -> f64 { self.0 }

    pub const fn y(&self) -> f64 { self.1 }

    pub const fn z(&self) -> f64 { self.2 }

    pub const fn w(&self) -> f64 { 1.0 }
}

impl Default for Point3 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}

impl PartialEq for Point3 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.x(), rhs.x())
            && is_equal(self.y(), rhs.y())
            && is_equal(self.z(), rhs.z())
            && is_equal(self.w(), rhs.w())
    }
}

// Point + Vector = Point
impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

// Point - Point = Vector
impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

// Point - Vector = Point
impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl From<[f64; 3]> for Point3 {
    #[inline]
    fn from(a: [f64; 3]) -> Self { Self::new(a[0], a[1], a[2]) }
}

impl From<Point3> for [f64; 3] {
    #[inline]
    fn from(p: Point3) -> Self { [p.0, p.1, p.2] }
}
impl From<[f64; 4]> for Point3 {
    #[inline]
    fn from(a: [f64; 4]) -> Self { Self::new(a[0], a[1], a[2]) }
}

impl From<Point3> for [f64; 4] {
    #[inline]
    fn from(p: Point3) -> Self { [p.0, p.1, p.2, 0.0] }
}

impl From<(f64, f64, f64)> for Point3 {
    #[inline]
    fn from(t: (f64, f64, f64)) -> Self { Self::new(t.0, t.1, t.2) }
}

impl From<Point3> for (f64, f64, f64) {
    #[inline]
    fn from(p: Point3) -> Self { (p.0, p.1, p.2) }
}
impl From<(f64, f64, f64, f64)> for Point3 {
    #[inline]
    fn from(t: (f64, f64, f64, f64)) -> Self { Self::new(t.0, t.1, t.2) }
}

impl From<Point3> for (f64, f64, f64, f64) {
    #[inline]
    fn from(p: Point3) -> Self { (p.0, p.1, p.2, 0.0) }
}

impl TryFrom<Tuple4> for Point3 {
    type Error = &'static str;

    fn try_from(t: Tuple4) -> Result<Self, Self::Error> {
        if !is_equal(t.w(), 1.0) {
            return Err("Invalid w component for Point3");
        }
        Ok(Self(t.x(), t.y(), t.z()))
    }
}

impl TryFrom<&Tuple4> for Point3 {
    type Error = &'static str;

    fn try_from(t: &Tuple4) -> Result<Self, Self::Error> { Point3::try_from(*t) }
}

impl core::fmt::Display for Point3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.*}, {:.*}, {:.*}]", p, self.x(), p, self.y(), p, self.z())
    }
}
