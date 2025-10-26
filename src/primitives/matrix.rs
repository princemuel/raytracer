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
    #[inline(always)]
    pub const fn new() -> Self { Self { buffer: [0.0; N * N] } }

    // Keepin this around for potential use cases
    #[inline(always)]
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> f64,
    {
        let buffer = core::array::from_fn(|i| {
            let row = i / N;
            let col = i % N;
            f(row, col)
        });
        Self { buffer }
    }
}

impl<const N: usize> Matrix<N>
where
    [(); N * N]: Sized,
{
    pub const IDENTITY: Self = Self::diagonal(1.0);
}

impl<const N: usize> Matrix<N>
where
    [(); N * N]: Sized,
{
    #[inline(always)]
    pub const fn diagonal(value: f64) -> Self {
        let mut buffer = [0.0; N * N];
        let mut i = 0;
        while i < N {
            buffer[i * N + i] = value;
            i += 1;
        }
        Self { buffer }
    }

    pub const fn from_diagonal(values: [f64; N]) -> Self {
        let mut buffer = [0.0; N * N];
        let mut i = 0;
        while i < N {
            buffer[i * N + i] = values[i];
            i += 1;
        }
        Self { buffer }
    }

    pub const fn transpose(&self) -> Self {
        // let mut buffer = [0.0; N * N];
        // let mut row = 0;
        // while row < N {
        //     let mut col = 0;
        //     while col < N {
        //         buffer[col * N + row] = self.buffer[row * N + col];
        //         col += 1;
        //     }
        //     row += 1;
        // }

        let mut buffer = [0.0; N * N];
        let mut i = 0;

        while i < N * N {
            let row = i / N;
            let col = i % N;
            buffer[col * N + row] = self.buffer[i];
            i += 1;
        }

        Self { buffer }
    }
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
        let buffer = core::array::from_fn(|i| self.buffer[i] + rhs.buffer[i]);
        Self { buffer }
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
        let buffer = core::array::from_fn(|i| self.buffer[i] - rhs.buffer[i]);
        Self { buffer }
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

impl<const N: usize> Mul for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        // let mut matrix = Self::new();
        // for row in 0..N {
        //     for col in 0..N {
        //         matrix[(row, col)] = (0..N).map(|k| self[(row, k)] * rhs[(k,
        // col)]).sum();     }
        // }

        Self::from_fn(|row, col| (0..N).map(|k| self[(row, k)] * rhs[(k, col)]).sum())
    }
}

impl<const N: usize> Mul<&Self> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output { self.mul(*rhs) }
}

impl<const N: usize> Mul<&Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn mul(self, rhs: &Matrix<N>) -> Self::Output { (*self).mul(*rhs) }
}

impl<const N: usize> Mul<Matrix<N>> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Matrix<N>;

    #[inline]
    fn mul(self, rhs: Matrix<N>) -> Self::Output { (*self).mul(rhs) }
}

impl<const N: usize> Mul<Tuple4> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        let v = [rhs.x(), rhs.y(), rhs.z(), rhs.w()];
        // !NOTE: This currently only works for 4x4 matrices (0..N == 4)
        let result: [f64; 4] =
            core::array::from_fn(|row| (0..N).map(|col| self[(row, col)] * v[col]).sum());

        Tuple4::from(result)
    }
}

impl<const N: usize> Mul<&Tuple4> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Tuple4;

    #[inline]
    fn mul(self, rhs: &Tuple4) -> Self::Output { self.mul(*rhs) }
}

impl<const N: usize> Mul<&Tuple4> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Tuple4;

    #[inline]
    fn mul(self, rhs: &Tuple4) -> Self::Output { (*self).mul(*rhs) }
}

impl<const N: usize> Mul<Tuple4> for &Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = Tuple4;

    #[inline]
    fn mul(self, rhs: Tuple4) -> Self::Output { (*self).mul(rhs) }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N>
where
    [(); N * N]: Sized,
{
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output { &self.buffer[row * N + col] }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N>
where
    [(); N * N]: Sized,
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.buffer[row * N + col]
    }
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
    type Error = &'static str;

    fn try_from(src: &'a [f64]) -> Result<Self, Self::Error> {
        let buffer = src.try_into().map_err(|_| "slice length mismatch")?;
        Ok(Self { buffer })
    }
}

#[allow(unused)]
mod matrices {
    use super::*;

    pub const trait Submatrix {
        type Output;

        fn submatrix(&self, row: usize, col: usize) -> Self::Output;
    }

    pub const trait Determinant {
        fn determinant(&self) -> f64;
    }

    pub const trait Minor {
        fn minor(&self, row: usize, col: usize) -> f64;
    }

    trait Cofactor {
        fn cofactor(&self, row: usize, col: usize) -> f64;
    }

    pub trait Inverse: Sized {
        type Output;

        fn invertible(&self) -> bool;
        fn inverse(&self) -> Option<Self::Output>;
        // fn inverse(&self) -> Option<Self>;
    }

    // pub trait DefinedMatrix<const N: usize>
    // where
    //     [(); N * N]: Sized,
    // {
    //     fn identity() -> Matrix<N> {
    //         let mut matrix = Matrix::<N>::new();
    //         for i in 0..N {
    //             matrix[i][i] = 1.0;
    //         }
    //         matrix
    //     }
    // }

    pub type Mat2 = Matrix<2>;

    pub type Mat3 = Matrix<3>;
    pub type Mat4 = Matrix<4>;
}
pub use matrices::*;

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
