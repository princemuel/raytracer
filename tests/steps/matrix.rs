use cucumber::gherkin::Step;
use cucumber::{given, then};
use raytracer::prelude::*;

use crate::support::helpers::matrix::parse_matrix_table;
use crate::support::world::TestWorld;

// ===============================================================================
// Given Steps - Matrix Construction
// ===============================================================================
#[given(regex = r"^the following ([-+]?\d+)x([-+]?\d+) matrix ([a-zA-Z_][a-zA-Z0-9_]*):$")]
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

#[given(regex = r"^the following matrix ([a-zA-Z_][a-zA-Z0-9_]*):$")]
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

// ===============================================================================
// Then Steps - Matrix Element Access
// ===============================================================================
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

// ===============================================================================
// Then Steps - Matrix Equality
// ===============================================================================

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

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) is the following ([-+]?\d+)x([-+]?\d+) matrix:$"
)]
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
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* identity_matrix = ([a-zA-Z_][a-zA-Z0-9_]*)$")]
fn then_matrix_product_identity_should_be(world: &mut TestWorld, key: String, key2: String) {
    if let (Some(matrix), Some(expected)) = (world.get::<Mat4>(&key), world.get::<Mat4>(&key2)) {
        let actual = matrix * Mat4::IDENTITY;
        return assert_eq!(actual, *expected,);
    }

    if let (Some(matrix), Some(expected)) = (world.get::<Mat3>(&key), world.get::<Mat3>(&key2)) {
        let actual = matrix * Mat3::IDENTITY;
        return assert_eq!(actual, *expected,);
    }

    if let (Some(matrix), Some(expected)) = (world.get::<Mat2>(&key), world.get::<Mat2>(&key2)) {
        let actual = matrix * Mat2::IDENTITY;
        assert_eq!(actual, *expected,)
    }
}

// A * b = tuple(18, 24, 33, 1)
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([^,]+), ([^,]+), ([^,]+), ([^)]+)\)$"
)]
fn then_matrix_identity_mul_tuple_should_be(
    world: &mut TestWorld,
    key: String,
    key2: String,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let expected = tuple(x, y, z, w);
    let tup = world.get::<Tuple4>(&key2).unwrap();

    let actual = {
        let get = |world: &TestWorld, key: &String| -> Option<Tuple4> {
            world
                .get::<Mat2>(key)
                .map(|matrix| matrix * tup)
                .or_else(|| world.get::<Mat3>(key).map(|matrix| matrix * tup))
                .or_else(|| world.get::<Mat4>(key).map(|matrix| matrix * tup))
        };

        get(world, &key).unwrap_or(Tuple4::ZERO)
    };

    assert_eq!(actual, expected);
}
