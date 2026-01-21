//! Möbius transformation module.
//!
//! Provides the core [`MobiusTransform`] type for working with conformal mappings
//! of the form: f(z) = (az + b) / (cz + d)

use num_complex::Complex64;
use ndarray::{Array1, Array2};
use crate::complex_utils::{is_infinity, normalize_infinity, COMPLEX_INFINITY};
use std::fmt;

/// Error type for Möbius transformation operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TransformError {
    /// The determinant (ad - bc) is zero or too close to zero.
    SingularTransform,
    /// One or more coefficients contain infinity.
    InfiniteCoefficient,
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformError::SingularTransform => {
                write!(f, "Determinant must be non-zero for a valid Möbius transformation")
            }
            TransformError::InfiniteCoefficient => {
                write!(f, "Coefficients must be finite for a valid Möbius transformation")
            }
        }
    }
}

impl std::error::Error for TransformError {}

/// Represents a Möbius transformation with complex coefficients.
///
/// The transformation is defined as: f(z) = (az + b) / (cz + d)
/// where ad - bc ≠ 0 (the determinant must be non-zero).
///
/// All transformations properly handle the point at infinity on the extended
/// complex plane (Riemann sphere).
#[derive(Debug, Clone, Copy)]
pub struct MobiusTransform {
    a: Complex64,
    b: Complex64,
    c: Complex64,
    d: Complex64,
}

impl MobiusTransform {
    /// Creates a new Möbius transformation.
    ///
    /// # Errors
    /// Returns `TransformError::InfiniteCoefficient` if any coefficient is infinite.
    /// Returns `TransformError::SingularTransform` if the determinant (ad - bc) is zero.
    pub fn new(a: Complex64, b: Complex64, c: Complex64, d: Complex64) -> Result<Self, TransformError> {
        if is_infinity(a) || is_infinity(b) || is_infinity(c) || is_infinity(d) {
            return Err(TransformError::InfiniteCoefficient);
        }
        
        let det = a * d - b * c;
        if det.norm() <= 1e-10 {
            return Err(TransformError::SingularTransform);
        }
        
        Ok(Self { a, b, c, d })
    }

