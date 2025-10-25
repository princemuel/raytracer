use cucumber::{given, then, when};
use raytracer::prelude::*;
use raytracer::primitives::Tuple4;

use crate::support::world::TestWorld;

// ===============================================================================
// Given Steps - Tuple, Vector, Point, Color Construction
// ===============================================================================
#[given(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn given_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.insert(&name, tuple(x, y, z, w));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, Tuple4::from(point(x, y, z)));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, Tuple4::from(vector(x, y, z)));
}

#[given(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn given_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    world.insert(&name, color(r, g, b));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← vector\(√2/2, √2/2, 0\)$")]
fn given_vector_sqrt2_half(world: &mut TestWorld, name: String) {
    let val = 2_f64.sqrt() / 2.0;
    world.insert(&name, Tuple4::from(vector(val, val, 0)));
}

// ===============================================================================
// When Steps -
// ===============================================================================
#[when(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\)$")]
fn when_normalize(world: &mut TestWorld, key: String, name: String) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let value = v.try_normalize().unwrap();
    world.insert(&key, Tuple4::from(value));
}

#[when(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← reflect\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\)$"
)]
fn when_reflect(world: &mut TestWorld, key: String, name: String, normal: String) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let n = world.get::<Tuple4>(&normal).unwrap();
    let n = Vec3::try_from(n).unwrap();

    let value = v.reflect(n);
    world.insert(&key, Tuple4::from(value));
}

// ===============================================================================
// Then Steps - Tuple Properties
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.x = ([-+]?\d*\.?\d+)$")]
fn then_x_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.x(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.y = ([-+]?\d*\.?\d+)$")]
fn then_y_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.y(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.z = ([-+]?\d*\.?\d+)$")]
fn then_z_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.z(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.w = ([-+]?\d*\.?\d+)$")]
fn then_w_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(is_equal(t1.w(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.red = ([-+]?\d*\.?\d+)$")]
fn then_red_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.r(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.green = ([-+]?\d*\.?\d+)$")]
fn then_green_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.g(), expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.blue = ([-+]?\d*\.?\d+)$")]
fn then_blue_equals(world: &mut TestWorld, name: String, expected: f64) {
    let c1 = world.get::<Color>(&name).unwrap();
    assert!(is_equal(c1.b(), expected));
}

// ===============================================================================
// Then Steps - Tuple Type Checking
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a point$")]
fn then_is_point(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(t1.is_point());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a point$")]
fn then_is_not_point(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(!t1.is_point());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a vector$")]
fn then_is_vector(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(t1.is_vector());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a vector$")]
fn then_is_not_vector(world: &mut TestWorld, name: String) {
    let t1 = world.get::<Tuple4>(&name).unwrap();
    assert!(!t1.is_vector());
}

// ===============================================================================
// Then Steps - Equality
// ===============================================================================
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = tuple(x, y, z, w);
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = Tuple4::from(point(x, y, z));
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let actual = world.get::<Tuple4>(&name).unwrap();
    let expected = Tuple4::from(vector(x, y, z));
    assert_eq!(*actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    let actual = world.get::<Color>(&name).unwrap();
    let expected = color(r, g, b);
    assert_eq!(*actual, expected);
}

// ===============================================================================
// Then Steps - Operations
// ===============================================================================
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \+ ([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_add_equals_tuple(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64, w: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a + b;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \+ ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_add_equals_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a + color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a - b;
    let expected = Tuple4::from(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_point(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let a = world.get::<Tuple4>(&a).unwrap();
    let b = world.get::<Tuple4>(&b).unwrap();

    let actual = a - b;
    let expected = Tuple4::from(point(x, y, z));
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a - color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^-([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_neg_equals_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = -t1;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([-+]?\d*\.?\d+) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_scalar_equals_tuple(
    world: &mut TestWorld,
    name: String,
    scalar: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = t1 * scalar;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([-+]?\d*\.?\d+) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_scalar_equals_color(world: &mut TestWorld, name: String, scalar: f64, r: f64, g: f64, b: f64) {
    let c1 = world.get::<Color>(&name).unwrap();

    let actual = c1 * scalar;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_equals_color(world: &mut TestWorld, ca: String, cb: String, r: f64, g: f64, b: f64) {
    let color_a = world.get::<Color>(&ca).unwrap();
    let color_b = world.get::<Color>(&cb).unwrap();

    let actual = color_a * color_b;
    let expected = color(r, g, b);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) / ([-+]?\d*\.?\d+) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_div_equals_tuple(world: &mut TestWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple4>(&name).unwrap();

    let actual = t1 / scalar;
    let expected = tuple(x, y, z, w);
    assert_eq!(actual, expected);
}

// Magnitude
#[then(regex = r"^magnitude\(([a-zA-Z_][a-zA-Z0-9_]*)\) = ([-+]?\d*\.?\d+)$")]
fn then_magnitude_equals(world: &mut TestWorld, name: String, expected: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.length();
    assert_eq!(actual, expected);
}

#[then(regex = r"^magnitude\(([a-zA-Z_][a-zA-Z0-9_]*)\) = √(\d+)$")]
fn then_magnitude_equals_sqrt(world: &mut TestWorld, name: String, value: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.length();
    let expected = value.sqrt();
    assert_eq!(actual, expected);
}

// Normalize
#[then(
    regex = r"^normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_normalize_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.try_normalize();
    let expected = Some(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\) = approximately vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_normalize_approximately_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let v = world.get::<Tuple4>(&name).unwrap();
    let v = Vec3::try_from(v).unwrap();

    let actual = v.try_normalize();
    let expected = Some(vector(x, y, z));
    assert_eq!(actual, expected);
}

#[then(regex = r"^dot\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\) = ([-+]?\d*\.?\d+)$")]
fn then_dot_equals(world: &mut TestWorld, a: String, b: String, expected: f64) {
    let v1 = world.get::<Tuple4>(&a).unwrap();
    let v1 = Vec3::try_from(v1).unwrap();
    let v2 = world.get::<Tuple4>(&b).unwrap();
    let v2 = Vec3::try_from(v2).unwrap();

    let actual = v1.dot(v2);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^cross\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_cross_equals_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let v1 = world.get::<Tuple4>(&a).unwrap();
    let v1 = Vec3::try_from(v1).unwrap();
    let v2 = world.get::<Tuple4>(&b).unwrap();
    let v2 = Vec3::try_from(v2).unwrap();

    let actual = v1.cross(v2);
    let expected = vector(x, y, z);
    assert_eq!(actual, expected);
}
