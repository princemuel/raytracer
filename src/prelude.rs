//! # Prelude
//!
//! Re-exports key types and functions used across the crate.
//!
//! ```rust
//! use raytracer::prelude::*;
//! ```
// ================================
// Core Mathematical Primitives
// ================================

#[rustfmt::skip] // Re-export fundamental types
pub use crate::math::primitives::{Color3, Point3, Vec3};
#[rustfmt::skip] // Re-export fundamental traits
pub use crate::math::primitives::{ColorRGB, Coordinate3D};
#[rustfmt::skip] // Re-export constructor functions
pub use crate::math::primitives::{color, point, vector};

// ================================
// Graphics & Rendering
// ================================
#[rustfmt::skip]
pub use crate::graphics::canvas::Canvas;

// ================================
// Constants & Utilities
// ================================
pub use crate::math::epsilon::EPSILON;

// Common numerical constants
#[rustfmt::skip]
pub use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, TAU};

// ================================
// Error Handling
// ================================
pub use crate::error::{RayTraceError, Result};

// ================================
// Convenience Type Aliases
// ================================

/// Convenience alias for 3D point
pub type Point = Point3;

/// Convenience alias for 3D vector
pub type Vector = Vec3;

/// Convenience alias for RGB color
pub type Color = Color3;
