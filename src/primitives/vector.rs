use crate::primitives::tuple::Tuple;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}
impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }

    fn zero() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn x(&self) -> f64 { self.x }

    fn y(&self) -> f64 { self.y }

    fn z(&self) -> f64 { self.z }

    fn w(&self) -> f64 { 0.0 }
}
