# MathCore

[![Crates.io](https://img.shields.io/crates/v/mathcore.svg)](https://crates.io/crates/mathcore)
[![Documentation](https://docs.rs/mathcore/badge.svg)](https://docs.rs/mathcore)
[![Build Status](https://github.com/Nonanti/mathcore/workflows/CI/badge.svg)](https://github.com/Nonanti/mathcore/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A symbolic math library for Rust. Think of it as a computer algebra system (CAS) that can do symbolic differentiation, integration, equation solving, and more.

## What it does

### Basic stuff
- Parse math expressions from strings (with proper precedence)
- Work with symbols, not just numbers
- Differentiate and integrate symbolically
- Solve equations (linear, quadratic, and some higher degree)
- Complex number support
- ASCII plots (for quick visualization)
- Expression simplification
- Variables and substitution

### Fancier features
- Limits (including one-sided and at infinity)
- Matrix operations and linear algebra
- Arbitrary precision arithmetic (BigInt/BigRational)
- Optimization (gradients, Hessian, autodiff)
- Taylor series expansion
- Numerical methods (Newton's method, gradient descent)
- ODEs and PDEs solvers
- FFT and signal processing

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mathcore = "0.1.0"
```

## Quick example

```rust
use mathcore::MathCore;
use std::collections::HashMap;

fn main() {
    let math = MathCore::new();
    
    // basic arithmetic
    let result = math.calculate("2 + 3 * 4").unwrap();
    println!("2 + 3 * 4 = {}", result);  // 14
    
    // take derivatives
    let derivative = MathCore::differentiate("x^2 + 2*x + 1", "x").unwrap();
    println!("d/dx(x^2 + 2*x + 1) = {}", derivative);  // 2*x + 2
    
    // solve equations
    let roots = MathCore::solve("x^2 - 4", "x").unwrap();
    println!("roots: {:?}", roots);  // [2, -2]
}
```

## Advanced Usage

### Limits

```rust
use mathcore::calculus::limits::{Limits, LimitDirection};

let expr = MathCore::parse("sin(x)/x").unwrap();
let limit = Limits::limit(&expr, "x", 0.0, LimitDirection::Both).unwrap();
println!("lim(x→0) sin(x)/x = {}", limit); // Should be 1

// Check continuity
let continuous = Limits::is_continuous_at(&expr, "x", 1.0).unwrap();
println!("Function is continuous: {}", continuous);
```

### Matrix Operations

```rust
use mathcore::matrix::{SymbolicMatrix, LinearAlgebra};
use nalgebra::{DMatrix, DVector};

// Symbolic matrices
let matrix = SymbolicMatrix::from_vec(vec![
    vec![1.0, 2.0],
    vec![3.0, 4.0],
]).unwrap();

let det = matrix.determinant().unwrap();
println!("Determinant: {}", det);

// Solve linear system Ax = b
let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
let b = DVector::from_row_slice(&[5.0, 11.0]);
let solution = LinearAlgebra::solve_system(&a, &b).unwrap();
println!("Solution: {:?}", solution);
```

### Arbitrary Precision

```rust
use mathcore::precision::{PrecisionNumber, ArbitraryPrecision};

// Exact rational arithmetic
let a = PrecisionNumber::from_str_with_precision("1/3").unwrap();
let b = PrecisionNumber::from_str_with_precision("1/6").unwrap();
let sum = a.add(&b);
println!("1/3 + 1/6 = {}", sum); // Outputs: 1/2

// Compute π with arbitrary precision
let pi = ArbitraryPrecision::compute_pi(100);
println!("π ≈ {}", pi);
```

### Optimization and Calculus

```rust
use mathcore::ml::{Optimization, SymbolicIntegration};

// Compute gradient
let loss = MathCore::parse("x^2 + y^2").unwrap();
let vars = vec!["x".to_string(), "y".to_string()];
let gradient = Optimization::gradient(&loss, &vars).unwrap();
println!("∇f = [{}, {}]", gradient[0], gradient[1]);

// Taylor series expansion
let func = MathCore::parse("exp(x)").unwrap();
let taylor = Optimization::taylor_series(&func, "x", 0.0, 5).unwrap();
println!("Taylor series: {}", taylor);

// Gradient descent optimization
let mut params = HashMap::new();
params.insert("x".to_string(), 10.0);
params.insert("y".to_string(), 10.0);
let optimized = Optimization::gradient_descent(
    &loss, params, 0.1, 100
).unwrap();
println!("Optimized parameters: {:?}", optimized);
```

### Working with Variables

```rust
let math = MathCore::new();
let mut vars = HashMap::new();
vars.insert("a".to_string(), 3.0);
vars.insert("b".to_string(), 4.0);

let result = math.evaluate_with_vars("sqrt(a^2 + b^2)", &vars).unwrap();
println!("Distance: {}", result);
```

### Symbolic Integration

```rust
let integral = MathCore::integrate("x^2", "x").unwrap();
println!("∫x² dx = {}", integral);

// Numerical integration
let area = MathCore::numerical_integrate("x^2", "x", 0.0, 1.0).unwrap();
println!("∫₀¹ x² dx = {}", area);
```

### Function Plotting

```rust
let plot = MathCore::plot_ascii("sin(x)", "x", -3.14, 3.14, 60, 20).unwrap();
println!("{}", plot);
```

### Complex Numbers

```rust
let math = MathCore::new();
let result = math.evaluate("(3+4i) * (2-i)").unwrap();
println!("(3+4i) * (2-i) = {}", result);
```

## Supported Functions

### Arithmetic Operations
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Power: `^`
- Modulo: `%`
- Factorial: `!`
- Absolute value: `|x|`

### Trigonometric Functions
- `sin(x)`, `cos(x)`, `tan(x)`
- `sec(x)` (through derivatives)

### Exponential & Logarithmic
- `exp(x)` - e^x
- `ln(x)` - Natural logarithm
- `log(x, base)` - Logarithm with custom base
- `sqrt(x)` - Square root

### Utility Functions
- `min(a, b, ...)` - Minimum value
- `max(a, b, ...)` - Maximum value
- `abs(x)` - Absolute value

## Mathematical Constants

The following constants are predefined:
- `pi` - π (3.14159...)
- `e` - Euler's number (2.71828...)
- `tau` - τ = 2π (6.28318...)

## Expression Syntax

### Basic Examples
```
2 + 3 * 4           # Arithmetic
x^2 - 5*x + 6       # Polynomial
sin(x) + cos(x)     # Trigonometric
e^x                 # Exponential (using constant e)
3! + 4!            # Factorials
|x - 5|            # Absolute value
3 + 4i             # Complex numbers
```

### Differentiation
```rust
MathCore::differentiate("sin(x) * x^2", "x")
// Returns: (cos(x) * x^2 + sin(x) * 2*x)
```

### Integration
```rust
MathCore::integrate("2*x", "x")
// Returns: x^2
```

### Equation Solving
```rust
MathCore::solve("x^2 + x - 6", "x")
// Returns: [2, -3]
```

## Performance

Pretty fast. Uses LTO in release builds. Some rough numbers:
- Expression parsing: ~1μs 
- Differentiation: ~10μs for polynomials
- Matrix ops use nalgebra (which uses BLAS when available)
- Exact arithmetic with rationals (no precision loss)

## When to use this

- Scientific computing (physics simulations, engineering calcs)
- ML/optimization (automatic differentiation)
- Education (demonstrating calculus concepts)
- Financial calculations (need exact arithmetic)
- Any time you need symbolic math in Rust

## Contributing

PRs welcome! 

```bash
# run tests
cargo test

# benchmarks
cargo bench

# docs
cargo doc --open
```

## License

MIT

© 2025 Nonanti

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.