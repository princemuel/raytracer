/// Domain-specific epsilon constants for different precision requirements
/// in ray tracing and geometric computation.
pub struct Epsilon;

impl Epsilon {
    /// Very loose epsilon for coarse geometric approximations.
    /// Use for: bounding box checks, rough collision detection
    pub const COARSE: f64 = 1e-3;
    /// Loose epsilon for geometric intersections and ray tracing.
    /// Use for: ray-surface intersections, shadow calculations, accumulated
    /// errors
    pub const INTERSECT: f64 = 1e-4;
    /// Standard epsilon for general floating-point comparisons.
    /// Use for: general numeric comparisons, intermediate calculations
    pub const STANDARD: f64 = 1e-6;
    /// Tight epsilon for mathematical operations and unit tests.
    /// Use for: arithmetic operations, conversions, exact comparisons
    pub const TIGHT: f64 = 1e-9;
}
