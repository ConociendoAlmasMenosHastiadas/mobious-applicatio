# mobius-applicatio

A lean Rust library for working with Möbius transformations on the complex plane.

## Overview

Möbius transformations (also called linear fractional transformations) are conformal mappings of the form:

```
f(z) = (az + b) / (cz + d)
```

where `a`, `b`, `c`, `d` are complex numbers and `ad - bc ≠ 0`.

## Features

- Create and manipulate Möbius transformations
- Apply transformations to complex numbers and arrays
- Compose transformations
- Compute inverse transformations
- Matrix representation support

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
mobius-applicatio = "0.1.0"
```

Basic example:

```rust
use mobius_applicatio::MobiusTransform;
use num_complex::Complex64;

// Create a transformation: f(z) = (2z + 1) / (z + 1)
let transform = MobiusTransform::new(
    Complex64::new(2.0, 0.0),  // a
    Complex64::new(1.0, 0.0),  // b
    Complex64::new(1.0, 0.0),  // c
    Complex64::new(1.0, 0.0),  // d
);

// Apply to a point
let z = Complex64::new(1.0, 1.0);
let result = transform.apply(z);

// Compose transformations
let inverse = transform.inverse();
let identity = transform.compose(&inverse);
```

## Visualization Example

Run the interactive visualization tool:

```bash
cargo run --example visualize
```

This opens a GUI where you can:
- Adjust the transformation parameters (a, b, c, d)
- See the transformed complex plane grid in real-time
- Reset to identity transformation

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
