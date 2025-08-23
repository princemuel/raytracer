use cucumber::{given, then};
use raytracer::primitives::{point, tuple, vector};

use crate::support::helpers::tuple::TupleVariant;
// use crate::support::helpers::{Float64, get_tuple};
use crate::support::world::TestWorld;

// ------------------------------------------------------------------------------
#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← tuple\(([^,]+), ([^,]+), ([^,]+), ([^)]+)\)$")]
async fn given_a_tuple(world: &mut TestWorld, name: String, x: f32, y: f32, z: f32, w: f32) {
    let t = tuple(x, y, z, w);
    world.insert(&name, TupleVariant::Generic(t));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← point\(([^,]+), ([^,]+), ([^)]+)\)$")]
fn given_a_point(world: &mut TestWorld, name: String, x: f32, y: f32, z: f32) {
    let p = point(x, y, z);
    world.insert(&name, TupleVariant::Point(p));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← vector\(([^,]+), ([^,]+), ([^)]+)\)$")]
fn given_a_vector(world: &mut TestWorld, name: String, x: f32, y: f32, z: f32) {
    let v = vector(x, y, z);
    world.insert(&name, TupleVariant::Vector(v));
}

// ------------------------------------------------------------------------------
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.x = ([^$]+)$")]
fn then_x_equals(world: &mut TestWorld, name: String, expected: f32) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(
        (t.x() - expected).abs() < f32::EPSILON,
        "Expected x={}, got x={}",
        expected,
        t.x()
    );
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.y = ([^$]+)$")]
fn then_y_equals(world: &mut TestWorld, name: String, expected: f32) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(
        (t.y() - expected).abs() < f32::EPSILON,
        "Expected y={}, got y={}",
        expected,
        t.y()
    );
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.z = ([^$]+)$")]
fn then_z_equals(world: &mut TestWorld, name: String, expected: f32) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(
        (t.z() - expected).abs() < f32::EPSILON,
        "Expected z={}, got z={}",
        expected,
        t.z()
    );
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.w = ([^$]+)$")]
fn then_w_equals(world: &mut TestWorld, name: String, expected: f32) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(
        (t.w() - expected).abs() < f32::EPSILON,
        "Expected w={}, got w={}",
        expected,
        t.w()
    );
}

// ------------------------------------------------------------------------------
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a point$")]
fn then_is_point(world: &mut TestWorld, name: String) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(t.is_point(), "Expected {} to be a point", name);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a point$")]
fn then_is_not_point(world: &mut TestWorld, name: String) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(!t.is_point(), "Expected {} to not be a point", name);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a vector$")]
fn then_is_vector(world: &mut TestWorld, name: String) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(t.is_vector(), "Expected {} to be a vector", name);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a vector$")]
fn then_is_not_vector(world: &mut TestWorld, name: String) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    assert!(!t.is_vector(), "Expected {} to not be a vector", name);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([^,]+), ([^,]+), ([^,]+), ([^)]+)\)$")]
fn then_equals_tuple(world: &mut TestWorld, name: String, x: f32, y: f32, z: f32, w: f32) {
    let t = world.get::<TupleVariant>(&name).unwrap();
    let expected = tuple(x, y, z, w);
    assert!(
        t.as_generic().approx_eq(&expected, f32::EPSILON),
        "Expected {:?}, got {:?}",
        expected,
        t.as_generic()
    );
}
