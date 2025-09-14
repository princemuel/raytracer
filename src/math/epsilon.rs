//! # Floating-Point Precision Utilities
//!
//! This module provides epsilon values and comparison helpers used by
//! geometry types in the ray tracer. Equality on types like `Point` and
//! `Vector` is based on tolerant comparison.
//!
//! ```rust
//! use raytracer::prelude::*;
//!
//! let a = point(0.1 + 0.2, 0.0, 0.0);
//! let b = point(0.3, 0.0, 0.0);
//! assert_eq!(a, b); // works because PartialEq uses epsilon
//! ```
//!
//! For raw `f64` values, use [`is_equal_float`]:
//!
//! ```rust
//! use raytracer::math::epsilon::is_equal_float;
//!
//! let a = 0.1_f64 + 0.2;
//! let b = 0.3_f64;
//! assert!(is_equal_float(a, b));
//! ```
// ================================
// Epsilon Constants
// ================================

/// Standard epsilon for f64 floating-point comparisons
pub const EPSILON_F64: f64 = 1e-10;

/// Standard epsilon for f32 floating-point comparisons
pub const EPSILON_F32: f32 = 1e-6;

/// Stricter epsilon for high-precision f64 operations
pub const EPSILON_F64_STRICT: f64 = 1e-12;

/// Stricter epsilon for high-precision f32 operations
pub const EPSILON_F32_STRICT: f32 = 1e-8;

/// More permissive epsilon for f64 rendering operations
pub const EPSILON_F64_LOOSE: f64 = 1e-8;

/// More permissive epsilon for f32 rendering operations
pub const EPSILON_F32_LOOSE: f32 = 1e-4;

/// Default epsilon for the primary floating-point type used in ray tracing
pub const EPSILON: f64 = 1e-5;

pub const fn is_equal_float(a: f64, b: f64) -> bool {
    // Fast path: exact equality (handles infinities, zeros, and exact matches)
    if a == b {
        return true;
    }

    // Handle NaN cases - NaN should never equal anything
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinite cases that aren't exactly equal
    if a.is_infinite() || b.is_infinite() {
        return false; // Different infinities or one infinite, one finite
    }

    let diff = (a - b).abs();

    // For very small numbers near zero, use absolute epsilon
    if a.abs().max(b.abs()) < 1.0 {
        return diff < EPSILON;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = EPSILON * a.abs().max(b.abs());

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    diff < EPSILON.max(relative_epsilon)
}
