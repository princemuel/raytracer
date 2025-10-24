use core::ops::{Add, Mul, Sub};

use crate::cmp::float::is_equal;
use crate::primitives::components::ColorRGB;

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

/// RGB Color with floating-point components
#[derive(Clone, Copy, Debug, Default)]
pub struct Color3(f64, f64, f64);

impl Color3 {
    #[inline]
    pub const fn new(r: f64, g: f64, b: f64) -> Self { Self(r, g, b) }

    #[inline]
    pub const fn black() -> Self { Self::new(0.0, 0.0, 0.0) }

    #[inline]
    pub const fn white() -> Self { Self::new(1.0, 1.0, 1.0) }

    #[inline]
    pub const fn red() -> Self { Self::new(1.0, 0.0, 0.0) }

    #[inline]
    pub const fn green() -> Self { Self::new(0.0, 1.0, 0.0) }

    #[inline]
    pub const fn blue() -> Self { Self::new(0.0, 0.0, 1.0) }
}

impl ColorRGB for Color3 {
    fn r(&self) -> f64 { self.0 }

    fn g(&self) -> f64 { self.1 }

    fn b(&self) -> f64 { self.2 }
}

impl PartialEq for Color3 {
    fn eq(&self, rhs: &Self) -> bool {
        is_equal(self.0, rhs.0) && is_equal(self.1, rhs.1) && is_equal(self.2, rhs.2)
    }
}

// Color3 arithmetic
impl Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl Sub for Color3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

// Scalar multiplication
impl Mul<f64> for Color3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { Self::new(self.r() * rhs, self.g() * rhs, self.b() * rhs) }
}

impl Mul<Color3> for f64 {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Self::Output { rhs * self }
}

// Hadamard product
impl Mul for Color3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

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
        write!(f, "Color3({:.3}, {:.3}, {:.3})", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constructor() {
        let c1 = color(-0.5, 0.4, 1.7);

        assert_eq!(c1.r(), -0.5);
        assert_eq!(c1.g(), 0.4);
        assert_eq!(c1.b(), 1.7);
    }

    #[test]
    fn test_can_be_muliplied_by_a_scalar() {
        let c1 = color(0.2, 0.3, 0.4);

        let actual = c1 * 2.0;
        let expected = color(0.4, 0.6, 0.8);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_be_muliplied_with_each_other() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.2);

        let actual = c1 * c2;
        let expected = color(0.9, 0.2, 0.08);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_round_trip_rgb_conversion() {
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
