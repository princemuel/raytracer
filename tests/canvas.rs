#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod common;

use cucumber::gherkin::Step;
use cucumber::{World as _, given, then, when};
use raytracer::prelude::*;

use crate::common::TestWorld;

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/canvas.feature")
        .await;
}

// ====================================================================
// Given Steps - Canvas Construction
// ====================================================================
#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_canvas(world: &mut TestWorld, name: String, width: usize, height: usize) {
    world.insert(&name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    world.insert(&name, color(r, g, b));
}

// ====================================================================
// When Steps -
// ====================================================================
#[when(expr = r"write_pixel\({word}, {int}, {int}, {word}\)")]
fn when_write_pixel(world: &mut TestWorld, canvas: String, x: usize, y: usize, color: String) {
    let color = *world.get::<Color>(&color).unwrap();
    let canvas = world.get_mut::<Canvas>(&canvas).unwrap();

    canvas.write_pixel(x, y, color);
}

#[when(expr = r"{word} ← canvas_to_ppm\({word}\)")]
fn when_canvas_to_ppm(world: &mut TestWorld, key: String, canvas: String) {
    let canvas = world.get::<Canvas>(&canvas).unwrap();
    let value = canvas.to_ppm();
    world.insert(&key, value);
}

#[when(expr = r"every pixel of {word} is set to color\({float}, {float}, {float}\)")]
fn when_pixel_is_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    let canvas = world.get_mut::<Canvas>(&name).expect("Canvas not found");
    let color = color(r, g, b);
    canvas.pixels_mut().iter_mut().for_each(|pixel| *pixel = color);
}

// ====================================================================
// Then Steps - Canvas Properties
// ====================================================================
#[then(expr = "{word}.width = {int}")]
fn then_width_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world.get::<Canvas>(&name).unwrap();
    assert_eq!(canvas.width(), expected);
}

#[then(expr = "{word}.height = {int}")]
fn then_height_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world.get::<Canvas>(&name).unwrap();
    assert_eq!(canvas.height(), expected);
}

// ====================================================================
// Then Steps - Canvas Equality
// ====================================================================
#[then(expr = r"pixel_at\({word}, {int}, {int}\) = {word}")]
fn then_pixel_at_equals_color(world: &mut TestWorld, canvas: String, x: usize, y: usize, color: String) {
    let expected = *world.get::<Color>(&color).unwrap();
    let canvas = world.get::<Canvas>(&canvas).unwrap();

    let actual = canvas[y][x];
    assert_eq!(actual, expected);
}

#[then(expr = r"every pixel of {word} is color\({float}, {float}, {float}\)")]
fn then_every_pixel_is_color(world: &mut TestWorld, canvas: String, r: f64, g: f64, b: f64) {
    let canvas = world.get::<Canvas>(&canvas).unwrap();
    let expected = color(r, g, b);

    if let Some((x, y, pixel)) = (0..canvas.height())
        .flat_map(|y| (0..canvas.width()).map(move |x| (x, y)))
        .find_map(|(x, y)| {
            let pixel = canvas[y][x];
            (pixel != expected).then_some((x, y, pixel))
        })
    {
        panic!("Pixel at ({x}, {y}) was {pixel:?}, expected {expected:?}");
    }
}

#[then(expr = "lines {int}-{int} of {word} are")]
fn then_lines_of_ppm_are(world: &mut TestWorld, step: &Step, start: usize, end: usize, name: String) {
    let ppm = world.get::<String>(&name).unwrap();
    let expected = step.docstring().expect("Expected docstring with PPM content");

    let ppm_lines = ppm.lines().skip(start - 1).take(end - start + 1);
    let lines = expected.trim().lines();

    for (actual, expected) in ppm_lines.zip(lines) {
        assert_eq!(actual.trim(), expected.trim());
    }
}

#[then(expr = "{word} ends with a newline character")]
fn then_ppm_ends_with_newline(world: &mut TestWorld, name: String) {
    let ppm = world.get::<String>(&name).unwrap();
    assert!(
        ppm.ends_with('\n'),
        "PPM string should end with a newline character"
    );
}
