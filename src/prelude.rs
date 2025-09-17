//! # Prelude
//!
//! Re-exports key types and functions used across the crate.
//!
//! ```rust
//! use raytracer::prelude::*;
//! ```
// ================================
// Core Primitives
// ================================

#[rustfmt::skip]
// Re-export fundamental types
pub use crate::primitives::{Color3, Point3, Vec3};

#[rustfmt::skip]
// Re-export fundamental traits
pub use crate::primitives::{ColorRGB, Tuple};

#[rustfmt::skip]
// Re-export constructor functions
pub use crate::primitives::{color, point, vector};

// Re-export matrix and transformation types
pub use crate::primitives::Matrix;

// ================================
// Graphics & Rendering
// ================================
#[rustfmt::skip]
pub use crate::graphics::canvas::Canvas;

// ================================
// Constants & Utilities
// ================================
pub use crate::cmp::epsilon::EPSILON;
pub use crate::cmp::float::is_equal;

// Common numerical constants
#[rustfmt::skip]
pub use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, TAU};

// ================================
// Error Handling
// ================================
pub use crate::error::TracerError;

// ================================
// Convenience Type Aliases
// ================================

/// Convenience alias for 3D point
pub type Point = Point3;

/// Convenience alias for 3D vector
pub type Vector = Vec3;

/// Convenience alias for RGB color
pub type Color = Color3;

/// Convenient Result type alias
pub type Result<T> = core::result::Result<T, TracerError>;
// ================================
// Debug & Visualization Helpers
// ================================

/// Helper functions for debugging and visualization
pub mod debug {
    use super::*;
    /// Print a 3D coordinate with nice formatting
    pub fn print_coord(name: &str, point: Point3) {
        println!("{}: ({:.3}, {:.3}, {:.3})", name, point.x(), point.y(), point.z());
    }
}
