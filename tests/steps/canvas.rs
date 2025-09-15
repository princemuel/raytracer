use cucumber::{given, then, when};
use raytracer::prelude::*;

use crate::support::world::TestWorld;

// ===============================================================================
// Givens
// ===============================================================================
#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← canvas\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$")]
fn given_canvas(world: &mut TestWorld, name: String, width: usize, height: usize) {
    world.insert(&name, Canvas::new(width, height));
}

// ===============================================================================
// Whens
// ===============================================================================
#[when(
    regex = r"^write_pixel\(([a-zA-Z_][a-zA-Z0-9_]*), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([a-zA-Z_][a-zA-Z0-9_]*)\)$"
)]
fn when_write_pixel(world: &mut TestWorld, name: String, x: usize, y: usize, color: String) {
    let color = *world.get::<Color>(&color).unwrap();
    let canvas = world.get::<Canvas>(&name).expect("Canvas not found");

    let canvas = canvas.clone().write_pixel(x, y, color);
    world.insert(&name, canvas);
}

#[when(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← canvas_to_ppm\(([a-zA-Z_][a-zA-Z0-9_]*)\)$")]
fn when_canvas_to_ppm(world: &mut TestWorld, name: String, canvas: String) {
    let canvas = world.get::<Canvas>(&canvas).expect("Canvas not found");

    let result = canvas.to_ppm();
    world.insert(&name, result);
}

// ===============================================================================
// Thens - Properties
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.width = ([-+]?\d*\.?\d+)$")]
fn then_width_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world.get::<Canvas>(&name).unwrap();
    assert!(canvas.width() == expected);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.height = ([-+]?\d*\.?\d+)$")]
fn then_height_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world.get::<Canvas>(&name).unwrap();
    assert!(canvas.height() == expected);
}
