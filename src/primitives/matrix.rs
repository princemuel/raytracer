use core::fmt::Debug;
use core::ops::{Index, IndexMut, Mul};


// pub trait Transpose {
//     type Output;
//     fn transpose(&self) -> Self::Output;
// }
// trait SubMatrix {
//     type Output;
//     fn submatrix(&self, row: usize, col: usize) -> Self::Output;
// }
// trait Determinant {
//     fn determinant(&self) -> f64;
// }
// trait Minor {
//     fn minor(&self, row: usize, col: usize) -> f64;
// }
// trait Cofactor {
//     fn cofactor(&self, row: usize, col: usize) -> f64;
// }

// pub trait Inverse: Sized {
//     fn invertible(&self) -> bool;
//     fn inverse(&self) -> Option<Self>;
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: Vec<T>,
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub const COLS: usize = COLS;
    pub const ROWS: usize = ROWS;

    pub fn new() -> Self {
        Self {
            data: vec![T::default(); ROWS * COLS],
        }
    }

    pub fn from_row_major(data: &[T]) -> Self {
        assert_eq!(
            data.len(),
            ROWS * COLS,
            "Data length {} doesn't match matrix size {ROWS}x{COLS} ({})",
            data.len(),
            ROWS * COLS
        );
        Self { data: data.into() }
    }

    pub fn data(&self) -> &[T] { &self.data }
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self { Self::new() }
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> From<&[T]> for Matrix<T, ROWS, COLS> {
    fn from(buffer: &[T]) -> Self { Self::from_row_major(buffer) }
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        (row < ROWS && col < COLS).then(|| &self.data[row * COLS + col])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        (row < ROWS && col < COLS).then(|| &mut self.data[row * COLS + col])
    }

    /// Returns a reference to an element or subslice, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get`][Matrix::get].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// You can think of this like `.get(index).unwrap_unchecked()`.  It's UB
    /// to call `.get_unchecked(len)`, even if you immediately convert to a
    /// pointer.  And it's UB to call `.get_unchecked(..len + 1)`,
    /// `.get_unchecked(..=len)`, or similar.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        // SAFETY: the caller must uphold most of the safety requirements for
        // `get_unchecked`; the slice is dereferenceable because `self` is a safe
        // reference. The returned pointer is safe because impls of  `SliceIndex` have
        // to guarantee that it is.
        unsafe { self.data.get_unchecked(row * COLS + col) }
    }

    /// Returns a mutable reference to [`data`][`Matrix::data`], without doing
    /// bounds checking.
    /// For a safe alternative see [`get_mut`][Matrix::get_mut].
    ///
    /// # Safety
    ///
    ///
    /// Calling this method with an out-of-bounds index is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_mut(index).unwrap_unchecked()`.  It's
    /// UB to call `.get_unchecked_mut(len)`, even if you immediately convert
    /// to a pointer.  And it's UB to call `.get_unchecked_mut(..len + 1)`,
    /// `.get_unchecked_mut(..=len)`, or similar.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        // SAFETY: the caller must uphold the safety requirements for
        // `get_unchecked_mut`; the slice is dereferenceable because `self` is a
        // safe reference. The returned pointer is safe because impls of
        // `SliceIndex` have to guarantee that it is.
        unsafe { self.data.get_unchecked_mut(row * COLS + col) }
    }
}

// impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize>
// PartialEq for Matrix<T, ROWS, COLS> {     fn eq(&self, other: &Self) -> bool
// {         let Self { data: self_data } = self;
//         let Self { data: other_data } = other;

//         other.SIZE;

//         if self.data.len() != other.data.len() || COLS != COLS {
//             return false;
//         }

//         for i in 0..self.data.len() {
//             if !is_equal(self.data[i], other.data[i]) {
//                 return false;
//             }
//         }

//         true
//     }
// }

// Row indexing - returns a slice representing a row
impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> Index<usize>
    for Matrix<T, ROWS, COLS>
{
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        debug_assert!(
            row < ROWS,
            "Row index {row} out of bounds for {ROWS}x{COLS} matrix",
        );
        let start = row * COLS;
        &self.data[start..start + COLS]
    }
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> IndexMut<usize>
    for Matrix<T, ROWS, COLS>
{
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        debug_assert!(
            row < ROWS,
            "Row index {row} out of bounds for {ROWS}x{COLS} matrix",
        );
        let start = row * COLS;
        &mut self.data[start..start + COLS]
    }
}

// Tuple indexing for direct element access
impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> Index<(usize, usize)>
    for Matrix<T, ROWS, COLS>
{
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        debug_assert!(
            row < ROWS,
            "Row index {row} out of bounds for {ROWS}x{COLS} matrix",
        );
        debug_assert!(
            col < COLS,
            "Column index {col} out of bounds for {ROWS}x{COLS} matrix",
        );
        &self.data[row * COLS + col]
    }
}

impl<T: Debug + Default + Copy, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)>
    for Matrix<T, ROWS, COLS>
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        debug_assert!(
            row < ROWS,
            "Row index {row} out of bounds for {ROWS}x{COLS} matrix",
        );
        debug_assert!(
            col < COLS,
            "Column index {col} out of bounds for {ROWS}x{COLS} matrix",
        );
        &mut self.data[row * COLS + col]
    }
}

// Type aliases for common matrices
pub type Matrix2 = Matrix<f64, 2, 2>;
pub type Matrix3 = Matrix<f64, 3, 3>;
pub type Matrix4 = Matrix<f64, 4, 4>;

impl Matrix2 {
    pub fn determinant(&self) -> f64 { self[0][0] * self[1][1] - self[0][1] * self[1][0] }
}

impl Matrix3 {}

// impl SubMatrix for Matrix3 {
//     type Output = Matrix2;

//     fn submatrix(&self, _row: usize, _col: usize) -> Self::Output { todo!() }
// }

impl Matrix4 {}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Self::default();

        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                matrix[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }

        matrix
    }
}
