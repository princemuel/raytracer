use raytracer::primitives::{GenericTuple, Point, Vector};

#[derive(Debug)]
pub enum TupleVariant {
    Generic(GenericTuple),
    Point(Point),
    Vector(Vector),
}

impl TupleVariant {
    pub fn is_point(&self) -> bool {
        match self {
            TupleVariant::Point(_) => true,
            TupleVariant::Generic(t) => (t.w() - 1.0).abs() < f32::EPSILON,
            _ => false,
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            TupleVariant::Vector(_) => true,
            TupleVariant::Generic(t) => t.w().abs() < f32::EPSILON,
            _ => false,
        }
    }

    pub fn x(&self) -> f32 {
        match self {
            TupleVariant::Generic(t) => t.x(),
            TupleVariant::Point(p) => p.x(),
            TupleVariant::Vector(v) => v.x(),
        }
    }

    pub fn y(&self) -> f32 {
        match self {
            TupleVariant::Generic(t) => t.y(),
            TupleVariant::Point(p) => p.y(),
            TupleVariant::Vector(v) => v.y(),
        }
    }

    pub fn z(&self) -> f32 {
        match self {
            TupleVariant::Generic(t) => t.z(),
            TupleVariant::Point(p) => p.z(),
            TupleVariant::Vector(v) => v.z(),
        }
    }

    pub fn w(&self) -> f32 {
        match self {
            TupleVariant::Generic(t) => t.w(),
            TupleVariant::Point(p) => p.w(),
            TupleVariant::Vector(v) => v.w(),
        }
    }

    pub fn as_generic(&self) -> GenericTuple {
        match self {
            TupleVariant::Generic(t) => *t,
            TupleVariant::Point(p) => p.cast(),
            TupleVariant::Vector(v) => v.cast(),
        }
    }

    pub fn as_vector(&self) -> Vector {
        match self {
            TupleVariant::Vector(v) => *v,
            _ => panic!("Expected vector"),
        }
    }

    pub fn as_point(&self) -> Point {
        match self {
            TupleVariant::Point(p) => *p,
            _ => panic!("Expected point"),
        }
    }
}
