# mobius-applicatio

A lean Rust library for working with Möbius transformations on the extended complex plane (Riemann sphere).

## Overview

Möbius transformations (also called linear fractional transformations) are conformal mappings of the form:

```
f(z) = (az + b) / (cz + d)
```

where `a`, `b`, `c`, `d` are complex numbers and `ad - bc ≠ 0`.

This library provides mathematically rigorous handling of the extended complex plane, including proper treatment of infinity as a single point regardless of sign.

## Features

- **Core Transformations**: Create and manipulate Möbius transformations with error handling
- **Extended Complex Plane**: Graceful handling of infinity in all operations
- **Transform Operations**: Apply transformations to points, compose, invert, and normalize
- **Plane Functions**: Boolean grid functions for visualization (vertical, horizontal, radial, angular)
- **Type Safety**: Non-panicking API using `Result` for invalid transformations
- **Zero GUI Dependencies**: Core library is lean - visualization tools only in examples

## What's New in 0.1.1

### Architecture Improvements
- **Modular Structure**: Separated transforms, plane functions, and complex utilities into dedicated modules
- **Error Handling**: Replaced panics with `Result<T, TransformError>` for transform creation
- **Immutable Fields**: Transform coefficients are now private and immutable

### Infinity Support
- **`complex_utils` module**: `COMPLEX_INFINITY` constant, `is_infinity()`, and `normalize_infinity()` helpers
- **Proper `apply(infinity)` behavior**: Handles all cases (c≠0/a=0 → 0, c=0/a≠0 → ∞, c≠0/a≠0 → a/c)
- **Plane functions respect infinity**: Grid functions return appropriate values for the point at infinity

### Visualization Functions
- **Boolean grid functions**: `vertical_grid()`, `horizontal_grid()`, `radial_grid()`, `angular_grid()`
- **Separation of concerns**: Library provides math, examples handle rendering

## Dependencies

The core library has minimal dependencies:
- `num-complex` - Complex number support
- `ndarray` - N-dimensional arrays
- `ndarray-linalg` - Linear algebra operations

GUI dependencies (egui/eframe) are only used in examples and are not included in the compiled library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mobius-applicatio = "0.1.1"
```

Basic example:

```rust
use mobius_applicatio::{MobiusTransform, TransformError};
use num_complex::Complex64;

// Create a transformation: f(z) = (2z + 1) / (z + 1)
let transform = MobiusTransform::new(
    Complex64::new(2.0, 0.0),  // a
    Complex64::new(1.0, 0.0),  // b
    Complex64::new(1.0, 0.0),  // c
    Complex64::new(1.0, 0.0),  // d
)?; // Returns Result<MobiusTransform, TransformError>

// Apply to a point
let z = Complex64::new(1.0, 1.0);
let result = transform.apply(z);

// Compose transformations
let inverse = transform.inverse();
let identity = transform.compose(&inverse);
```

Working with infinity:

```rust
use mobius_applicatio::complex_utils::{COMPLEX_INFINITY, is_infinity};

let result = transform.apply(COMPLEX_INFINITY);
if is_infinity(result) {
    println!("Maps to infinity");
}
```

Using plane functions:

```rust
use mobius_applicatio::plane_functions;
use num_complex::Complex64;

let z = Complex64::new(1.0, 2.0);
if plane_functions::vertical_grid(z, 0.5, 0.01) {
    // Point is on a vertical grid line
}
```

## Visualization Example

Run the interactive visualization tool:

```bash
cargo run --example visualize
```

This displays transformed grid patterns on the complex plane using different Möbius transformations.

## Running Tests

```bash
cargo test
```

## Project Structure

- `src/lib.rs` - Core library implementation (lean, no GUI dependencies)
- `examples/visualize.rs` - Interactive visualization tool (uses egui)
- `Cargo.toml` - Dependencies separated: core vs dev-dependencies

## License

MIT or Apache-2.0 (choose your preference)
