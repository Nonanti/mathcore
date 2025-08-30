use mathcore::{Expr, MathCore};
use std::collections::HashMap;

#[test]
fn test_complete_workflow() {
    let math = MathCore::new();

    // Parse expression
    let _expr = MathCore::parse("x^2 + 2*x + 1").unwrap();

    // Evaluate with variables
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 3.0);
    let result = math.evaluate_with_vars("x^2 + 2*x + 1", &vars).unwrap();
    assert_eq!(result, 16.0);

    // Differentiate
    let _derivative = MathCore::differentiate("x^2 + 2*x + 1", "x").unwrap();

    // Integrate
    let _integral = MathCore::integrate("2*x + 2", "x").unwrap();

    // Solve equation
    let roots = MathCore::solve("x^2 + 2*x + 1", "x").unwrap();
    assert_eq!(roots.len(), 1); // Perfect square has one root
}

#[test]
fn test_complex_calculations() {
    let math = MathCore::new();

    // Composite functions
    let result = math.calculate("sin(pi/2) + cos(0) + exp(0)").unwrap();
    assert!((result - 3.0).abs() < 1e-10);

    // Nested operations
    let result = math.calculate("sqrt(16) * log(100, 10)").unwrap();
    assert!((result - 8.0).abs() < 1e-10);
}

#[test]
fn test_error_handling() {
    let math = MathCore::new();

    // Division by zero
    assert!(math.calculate("1/0").is_err());

    // Undefined variable
    assert!(math.calculate("undefined_var + 1").is_err());

    // Invalid syntax
    assert!(MathCore::parse("2 ++ 3").is_err());
}

#[test]
fn test_scientific_constants() {
    let math = MathCore::new();

    let pi = math.calculate("pi").unwrap();
    assert!((pi - std::f64::consts::PI).abs() < 1e-10);

    let e = math.calculate("e").unwrap();
    assert!((e - std::f64::consts::E).abs() < 1e-10);

    let tau = math.calculate("tau").unwrap();
    assert!((tau - std::f64::consts::TAU).abs() < 1e-10);
}

#[test]
fn test_factorials_and_powers() {
    let math = MathCore::new();

    assert_eq!(math.calculate("5!").unwrap(), 120.0);
    assert_eq!(math.calculate("2^10").unwrap(), 1024.0);
    assert_eq!(math.calculate("27^(1/3)").unwrap(), 3.0);
}

#[test]
fn test_equation_solving() {
    // Linear equation
    let roots = MathCore::solve("3*x - 9", "x").unwrap();
    assert_eq!(roots.len(), 1);
    if let Expr::Number(n) = &roots[0] {
        assert!((n - 3.0).abs() < 1e-10);
    }

    // Quadratic equation
    let roots = MathCore::solve("x^2 - 5*x + 6", "x").unwrap();
    assert_eq!(roots.len(), 2);

    // Complex roots
    let roots = MathCore::solve("x^2 + 1", "x").unwrap();
    assert_eq!(roots.len(), 2);
    assert!(matches!(&roots[0], Expr::Complex(_)));
}

#[test]
fn test_symbolic_operations() {
    // Differentiation chain rule
    let deriv = MathCore::differentiate("sin(x^2)", "x").unwrap();
    println!("d/dx(sin(x^2)) = {}", deriv);

    // Integration by substitution
    let integral = MathCore::integrate("2*x", "x").unwrap();
    println!("∫2x dx = {}", integral);

    // Simplification
    let simplified = MathCore::simplify("x + x + x").unwrap();
    println!("x + x + x = {}", simplified);
}

#[test]
fn test_numerical_methods() {
    // Numerical integration (Simpson's rule)
    let area = MathCore::numerical_integrate("x^2", "x", 0.0, 1.0).unwrap();
    assert!((area - 1.0 / 3.0).abs() < 1e-6);

    // Numerical integration of trig function
    let area = MathCore::numerical_integrate("sin(x)", "x", 0.0, 3.14159).unwrap();
    assert!((area - 2.0).abs() < 1e-4);
}
