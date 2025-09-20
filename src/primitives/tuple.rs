use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::cmp::float::is_equal;
use crate::primitives::components::Tuple;
use crate::primitives::point::Point3;
use crate::primitives::vector::Vec3;

/// Create a 4D tuple
#[inline]
pub fn tuple<X, Y, Z, W>(x: X, y: Y, z: Z, w: W) -> Tuple4
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
    W: Into<f64>,
{
    Tuple4::new(x.into(), y.into(), z.into(), w.into())
}

/// 4D Tuple - represents direction and magnitude
#[derive(Default, Debug, Clone, Copy)]
pub struct Tuple4(f64, f64, f64, f64);
impl Tuple4 {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self(x, y, z, w) }

    pub const fn zero() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
}

impl Tuple for Tuple4 {
    fn x(&self) -> f64 { self.0 }

    fn y(&self) -> f64 { self.1 }

    fn z(&self) -> f64 { self.2 }

    fn w(&self) -> f64 { self.3 }
}

impl Tuple4 {
    pub fn is_point(&self) -> bool { is_equal(self.3, 1.0) }

    pub fn is_vector(&self) -> bool { is_equal(self.3, 0.0) }
}

impl PartialEq for Tuple4 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.0, rhs.0)
            && is_equal(self.1, rhs.1)
            && is_equal(self.2, rhs.2)
            && is_equal(self.3, rhs.3)
    }
}

impl Add for Tuple4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl Sub for Tuple4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}

impl Neg for Tuple4 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2, -self.3) }
}

impl Mul<f64> for Tuple4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Mul<Tuple4> for f64 {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output { rhs * self }
}

impl Div<f64> for Tuple4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let inverse = 1.0 / rhs;
        Self::new(
            self.0 * inverse,
            self.1 * inverse,
            self.2 * inverse,
            self.3 * inverse,
        )
    }
}

impl From<Point3> for Tuple4 {
    fn from(p: Point3) -> Self { Self::new(p.x(), p.y(), p.z(), 1.0) }
}

impl From<Vec3> for Tuple4 {
    fn from(v: Vec3) -> Self { Self::new(v.x(), v.y(), v.z(), 0.0) }
}

impl core::fmt::Display for Tuple4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self(x, y, z, w) = self;
        write!(f, "Tuple4({x:.3}, {y:.3}, {z:.3}, {w:.3})")
    }
}
