use cucumber::gherkin::Step;
use cucumber::{given, then};
use raytracer::prelude::*;

use crate::support::helpers::matrix::{MatrixN, parse_matrix_table};
use crate::support::world::TestWorld;

// ===============================================================================
// Given Steps - Matrix Construction
// ===============================================================================

#[given(regex = r"^the following ([-+]?\d+)x([-+]?\d+) matrix ([a-zA-Z_][a-zA-Z0-9_]*):$")]
fn given_matrix_w_dimensions(world: &mut TestWorld, step: &Step, rows: usize, cols: usize, name: String) {
    let data = step.table.clone().expect("Matrix data table is required");
    let buffer = parse_matrix_table(&data);

    assert_eq!(
        buffer.len(),
        rows * cols,
        "Expected {rows}x{cols} matrix ({} elements), got {}",
        rows * cols,
        buffer.len()
    );

    world.insert(&name, MatrixN::new(rows, cols, &buffer));
}

#[given(regex = r"^the following matrix ([a-zA-Z_][a-zA-Z0-9_]*):$")]
fn given_matrix_wo_dimensions(world: &mut TestWorld, step: &Step, name: String) {
    let data = step.table.clone().expect("Matrix data table is required");
    let rows = data.rows.len();

    let cols = if rows > 0 { data.rows[0].len() } else { 0 };

    // Verify it's a square matrix
    for (idx, row) in data.rows.iter().enumerate() {
        assert_eq!(
            row.len(),
            cols,
            "Row {idx} has {} columns, expected {}",
            row.len(),
            cols
        );
    }

    assert!(
        rows == cols,
        "Non-square matrices must specify dimensions explicitly"
    );

    let buffer = parse_matrix_table(&data);
    world.insert(&name, MatrixN::new(rows, cols, &buffer));
}

// ===============================================================================
// Then Steps - Matrix Element Access
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\[([-+]?\d+),([-+]?\d+)\] = ([-+]?\d*\.?\d+)$")]
fn then_value_at_idx_equals(world: &mut TestWorld, name: String, row: usize, col: usize, expected: f64) {
    let matrix = world
        .get::<MatrixN>(&name)
        .unwrap_or_else(|| panic!("Matrix '{name}' not found",));

    let actual = match matrix {
        MatrixN::Matrix2(m) => {
            assert!(
                row < 2 && col < 2,
                "Index [{row},{col}] out of bounds for 2x2 matrix"
            );
            m[(row, col)]
        },
        MatrixN::Matrix3(m) => {
            assert!(
                row < 3 && col < 3,
                "Index [{row},{col}] out of bounds for 3x3 matrix"
            );
            m[(row, col)]
        },
        MatrixN::Matrix4(m) => {
            assert!(
                row < 4 && col < 4,
                "Index [{row},{col}] out of bounds for 4x4 matrix"
            );
            m[(row, col)]
        },
    };

    assert!(
        is_equal(actual, expected),
        "Expected {name}[{row},{col}] = {expected}, but got {actual}",
    );
}

// ===============================================================================
// Then Steps - Matrix Equality
// ===============================================================================

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrix_equals(world: &mut TestWorld, a: String, b: String) {
    let matrix_a = world
        .get::<MatrixN>(&a)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", a));
    let matrix_b = world
        .get::<MatrixN>(&b)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", b));

    assert_eq!(matrix_a, matrix_b, "Matrices {} and {} are not equal", a, b);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) != ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrix_not_equals(world: &mut TestWorld, a: String, b: String) {
    let matrix_a = world
        .get::<MatrixN>(&a)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", a));
    let matrix_b = world
        .get::<MatrixN>(&b)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", b));

    assert_ne!(matrix_a, matrix_b, "Matrices {} and {} should not be equal", a, b);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) is the following ([-+]?\d+)x([-+]?\d+) matrix:$"
)]
fn then_mul_equals_matrix(
    world: &mut TestWorld,
    step: &Step,
    a: String,
    b: String,
    rows: usize,
    cols: usize,
) {
    let matrix_a = world
        .get::<MatrixN>(&a)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", a));
    let matrix_b = world
        .get::<MatrixN>(&b)
        .unwrap_or_else(|| panic!("Matrix '{}' not found", b));

    let data = step.table.clone().expect("Matrix data table is required");
    let buffer = parse_matrix_table(&data);

    assert_eq!(
        buffer.len(),
        rows * cols,
        "Expected {rows}x{cols} matrix ({} elements), got {}",
        rows * cols,
        buffer.len()
    );

    if let (MatrixN::Matrix4(a), MatrixN::Matrix4(b)) = (matrix_a, matrix_b) {
        let expected = Matrix4::from_row_major(&buffer);
        let actual = a.clone() * b.clone();

        dbg!((actual.clone(), expected.clone()));

        assert_eq!(actual, expected);
    }
}
