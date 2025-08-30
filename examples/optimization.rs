use mathcore::ml::Optimization;
use mathcore::parser::Parser;
use std::collections::HashMap;

fn main() {
    println!("Optimization Examples\n");

    gradient_descent_example();
    newton_method_example();
    lagrange_multipliers_example();
    taylor_series_example();
}

fn gradient_descent_example() {
    println!("=== Gradient Descent Optimization ===");

    // Minimize f(x,y) = (x-3)^2 + (y-2)^2
    let loss_fn = Parser::parse("(x-3)^2 + (y-2)^2").unwrap();

    // Starting point
    let mut initial_params = HashMap::new();
    initial_params.insert("x".to_string(), 0.0);
    initial_params.insert("y".to_string(), 0.0);

    println!("Minimizing f(x,y) = (x-3)² + (y-2)²");
    println!("Starting point: x=0, y=0");

    let optimized = Optimization::gradient_descent(
        &loss_fn,
        initial_params,
        0.1, // learning rate
        100, // iterations
    )
    .unwrap();

    println!(
        "Optimized point: x={:.4}, y={:.4}",
        optimized["x"], optimized["y"]
    );
    println!("Expected minimum: x=3, y=2\n");
}

fn newton_method_example() {
    println!("=== Newton's Method for Optimization ===");

    // Find minimum of f(x) = x^4 - 2x^2 + x
    let func = Parser::parse("x^4 - 2*x^2 + x").unwrap();

    println!("Finding critical points of f(x) = x⁴ - 2x² + x");

    let critical_points = vec![-1.5, 0.0, 1.0];

    for initial in critical_points {
        let result = Optimization::optimize_newton(
            &func, "x", initial, 1e-8, // tolerance
            50,   // max iterations
        )
        .unwrap();

        println!(
            "Starting from x={:.1}: converged to x={:.6}",
            initial, result
        );
    }
    println!();
}

fn lagrange_multipliers_example() {
    println!("=== Lagrange Multipliers ===");

    // Maximize f(x,y) = x*y subject to x^2 + y^2 = 1
    let objective = Parser::parse("x*y").unwrap();
    let constraint = Parser::parse("x^2 + y^2 - 1").unwrap();

    println!("Maximize f(x,y) = xy");
    println!("Subject to: x² + y² = 1");

    let gradient_eqs = Optimization::lagrange_multipliers(
        &objective,
        &[constraint],
        &["x".to_string(), "y".to_string()],
    )
    .unwrap();

    println!("\nLagrangian gradient equations:");
    for (i, eq) in gradient_eqs.iter().enumerate() {
        println!("  ∂L/∂var_{} = {}", i, eq);
    }

    println!("\nAnalytical solution: x = ±1/√2, y = ±1/√2");
    println!("Maximum value: f = ±0.5\n");
}

fn taylor_series_example() {
    println!("=== Taylor Series Expansion ===");

    let functions = vec![
        ("sin(x)", 0.0, 7),
        ("exp(x)", 0.0, 5),
        ("ln(1+x)", 0.0, 5),
        ("1/(1-x)", 0.0, 5),
    ];

    for (func_str, center, order) in functions {
        let func = Parser::parse(func_str).unwrap();
        let taylor = Optimization::taylor_series(&func, "x", center, order).unwrap();

        println!("Taylor series of {} around x={}:", func_str, center);
        println!("  {}", taylor);
        println!();
    }

    // Verify accuracy
    println!("=== Taylor Series Accuracy ===");
    let sin_expr = Parser::parse("sin(x)").unwrap();
    let sin_taylor = Optimization::taylor_series(&sin_expr, "x", 0.0, 9).unwrap();

    println!("sin(x) Taylor series (order 9) vs actual:");
    println!("x\tTaylor\t\tActual\t\tError");

    use mathcore::engine::Engine;
    let engine = Engine::new();

    for x in [0.1, 0.5, 1.0, 1.5] {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), x);

        let taylor_val = engine.evaluate_with_vars(&sin_taylor, &vars).unwrap();
        let actual = x.sin();

        if let mathcore::Expr::Number(t) = taylor_val {
            let error = (t - actual).abs();
            println!("{:.1}\t{:.6}\t{:.6}\t{:.2e}", x, t, actual, error);
        }
    }
}