    /// Creates the identity transformation.
    pub fn identity() -> Self {
        Self::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).expect("Identity transformation should always be valid")
    }

    /// Applies the transformation to a complex number.
    ///
    /// Properly handles the point at infinity according to the rules:
    /// - If z is infinity and c ≠ 0, a == 0: returns 0
    /// - If z is infinity and c == 0, a ≠ 0: returns infinity
    /// - If z is infinity and c ≠ 0, a ≠ 0: returns a/c
    /// - If denominator (cz + d) approaches zero: returns infinity
    /// - Otherwise: returns (az + b) / (cz + d)
    pub fn apply(&self, z: Complex64) -> Complex64 {
        // Handle input infinity
        if is_infinity(z) {
            let c_is_zero = self.c.norm() < 1e-10;
            let a_is_zero = self.a.norm() < 1e-10;
            
            if c_is_zero && !a_is_zero {
                // Case: c == 0 && a != 0 -> infinity
                return normalize_infinity(COMPLEX_INFINITY);
            } else if !c_is_zero && a_is_zero {
                // Case: c != 0 && a == 0 -> 0
                return Complex64::new(0.0, 0.0);
            } else if !c_is_zero && !a_is_zero {
                // Case: c != 0 && a != 0 -> a/c
                return normalize_infinity(self.a / self.c);
            }
            // Case: c == 0 && a == 0 should have been caught at construction
            // (degenerate transform), but return infinity as fallback
            return normalize_infinity(COMPLEX_INFINITY);
        }
        
        let numerator = self.a * z + self.b;
        let denominator = self.c * z + self.d;
        
        if denominator.norm() < 1e-10 {
            // Point maps to infinity
            normalize_infinity(COMPLEX_INFINITY)
        } else {
            let result = numerator / denominator;
            normalize_infinity(result)
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
    ///
    /// The composition of two valid Möbius transformations is always a valid
    /// Möbius transformation, so this operation cannot fail.
    pub fn compose(&self, other: &MobiusTransform) -> MobiusTransform {
        // Mathematical guarantee: composition of valid transforms is valid
        MobiusTransform::new(
            self.a * other.a + self.b * other.c,
            self.a * other.b + self.b * other.d,
            self.c * other.a + self.d * other.c,
            self.c * other.b + self.d * other.d,
        ).expect("Composition of valid transforms should always be valid")
    }

    /// Returns the inverse transformation.
    ///
    /// Since the determinant is guaranteed to be non-zero (checked at creation),
    /// the inverse always exists.
    pub fn inverse(&self) -> MobiusTransform {
        let det = self.a * self.d - self.b * self.c;
        MobiusTransform::new(
            self.d / det,
            -self.b / det,
            -self.c / det,
            self.a / det,
        ).expect("Inverse of a valid transform should always be valid")
    }

    /// Returns the determinant ad - bc.
    pub fn determinant(&self) -> Complex64 {
        self.a * self.d - self.b * self.c
    }

    /// Normalizes the transformation so that ad - bc = 1.
    ///
    /// Since the determinant is guaranteed to be non-zero, the normalization
    /// produces a valid transformation.
    pub fn normalize(&self) -> MobiusTransform {
        let det_sqrt = self.determinant().sqrt();
        MobiusTransform::new(
            self.a / det_sqrt,
            self.b / det_sqrt,
            self.c / det_sqrt,
            self.d / det_sqrt,
        ).expect("Normalization of a valid transform should always be valid")
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
    fn test_identity_at_infinity() {
        let id = MobiusTransform::identity();
        let result = id.apply(COMPLEX_INFINITY);
        assert!(is_infinity(result));
    }

    #[test]
    fn test_composition() {
        let m1 = MobiusTransform::new(
            Complex64::new(2.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).unwrap();
        let m2 = MobiusTransform::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).unwrap();
        
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
        ).unwrap();
        let inv = m.inverse();
        let z = Complex64::new(2.0, 3.0);
        
        let result = inv.apply(m.apply(z));
        assert!((result - z).norm() < 1e-10);
    }
    
    #[test]
    fn test_apply_infinity_when_c_nonzero_a_nonzero() {
        // f(z) = (2z + 1) / (z + 1)
        // c ≠ 0 && a ≠ 0, so f(∞) = a/c = 2/1 = 2
        let m = MobiusTransform::new(
            Complex64::new(2.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).unwrap();
        let result = m.apply(COMPLEX_INFINITY);
        assert!((result - Complex64::new(2.0, 0.0)).norm() < 1e-10);
    }
    
    #[test]
    fn test_apply_infinity_when_c_nonzero_a_zero() {
        // f(z) = 1 / (z + 1) (with a=0, c≠0)
        // c ≠ 0 && a == 0, so f(∞) = 0
        let m = MobiusTransform::new(
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).unwrap();
        let result = m.apply(COMPLEX_INFINITY);
        assert_eq!(result, Complex64::new(0.0, 0.0));
    }
    
    #[test]
    fn test_apply_infinity_when_c_zero_a_nonzero() {
        // f(z) = 2z + 1 (with c=0, a≠0)
        // c == 0 && a ≠ 0, so f(∞) = ∞
        let m = MobiusTransform::new(
            Complex64::new(2.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ).unwrap();
        let result = m.apply(COMPLEX_INFINITY);
        assert!(is_infinity(result));
    }
    
    #[test]
    fn test_apply_maps_to_infinity() {
        // f(z) = 1/z
        // f(0) = ∞
        let m = MobiusTransform::new(
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
        ).unwrap();
        let result = m.apply(Complex64::new(0.0, 0.0));
        assert!(is_infinity(result));
    }

    #[test]
    fn test_zero_determinant() {
        let result = MobiusTransform::new(
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(4.0, 0.0),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TransformError::SingularTransform);
    }
    
    #[test]
    fn test_infinite_coefficient() {
        let result = MobiusTransform::new(
            COMPLEX_INFINITY,
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TransformError::InfiniteCoefficient);
    }
}
