use raytracer::prelude::*;

use crate::support::world::TestWorld;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuple4(f64, f64, f64, f64);

impl Tuple4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self(x, y, z, w) }

    pub fn x(&self) -> f64 { self.0 }

    pub fn y(&self) -> f64 { self.1 }

    pub fn z(&self) -> f64 { self.2 }

    pub fn w(&self) -> f64 { self.3 }

    pub fn is_point(&self) -> bool { (self.3 - 1.0).abs() < EPSILON }

    pub fn is_vector(&self) -> bool { self.3.abs() < EPSILON }
}

pub fn get_as_tuple(world: &TestWorld, name: &str) -> Tuple4 {
    if let Some(tuple) = world.get::<Tuple4>(name) {
        *tuple
    } else if let Some(point) = world.get::<Point3>(name) {
        Tuple4::new(point.x(), point.y(), point.z(), point.w())
    } else if let Some(vector) = world.get::<Vec3>(name) {
        Tuple4::new(vector.x(), vector.y(), vector.z(), vector.w())
    } else {
        panic!("Variable {} not found", name);
    }
}
