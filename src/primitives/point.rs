use crate::primitives::tuple::Tuple;

pub fn point(x: f32, y: f32, z: f32) -> Point { Point::new(x, y, z, 1.0) }

#[derive(Debug, Clone, Copy)]
pub struct PointTag;
pub type Point = Tuple<PointTag>;
