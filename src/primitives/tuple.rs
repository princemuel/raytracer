use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::epsilon::Precision;

#[derive(Default, Debug, Clone, Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { x, y, z, w } }

    pub const fn x(&self) -> f64 { self.x }

    pub const fn y(&self) -> f64 { self.y }

    pub const fn z(&self) -> f64 { self.z }

    pub const fn w(&self) -> f64 { self.w }
}

pub const fn point(x: f64, y: f64, z: f64) -> Tuple { Tuple { x, y, z, w: 1.0 } }

pub const fn vector(x: f64, y: f64, z: f64) -> Tuple { Tuple { x, y, z, w: 0.0 } }

impl Tuple {
    pub const fn is_point(&self) -> bool { is_equal_float(self.w, 1.0) }

    pub const fn is_vector(&self) -> bool { is_equal_float(self.w, 0.0) }
}

impl Tuple {
    pub fn magnitude(&self) -> f64 { todo!() }

    pub fn normalize(&self) -> Self { todo!() }

    pub fn dot(&self, _rhs: &Self) -> f64 { todo!() }

    pub fn cross(&self, _rhs: &Self) -> Self { todo!() }

    pub fn reflect(&self, _rhs: &Self) -> Self { todo!() }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal_float(self.x, rhs.x)
            && is_equal_float(self.y, rhs.y)
            && is_equal_float(self.z, rhs.z)
            && is_equal_float(self.w, rhs.w)
    }
}
impl Eq for Tuple {}

impl From<(i32, i32, i32)> for Tuple {
    fn from((x, y, z): (i32, i32, i32)) -> Self { point(x as f64, y as f64, z as f64) }
}

impl<T> From<Point<T>> for Tuple
where
    T: Into<f64>,
{
    fn from(Point(x, y, z): Point<T>) -> Self { point(x.into(), y.into(), z.into()) }
}

impl<T> From<Vector<T>> for Tuple
where
    T: Into<f64>,
{
    fn from(Vector(x, y, z): Vector<T>) -> Self { vector(x.into(), y.into(), z.into()) }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z, -self.w) }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { rhs * self }
}

impl Div<f64> for Tuple {
    type Output = Self;

    // diverges from the book i.e not dividing each tuple component by the scalar
    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}

impl core::fmt::Display for Tuple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

pub struct Point<T>(T, T, T);
pub struct Vector<T>(T, T, T);

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
        return diff < Precision::STANDARD;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = Precision::STANDARD * a.abs().max(b.abs());

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    diff < Precision::STANDARD.max(relative_epsilon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_point() {
        let p = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(p.x(), 4.3);
        assert_eq!(p.y(), -4.2);
        assert_eq!(p.z(), 3.1);
        assert_eq!(p.w(), 1.0);

        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn creating_vector() {
        let v = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(v.x(), 4.3);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);

        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn point_factory() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_factory() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn point_from_coords_i32() {
        let p = Tuple::from(Point(4, -4, 3));

        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn vector_from_coords_i32() {
        let v = Tuple::from(Vector(4, -4, 3));
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn point_from_coords_f64() {
        let p = Tuple::from(Point(4.5, -4.2, 3.1));
        assert_eq!(p, Tuple::new(4.5, -4.2, 3.1, 1.0));
        assert!(p.is_point());
    }

    #[test]
    fn vector_from_coords_f64() {
        let v = Tuple::from(Vector(4.5, -4.2, 3.1));
        assert_eq!(v, Tuple::new(4.5, -4.2, 3.1, 0.0));
        assert!(v.is_vector());
    }

    #[test]
    fn tuple_equality_exact() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn tuple_equality_within_epsilon() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0000005, 2.0000001, 2.9999999, 4.0000001);
        assert_eq!(t1, t2);
    }

    #[test]
    fn tuple_inequality_outside_epsilon() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.00001, 2.0, 3.0, 4.0);
        assert_ne!(t1, t2);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn adding_point_and_vector() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);
        assert_eq!(p + v, point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_vector_x() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_vector_y() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_vector_z() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_positive_vector() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0_f64).sqrt());
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_negative_vector() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0_f64).sqrt());
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn normalizing_vector_4_0_0() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn normalizing_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        let normalized = v.normalize();
        let sqrt14 = (14.0_f64).sqrt();
        assert_eq!(normalized, vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn magnitude_of_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert!(is_equal_float(norm.magnitude(), 1.0));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn dot_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn reflecting_vector_approaching_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let sqrt2_div2 = (2.0_f64).sqrt() / 2.0;
        let n = vector(sqrt2_div2, sqrt2_div2, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    // Edge cases and special values
    // TODO: Fix logic
    #[test]
    #[ignore]
    fn zero_vector_magnitude() {
        let v = vector(0.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 0.0);
    }

    // TODO: Fix logic
    #[test]
    #[ignore]
    #[should_panic]
    fn normalizing_zero_vector_should_panic() {
        let v = vector(0.0, 0.0, 0.0);
        v.normalize();
    }

    #[test]
    fn tuple_equality_with_epsilon_edge_case() {
        let epsilon = 1e-6;
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0 + epsilon * 0.9, 2.0, 3.0, 4.0);
        assert_eq!(t1, t2);

        let t3 = Tuple::new(1.0 + epsilon * 1.1, 2.0, 3.0, 4.0);
        assert_ne!(t1, t3);
    }
}
