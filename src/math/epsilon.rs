/// Geometric tolerances for raytracing
pub struct Epsilon;

impl Epsilon {
    /// rare: sanity checks, math identity
    pub const INTERSECT: f64 = 1e-6;
    /// hit tests, "close enough"
    pub const SHADOW: f64 = 1e-5;
    /// shadow ray bias, avoid self-hit acne
    pub const TIGHT: f64 = 1e-10;
}
