#![allow(unused)]

use core::iter::{Product, Sum};
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::cmp::epsilon::EPSILON;
use crate::cmp::float::is_equal;
use crate::math;
use crate::prelude::Tuple4;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix<const N: usize>
where
    [(); N * N]: Sized,
{
    buffer: [f64; N * N],
}

impl<const N: usize> Matrix<N>
where
    [(); N * N]: Sized,
{
    pub const fn new() -> Self { Self { buffer: [0.0; N * N] } }
}

impl<const N: usize> Default for Matrix<N>
where
    [(); N * N]: Sized,
{
    #[inline(always)]
    fn default() -> Self { Self::new() }
}

impl<const N: usize> PartialEq for Matrix<N>
where
    [(); N * N]: Sized,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.buffer
            .into_iter()
            .zip(rhs.buffer)
            .all(|(a, b)| is_equal(a, b))
    }
}

impl<const N: usize> Add for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            buffer: core::array::from_fn(|i| self.buffer[i] + rhs.buffer[i]),
        }
    }
}

impl<const N: usize> Add<&Self> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output { self.add(*rhs) }
}

impl<const N: usize> Add<&Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn add(self, rhs: &Matrix<N>) -> Self::Output { (*self).add(*rhs) }
}

impl<const N: usize> Add<Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn add(self, rhs: Matrix<N>) -> Self::Output { (*self).add(rhs) }
}

impl<const N: usize> Sub for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            buffer: core::array::from_fn(|i| self.buffer[i] - rhs.buffer[i]),
        }
    }
}

impl<const N: usize> Sub<&Self> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output { self.sub(*rhs) }
}

impl<const N: usize> Sub<&Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn sub(self, rhs: &Matrix<N>) -> Self::Output { (*self).sub(*rhs) }
}

impl<const N: usize> Sub<Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn sub(self, rhs: Matrix<N>) -> Self::Output { (*self).sub(rhs) }
}

impl<const N: usize> Index<usize> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * N;
        &self.buffer[start..start + N]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N>
where
    [(); N * N]: Sized,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * N;
        &mut self.buffer[start..start + N]
    }
}

impl<const N: usize> From<[f64; N * N]> for Matrix<N>
where
    [(); N * N]: Sized,
{
    #[inline]
    fn from(buffer: [f64; N * N]) -> Self { Self { buffer } }
}

impl<'a, const N: usize> TryFrom<&'a [f64]> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Error = String;

    fn try_from(src: &'a [f64]) -> Result<Self, Self::Error> {
        if src.len() != N * N {
            return Err(format!(
                "source length ({}) does not match destination length ({})",
                src.len(),
                N * N
            ));
        }

        let mut result = Self {
            buffer: [Default::default(); N * N],
        };
        result.buffer.copy_from_slice(src);
        Ok(result)
    }
}

pub type Mat2 = Matrix<2>;
pub type Mat3 = Matrix<3>;
pub type Mat4 = Matrix<4>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = Mat4::from([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn test_matrix_equality() {
        let a = Mat4::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        let b = Mat4::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_matrix_addition() {
        let a = Mat2::from([7.0, 3.0, -4.0, 2.0]);
        let b = Mat2::from([1.0, 4.0, 6.0, -5.0]);

        let actual = a + b;
        let expected = Mat2::from([8.0, 7.0, 2.0, -3.0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_matrix_subtraction() {
        let a = Mat2::from([2.0, -4.0, 3.0, 1.0]);
        let b = Mat2::from([1.0, 4.0, -2.0, 3.0]);

        let actual = a - b;
        let expected = Mat2::from([1.0, -8.0, 5.0, -2.0]);
        assert_eq!(actual, expected);
    }
}
