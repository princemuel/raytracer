#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod common;

use cucumber::gherkin::Step;
use cucumber::{World as _, given, then};
use raytracer::prelude::*;

use crate::common::{TestWorld, parse_matrix_table};

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/matrices.feature")
        .await;
}

// ====================================================================
// Given Steps - Construction
// ====================================================================
#[given(expr = r"the following {int}x{int} matrix {word}:")]
fn given_matrix_with_dimensions(world: &mut TestWorld, step: &Step, rows: usize, cols: usize, key: String) {
    let table = step.table.clone().expect("Matrix data table is required");
    if rows != cols {
        panic!("Expected a square matrix, got {rows}x{cols}");
    }

    let buffer = parse_matrix_table(&table);
    match rows * cols {
        4 => world.insert(&key, Mat2::try_from(&buffer[..]).expect("Invalid 2x2 matrix")),
        9 => world.insert(&key, Mat3::try_from(&buffer[..]).expect("Invalid 3x3 matrix")),
        16 => world.insert(&key, Mat4::try_from(&buffer[..]).expect("Invalid 4x4 matrix")),
        n => panic!("Unsupported matrix size {rows}x{cols} = {n} elements"),
    }
}

#[given(expr = "the following matrix {word}:")]
fn given_matrix(world: &mut TestWorld, step: &Step, key: String) {
    let table = step.table.clone().expect("Matrix data table is required");

    let rows = table.rows.len();
    let cols = table.rows.first().map_or_else(|| 0, |row| row.len());
    if rows != cols {
        panic!("Expected a square matrix, got {rows}x{cols}");
    }

    let buffer = parse_matrix_table(&table);
    match rows * cols {
        4 => world.insert(&key, Mat2::try_from(&buffer[..]).expect("Invalid 2x2 matrix")),
        9 => world.insert(&key, Mat3::try_from(&buffer[..]).expect("Invalid 3x3 matrix")),
        16 => world.insert(&key, Mat4::try_from(&buffer[..]).expect("Invalid 4x4 matrix")),
        n => panic!("Unsupported matrix size {rows}x{cols} = {n} elements"),
    }
}

#[given(expr = "{word} ← transpose(identity_matrix)")]
fn given_transposed_matrix(world: &mut TestWorld, key: String) {
    world.insert(&key, Mat4::IDENTITY.transpose());
}

#[given(expr = r"{word} ← tuple\({float}, {float}, {float}, {float}\)")]
fn given_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.insert(&name, tuple(x, y, z, w));
}

// ====================================================================
// Then Steps - Element Access
// ====================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\[([-+]?\d+),([-+]?\d+)\] = ([-+]?\d*\.?\d+)$")]
fn then_element_at_idx_is(world: &mut TestWorld, key: String, row: usize, col: usize, expected: f64) {
    let actual = {
        let get = |world: &TestWorld, key: &String| -> Option<f64> {
            world
                .get::<Mat2>(key)
                .map(|m| m[(row, col)])
                .or_else(|| world.get::<Mat3>(key).map(|m| m[(row, col)]))
                .or_else(|| world.get::<Mat4>(key).map(|m| m[(row, col)]))
        };

        get(world, &key).unwrap_or(0.0)
    };

    assert!(
        is_equal(actual, expected),
        "Expected {key}[{row},{col}] = {expected}, but got {actual}",
    );
}

// ====================================================================
// Then Steps - Equality
// ====================================================================

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrices_should_be_equal(world: &mut TestWorld, a: String, b: String) {
    if let (Some(ma), Some(mb)) = (world.get::<Mat4>(&a), world.get::<Mat4>(&b)) {
        assert_eq!(ma, mb);
    } else if let (Some(ma), Some(mb)) = (world.get::<Mat3>(&a), world.get::<Mat3>(&b)) {
        assert_eq!(ma, mb);
    } else if let (Some(ma), Some(mb)) = (world.get::<Mat2>(&a), world.get::<Mat2>(&b)) {
        assert_eq!(ma, mb);
    } else {
        panic!("Matrices {a} and {b} not found or size mismatch",);
    }
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) != ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrices_should_not_be_equal(world: &mut TestWorld, a: String, b: String) {
    if let (Some(ma), Some(mb)) = (world.get::<Mat4>(&a), world.get::<Mat4>(&b)) {
        return assert_ne!(ma, mb);
    }

    if let (Some(ma), Some(mb)) = (world.get::<Mat3>(&a), world.get::<Mat3>(&b)) {
        return assert_ne!(ma, mb);
    }

    if let (Some(ma), Some(mb)) = (world.get::<Mat2>(&a), world.get::<Mat2>(&b)) {
        assert_ne!(ma, mb);
    }
}

