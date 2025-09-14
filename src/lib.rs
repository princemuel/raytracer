//! # Ray Tracer
//!
//! A complete ray tracer implementation following "The Ray Tracer Challenge"
//! by Jamis Buck.
//!
//! ## Quick Start
//!
//! ```rust
//! use raytracer::prelude::*;
//!
//! // Create a simple scene
//! let world = scenes::single_sphere();
//! let camera = Camera::new(800, 600, PI / 3).with_transform(view_transform(
//!     point(0, 1.5, -5),
//!     point(0, 1, 0),
//!     vector(0, 1, 0),
//! ));
//!
//! // Render the scene
//! let canvas = camera.render(&world);
//! canvas.save_ppm("output.ppm")?;
//! ```
#![warn(clippy::self_named_module_files)]
pub mod error;
pub mod geometry;
pub mod graphics;
pub mod math;
pub mod shading;
pub mod world;

pub mod prelude;

// Re-export at crate root for convenience
pub use error::{RayTraceError, Result};
