use mathcore::MathCore;
use std::collections::HashMap;

fn main() {
    println!("MathCore Demonstration\n");

    let math = MathCore::new();

    println!("=== Basic Arithmetic ===");
    println!("2 + 3 * 4 = {}", math.calculate("2 + 3 * 4").unwrap());
    println!("(2 + 3) * 4 = {}", math.calculate("(2 + 3) * 4").unwrap());
    println!("2^8 = {}", math.calculate("2^8").unwrap());
    println!("5! = {}", math.calculate("5!").unwrap());

    println!("\n=== Trigonometric Functions ===");
    println!("sin(0) = {}", math.calculate("sin(0)").unwrap());
    println!("cos(pi) = {}", math.calculate("cos(pi)").unwrap());
    println!("tan(pi/4) = {:.4}", math.calculate("tan(pi/4)").unwrap());

    println!("\n=== Complex Numbers ===");
    let complex_result = MathCore::parse("3+4i").unwrap();
    println!("Complex number parsed: {}", complex_result);

    println!("\n=== Variables ===");
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 3.0);
    vars.insert("y".to_string(), 4.0);
    let pythagoras = math.evaluate_with_vars("sqrt(x^2 + y^2)", &vars).unwrap();
    println!("sqrt(3² + 4²) = {}", pythagoras);

    println!("\n=== Differentiation ===");
    let derivatives = vec![
        ("x^2", "x"),
        ("x^3 + 2*x^2 + x + 1", "x"),
        ("sin(x)", "x"),
        ("x * sin(x)", "x"),
        ("e^x", "x"),
    ];

    for (expr, var) in derivatives {
        let deriv = MathCore::differentiate(expr, var).unwrap();
        println!("d/d{}({}) = {}", var, expr, deriv);
    }

    println!("\n=== Integration ===");
    let integrals = vec![
        ("x", "x"),
        ("x^2", "x"),
        ("2*x + 3", "x"),
        ("sin(x)", "x"),
        ("cos(x)", "x"),
    ];

    for (expr, var) in integrals {
        let integral = MathCore::integrate(expr, var).unwrap();
        println!("∫{} d{} = {}", expr, var, integral);
    }

    println!("\n=== Numerical Integration ===");
    let area = MathCore::numerical_integrate("x^2", "x", 0.0, 1.0).unwrap();
    println!("∫₀¹ x² dx = {:.6}", area);

    let area = MathCore::numerical_integrate("sin(x)", "x", 0.0, 3.14159).unwrap();
    println!("∫₀^π sin(x) dx = {:.6}", area);

    println!("\n=== Equation Solving ===");
    let equations = vec!["x^2 - 4", "x^2 + x - 6", "x^2 + 1", "2*x - 10"];

    for eq in equations {
        let roots = MathCore::solve(eq, "x").unwrap();
        println!("{} = 0, solutions: {:?}", eq, roots);
    }

    println!("\n=== Expression Simplification ===");
    let expressions = vec!["x - x", "0 * x", "1 * x", "x^0", "x^1"];

    for expr in expressions {
        let simplified = MathCore::simplify(expr).unwrap();
        println!("{} simplifies to: {}", expr, simplified);
    }

    println!("\n=== Function Plotting ===");
    let plot = MathCore::plot_ascii("x^2", "x", -2.0, 2.0, 40, 15).unwrap();
    println!("{}", plot);

    println!("\n=== Advanced Example: Taylor Series ===");
    println!("Taylor series expansion of e^x around x=0:");
    let mut taylor = "1".to_string();
    for n in 1..=5 {
        taylor.push_str(&format!(" + x^{}/{}", n, factorial(n)));
        let expr = taylor.replace("/", " / ");
        let value = math
            .evaluate_with_vars(&expr, &HashMap::from([("x".to_string(), 1.0)]))
            .unwrap();
        println!("n={}: e ≈ {:.6}", n, value);
    }

    let actual_e = math.calculate("e").unwrap();
    println!("Actual e = {:.6}", actual_e);
}

fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