// ====================================================================
// Then Steps - Operations
// ====================================================================
#[then(expr = "{word} * {word} is the following {int}x{int} matrix:")]
fn then_matrix_product_should_be(
    world: &mut TestWorld,
    step: &Step,
    a: String,
    b: String,
    rows: usize,
    cols: usize,
) {
    let table = step.table.clone().expect("Matrix data table is required");
    let inferred_rows = table.rows.len();
    let inferred_cols = table.rows.first().map_or_else(|| 0, |row| row.len());

    let all = [rows, cols, inferred_rows, inferred_cols];
    assert!(all.iter().all(|&x| x == all[0]), "Matrix dimensions mismatch");

    let buffer = parse_matrix_table(&table);
    let size = rows * cols;
    match size {
        4 => {
            // 2x2
            let (Some(ma), Some(mb)) = (world.get::<Mat2>(&a), world.get::<Mat2>(&b)) else {
                panic!("Could not retrieve 2x2 matrices {} and {}", a, b);
            };
            let expected = Mat2::try_from(&buffer[..]).expect("Invalid 2x2 matrix");
            let actual = ma * mb;
            assert_eq!(actual, expected);
        },
        9 => {
            // 3x3
            let (Some(ma), Some(mb)) = (world.get::<Mat3>(&a), world.get::<Mat3>(&b)) else {
                panic!("Could not retrieve 3x3 matrices {} and {}", a, b);
            };
            let expected = Mat3::try_from(&buffer[..]).expect("Invalid 3x3 matrix");
            let actual = ma * mb;
            assert_eq!(actual, expected);
        },
        16 => {
            // 4x4
            let (Some(ma), Some(mb)) = (world.get::<Mat4>(&a), world.get::<Mat4>(&b)) else {
                panic!("Could not retrieve 4x4 matrices {} and {}", a, b);
            };
            let expected = Mat4::try_from(&buffer[..]).expect("Invalid 4x4 matrix");
            let actual = ma * mb;
            assert_eq!(actual, expected);
        },
        _ => panic!(
            "Unsupported square matrix size: {}x{}. Only 2x2, 3x3, and 4x4 are supported.",
            rows, cols
        ),
    }
}
// A * identity_matrix = A
#[then(expr = "{word} * identity_matrix = A")]
fn then_matrix_product_identity_should_be(world: &mut TestWorld, key: String) {
    if let Some(matrix) = world.get::<Mat4>(&key) {
        let expected = *matrix;
        let actual = matrix * Mat4::IDENTITY;
        return assert_eq!(actual, expected);
    }

    if let Some(matrix) = world.get::<Mat3>(&key) {
        let expected = *matrix;
        let actual = matrix * Mat3::IDENTITY;
        return assert_eq!(actual, expected);
    }

    if let Some(matrix) = world.get::<Mat2>(&key) {
        let expected = *matrix;
        let actual = matrix * Mat2::IDENTITY;
        assert_eq!(actual, expected)
    }
}

// A * b = tuple(18, 24, 33, 1)
#[then(expr = r"{word} * {word} = tuple\({float}, {float}, {float}, {float}\)")]
fn then_matrix_identity_mul_tuple_should_be(
    world: &mut TestWorld,
    a: String,
    b: String,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let expected = tuple(x, y, z, w);
    let tup = world.get::<Tuple4>(&b).unwrap();

    let actual = {
        let get = |world: &TestWorld, key: &String| -> Option<Tuple4> {
            world
                .get::<Mat2>(key)
                .map(|matrix| matrix * tup)
                .or_else(|| world.get::<Mat3>(key).map(|matrix| matrix * tup))
                .or_else(|| world.get::<Mat4>(key).map(|matrix| matrix * tup))
        };

        get(world, &a).unwrap_or(Tuple4::ZERO)
    };

    assert_eq!(actual, expected);
}

// identity_matrix * a = a
#[then(expr = "identity_matrix * {word} = a")]
fn then_identity_matrix_mul_tuple_should_be(world: &mut TestWorld, key: String) {
    let expected = *world.get::<Tuple4>(&key).unwrap();
    let actual = Mat4::IDENTITY * expected;

    assert_eq!(actual, expected);
}

#[then(expr = r"transpose\({word}\) is the following matrix:")]
fn transposed_identity_matrix_is(world: &mut TestWorld, step: &Step, key: String) {
    let table = step.table.clone().expect("Matrix data table is required");
    let buffer = parse_matrix_table(&table);

    if let Some(matrix) = world.get::<Mat4>(&key) {
        let expected = Matrix::try_from(&buffer[..]).unwrap();
        let actual = matrix.transpose();
        return assert_eq!(actual, expected);
    }

    if let Some(matrix) = world.get::<Mat3>(&key) {
        let expected = Matrix::try_from(&buffer[..]).unwrap();
        let actual = matrix.transpose();
        return assert_eq!(actual, expected);
    }

    if let Some(matrix) = world.get::<Mat2>(&key) {
        let expected = Matrix::try_from(&buffer[..]).unwrap();
        let actual = matrix.transpose();
        assert_eq!(actual, expected)
    }
}

#[then(expr = "{word} = identity_matrix")]
fn is_transposed_identity_matrix(world: &mut TestWorld, key: String) {
    if let Some(matrix) = world.get::<Mat4>(&key) {
        let expected = Mat4::IDENTITY;
        let actual = *matrix;
        return assert_eq!(actual, expected);
    }
    if let Some(matrix) = world.get::<Mat3>(&key) {
        let expected = Mat3::IDENTITY;
        let actual = *matrix;
        return assert_eq!(actual, expected);
    }
    if let Some(matrix) = world.get::<Mat2>(&key) {
        let expected = Mat2::IDENTITY;
        let actual = *matrix;
        assert_eq!(actual, expected);
    }
}
