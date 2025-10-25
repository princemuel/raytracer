//! # Floating-Point Precision Utilities
//!
//! This module provides epsilon values and comparison helpers used by
//! geometry types in this ray tracer lib. Equality on types like `Point3` and
//! `Vec3` is based on tolerant comparison.
//!
//! ```rust
//! use raytracer::prelude::*;
//!
//! let a = point(0.1 + 0.2, 0.0, 0.0);
//! let b = point(0.3, 0.0, 0.0);
//! assert_eq!(a, b); // works because PartialEq uses epsilon
//! ```
//!
//! For raw `f64` values, use [`is_equal`]:
//!
//! ```rust
//! use raytracer::cmp::float::is_equal;
//!
//! let a = 0.1_f64 + 0.2;
//! let b = 0.3_f64;
//! assert!(is_equal(a, b));
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
