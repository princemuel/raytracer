//! # Ray Tracer
//!
//! A ray tracer implementation based on *The Ray Tracer Challenge* by Jamis
//! Buck.
//!
//! ## Quick Start
//!
//! ```rust
//! use raytracer::prelude::*;
//! ```
#![warn(clippy::self_named_module_files)]
// #![warn(missing_docs)]
#![feature(stmt_expr_attributes)]
pub mod cmp;
pub mod error;
pub mod geometry;
pub mod graphics;
pub mod primitives;
pub mod shading;
pub mod world;

pub mod prelude;

// Re-export at crate root for convenience
pub use error::TracerError;
pub use prelude::Result;
