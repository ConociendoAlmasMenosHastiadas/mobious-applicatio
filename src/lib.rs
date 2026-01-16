//! A library for working with Möbius transformations on the complex plane.
//!
//! Möbius transformations are conformal mappings of the form:
//! f(z) = (az + b) / (cz + d)
//! where a, b, c, d are complex numbers and ad - bc ≠ 0.

use num_complex::Complex64;
use ndarray::{Array1, Array2};

/// Represents a Möbius transformation with complex coefficients.
///
/// The transformation is defined as: f(z) = (az + b) / (cz + d)
/// where ad - bc ≠ 0 (the determinant must be non-zero).
#[derive(Debug, Clone, Copy)]
pub struct MobiusTransform {
    pub a: Complex64,
    pub b: Complex64,
    pub c: Complex64,
    pub d: Complex64,
}

impl MobiusTransform {
    /// Creates a new Möbius transformation.
    ///
    /// # Panics
    /// Panics if the determinant (ad - bc) is zero.
    pub fn new(a: Complex64, b: Complex64, c: Complex64, d: Complex64) -> Self {
        let det = a * d - b * c;
        assert!(
            det.norm() > 1e-10,
            "Determinant must be non-zero for a valid Möbius transformation"
        );
        Self { a, b, c, d }
    }

    /// Creates the identity transformation.
    pub fn identity() -> Self {
        Self::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        )
    }

    /// Applies the transformation to a complex number.
    pub fn apply(&self, z: Complex64) -> Complex64 {
        let numerator = self.a * z + self.b;
        let denominator = self.c * z + self.d;
        
        if denominator.norm() < 1e-10 {
            // Point maps to infinity
            Complex64::new(f64::INFINITY, f64::INFINITY)
        } else {
            numerator / denominator
        }
    }

    /// Applies the transformation to a vector of complex numbers.
    pub fn apply_batch(&self, points: &Array1<Complex64>) -> Array1<Complex64> {
        points.mapv(|z| self.apply(z))
    }

    /// Returns the matrix representation of the transformation.
    pub fn to_matrix(&self) -> Array2<Complex64> {
        Array2::from_shape_vec((2, 2), vec![self.a, self.b, self.c, self.d])
            .expect("Valid 2x2 matrix")
    }

    /// Composes this transformation with another: (self ∘ other)(z) = self(other(z))
    pub fn compose(&self, other: &MobiusTransform) -> MobiusTransform {
        MobiusTransform::new(
            self.a * other.a + self.b * other.c,
            self.a * other.b + self.b * other.d,
            self.c * other.a + self.d * other.c,
            self.c * other.b + self.d * other.d,
        )
    }

    /// Returns the inverse transformation.
    pub fn inverse(&self) -> MobiusTransform {
        let det = self.a * self.d - self.b * self.c;
        MobiusTransform::new(
            self.d / det,
            -self.b / det,
            -self.c / det,
            self.a / det,
        )
    }

    /// Returns the determinant ad - bc.
    pub fn determinant(&self) -> Complex64 {
        self.a * self.d - self.b * self.c
    }

    /// Normalizes the transformation so that ad - bc = 1.
    pub fn normalize(&self) -> MobiusTransform {
        let det_sqrt = self.determinant().sqrt();
        MobiusTransform::new(
            self.a / det_sqrt,
            self.b / det_sqrt,
            self.c / det_sqrt,
            self.d / det_sqrt,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = MobiusTransform::identity();
        let z = Complex64::new(3.0, 4.0);
        let result = id.apply(z);
        assert!((result - z).norm() < 1e-10);
    }

    #[test]
    fn test_composition() {
        let m1 = MobiusTransform::new(
            Complex64::new(2.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        );
        let m2 = MobiusTransform::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        );
        
        let composed = m1.compose(&m2);
        let z = Complex64::new(1.0, 1.0);
        
        let result1 = m1.apply(m2.apply(z));
        let result2 = composed.apply(z);
        
        assert!((result1 - result2).norm() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let m = MobiusTransform::new(
            Complex64::new(2.0, 1.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 1.0),
            Complex64::new(3.0, 0.0),
        );
        let inv = m.inverse();
        let z = Complex64::new(2.0, 3.0);
        
        let result = inv.apply(m.apply(z));
        assert!((result - z).norm() < 1e-10);
    }

    #[test]
    #[should_panic]
    fn test_zero_determinant() {
        MobiusTransform::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(4.0, 0.0),
        );
    }
}
