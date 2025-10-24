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
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]
pub mod cmp;
pub mod error;
pub mod geometry;
pub mod graphics;
pub mod math;
pub mod prelude;
pub mod primitives;
pub mod shading;
pub mod world;
// Re-export at crate root for convenience
pub use error::TracerError;
pub use prelude::Result;
