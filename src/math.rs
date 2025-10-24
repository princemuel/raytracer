mod std_math {
    #[inline(always)]
    pub(crate) const fn abs(f: f64) -> f64 { f64::abs(f) }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f64) -> f64 { f64::acos(f64::clamp(f, -1.0, 1.0)) }

    #[inline(always)]
    pub(crate) fn atan2(f: f64, other: f64) -> f64 { f64::atan2(f, other) }

    #[inline(always)]
    pub(crate) fn sin(f: f64) -> f64 { f64::sin(f) }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f64) -> (f64, f64) { f64::sin_cos(f) }

    #[inline(always)]
    pub(crate) fn tan(f: f64) -> f64 { f64::tan(f) }

    #[inline(always)]
    pub(crate) fn sqrt(f: f64) -> f64 { f64::sqrt(f) }

    #[inline(always)]
    pub(crate) const fn copysign(f: f64, sign: f64) -> f64 { f64::copysign(f, sign) }

    #[inline(always)]
    pub(crate) const fn signum(f: f64) -> f64 { f64::signum(f) }

    #[inline(always)]
    pub(crate) const fn round(f: f64) -> f64 { f64::round(f) }

    #[inline(always)]
    pub(crate) const fn trunc(f: f64) -> f64 { f64::trunc(f) }

    #[inline(always)]
    pub(crate) const fn ceil(f: f64) -> f64 { f64::ceil(f) }

    #[inline(always)]
    pub(crate) const fn floor(f: f64) -> f64 { f64::floor(f) }

    #[inline(always)]
    pub(crate) fn exp(f: f64) -> f64 { f64::exp(f) }

    #[inline(always)]
    pub(crate) fn powf(f: f64, n: f64) -> f64 { f64::powf(f, n) }

    #[inline(always)]
    pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 { f64::mul_add(a, b, c) }

    #[inline]
    pub fn div_euclid(a: f64, b: f64) -> f64 { f64::div_euclid(a, b) }

    #[inline]
    pub fn rem_euclid(a: f64, b: f64) -> f64 { f64::rem_euclid(a, b) }
}

pub(crate) use std_math::*;
