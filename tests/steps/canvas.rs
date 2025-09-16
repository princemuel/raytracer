use cucumber::gherkin::Step;
use cucumber::{given, then, when};
use raytracer::prelude::*;

use crate::support::world::TestWorld;

// ===============================================================================
// Givens
// ===============================================================================
#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← canvas\(([-+]?\d+), ([-+]?\d+)\)$")]
fn given_canvas(world: &mut TestWorld, name: String, width: usize, height: usize) {
    world.insert(&name, canvas(width, height));
}

// ===============================================================================
// Whens
// ===============================================================================
#[when(
    regex = r"^write_pixel\(([a-zA-Z_][a-zA-Z0-9_]*), ([-+]?\d+), ([-+]?\d+), ([a-zA-Z_][a-zA-Z0-9_]*)\)$"
)]
fn when_write_pixel(world: &mut TestWorld, canvas: String, x: usize, y: usize, color: String) {
    let color = *world
        .get::<Color>(&color)
        .unwrap_or_else(|| panic!("Color {} not found", color));
    let canvas = world
        .get_mut::<Canvas>(&canvas)
        .unwrap_or_else(|| panic!("Canvas {} not found", canvas));

    canvas.write_pixel(x, y, color);
}

#[when(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← canvas_to_ppm\(([a-zA-Z_][a-zA-Z0-9_]*)\)$")]
fn when_canvas_to_ppm(world: &mut TestWorld, ppm: String, canvas: String) {
    let canvas = world
        .get::<Canvas>(&canvas)
        .unwrap_or_else(|| panic!("Canvas {} not found", canvas));
    let ppm_string = canvas.to_ppm();
    world.insert(&ppm, ppm_string);
}

#[when(
    regex = r"^every pixel of ([a-zA-Z_][a-zA-Z0-9_]*) is set to color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn when_pixel_is_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    let canvas = world.get_mut::<Canvas>(&name).expect("Canvas not found");
    let color = color(r, g, b);
    canvas.pixels_mut().iter_mut().for_each(|pixel| *pixel = color);
}

// ===============================================================================
// Thens - Properties
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.width = ([-+]?\d+)$")]
fn then_width_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world
        .get::<Canvas>(&name)
        .unwrap_or_else(|| panic!("Canvas {} not found", name));
    assert_eq!(canvas.width(), expected);
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.height = ([-+]?\d+)$")]
fn then_height_equals(world: &mut TestWorld, name: String, expected: usize) {
    let canvas = world
        .get::<Canvas>(&name)
        .unwrap_or_else(|| panic!("Canvas {} not found", name));
    assert_eq!(canvas.height(), expected);
}

// ===============================================================================
// Thens - Equality
// ===============================================================================
#[then(
    regex = r"^pixel_at\(([a-zA-Z_][a-zA-Z0-9_]*), ([-+]?\d+), ([-+]?\d+)\) = ([a-zA-Z_][a-zA-Z0-9_]*)$"
)]
fn then_pixel_at_equals_color(world: &mut TestWorld, canvas: String, x: usize, y: usize, color: String) {
    let expected = *world
        .get::<Color>(&color)
        .unwrap_or_else(|| panic!("Color {} not found", color));
    let canvas = world
        .get::<Canvas>(&canvas)
        .unwrap_or_else(|| panic!("Canvas {} not found", canvas));

    let actual = canvas.pixel_at(x, y);
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^every pixel of ([a-zA-Z_][a-zA-Z0-9_]*) is color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_every_pixel_is_color(world: &mut TestWorld, canvas: String, r: f64, g: f64, b: f64) {
    let canvas = world
        .get::<Canvas>(&canvas)
        .unwrap_or_else(|| panic!("Canvas {} not found", canvas));
    let expected = color(r, g, b);

    if let Some((x, y, pixel)) = (0..canvas.height())
        .flat_map(|y| (0..canvas.width()).map(move |x| (x, y)))
        .find_map(|(x, y)| {
            let pixel = canvas.pixel_at(x, y);
            (pixel != expected).then_some((x, y, pixel))
        })
    {
        panic!("Pixel at ({x}, {y}) was {pixel:?}, expected {expected:?}");
    }
}

#[then(regex = r"^lines ([-+]?\d+)-([-+]?\d+) of ([a-zA-Z_][a-zA-Z0-9_]*) are$")]
fn then_lines_of_ppm_are(world: &mut TestWorld, step: &Step, start: usize, end: usize, name: String) {
    let ppm = world
        .get::<String>(&name)
        .unwrap_or_else(|| panic!("PPM string {} not found", name));
    let expected = step.docstring().expect("Expected docstring with PPM content");

    let ppm_lines = ppm.lines().skip(start - 1).take(end - start + 1);
    let lines = expected.trim().lines();

    for (actual, expected) in ppm_lines.zip(lines) {
        assert_eq!(actual.trim(), expected.trim());
    }
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ends with a newline character$")]
fn then_ppm_ends_with_newline(world: &mut TestWorld, name: String) {
    let ppm = world
        .get::<String>(&name)
        .unwrap_or_else(|| panic!("PPM string {} not found", name));
    assert!(
        ppm.ends_with('\n'),
        "PPM string should end with a newline character"
    );
}
