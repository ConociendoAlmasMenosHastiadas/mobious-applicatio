//! A library for working with Möbius transformations on the complex plane.
//!
//! Möbius transformations are conformal mappings of the form:
//! f(z) = (az + b) / (cz + d)
//! where a, b, c, d are complex numbers and ad - bc ≠ 0.

mod transforms;
pub mod plane_functions;
pub mod complex_utils;

pub use transforms::{MobiusTransform, TransformError};
