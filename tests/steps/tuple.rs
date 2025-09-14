use cucumber::{given, then, when};
use raytracer::prelude::*;

use crate::support::helpers::tuple::{Tuple4, get_as_tuple};
use crate::support::world::TestWorld;

// ===============================================================================
// Givens
// ===============================================================================
#[given(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$"
)]
fn given_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.insert(&name, Tuple4::new(x, y, z, w));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, point(x, y, z));
}

#[given(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([^)]+)\)$")]
fn given_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    world.insert(&name, vector(x, y, z));
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
    world.insert(&name, Vec3::new(val, val, 0.0));
}

// ===============================================================================
// Whens
// ===============================================================================
#[when(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\)$")]
fn when_normalize(world: &mut TestWorld, result: String, vector: String) {
    let v = world.get::<Vec3>(&vector).unwrap();
    let normalized = v.normalize().unwrap();
    world.insert(&result, normalized);
}

#[when(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) ← reflect\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\)$"
)]
fn when_reflect(world: &mut TestWorld, result: String, vector: String, normal: String) {
    let v = world.get::<Vec3>(&vector).unwrap();
    let n = world.get::<Vec3>(&normal).unwrap();
    let reflected = v.reflect(*n);
    world.insert(&result, reflected);
}

// ===============================================================================
// Thens - Properties
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.x = ([-+]?\d*\.?\d+)$")]
fn then_x_equals(world: &mut TestWorld, name: String, expected: f64) {
    if let Some(tuple) = world.get::<Tuple4>(&name) {
        assert!((tuple.x() == expected));
    } else if let Some(point) = world.get::<Point3>(&name) {
        assert!((point.x() == expected));
    } else if let Some(vector) = world.get::<Vec3>(&name) {
        assert!((vector.x() == expected));
    } else {
        panic!("Variable {} not found", name);
    }
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.y = ([-+]?\d*\.?\d+)$")]
fn then_y_equals(world: &mut TestWorld, name: String, expected: f64) {
    if let Some(tuple) = world.get::<Tuple4>(&name) {
        assert!((tuple.y() == expected));
    } else if let Some(point) = world.get::<Point3>(&name) {
        assert!((point.y() == expected));
    } else if let Some(vector) = world.get::<Vec3>(&name) {
        assert!((vector.y() == expected));
    } else {
        panic!("Variable {} not found", name);
    }
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.z = ([-+]?\d*\.?\d+)$")]
fn then_z_equals(world: &mut TestWorld, name: String, expected: f64) {
    if let Some(tuple) = world.get::<Tuple4>(&name) {
        assert!((tuple.z() == expected));
    } else if let Some(point) = world.get::<Point3>(&name) {
        assert!((point.z() == expected));
    } else if let Some(vector) = world.get::<Vec3>(&name) {
        assert!((vector.z() == expected));
    } else {
        panic!("Variable {} not found", name);
    }
}
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.w = ([-+]?\d*\.?\d+)$")]
fn then_w_equals(world: &mut TestWorld, name: String, expected: f64) {
    if let Some(tuple) = world.get::<Tuple4>(&name) {
        assert!((tuple.w() == expected));
    } else if world.get::<Point3>(&name).is_some() {
        assert!((1.0 == expected));
    } else if world.get::<Vec3>(&name).is_some() {
        assert!((0.0 == expected));
    } else {
        panic!("Variable {} not found", name);
    }
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.red = ([-+]?\d*\.?\d+)$")]
fn then_red_equals(world: &mut TestWorld, name: String, expected: f64) {
    let color = world
        .get::<Color>(&name)
        .unwrap_or_else(|| panic!("Color {name} not found"));
    assert!((color.r() == expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.green = ([-+]?\d*\.?\d+)$")]
fn then_green_equals(world: &mut TestWorld, name: String, expected: f64) {
    let color = world
        .get::<Color>(&name)
        .unwrap_or_else(|| panic!("Color {name} not found"));
    assert!((color.g() == expected));
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*)\.blue = ([-+]?\d*\.?\d+)$")]
fn then_blue_equals(world: &mut TestWorld, name: String, expected: f64) {
    let color = world
        .get::<Color>(&name)
        .unwrap_or_else(|| panic!("Color {name} not found"));
    assert!((color.b() == expected));
}

// ===============================================================================
// Thens - Type Checks
// ===============================================================================
#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a point$")]
fn then_is_point(world: &mut TestWorld, name: String) {
    let tuple = world
        .get::<Tuple4>(&name)
        .unwrap_or_else(|| panic!("Tuple {} not found", name));
    assert!(tuple.is_point());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a point$")]
fn then_is_not_point(world: &mut TestWorld, name: String) {
    let tuple = world
        .get::<Tuple4>(&name)
        .unwrap_or_else(|| panic!("Tuple {} not found", name));
    assert!(!tuple.is_point());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is a vector$")]
fn then_is_vector(world: &mut TestWorld, name: String) {
    let tuple = world
        .get::<Tuple4>(&name)
        .unwrap_or_else(|| panic!("Tuple {} not found", name));
    assert!(tuple.is_vector());
}

#[then(regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) is not a vector$")]
fn then_is_not_vector(world: &mut TestWorld, name: String) {
    let tuple = world
        .get::<Tuple4>(&name)
        .unwrap_or_else(|| panic!("Tuple {} not found", name));
    assert!(!tuple.is_vector());
}

// ===============================================================================
// Thens - Equality
// ===============================================================================
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let expected = Tuple4::new(x, y, z, w);

    if let Some(tuple) = world.get::<Tuple4>(&name) {
        assert_eq!(*tuple, expected);
    } else if let Some(point) = world.get::<Point3>(&name) {
        let point_as_tuple = Tuple4::new(point.x(), point.y(), point.z(), 1.0);
        assert_eq!(point_as_tuple, expected);
    } else if let Some(vector) = world.get::<Vec3>(&name) {
        let vector_as_tuple = Tuple4::new(vector.x(), vector.y(), vector.z(), 0.0);
        assert_eq!(vector_as_tuple, expected);
    } else {
        panic!("Variable {} not found", name);
    }
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_point(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    if let Some(point) = world.get::<Point3>(&name) {
        let expected = Point3::new(x, y, z);
        assert_eq!(*point, expected);
    } else if let Some(tuple) = world.get::<Tuple4>(&name) {
        let expected = Tuple4::new(x, y, z, 1.0);
        assert_eq!(*tuple, expected);
    } else {
        panic!("Point or tuple {} not found", name);
    }
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    if let Some(vector) = world.get::<Vec3>(&name) {
        let expected = Vec3::new(x, y, z);
        assert_eq!(*vector, expected);
    } else if let Some(tuple) = world.get::<Tuple4>(&name) {
        let expected = Tuple4::new(x, y, z, 0.0);
        assert_eq!(*tuple, expected);
    } else {
        panic!("Vector or tuple {} not found", name);
    }
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_equals_color(world: &mut TestWorld, name: String, r: f64, g: f64, b: f64) {
    let color = world
        .get::<Color>(&name)
        .unwrap_or_else(|| panic!("Color {} not found", name));
    let expected = Color::new(r, g, b);
    assert_eq!(*color, expected);
}

// ===============================================================================
// Thens - Operations
// ===============================================================================
#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \+ ([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_add_equals_tuple(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64, w: f64) {
    let expected = Tuple4::new(x, y, z, w);

    let a = get_as_tuple(world, &a);
    let b = get_as_tuple(world, &b);

    let result = Tuple4::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z(), a.w() + b.w());
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \+ ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_add_equals_color(world: &mut TestWorld, x: String, y: String, r: f64, g: f64, b: f64) {
    let color_a = world
        .get::<Color>(&x)
        .unwrap_or_else(|| panic!("Color {} not found", x));
    let color_b = world
        .get::<Color>(&y)
        .unwrap_or_else(|| panic!("Color {} not found", y));

    // Assuming Color has Add trait implemented
    let result = *color_a + *color_b;
    let expected = Color::new(r, g, b);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let expected = vector(x, y, z);

    // Handle Point - Point = Vector or Vector - Vector = Vector
    if let (Some(p1), Some(p2)) = (world.get::<Point3>(&a), world.get::<Point3>(&b)) {
        let result = *p1 - *p2;
        assert_eq!(result, expected);
    } else if let (Some(v1), Some(v2)) = (world.get::<Vec3>(&a), world.get::<Vec3>(&b)) {
        let result = *v1 - *v2;
        assert_eq!(result, expected);
    } else {
        panic!("Cannot subtract {} - {}", a, b);
    }
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = point\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_point(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let expected = point(x, y, z);

    // Handle Point - Vector = Point
    let point = world
        .get::<Point3>(&a)
        .unwrap_or_else(|| panic!("Point {} not found", a));
    let vector = world
        .get::<Vec3>(&b)
        .unwrap_or_else(|| panic!("Vector {} not found", b));
    let result = *point - *vector;
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) - ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_sub_equals_color(world: &mut TestWorld, x: String, y: String, r: f64, g: f64, b: f64) {
    let expected = color(r, g, b);

    let color_a = world
        .get::<Color>(&x)
        .unwrap_or_else(|| panic!("Color {} not found", x));
    let color_b = world
        .get::<Color>(&y)
        .unwrap_or_else(|| panic!("Color {} not found", y));

    let result = *color_a - *color_b;
    assert_eq!(result, expected);
}

#[then(
    regex = r"^-([a-zA-Z_][a-zA-Z0-9_]*) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_neg_equals_tuple(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t = get_as_tuple(world, &name);

    let result = Tuple4::new(-t.x(), -t.y(), -t.z(), -t.w());
    let expected = Tuple4::new(x, y, z, w);
    assert_eq!(result, expected);
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
    let t = get_as_tuple(world, &name);

    let expected = Tuple4::new(x, y, z, w);
    let result = Tuple4::new(t.x() * scalar, t.y() * scalar, t.z() * scalar, t.w() * scalar);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([-+]?\d*\.?\d+) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_scalar_equals_color(world: &mut TestWorld, name: String, scalar: f64, r: f64, g: f64, b: f64) {
    let color = world
        .get::<Color>(&name)
        .unwrap_or_else(|| panic!("Color {} not found", name));

    let result = *color * scalar;
    let expected = Color::new(r, g, b);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) \* ([a-zA-Z_][a-zA-Z0-9_]*) = color\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_mul_equals_color(world: &mut TestWorld, x: String, y: String, r: f64, g: f64, b: f64) {
    let color_a = world
        .get::<Color>(&x)
        .unwrap_or_else(|| panic!("Color {} not found", x));
    let color_b = world
        .get::<Color>(&y)
        .unwrap_or_else(|| panic!("Color {} not found", y));

    let result = *color_a * *color_b;
    let expected = Color::new(r, g, b);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^([a-zA-Z_][a-zA-Z0-9_]*) / ([-+]?\d*\.?\d+) = tuple\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_div_equals_tuple(
    world: &mut TestWorld,
    name: String,
    divisor: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t = get_as_tuple(world, &name);

    let result = Tuple4::new(t.x() / divisor, t.y() / divisor, t.z() / divisor, t.w() / divisor);
    let expected = Tuple4::new(x, y, z, w);
    assert_eq!(result, expected);
}

// Magnitude
#[then(regex = r"^magnitude\(([a-zA-Z_][a-zA-Z0-9_]*)\) = ([-+]?\d*\.?\d+)$")]
fn then_magnitude_equals(world: &mut TestWorld, name: String, expected: f64) {
    let vector = world
        .get::<Vec3>(&name)
        .unwrap_or_else(|| panic!("Vector {} not found", name));
    let magnitude = vector.magnitude();
    assert!(magnitude == expected);
}

#[then(regex = r"^magnitude\(([a-zA-Z_][a-zA-Z0-9_]*)\) = √(\d+)$")]
fn then_magnitude_equals_sqrt(world: &mut TestWorld, name: String, value: f64) {
    let vector = world
        .get::<Vec3>(&name)
        .unwrap_or_else(|| panic!("Vector {} not found", name));
    let magnitude = vector.magnitude();
    let expected = value.sqrt();
    assert!(magnitude == expected);
}

// Normalize
#[then(
    regex = r"^normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_normalize_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let vector = world
        .get::<Vec3>(&name)
        .unwrap_or_else(|| panic!("Vector {} not found", name));
    let result = vector.normalize().expect("Cannot normalize zero vector");
    let expected = Vec3::new(x, y, z);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^normalize\(([a-zA-Z_][a-zA-Z0-9_]*)\) = approximately vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_normalize_approximately_equals_vector(world: &mut TestWorld, name: String, x: f64, y: f64, z: f64) {
    let vector = world
        .get::<Vec3>(&name)
        .unwrap_or_else(|| panic!("Vector {} not found", name));
    let result = vector.normalize().expect("Cannot normalize zero vector");
    let expected = Vec3::new(x, y, z);
    assert_eq!(result, expected);
}

#[then(regex = r"^dot\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\) = ([-+]?\d*\.?\d+)$")]
fn then_dot_equals(world: &mut TestWorld, a: String, b: String, expected: f64) {
    let vector_a = world
        .get::<Vec3>(&a)
        .unwrap_or_else(|| panic!("Vector {} not found", a));
    let vector_b = world
        .get::<Vec3>(&b)
        .unwrap_or_else(|| panic!("Vector {} not found", b));
    let result = vector_a.dot(*vector_b);
    assert_eq!(result, expected);
}

#[then(
    regex = r"^cross\(([a-zA-Z_][a-zA-Z0-9_]*), ([a-zA-Z_][a-zA-Z0-9_]*)\) = vector\(([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+), ([-+]?\d*\.?\d+)\)$"
)]
fn then_cross_equals_vector(world: &mut TestWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let vector_a = world
        .get::<Vec3>(&a)
        .unwrap_or_else(|| panic!("Vector {} not found", a));
    let vector_b = world
        .get::<Vec3>(&b)
        .unwrap_or_else(|| panic!("Vector {} not found", b));
    let result = vector_a.cross(*vector_b);
    let expected = Vec3::new(x, y, z);
    assert_eq!(result, expected);
}
