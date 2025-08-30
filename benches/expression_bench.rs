use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mathcore::differential::DifferentialEquations;
use mathcore::{calculus::Calculus, engine::Engine, parser::Parser, MathCore};
use std::collections::HashMap;

fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    let expressions = vec![
        ("simple", "2 + 3 * 4"),
        ("complex", "sin(x^2) + cos(y) * exp(-x/2)"),
        ("nested", "((a + b) * (c - d)) / (e^2 + f^2)"),
        ("polynomial", "x^5 + 4*x^4 - 3*x^3 + 2*x^2 - x + 1"),
    ];

    for (name, expr) in expressions {
        group.bench_with_input(BenchmarkId::new("parse", name), expr, |b, expr| {
            b.iter(|| Parser::parse(black_box(expr)))
        });
    }

    group.finish();
}

fn evaluation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluation");
    let engine = Engine::new();

    let test_cases = vec![
        ("arithmetic", "2^8 + 3^5 - 4^3"),
        ("trigonometric", "sin(1.5) * cos(2.3) + tan(0.7)"),
        ("logarithmic", "ln(10) + log(100, 10) - exp(2)"),
    ];

    for (name, expr_str) in test_cases {
        let expr = Parser::parse(expr_str).unwrap();
        group.bench_with_input(BenchmarkId::new("evaluate", name), &expr, |b, expr| {
            b.iter(|| engine.evaluate(black_box(expr)))
        });
    }

    group.finish();
}

fn differentiation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("differentiation");

    let functions = vec![
        ("polynomial", "x^3 + 2*x^2 - 5*x + 3"),
        ("trigonometric", "sin(x) * cos(x)"),
        ("exponential", "exp(x^2)"),
        ("composite", "ln(sin(x^2) + cos(x))"),
    ];

    for (name, expr_str) in functions {
        let expr = Parser::parse(expr_str).unwrap();
        group.bench_with_input(BenchmarkId::new("differentiate", name), &expr, |b, expr| {
            b.iter(|| Calculus::differentiate(black_box(expr), "x"))
        });
    }

    group.finish();
}

fn integration_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("integration");

    let functions = vec![
        ("polynomial", "x^2"),
        ("trigonometric", "sin(x)"),
        ("rational", "1/x"),
    ];

    for (name, expr_str) in functions {
        let expr = Parser::parse(expr_str).unwrap();
        group.bench_with_input(BenchmarkId::new("integrate", name), &expr, |b, expr| {
            b.iter(|| Calculus::integrate(black_box(expr), "x"))
        });
    }

    // Numerical integration
    let expr = Parser::parse("x^2").unwrap();
    group.bench_function("numerical_integrate", |b| {
        b.iter(|| Calculus::numerical_integrate(&expr, "x", black_box(0.0), black_box(1.0), 1000))
    });

    group.finish();
}

fn solver_benchmark(c: &mut Criterion) {
    use mathcore::solver::Solver;

    let mut group = c.benchmark_group("solver");

    let equations = vec![
        ("linear", "2*x - 10"),
        ("quadratic", "x^2 - 5*x + 6"),
        ("cubic", "x^3 - 6*x^2 + 11*x - 6"),
    ];

    for (name, expr_str) in equations {
        let expr = Parser::parse(expr_str).unwrap();
        group.bench_with_input(BenchmarkId::new("solve", name), &expr, |b, expr| {
            b.iter(|| Solver::solve(black_box(expr), "x"))
        });
    }

    group.finish();
}

fn matrix_benchmark(c: &mut Criterion) {
    use mathcore::matrix::SymbolicMatrix;

    let mut group = c.benchmark_group("matrix");

    let sizes = vec![2, 4, 8, 16, 32];

    for size in sizes {
        let matrix_data: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| (i * size + j) as f64).collect())
            .collect();

        let matrix = SymbolicMatrix::from_vec(matrix_data.clone()).unwrap();

        group.bench_with_input(BenchmarkId::new("multiply", size), &matrix, |b, m| {
            b.iter(|| m.multiply(black_box(m)))
        });

        if size <= 8 {
            group.bench_with_input(BenchmarkId::new("determinant", size), &matrix, |b, m| {
                b.iter(|| m.determinant())
            });
        }
    }

    group.finish();
}

fn ode_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("differential_equations");

    // Simple ODE: dy/dt = -y
    let expr = Parser::parse("-y").unwrap();

    group.bench_function("runge_kutta_4", |b| {
        b.iter(|| {
            DifferentialEquations::solve_ode_first_order(
                &expr,
                "t",
                "y",
                black_box((0.0, 1.0)),
                black_box(10.0),
                black_box(100),
            )
        })
    });

    group.bench_function("euler_method", |b| {
        b.iter(|| {
            DifferentialEquations::euler_method(
                &expr,
                "t",
                "y",
                black_box((0.0, 1.0)),
                black_box(10.0),
                black_box(100),
            )
        })
    });

    group.finish();
}

fn precision_benchmark(c: &mut Criterion) {
    use mathcore::precision::{ArbitraryPrecision, PrecisionNumber};

    let mut group = c.benchmark_group("arbitrary_precision");

    let a = PrecisionNumber::from_str_with_precision("123456789012345678901234567890").unwrap();
    let b = PrecisionNumber::from_str_with_precision("987654321098765432109876543210").unwrap();

    group.bench_function("big_multiply", |b| b.iter(|| a.multiply(black_box(&b))));

    group.bench_function("compute_pi", |b| {
        b.iter(|| ArbitraryPrecision::compute_pi(black_box(50)))
    });

    group.bench_function("compute_e", |b| {
        b.iter(|| ArbitraryPrecision::compute_e(black_box(50)))
    });

    group.finish();
}

criterion_group!(
    benches,
    parse_benchmark,
    evaluation_benchmark,
    differentiation_benchmark,
    integration_benchmark,
    solver_benchmark,
    matrix_benchmark,
    ode_benchmark,
    precision_benchmark
);

criterion_main!(benches);
