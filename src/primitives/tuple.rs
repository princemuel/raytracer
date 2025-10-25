use core::iter::{Product, Sum};
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::cmp::float::is_equal;
use crate::primitives::point::Point3;
use crate::primitives::vector::Vec3;

/// Creates a 4-dimensional tuple
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

/// A 4-dimensional tuple. Can represent either a point or a vector.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Tuple4(f64, f64, f64, f64);
impl Tuple4 {
    /// Creates a new tuple.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self(x, y, z, w) }

    /// Creates a tuple with all elements set to `value`.
    #[inline]
    #[must_use]
    pub const fn splat(value: f64) -> Self { Self(value, value, value, value) }
}

impl Tuple4 {
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
    /// All ones.
    pub const ONE: Self = Self::splat(1.0);
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);
}

impl Tuple4 {
    pub const fn x(&self) -> f64 { self.0 }

    pub const fn y(&self) -> f64 { self.1 }

    pub const fn z(&self) -> f64 { self.2 }

    pub const fn w(&self) -> f64 { self.3 }
}

impl Tuple4 {
    /// Returns a tuple containing each element of `self` modified by a mapping
    /// function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self::new(f(self.x()), f(self.y()), f(self.z()), f(self.w()))
    }

    pub fn is_point(&self) -> bool { is_equal(self.3, 1.0) }

    pub fn is_vector(&self) -> bool { is_equal(self.3, 0.0) }
}

impl Default for Tuple4 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}

impl PartialEq for Tuple4 {
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
                Self(self.x() $op rhs.x(), self.y() $op rhs.y(), self.z() $op rhs.z(), self.w() $op rhs.w())
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
                Self(self.x() $op rhs, self.y() $op rhs, self.z() $op rhs, self.w() $op rhs)
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

impl_ops!(Tuple4, f64, Mul, mul, *);
impl_ops!(Tuple4, f64, Div, div, /);
impl_ops!(Tuple4, f64, Add, add, +);
impl_ops!(Tuple4, f64, Sub, sub, -);

impl Neg for Tuple4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self { Self(self.0.neg(), self.1.neg(), self.2.neg(), self.3.neg()) }
}

impl Neg for &Tuple4 {
    type Output = Tuple4;

    #[inline]
    fn neg(self) -> Self::Output { (*self).neg() }
}

impl AsRef<[f64; 4]> for Tuple4 {
    #[inline]
    fn as_ref(&self) -> &[f64; 4] { unsafe { &*(self as *const Self as *const [f64; 4]) } }
}

impl AsMut<[f64; 4]> for Tuple4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f64; 4] { unsafe { &mut *(self as *mut Self as *mut [f64; 4]) } }
}

impl Sum for Tuple4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Tuple4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl Product for Tuple4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Tuple4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl Index<usize> for Tuple4 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Tuple4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("index out of bounds"),
        }
    }
}

impl core::fmt::Display for Tuple4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}, {:.*}]",
                p,
                self.x(),
                p,
                self.y(),
                p,
                self.z(),
                p,
                self.w()
            )
        } else {
            write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
        }
    }
}

impl From<Point3> for Tuple4 {
    fn from(p: Point3) -> Self { Self::new(p.x(), p.y(), p.z(), 1.0) }
}

impl From<Vec3> for Tuple4 {
    fn from(v: Vec3) -> Self { Self::new(v.x(), v.y(), v.z(), 0.0) }
}

impl From<[f64; 3]> for Tuple4 {
    #[inline]
    fn from(a: [f64; 3]) -> Self { Self::new(a[0], a[1], a[2], 0.0) }
}
impl From<Tuple4> for [f64; 3] {
    #[inline]
    fn from(v: Tuple4) -> Self { [v.0, v.1, v.2] }
}
impl From<[f64; 4]> for Tuple4 {
    #[inline]
    fn from(a: [f64; 4]) -> Self { Self::new(a[0], a[1], a[2], a[3]) }
}

impl From<Tuple4> for [f64; 4] {
    #[inline]
    fn from(v: Tuple4) -> Self { [v.0, v.1, v.2, v.3] }
}

impl From<(f64, f64, f64, f64)> for Tuple4 {
    #[inline]
    fn from(t: (f64, f64, f64, f64)) -> Self { Self::new(t.0, t.1, t.2, t.3) }
}

impl From<Tuple4> for (f64, f64, f64, f64) {
    #[inline]
    fn from(v: Tuple4) -> Self { (v.0, v.1, v.2, v.3) }
}
