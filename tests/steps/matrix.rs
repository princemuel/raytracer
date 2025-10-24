use cucumber::gherkin::Step;
use cucumber::{given, then};
use raytracer::prelude::*;

use crate::support::helpers::matrix::parse_matrix_table;
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

    world.insert(&name, Matrix::new(rows, cols, &buffer));
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
    world.insert(&name, Matrix::new(rows, cols, &buffer));
}

// ===============================================================================
// Then Steps - Matrix Element Access
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\[([-+]?\d+),([-+]?\d+)\] = ([-+]?\d*\.?\d+)$")]
fn then_value_at_idx_equals(world: &mut TestWorld, name: String, row: usize, col: usize, expected: f64) {
    let matrix = world
        .get::<Matrix>(&name)
        .unwrap_or_else(|| panic!("Matrix '{name}' not found",));

    let actual = matrix[row][col];

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
    let matrix_a = world.get::<Matrix>(&a).unwrap();
    let matrix_b = world.get::<Matrix>(&b).unwrap();
    assert_eq!(matrix_a, matrix_b, "Matrices {a} and {b} are not equal");
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) != ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrix_not_equals(world: &mut TestWorld, a: String, b: String) {
    let matrix_a = world.get::<Matrix>(&a).unwrap();
    let matrix_b = world.get::<Matrix>(&b).unwrap();
    assert_ne!(matrix_a, matrix_b, "Matrices {a} and {b} should not be equal");
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
    let matrix_a = world.get::<Matrix>(&a).unwrap();
    let matrix_b = world.get::<Matrix>(&b).unwrap();

    let data = step.table.clone().expect("Matrix data table is required");
    let buffer = parse_matrix_table(&data);

    assert_eq!(
        buffer.len(),
        rows * cols,
        "Expected {rows}x{cols} matrix ({} elements), got {}",
        rows * cols,
        buffer.len()
    );

    let actual = matrix_a.clone() * matrix_b.clone();
    let expected = Matrix::new(rows, cols, &buffer);

    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_equals_tuple(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple4 = *world.get::<Tuple4>(&b).unwrap();
    let matrix = world.get::<Matrix>(&a).unwrap();

    let actual = matrix.clone() * tuple4;
    let expected = tuple(x, y, z, w);

    assert_eq!(actual, expected);
}

#[then(
    regex = r"^submatrix\(([a-zA-Z_][a-zA-Z0-9_]*), ([-+]?\d+), ([-+]?\d+)\) is the following ([-+]?\d+)x([-+]?\d+) matrix:$"
)]
fn then_submatrix_equals_matrix(
    world: &mut TestWorld,
    step: &Step,
    name: String,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) {
    let matrix = world.get::<Matrix>(&name).unwrap();

    let data = step.table.clone().expect("Matrix data table is required");
    let buffer = parse_matrix_table(&data);

    assert_eq!(
        buffer.len(),
        rows * cols,
        "Expected {rows}x{cols} matrix ({} elements), got {}",
        rows * cols,
        buffer.len()
    );

    let actual = matrix.submatrix(row, col);
    let expected = Matrix::new(rows, cols, &buffer);

    assert_eq!(actual, expected);
}
