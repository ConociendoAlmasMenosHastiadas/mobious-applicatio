//! Functions for visualizing the complex plane with various grid patterns.
//!
//! These functions test whether a point in the complex plane falls on a grid line
//! and return boolean values, allowing the caller to decide how to render them.

use num_complex::Complex64;
use crate::complex_utils::is_infinity;

/// Test if a point falls on vertical grid lines.
///
/// Returns true for points at regular intervals along the real axis.
/// Vertical lines extend to infinity, so this returns true for the point at infinity.
///
/// # Arguments
/// * `z` - The complex number to test
/// * `period` - The spacing between grid lines
/// * `thickness` - The half-width of each grid line
pub fn vertical_grid(z: Complex64, period: f64, thickness: f64) -> bool {
    if is_infinity(z) {
        return true; // Vertical lines pass through infinity
    }
    
    let re_mod = (z.re.abs() % period).abs();
    let half_period = period / 2.0;
    re_mod >= half_period - thickness && re_mod < half_period + thickness
}

/// Test if a point falls on horizontal grid lines.
///
/// Returns true for points at regular intervals along the imaginary axis.
/// Horizontal lines extend to infinity, so this returns true for the point at infinity.
///
/// # Arguments
/// * `z` - The complex number to test
/// * `period` - The spacing between grid lines
/// * `thickness` - The half-width of each grid line
pub fn horizontal_grid(z: Complex64, period: f64, thickness: f64) -> bool {
    if is_infinity(z) {
        return true; // Horizontal lines pass through infinity
    }
    
    let im_mod = (z.im.abs() % period).abs();
    let half_period = period / 2.0;
    im_mod >= half_period - thickness && im_mod < half_period + thickness
}

/// Test if a point falls on radial grid circles.
///
/// Returns true for points at regular intervals of magnitude from the origin.
/// Circles do not reach infinity (they are always at finite distance from the origin),
/// so this returns false for the point at infinity.
///
/// # Arguments
/// * `z` - The complex number to test
/// * `period` - The spacing between circles
/// * `thickness` - The half-width of each circle line
pub fn radial_grid(z: Complex64, period: f64, thickness: f64) -> bool {
    if is_infinity(z) {
        return false; // Circles don't reach infinity
    }
    
    let magnitude = z.norm();
    let mag_mod = magnitude % period;
    let half_period = period / 2.0;
    mag_mod >= half_period - thickness && mag_mod < half_period + thickness
}

/// Test if a point falls on angular grid lines.
///
/// Returns true for points at regular angular intervals from the positive real axis.
/// Every angular ray passes through infinity, so this returns true for the point at infinity.
///
/// # Arguments
/// * `z` - The complex number to test
/// * `period` - The angular spacing between lines (in radians)
/// * `thickness` - The angular half-width of each line (in radians)
pub fn angular_grid(z: Complex64, period: f64, thickness: f64) -> bool {
    if is_infinity(z) {
        return true; // All angular rays pass through infinity
    }
    
    let angle = z.arg(); // Returns angle in radians [-π, π]
    let angle_positive = if angle < 0.0 { angle + 2.0 * std::f64::consts::PI } else { angle };
    let angle_mod = angle_positive % period;
    let half_period = period / 2.0;
    angle_mod >= half_period - thickness && angle_mod < half_period + thickness
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::complex_utils::COMPLEX_INFINITY;

    #[test]
    fn test_vertical_grid() {
        // Test point on vertical grid line
        let z = Complex64::new(0.5, 1.0);
        assert!(vertical_grid(z, 0.2, 0.01));
        
        // Test point off vertical grid line
        let z = Complex64::new(0.45, 1.0);
        assert!(!vertical_grid(z, 0.2, 0.01));
        
        // Test infinity (vertical lines pass through infinity)
        assert!(vertical_grid(COMPLEX_INFINITY, 0.2, 0.01));
    }

    #[test]
    fn test_horizontal_grid() {
        // Test point on horizontal grid line
        let z = Complex64::new(1.0, 0.5);
        assert!(horizontal_grid(z, 0.2, 0.01));
        
        // Test point off horizontal grid line
        let z = Complex64::new(1.0, 0.45);
        assert!(!horizontal_grid(z, 0.2, 0.01));
        
        // Test infinity (horizontal lines pass through infinity)
        assert!(horizontal_grid(COMPLEX_INFINITY, 0.2, 0.01));
    }

    #[test]
    fn test_radial_grid() {
        // Test point on radial grid
        let z = Complex64::new(0.5, 0.0);
        assert!(radial_grid(z, 0.2, 0.01));
        
        // Test point off radial grid
        let z = Complex64::new(0.45, 0.0);
        assert!(!radial_grid(z, 0.2, 0.01));
        
        // Test infinity (circles don't reach infinity)
        assert!(!radial_grid(COMPLEX_INFINITY, 0.2, 0.01));
    }

    #[test]
    fn test_angular_grid() {
        // Test point on angular grid
        // With period π/12, grid lines appear at angles where (angle % period) ≈ period/2
        // So at π/24, π/12 + π/24 = 3π/24, 5π/24, etc.
        let period = PI / 12.0;
        let thickness = 0.02;
        
        // Test at π/24 (which is period/2, should be on the grid line)
        let angle = period / 2.0;
        let z = Complex64::new(angle.cos(), angle.sin());
        assert!(angular_grid(z, period, thickness));
        
        // Test point off angular grid (at period/4, not near any grid line)
        let angle = period / 4.0;
        let z = Complex64::new(angle.cos(), angle.sin());
        assert!(!angular_grid(z, period, 0.01));
        
        // Test infinity (all angular rays pass through infinity)
        assert!(angular_grid(COMPLEX_INFINITY, period, thickness));
    }
}
