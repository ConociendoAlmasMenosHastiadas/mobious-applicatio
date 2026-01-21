//! Utilities for working with complex numbers on the extended complex plane.
//!
//! The extended complex plane includes all points in ℂ plus a single point at infinity.

use num_complex::Complex64;

/// The point at infinity on the extended complex plane.
///
/// On the extended complex plane (Riemann sphere), there is a single point at infinity
/// regardless of the direction. This constant represents that point.
pub const COMPLEX_INFINITY: Complex64 = Complex64::new(f64::INFINITY, f64::INFINITY);

/// Tests whether a complex number represents the point at infinity.
///
/// Returns true if either the real or imaginary component is infinite.
/// This follows the convention that any infinity (including signed infinities)
/// represents the same point at infinity on the extended complex plane.
///
/// # Examples
/// ```
/// use mobius_applicatio::complex_utils::{is_infinity, COMPLEX_INFINITY};
/// use num_complex::Complex64;
///
/// assert!(is_infinity(COMPLEX_INFINITY));
/// assert!(is_infinity(Complex64::new(f64::INFINITY, 0.0)));
/// assert!(is_infinity(Complex64::new(0.0, f64::NEG_INFINITY)));
/// assert!(!is_infinity(Complex64::new(1.0, 2.0)));
/// ```
pub fn is_infinity(z: Complex64) -> bool {
    z.re.is_infinite() || z.im.is_infinite()
}

/// Normalizes any infinity representation to the standard COMPLEX_INFINITY.
///
/// This ensures that signed infinities (±∞) are converted to the canonical
/// representation of infinity on the extended complex plane.
///
/// # Examples
/// ```
/// use mobius_applicatio::complex_utils::{normalize_infinity, COMPLEX_INFINITY};
/// use num_complex::Complex64;
///
/// let z = Complex64::new(f64::NEG_INFINITY, 0.0);
/// assert_eq!(normalize_infinity(z), COMPLEX_INFINITY);
///
/// let w = Complex64::new(1.0, 2.0);
/// assert_eq!(normalize_infinity(w), w);
/// ```
pub fn normalize_infinity(z: Complex64) -> Complex64 {
    if is_infinity(z) {
        COMPLEX_INFINITY
    } else {
        z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_infinity() {
        assert!(is_infinity(COMPLEX_INFINITY));
        assert!(is_infinity(Complex64::new(f64::INFINITY, 0.0)));
        assert!(is_infinity(Complex64::new(f64::NEG_INFINITY, 0.0)));
        assert!(is_infinity(Complex64::new(0.0, f64::INFINITY)));
        assert!(is_infinity(Complex64::new(0.0, f64::NEG_INFINITY)));
        assert!(is_infinity(Complex64::new(f64::INFINITY, f64::INFINITY)));
        assert!(is_infinity(Complex64::new(f64::NEG_INFINITY, f64::NEG_INFINITY)));
        
        assert!(!is_infinity(Complex64::new(0.0, 0.0)));
        assert!(!is_infinity(Complex64::new(1.0, 2.0)));
        assert!(!is_infinity(Complex64::new(1e100, 1e100)));
    }

    #[test]
    fn test_normalize_infinity() {
        // All infinities normalize to COMPLEX_INFINITY
        assert_eq!(normalize_infinity(Complex64::new(f64::INFINITY, 0.0)), COMPLEX_INFINITY);
        assert_eq!(normalize_infinity(Complex64::new(f64::NEG_INFINITY, 0.0)), COMPLEX_INFINITY);
        assert_eq!(normalize_infinity(Complex64::new(0.0, f64::INFINITY)), COMPLEX_INFINITY);
        assert_eq!(normalize_infinity(Complex64::new(0.0, f64::NEG_INFINITY)), COMPLEX_INFINITY);
        
        // Finite values unchanged
        let z = Complex64::new(1.0, 2.0);
        assert_eq!(normalize_infinity(z), z);
    }
}
