use core::ops::{Add, Sub};

use crate::math::epsilon::is_equal_float;
use crate::math::primitives::components::Coordinate3D;
use crate::math::primitives::vector::Vec3;

/// Create a 3D point
#[inline]
pub fn point<T>(x: T, y: T, z: T) -> Point3
where
    T: core::convert::Into<f64>,
{
    Point3::new(x.into(), y.into(), z.into())
}

/// 3D Point - represents a position in space
#[derive(Default, Debug, Clone, Copy)]
pub struct Point3(f64, f64, f64);
impl Point3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self(x, y, z) }

    pub const fn origin() -> Self { Self::new(0.0, 0.0, 0.0) }
}

impl Coordinate3D for Point3 {
    fn x(&self) -> f64 { self.0 }

    fn y(&self) -> f64 { self.1 }

    fn z(&self) -> f64 { self.2 }
}

impl PartialEq for Point3 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal_float(self.0, rhs.0) && is_equal_float(self.1, rhs.1) && is_equal_float(self.2, rhs.2)
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

impl From<Point3> for [f64; 4] {
    fn from(p: Point3) -> Self { [p.0, p.1, p.2, 1.0] }
}

impl core::fmt::Display for Point3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Point3({:.3}, {:.3}, {:.3})", self.x(), self.y(), self.z())
    }
}
