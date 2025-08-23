use crate::primitives::tuple::Tuple;

pub fn vector(x: f32, y: f32, z: f32) -> Vector { Vector::new(x, y, z, 0.0) }
#[derive(Debug, Clone, Copy)]
pub struct VectorTag;
pub type Vector = Tuple<VectorTag>;
