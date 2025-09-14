use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::epsilon::EPSILON;

#[derive(Default, Debug, Clone, Copy)]
pub struct Tuple(f64, f64, f64, f64); //Tuple(x, y, z, w)

impl Tuple {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self(x, y, z, w) }

    pub const fn x(&self) -> f64 { self.0 }

    pub const fn y(&self) -> f64 { self.1 }

    pub const fn z(&self) -> f64 { self.2 }

    pub const fn w(&self) -> f64 { self.3 }
}

pub fn point<T>(x: T, y: T, z: T) -> Tuple
where
    T: core::convert::Into<f64>,
{
    Tuple(x.into(), y.into(), z.into(), 1.0)
}
pub fn vector<T>(x: T, y: T, z: T) -> Tuple
where
    T: core::convert::Into<f64>,
{
    Tuple(x.into(), y.into(), z.into(), 0.0)
}

impl Tuple {
    pub const fn is_point(&self) -> bool { is_equal_float(self.w(), 1.0) }

    pub const fn is_vector(&self) -> bool { is_equal_float(self.w(), 0.0) }
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        assert!(self.is_vector());
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Option<Self> {
        assert!(self.is_vector());
        let magnitude = self.magnitude();
        (magnitude > 0.0).then(|| *self / magnitude)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        assert!(self.is_vector() && rhs.is_vector());
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2) + (self.3 * rhs.3)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        assert!(self.is_vector() && rhs.is_vector());
        vector(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn reflect(&self, rhs: &Self) -> Self {
        assert!(self.is_vector() && rhs.is_vector());
        todo!()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal_float(self.0, rhs.0)
            && is_equal_float(self.1, rhs.1)
            && is_equal_float(self.2, rhs.2)
            && is_equal_float(self.3, rhs.3)
    }
}
// impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2, -self.3) }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { rhs * self }
}

impl Div<f64> for Tuple {
    type Output = Self;

    // diverges from the book i.e does not each tuple component by the scalar.
    // more idiomatic
    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}

impl core::fmt::Display for Tuple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

pub struct Point<T>(pub T, pub T, pub T);
pub struct Vector<T>(pub T, pub T, pub T);

const fn is_equal_float(a: f64, b: f64) -> bool {
    // Fast path: exact equality (handles infinities, zeros, and exact matches)
    if a == b {
        return true;
    }

    // Handle NaN cases - NaN should never equal anything
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinite cases that aren't exactly equal
    if a.is_infinite() || b.is_infinite() {
        return false; // Different infinities or one infinite, one finite
    }

    let diff = (a - b).abs();

    // For very small numbers near zero, use absolute epsilon
    if a.abs().max(b.abs()) < 1.0 {
        return diff < EPSILON;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = EPSILON * a.abs().max(b.abs());

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    diff < EPSILON.max(relative_epsilon)
}
