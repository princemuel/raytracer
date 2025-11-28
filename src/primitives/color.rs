use core::ops::{Add, Mul, Sub};

use crate::cmp::float::is_equal;
/// Creates a color
#[inline]
pub fn color<R, G, B>(r: R, g: G, b: B) -> Color3
where
    R: Into<f64>,
    G: Into<f64>,
    B: Into<f64>,
{
    Color3::new(r.into(), g.into(), b.into())
}

/// A 3-dimensional Color in RGB with floating-point components
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color3(f64, f64, f64);
impl Color3 {
    /// Creates a new color.
    #[must_use]
    #[inline(always)]
    pub const fn new(r: f64, g: f64, b: f64) -> Self { Self(r, g, b) }

    /// Creates a color with all elements set to `value`.
    #[must_use]
    #[inline(always)]
    pub const fn splat(value: f64) -> Self { Self(value, value, value) }
}

impl Color3 {
    /// All zeroes.
    // #000
    pub const BLACK: Self = Self::splat(0.0);
    /// A unit color with the `blue` component set to 1
    // #00f
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    /// A unit color with the `green` and `blue` components set to 1
    // #0ff
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);
    /// A unit color with the `green` component set to 1
    // #0f0
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit color with the `red` and `blue` components set to 1
    // #f0f
    pub const PINK: Self = Self::new(1.0, 0.0, 1.0);
    /// A unit color with the `red` component set to 1
    // #f00
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    /// All ones.
    // #fff
    pub const WHITE: Self = Self::splat(1.0);
    /// A unit color with the `red` and `green` components set to 1
    // #ff0
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);
}

impl Color3 {
    pub const fn r(&self) -> f64 { self.0 }

    pub const fn g(&self) -> f64 { self.1 }

    pub const fn b(&self) -> f64 { self.2 }

    pub const fn w(&self) -> f64 { 1.0 }
}

impl Default for Color3 {
    #[inline(always)]
    fn default() -> Self { Self::BLACK }
}

impl PartialEq for Color3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.0, rhs.0) && is_equal(self.1, rhs.1) && is_equal(self.2, rhs.2)
    }
}

macro_rules! impl_ops {
    ($Struct:ident, $t:ty, $Trait:ident, $func:ident, $op:tt) => {
        impl $Trait for $Struct {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: Self) -> Self::Output {
                Self(self.r() $op rhs.r(), self.g() $op rhs.g(), self.b() $op rhs.b())
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
                Self(self.r() $op rhs, self.g() $op rhs, self.b() $op rhs)
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

impl_ops!(Color3, f64, Mul, mul, *);
impl_ops!(Color3, f64, Add, add, +);
impl_ops!(Color3, f64, Sub, sub, -);

const INV_255: f64 = 1.0 / 255.0;

impl From<(u8, u8, u8)> for Color3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r as f64 * INV_255, g as f64 * INV_255, b as f64 * INV_255)
    }
}
impl From<[u8; 3]> for Color3 {
    fn from(rgb: [u8; 3]) -> Self {
        Self::new(
            rgb[0] as f64 * INV_255,
            rgb[1] as f64 * INV_255,
            rgb[2] as f64 * INV_255,
        )
    }
}
impl From<Color3> for [u8; 3] {
    fn from(color: Color3) -> Self {
        [
            (color.r().clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.g().clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.b().clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }
}

impl core::fmt::Display for Color3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.p$}, {:.p$}, {:.p$}]", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = color(-0.5, 0.4, 1.7);

        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let a = color(0.9, 0.6, 0.75);
        let b = color(0.7, 0.1, 0.25);

        assert_eq!(a + b, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let a = color(0.9, 0.6, 0.75);
        let b = color(0.7, 0.1, 0.25);

        assert_eq!(a - b, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let a = color(1, 0.2, 0.4);
        let b = color(0.9, 1, 0.1);

        assert_eq!(a * b, color(0.9, 0.2, 0.04));
    }

    #[test]
    fn round_trip_rgb_conversion() {
        for r in 0..=255u8 {
            for g in [0, 64, 128, 192, 255] {
                for b in [0, 127, 255] {
                    let rgb = [r, g, b];
                    let c: Color3 = rgb.into();
                    let out: [u8; 3] = c.into();
                    assert_eq!(out, rgb, "round trip failed for {:?}", rgb);
                }
            }
        }
    }
}
