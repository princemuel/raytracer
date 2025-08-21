use cucumber::{given, then};
use raytracer::math::approx::ApproxEq as _;
use raytracer::primitives::{Point, Tuple, Vector};

use crate::support::helpers::Float64;
use crate::support::world::TestWorld;

#[given(
    regex = r"^a ← tuple\((?P<x>[-0-9.]+), (?P<y>[-0-9.]+), (?P<z>[-0-9.]+), (?P<w>[-0-9.]+)\)$"
)]
async fn given_a_tuple(world: &mut TestWorld, x: Float64, y: Float64, z: Float64, w: Float64) {
    let x = *x;
    let y = *y;
    let z = *z;
    let w = *w;

    if w.approx_eq_low_precision(1.0) {
        world.insert("a", Point::new(x, y, z));
    } else if w.approx_eq_low_precision(0.0) {
        world.insert("a", Vector::new(x, y, z));
    } else {
        panic!("unsupported tuple w value: {}", w);
    }
}
#[then(expr = "a.x = {float}")]
async fn then_a_x(world: &mut TestWorld, value: f64) {
    let expected = value;
    let actual = match (world.get::<Point>("a"), world.get::<Vector>("a")) {
        (Some(p), _) => p.x(),
        (_, Some(v)) => v.x(),
        _ => panic!("no value 'a' found in world"),
    };
    assert!(
        actual.approx_eq_low_precision(expected),
        "a.y = {actual} does not match expected {expected}",
    );
}
#[then(expr = "a.y = {float}")]
async fn then_a_y(world: &mut TestWorld, value: f64) {
    let expected = value;
    let actual = match (world.get::<Point>("a"), world.get::<Vector>("a")) {
        (Some(p), _) => p.y(),
        (_, Some(v)) => v.y(),
        _ => panic!("no value 'a' found in world"),
    };
    assert!(
        actual.approx_eq_low_precision(expected),
        "a.x= {actual} does not match expected {expected}",
    );
}

#[then(expr = "a.z = {float}")]
async fn then_a_z(world: &mut TestWorld, value: f64) {
    let expected = value;
    let actual = match (world.get::<Point>("a"), world.get::<Vector>("a")) {
        (Some(p), _) => p.z(),
        (_, Some(v)) => v.z(),
        _ => panic!("no value 'a' found in world"),
    };
    assert!(
        actual.approx_eq_low_precision(expected),
        "a.z= {actual} does not match expected {expected}",
    );
}

#[then(expr = "a.w = {float}")]
async fn then_a_w(world: &mut TestWorld, value: f64) {
    let expected = value;
    let actual = match (world.get::<Point>("a"), world.get::<Vector>("a")) {
        (Some(p), _) => p.w(),
        (_, Some(v)) => v.w(),
        _ => panic!("no value 'a' found in world"),
    };
    assert!(
        actual.approx_eq_low_precision(expected),
        "a.w= {actual} does not match expected {expected}",
    );
}

#[then(expr = "a is a point")]
async fn then_a_is_point(world: &mut TestWorld) {
    assert!(
        world.get::<Point>("a").is_some(),
        "expected 'a' to be a Point"
    );
}

#[then(expr = "a is not a vector")]
async fn then_a_is_not_vector(world: &mut TestWorld) {
    assert!(
        world.get::<Vector>("a").is_none(),
        "expected 'a' to not be a Vector"
    );
}

#[then(expr = "a is not a point")]
async fn then_a_is_not_point(world: &mut TestWorld) {
    assert!(
        world.get::<Point>("a").is_none(),
        "expected 'a' to not be a Point"
    );
}

#[then(expr = "a is a vector")]
async fn then_a_is_vector(world: &mut TestWorld) {
    assert!(
        world.get::<Vector>("a").is_some(),
        "expected 'a' to be a Vector"
    );
}
