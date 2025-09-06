use cucumber::{given, then};
use raytracer::primitives::tuple::{Tuple, point, vector};

use crate::support::world::TestWorld;

// ------------------------------------------------------------------------------
#[given(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
async fn given_a_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.insert(&name, Tuple::new(x, y, z, w));
}
#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_a_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, point(x, y, z));
}
#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_a_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, vector(x, y, z));
}

// ------------------------------------------------------------------------------
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.x = ([^$]+)$")]
fn then_x_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.x() == expected, "Expected x={}, got x={}", expected, t.x());
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.y = ([^$]+)$")]
fn then_y_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.y() == expected, "Expected y={}, got y={}", expected, t.y());
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.z = ([^$]+)$")]
fn then_z_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.z() == expected, "Expected z={}, got z={}", expected, t.z());
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.w = ([^$]+)$")]
fn then_w_equals(world: &mut TestWorld, name: String, expected: f64) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.w() == expected, "Expected w={}, got w={}", expected, t.w());
}

// ------------------------------------------------------------------------------
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a point$")]
fn then_is_point(world: &mut TestWorld, name: String) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.is_point(), "Expected {} to be a point", name);
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a point$")]
fn then_is_not_point(world: &mut TestWorld, name: String) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(!t.is_point(), "Expected {} to not be a point", name);
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a vector$")]
fn then_is_vector(world: &mut TestWorld, name: String) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(t.is_vector(), "Expected {} to be a vector", name);
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a vector$")]
fn then_is_not_vector(world: &mut TestWorld, name: String) {
    let t = world.get::<Tuple>(&name).unwrap();
    assert!(!t.is_vector(), "Expected {} to not be a vector", name);
}

// ------------------------------------------------------------------------------
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_variable_equals_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t = world.get::<Tuple>(&name).unwrap();
    let expected = Tuple::new(x, y, z, w);
    assert!(t == &expected, "Expected {:?}, got {:?}", expected, t);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \+ ([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_add_equals_tuple(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64, w: f64) {
    let t1 = world.get::<Tuple>(&a).unwrap();
    let t2 = world.get::<Tuple>(&b).unwrap();

    let actual = *t1 + *t2;
    let expected = Tuple::new(x, y, z, w);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \- ([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_sub_equals_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let t1 = world.get::<Tuple>(&a).unwrap();
    let t2 = world.get::<Tuple>(&b).unwrap();

    let actual = *t1 - *t2;
    let expected = vector(x, y, z);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \- ([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_sub_equals_point(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let t1 = world.get::<Tuple>(&a).unwrap();
    let t2 = world.get::<Tuple>(&b).unwrap();

    let actual = *t1 - *t2;
    let expected = point(x, y, z);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}

#[then(
    regex = r"^-([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_negate_var_equals_tuple(world: &mut TestWorld, a: String, x: f64, y: f64, z: f64, w: f64) {
    let t = *(world.get::<Tuple>(&a).unwrap());

    let actual = -t;
    let expected = Tuple::new(x, y, z, w);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([-+]?\d*\.?\d+) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_var_times_value_equals_tuple(
    world: &mut TestWorld,
    a: String,
    v: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t = *(world.get::<Tuple>(&a).unwrap());

    let actual = t * v;
    let expected = Tuple::new(x, y, z, w);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \/ ([-+]?\d*\.?\d+) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn then_var_divide_value_equals_tuple(
    world: &mut TestWorld,
    a: String,
    v: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t = *(world.get::<Tuple>(&a).unwrap());

    let actual = t / v;
    let expected = Tuple::new(x, y, z, w);

    assert!(actual == expected, "Expected {:?}, got {:?}", expected, actual);
}
