#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod common;

use cucumber::{World as _, given, then, when};
use raytracer::prelude::*;

use crate::common::TestWorld;

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/tuples.feature")
        .await;
}

// ====================================================================
// Given Steps - Tuple, Vector, Point, Color Construction
// ====================================================================
#[given(expr = r"{word} ← tuple\({float}, {float}, {float}, {float}\)")]
fn given_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.insert(&name, tuple(x, y, z, w));
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn given_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, Tuple4::from(point(x, y, z)));
}

#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn given_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, Tuple4::from(vector(x, y, z)));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    world.insert(&name, color(r, g, b));
}

#[given(expr = r"{word} ← vector\(√{float}\/{float}, √{float}\/{float}, {float}\)")]
fn given_vector_sqrt2_half(world: &mut TestWorld, name: String, a: f64, b: f64, c: f64, d: f64, z: f64) {
    let x = a.sqrt() / b;
    let y = c.sqrt() / d;
    world.insert(&name, Tuple4::from(vector(x, y, z)));
}

// ====================================================================
// When Steps -
// ====================================================================
#[when(expr = r"{word} ← normalize\({word}\)")]
fn when_normalize(world: &mut TestWorld, key: String, name: String) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let value = v.try_normalize().unwrap();
    world.insert(&key, Tuple4::from(value));
}

#[when(expr = r"{word} ← reflect\({word}, {word}\)")]
fn when_reflect(world: &mut TestWorld, key: String, name: String, normal: String) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let n = world.get::<Tuple4>(&normal).unwrap();
    let n = Vec3::try_from(n).unwrap();

    let value = v.reflect(n);
    world.insert(&key, Tuple4::from(value));
}

// ====================================================================
// Then Steps - Tuple Properties
// ====================================================================
#[then(expr = "{word}.x = {float}")]
fn then_x_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.x(), expected));
}

#[then(expr = "{word}.y = {float}")]
fn then_y_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.y(), expected));
}

#[then(expr = "{word}.z = {float}")]
fn then_z_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.z(), expected));
}

#[then(expr = "{word}.w = {float}")]
fn then_w_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.w(), expected));
}

#[then(expr = "{word}.red = {float}")]
fn then_red_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.r(), expected));
}

#[then(expr = "{word}.green = {float}")]
fn then_green_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.g(), expected));
}

#[then(expr = "{word}.blue = {float}")]
fn then_blue_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.b(), expected));
}

// ====================================================================
// Then Steps - Tuple Type Checking
// ====================================================================
#[then(expr = "{word} is a point")]
fn then_is_point(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(t1.is_point());
}

#[then(expr = "{word} is not a point")]
fn then_is_not_point(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(!t1.is_point());
}

#[then(expr = "{word} is a vector")]
fn then_is_vector(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(t1.is_vector());
}

#[then(expr = "{word} is not a vector")]
fn then_is_not_vector(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(!t1.is_vector());
}

// ====================================================================
// Then Steps - Equality
// ====================================================================
// ===============================================================================
// Then Steps - Equality
// ===============================================================================
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = tuple(x, y, z, w);
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = Tuple4::from(point(x, y, z));
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = Tuple4::from(vector(x, y, z));
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    let actual = world.get::<Color>(&name).unwrap();
    let expected = color(r, g, b);
    assert_eq!(*actual, expected);
}

// ====================================================================
// Then Steps - Operations
// ====================================================================
#[then(expr = r"{word} + {word} = tuple\({float}, {float}, {float}, {float}\)")]
fn then_add_tuple(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64, w: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a + b;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} + {word} = color\({float}, {float}, {float}\)")]
fn then_add_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a + color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} - {word} = vector\({float}, {float}, {float}\)")]
fn then_sub_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a - b;
    let expected = Tuple4::from(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} - {word} = point\({float}, {float}, {float}\)")]
fn then_sub_point(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a - b;
    let expected = Tuple4::from(point(x, y, z));
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} - {word} = color\({float}, {float}, {float}\)")]
fn then_sub_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a - color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^-([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_neg_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = -t1;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} * {float} = tuple\({float}, {float}, {float}, {float}\)")]
fn then_mul_scalar_tuple(world: &mut TestWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = t1 * scalar;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} * {float} = color\({float}, {float}, {float}\)")]
fn then_mul_scalar_color(world: &mut TestWorld, name: String, scalar: f64, r: f64, g: f64, b: f64) {
    let c1 = world.get::<Color>(&name).unwrap();

    let actual = c1 * scalar;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a * color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(expr = r"{word} \/ {float} = tuple\({float}, {float}, {float}, {float}\)")]
fn then_div_tuple(world: &mut TestWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = t1 / scalar;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

// Magnitude
#[then(expr = r"magnitude\({word}\) = {float}")]
fn then_magnitude(world: &mut TestWorld, name: String, expected: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.length();
    assert_eq!(actual, expected);
}

#[then(expr = r"magnitude\({word}\) = √{float}")]
fn then_magnitude_sqrt(world: &mut TestWorld, name: String, value: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.length();
    let expected = value.sqrt();
    assert_eq!(actual, expected);
}

// Normalize
#[then(expr = r"normalize\({word}\) = vector\({float}, {float}, {float}\)")]
fn then_normalize_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.try_normalize();
    let expected = Some(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(expr = r"normalize\({word}\) = approximately vector\({float}, {float}, {float}\)")]
fn then_normalize_approximately_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.try_normalize();
    let expected = Some(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(expr = r"dot\({word}, {word}\) = {float}")]
fn then_dot_vector(world: &mut TestWorld, a: String, b: String, expected: f64) {
    let v1 = world.get::<Tuple4>(&a).unwrap();
    let v1 = Vec3::try_from(v1).unwrap();
    let v2 = world.get::<Tuple4>(&b).unwrap();
    let v2 = Vec3::try_from(v2).unwrap();

    let actual = v1.dot(v2);
    assert_eq!(actual, expected);
}

#[then(expr = r"cross\({word}, {word}\) = vector\({float}, {float}, {float}\)")]
fn then_cross_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let v1 = world.get::<Tuple4>(&a).unwrap();
    let v1 = Vec3::try_from(v1).unwrap();
    let v2 = world.get::<Tuple4>(&b).unwrap();
    let v2 = Vec3::try_from(v2).unwrap();

    let actual = v1.cross(v2);
    let expected = vector(x, y, z);
    assert_eq!(actual, expected);
}
