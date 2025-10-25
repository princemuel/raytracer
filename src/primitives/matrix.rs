#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize>
where
    [(); ROWS * COLS]:,
{
    buffer: [T; ROWS * COLS],
}

impl<T, const N: usize> Matrix<T, N, N>
where
    [(); N * N]:,
{
    pub const fn new(buffer: [T; N * N]) -> Self { Self { buffer } }
}

impl<T, const N: usize> From<[T; N * N]> for Matrix<T, N, N>
where
    [(); N * N]:,
{
    #[inline]
    fn from(buffer: [T; N * N]) -> Self { Self { buffer } }
}

impl<T: Default + Copy, const N: usize> From<Vec<T>> for Matrix<T, N, N>
where
    [(); N * N]:,
{
    #[inline]
    fn from(v: Vec<T>) -> Self {
        debug_assert_eq!(N * N, v.len());
        let mut result = Self {
            buffer: [T::default(); N * N],
        };
        result.buffer.copy_from_slice(&v);
        result
    }
}

pub type Mat2 = Matrix<f64, 2, 2>;
pub type Mat3 = Matrix<f64, 3, 3>;
pub type Mat4 = Matrix<f64, 4, 4>;
