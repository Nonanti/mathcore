# Social Media Posts for MathCore

## Reddit r/rust Post

```markdown
Title: [ANN] MathCore 0.3.0 - Symbolic math library for Rust (like SymPy but for Rust!)

Hey everyone! 

I'm excited to share MathCore, a symbolic mathematics library I've been working on. It brings computer algebra system capabilities to Rust.

## What it does

- **Symbolic math**: Parse and manipulate expressions like "x^2 + 2*x + 1"
- **Calculus**: Differentiate and integrate symbolically
- **Equation solving**: From simple linear to complex polynomial equations  
- **Differential equations**: Solve ODEs and PDEs
- **Matrix operations**: Powered by nalgebra
- **Arbitrary precision**: BigInt/BigRational for exact arithmetic
- **And more**: FFT, complex numbers, numerical methods

## Quick example

```rust
use mathcore::MathCore;

fn main() {
    let math = MathCore::new();
    
    // Symbolic differentiation
    let derivative = MathCore::differentiate("sin(x^2)", "x").unwrap();
    println!("{}", derivative); // 2*x*cos(x^2)
    
    // Solve equations
    let roots = MathCore::solve("x^2 - 5*x + 6", "x").unwrap();
    // roots: [2, 3]
    
    // Evaluate with variables
    let result = math.calculate("2*pi*r", &[("r", 5.0)]).unwrap();
    // result: 31.415...
}
```

## Why another math library?

I needed symbolic math in Rust for a physics simulation project. While there are great numerical libraries, I couldn't find a comprehensive CAS for Rust. MathCore fills that gap.

## Links

- **Crates.io**: https://crates.io/crates/mathcore
- **GitHub**: https://github.com/Nonanti/mathcore  
- **Docs**: https://docs.rs/mathcore

Would love to hear your feedback and use cases! PRs welcome 🦀
```

## Twitter/X Thread

```
🚀 Just released MathCore v0.3.0!

A symbolic math library for Rust that brings computer algebra capabilities to the ecosystem.

Think SymPy but for Rust 🦀

🧵 1/5
```

```
✨ Features:
• Parse & manipulate math expressions
• Symbolic differentiation/integration
• Solve equations & differential equations
• Matrix operations with nalgebra
• Arbitrary precision arithmetic
• WASM support for browsers

2/5
```

```
💻 Quick example:

use mathcore::MathCore;

// Differentiate symbolically
let d = MathCore::differentiate("sin(x^2)", "x")?;
// Returns: "2*x*cos(x^2)"

// Solve equations
let roots = MathCore::solve("x^2-5*x+6", "x")?;
// Returns: [2, 3]

3/5
```

```
🎯 Perfect for:
• Scientific computing
• Game physics
• Educational tools
• Data analysis
• Any project needing symbolic math

Built with performance and safety in mind!

4/5
```

```
📦 Get started:
cargo add mathcore

📚 Links:
• Crates.io: https://crates.io/crates/mathcore
• GitHub: https://github.com/Nonanti/mathcore
• Docs: https://docs.rs/mathcore

⭐ If you find it useful, consider starring on GitHub!

#rustlang #opensource #mathematics

5/5
```

## Hacker News Post

```
Title: Show HN: MathCore – A Symbolic Math Library for Rust

I built MathCore to bring symbolic mathematics capabilities to Rust, inspired by systems like SymPy and Mathematica.

The library handles symbolic expression manipulation, calculus, equation solving, and differential equations. It uses nom for parsing, nalgebra for matrix operations, and supports arbitrary precision arithmetic.

Key design decisions:
- Zero-cost abstractions where possible
- Optional parallelization with Rayon
- WASM support for browser deployment
- Minimal dependencies for core functionality

Example:
```rust
let math = MathCore::new();
let derivative = MathCore::differentiate("sin(x^2)", "x")?;
let integral = MathCore::integrate("2*x", "x")?;
let roots = MathCore::solve("x^2 - 5*x + 6", "x")?;
```

The parser handles standard mathematical notation and precedence rules. The engine can evaluate expressions symbolically or numerically with variable substitution.

One interesting challenge was implementing the simplification engine - balancing between aggressive simplification and preserving mathematical structure for further manipulation.

Future plans include GPU acceleration for numerical methods and a Jupyter kernel for interactive use.

GitHub: https://github.com/Nonanti/mathcore
Crates.io: https://crates.io/crates/mathcore

Would appreciate feedback on the API design and feature requests!
```

## Dev.to Article Outline

```markdown
# Building a Computer Algebra System in Rust: The MathCore Story

## Introduction
- Why Rust needs symbolic math
- The gap in the ecosystem

## Architecture Overview
- Parser (nom)
- Expression tree (AST)
- Evaluation engine
- Simplification strategies

## Key Features Deep Dive
1. Symbolic Differentiation
2. Pattern Matching for Integration
3. Equation Solving Algorithms
4. Numerical Methods

## Code Examples
[Detailed tutorials]

## Performance Considerations
- Memory management
- Parallel computation
- WASM optimizations

## Lessons Learned
- Rust's type system advantages
- Error handling strategies
- API design decisions

## Future Roadmap
- GPU acceleration
- More numerical methods
- Interactive notebooks

## Conclusion
- Call to action
- Links to resources
```

## LinkedIn Post

```
🎉 Excited to announce the release of MathCore v0.3.0!

MathCore is a symbolic mathematics library for Rust that I've been developing to bring computer algebra capabilities to the Rust ecosystem.

Key features:
✅ Symbolic expression manipulation
✅ Calculus (differentiation & integration)
✅ Equation solving
✅ Differential equations
✅ Arbitrary precision arithmetic
✅ WASM support

This project combines my passion for mathematics and systems programming. It's designed to be both powerful and ergonomic, leveraging Rust's safety guarantees for reliable scientific computing.

Perfect for researchers, educators, game developers, and anyone needing symbolic math in Rust.

Check it out:
🔗 GitHub: https://github.com/Nonanti/mathcore
📦 Crates.io: https://crates.io/crates/mathcore

#Rust #OpenSource #Mathematics #Programming #ScientificComputing
```