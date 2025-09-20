use core::ops::{Add, Sub};

use crate::cmp::float::is_equal;
use crate::primitives::components::Tuple;
use crate::primitives::tuple::Tuple4;
use crate::primitives::vector::Vec3;

/// Create a 3D point
#[inline]
pub fn point<X, Y, Z>(x: X, y: Y, z: Z) -> Point3
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
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

impl Tuple for Point3 {
    fn x(&self) -> f64 { self.0 }

    fn y(&self) -> f64 { self.1 }

    fn z(&self) -> f64 { self.2 }

    fn w(&self) -> f64 { 1.0 }
}

impl PartialEq for Point3 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.0, rhs.0) && is_equal(self.1, rhs.1) && is_equal(self.2, rhs.2)
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

impl TryFrom<&Tuple4> for Point3 {
    type Error = &'static str;

    fn try_from(t: &Tuple4) -> Result<Self, Self::Error> {
        if !is_equal(t.w(), 1.0) {
            return Err("Invalid w component for Point3");
        }
        Ok(Self(t.x(), t.y(), t.z()))
    }
}

impl core::fmt::Display for Point3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Point3({:.3}, {:.3}, {:.3})", self.0, self.1, self.2)
    }
}
