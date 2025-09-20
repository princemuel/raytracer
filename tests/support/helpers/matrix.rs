use raytracer::prelude::*;

#[derive(Debug, PartialEq)]
pub enum MatrixN {
    Matrix2(Matrix2),
    Matrix3(Matrix3),
    Matrix4(Matrix4),
}

impl MatrixN {
    pub fn new(rows: usize, cols: usize, buffer: &[f64]) -> Self {
        match (rows, cols) {
            (2, 2) => {
                assert_eq!(buffer.len(), 4, "Buffer size mismatch for 2x2 matrix");
                Self::Matrix2(Matrix2::from(buffer))
            },
            (3, 3) => {
                assert_eq!(buffer.len(), 9, "Buffer size mismatch for 3x3 matrix");
                Self::Matrix3(Matrix3::from(buffer))
            },
            (4, 4) => {
                assert_eq!(buffer.len(), 16, "Buffer size mismatch for 4x4 matrix");
                Self::Matrix4(Matrix4::from(buffer))
            },
            _ => panic!("Unsupported matrix dimensions: {rows}x{cols}"),
        }
    }

    pub fn _dimensions(&self) -> (usize, usize) {
        match self {
            Self::Matrix2(_) => (2, 2),
            Self::Matrix3(_) => (3, 3),
            Self::Matrix4(_) => (4, 4),
        }
    }

    pub fn _size(&self) -> usize {
        match self {
            Self::Matrix2(_) => 4,
            Self::Matrix3(_) => 9,
            Self::Matrix4(_) => 16,
        }
    }
}

// ===============================================================================
// Helper Functions
// ===============================================================================

pub fn parse_matrix_table(table_data: &cucumber::gherkin::Table) -> Vec<f64> {
    let mut buffer = Vec::with_capacity(table_data.rows.len() * table_data.rows.len());

    for line in &table_data.rows {
        for value in line {
            let trimmed = value.trim();
            let parsed = trimmed
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Invalid number format: '{trimmed}'",));
            buffer.push(parsed);
        }
    }

    buffer
}

// pub fn assert_matrix_elements_equal(actual: &MatrixN, expected: &MatrixN,
// name: &str) {     match (actual, expected) {
//         (MatrixN::Matrix2(a), MatrixN::Matrix2(e)) => {
//             for row in 0..2 {
//                 for col in 0..2 {
//                     assert!(
//                         (a[row][col] - e[row][col]).abs() < EPSILON,
//                         "Matrix {name} element [{row},{col}]: expected {},
// got {}",                         e[row][col],
//                         a[row][col]
//                     );
//                 }
//             }
//         },
//         (MatrixN::Matrix3(a), MatrixN::Matrix3(e)) => {
//             for row in 0..3 {
//                 for col in 0..3 {
//                     assert!(
//                         (a[row][col] - e[row][col]).abs() < EPSILON,
//                         "Matrix {name} element [{row},{col}]: expected {},
// got {}",                         e[row][col],
//                         a[row][col]
//                     );
//                 }
//             }
//         },
//         (MatrixN::Matrix4(a), MatrixN::Matrix4(e)) => {
//             for row in 0..4 {
//                 for col in 0..4 {
//                     assert!(
//                         (a[row][col] - e[row][col]).abs() < EPSILON,
//                         "Matrix {name} element [{row},{col}]: expected {},
// got {}",                         e[row][col],
//                         a[row][col]
//                     );
//                 }
//             }
//         },
//         _ => panic!("Matrix dimension mismatch in comparison"),
//     }
// }
