# Changelog

## [0.3.0] - 2025-08-28

### Added
- Differential equations solver (ODEs and PDEs)
  - RK4 method
  - Euler method
  - Stiff solver (implicit)
  - 2nd order ODEs
  - System of ODEs
  - Heat equation
  - Wave equation 
  - Laplace equation

### Changed
- Renamed ML module to Optimization
- Better docs
- PDE solvers more stable now

### Fixed
- Type errors in matrix ops
- Convergence issues

## [0.2.0] - 2025-07-15

### Added
- Limits (one-sided, at infinity)
- Matrix operations:
  - Basic ops (multiply, add, transpose)
  - Determinant, trace, eigenvalues
  - LU/QR/SVD decomposition
  - Linear system solver
- Arbitrary precision (BigInt/BigRational)
  - Exact rational math
  - Compute pi and e to arbitrary digits
- Optimization stuff:
  - Gradients and Hessians
  - Autodiff
  - Gradient descent
  - Taylor series
  - Newton's method
  - Lagrange multipliers
- Better integration:
  - Integration by parts
  - u-substitution
  - Partial fractions

### Improved
- Complex numbers work better
- Error handling

## [0.1.0] - 2025-05-20

Initial release!

- Expression parser (handles precedence correctly)
- Symbolic engine
- Basic math ops
- Differentiation/integration (symbolic)
- Equation solver
- Complex numbers
- ASCII plots
- Simplification
- Variables