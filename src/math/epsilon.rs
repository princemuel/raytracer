//! Domain-specific epsilon constants for ray tracing and geometric computation.
//!
//! These constants complement the built-in machine epsilon values
//! (`f32::EPSILON`, `f64::EPSILON`) by providing domain-specific precision
//! levels for different ray tracing scenarios. While machine epsilon represents
//! the smallest representable difference, these values are tuned for practical
//! geometric computation where accumulated errors and numerical stability
//! matter more than theoretical precision.

/// Precision levels for approximate equality comparisons.
///
/// Each constant is tuned for specific use cases in ray tracing:
/// - **COARSE**: Bounding box checks, LOD decisions
/// - **INTERSECT**: Ray-surface intersections, shadow rays
/// - **STANDARD**: General geometric calculations
/// - **TIGHT**: Mathematical operations, unit conversions
pub struct Precision;

impl Precision {
    /// Very loose precision for coarse geometric approximations (1e-3).
    ///
    /// Use for bounding box checks, level-of-detail decisions, and other
    /// operations where sub-millimeter accuracy is unnecessary.
    pub const COARSE: f64 = 1e-3;
    /// Loose precision for ray tracing intersections (1e-4).
    ///
    /// Balances accuracy with numerical stability for ray-surface
    /// intersections, shadow ray calculations, and accumulated
    /// floating-point errors.
    pub const INTERSECT: f64 = 1e-4;
    /// Standard precision for general computations (1e-5).
    ///
    /// Suitable for most geometric calculations, transformations, and
    /// intermediate results where good accuracy is needed.
    pub const STANDARD: f64 = 1e-5;
    /// High precision for mathematical operations (1e-10).
    ///
    /// For unit tests, exact mathematical operations, and cases where
    /// maximum floating-point precision is required.
    pub const TIGHT: f64 = 1e-10;
}
