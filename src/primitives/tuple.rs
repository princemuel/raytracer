use std::marker::PhantomData;

pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> GenericTuple { GenericTuple::new(x, y, z, w) }

pub type GenericTuple = Tuple<TupleTag>;

#[derive(Debug, Clone, Copy)]
pub struct TupleTag;
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Tuple<T> {
    x:        f32,
    y:        f32,
    z:        f32,
    w:        f32,
    _phantom: PhantomData<T>,
}

impl<T> Tuple<T> {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w,
            _phantom: PhantomData,
        }
    }

    pub fn cast<U>(self) -> Tuple<U> {
        Tuple {
            x:        self.x,
            y:        self.y,
            z:        self.z,
            w:        self.w,
            _phantom: PhantomData,
        }
    }

    // Approximate equality for floating point comparisons
    pub fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
            && (self.z - other.z).abs() < epsilon
            && (self.w - other.w).abs() < epsilon
    }

    pub fn x(&self) -> f32 { self.x }

    pub fn y(&self) -> f32 { self.y }

    pub fn z(&self) -> f32 { self.z }

    pub fn w(&self) -> f32 { self.w }
}
