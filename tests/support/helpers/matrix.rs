// use raytracer::prelude::*;

// #[derive(Clone, Debug, PartialEq)]
// pub enum MatrixN {
//     Matrix2(Matrix2),
//     Matrix3(Matrix3),
//     Matrix4(Matrix4),
// }

// impl MatrixN {
//     /// Create a new matrix from dimensions and data buffer
//     pub fn new(rows: usize, cols: usize, buffer: &[f64]) -> Self {
//         match (rows, cols) {
//             (2, 2) => {
//                 assert_eq!(buffer.len(), 4, "Buffer size mismatch for 2x2
// matrix");                 MatrixN::Matrix2(Matrix2::from_row_major(buffer))
//             },
//             (3, 3) => {
//                 assert_eq!(buffer.len(), 9, "Buffer size mismatch for 3x3
// matrix");                 MatrixN::Matrix3(Matrix3::from_row_major(buffer))
//             },
//             (4, 4) => {
//                 assert_eq!(buffer.len(), 16, "Buffer size mismatch for 4x4
// matrix");                 MatrixN::Matrix4(Matrix4::from_row_major(buffer))
//             },
//             _ => panic!("Unsupported matrix dimensions: {}x{}", rows, cols),
//         }
//     }

//     /// Get the dimensions (rows, cols) of the matrix
//     pub fn _dimensions(&self) -> (usize, usize) {
//         match self {
//             MatrixN::Matrix2(_) => (2, 2),
//             MatrixN::Matrix3(_) => (3, 3),
//             MatrixN::Matrix4(_) => (4, 4),
//         }
//     }

//     /// Get the total number of elements
//     pub fn _size(&self) -> usize {
//         match self {
//             MatrixN::Matrix2(_) => 4,
//             MatrixN::Matrix3(_) => 9,
//             MatrixN::Matrix4(_) => 16,
//         }
//     }

//     /// Access element at (row, col) - for testing purposes
//     pub fn _get(&self, row: usize, col: usize) -> f64 {
//         match self {
//             MatrixN::Matrix2(m) => {
//                 assert!(row < 2 && col < 2, "Index out of bounds for 2x2
// matrix");                 m[row][col]
//             },
//             MatrixN::Matrix3(m) => {
//                 assert!(row < 3 && col < 3, "Index out of bounds for 3x3
// matrix");                 m[row][col]
//             },
//             MatrixN::Matrix4(m) => {
//                 assert!(row < 4 && col < 4, "Index out of bounds for 4x4
// matrix");                 m[row][col]
//             },
//         }
//     }

//     // / Calculate determinant (if supported for this matrix size)
//     // pub fn determinant(&self) -> f64 {
//     //     match self {
//     //         MatrixN::Matrix2(m) => m.determinant(),
//     //         MatrixN::Matrix3(m) => m.determinant(),
//     //         MatrixN::Matrix4(m) => m.determinant(),
//     //     }
//     // }

//     // / Check if matrix is invertible (4x4 only)
//     // pub fn is_invertible(&self) -> bool {
//     //     match self {
//     //         MatrixN::Matrix4(m) => m.is_invertible(),
//     //         _ => false,
//     //     }
//     // }

//     // / Get the inverse (4x4 only)
//     // pub fn inverse(&self) -> Option<MatrixN> {
//     //     match self {
//     //         MatrixN::Matrix4(m) => m.inverse().map(MatrixN::Matrix4),
//     //         _ => None,
//     //     }
//     // }

//     // / Transpose the matrix (4x4 only in current implementation)
//     // pub fn transpose(&self) -> MatrixN {
//     //     match self {
//     //         MatrixN::Matrix4(m) => MatrixN::Matrix4(m.transpose()),
//     //         _ => panic!("Transpose only implemented for 4x4 matrices"),
//     //     }
//     // }

//     // /// Matrix multiplication (4x4 only)
//     // pub fn multiply(&self, other: &MatrixN) -> MatrixN {
//     //     match (self, other) {
//     //         (MatrixN::Matrix4(a), MatrixN::Matrix4(b)) =>
//     // MatrixN::Matrix4(a.clone() * b.clone()),         _ => panic!("Matrix
//     // multiplication only implemented for 4x4 matrices"),     }
//     // }
// }

/// Helper function to parse cucumber table data into a Vec<f64>
pub fn parse_matrix_table(table_data: &cucumber::gherkin::Table) -> Vec<f64> {
    table_data
        .rows
        .iter()
        .flatten()
        .filter_map(|item| item.parse().ok())
        .collect()
}

// // /// Helper function to assert matrix equality with epsilon tolerance
// // pub fn assert_matrix_elements_equal(actual: &MatrixN, expected: &MatrixN,
// // name: &str) {     const EPSILON: f64 = 1e-10;
// //     assert_matrix_elements_equal_with_epsilon(actual, expected, name,
// // EPSILON); }

// // /// Helper function to assert matrix equality with custom epsilon
// // pub fn assert_matrix_elements_equal_with_epsilon(
// //     actual: &MatrixN,
// //     expected: &MatrixN,
// //     name: &str,
// //     epsilon: f64,
// // ) {
// //     assert_eq!(
// //         actual.dimensions(),
// //         expected.dimensions(),
// //         "Matrix dimensions mismatch for {}",
// //         name
// //     );

// //     match (actual, expected) {
// //         (MatrixN::Matrix2(a), MatrixN::Matrix2(e)) => {
// //             assert!(
// //                 a == e,
// //                 "Matrix2 {} elements not approximately equal (epsilon:
// {})", //                 name,
// //                 epsilon
// //             );
// //         },
// //         (MatrixN::Matrix3(a), MatrixN::Matrix3(e)) => {
// //             assert!(
// //                 a == e,
// //                 "Matrix3 {} elements not approximately equal (epsilon:
// {})", //                 name,
// //                 epsilon
// //             );
// //         },
// //         (MatrixN::Matrix4(a), MatrixN::Matrix4(e)) => {
// //             assert!(
// //                 a == e,
// //                 "Matrix4 {} elements not approximately equal (epsilon:
// {})", //                 name,
// //                 epsilon
// //             );
// //         },
// //         _ => panic!("Matrix type mismatch in comparison for {}", name),
// //     }
// // }
