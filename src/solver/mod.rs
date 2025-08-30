use crate::calculus::Calculus;
use crate::engine::Engine;
use crate::types::{BinaryOp, Expr, MathError};
use num_complex::Complex64;
use std::collections::HashMap;

// solver for equations
pub struct Solver;

impl Solver {
    /// solve equation = 0
    pub fn solve(equation: &Expr, var: &str) -> Result<Vec<Expr>, MathError> {
        let equation = Self::normalize_equation(equation)?;

        let degree = equation.degree(var);

        match degree {
            0 => Self::solve_constant(&equation),
            1 => Self::solve_linear(&equation, var),
            2 => Self::solve_quadratic(&equation, var),
            _ => Self::solve_numerical(&equation, var),
        }
    }

    fn normalize_equation(equation: &Expr) -> Result<Expr, MathError> {
        match equation {
            Expr::Binary {
                op: BinaryOp::Subtract,
                ..
            } => Ok(equation.clone()),
            _ => Ok(Expr::Binary {
                op: BinaryOp::Subtract,
                left: Box::new(equation.clone()),
                right: Box::new(Expr::zero()),
            }),
        }
    }

    fn solve_constant(equation: &Expr) -> Result<Vec<Expr>, MathError> {
        let engine = Engine::new();
        let result = engine.evaluate(equation)?;

        match result {
            Expr::Number(n) if n.abs() < f64::EPSILON => {
                Err(MathError::SolverError("Infinite solutions".to_string()))
            }
            _ => Ok(vec![]),
        }
    }

    fn solve_linear(equation: &Expr, var: &str) -> Result<Vec<Expr>, MathError> {
        let (a, b) = Self::extract_linear_coefficients(equation, var)?;

        if a.abs() < f64::EPSILON {
            if b.abs() < f64::EPSILON {
                return Err(MathError::SolverError("Infinite solutions".to_string()));
            } else {
                return Ok(vec![]);
            }
        }

        Ok(vec![Expr::Number(-b / a)])
    }

    fn solve_quadratic(equation: &Expr, var: &str) -> Result<Vec<Expr>, MathError> {
        let (a, b, c) = Self::extract_quadratic_coefficients(equation, var)?;

        if a.abs() < f64::EPSILON {
            return Self::solve_linear(&Self::create_linear(b, c, var), var);
        }

        // quadratic formula: x = (-b ± √(b²-4ac)) / 2a
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > f64::EPSILON {
            // two real roots
            let sqrt_disc = discriminant.sqrt();
            Ok(vec![
                Expr::Number((-b + sqrt_disc) / (2.0 * a)),
                Expr::Number((-b - sqrt_disc) / (2.0 * a)),
            ])
        } else if discriminant.abs() < f64::EPSILON {
            // one repeated root
            Ok(vec![Expr::Number(-b / (2.0 * a))])
        } else {
            // TODO: return complex roots instead of real/imag parts
            let real_part = -b / (2.0 * a);
            let imag_part = (-discriminant).sqrt() / (2.0 * a);
            Ok(vec![
                Expr::Complex(Complex64::new(real_part, imag_part)),
                Expr::Complex(Complex64::new(real_part, -imag_part)),
            ])
        }
    }

    fn solve_numerical(equation: &Expr, var: &str) -> Result<Vec<Expr>, MathError> {
        let mut roots = Vec::new();
        let engine = Engine::new();

        let search_points = [-100.0, -10.0, -1.0, 0.0, 1.0, 10.0, 100.0];

        for i in 0..search_points.len() - 1 {
            let x0 = search_points[i];
            let x1 = search_points[i + 1];

            if let Some(root) = Self::newton_raphson(equation, var, (x0 + x1) / 2.0, &engine)? {
                let is_duplicate = roots.iter().any(|r| {
                    if let (Expr::Number(r1), Expr::Number(r2)) = (r, &root) {
                        (r1 - r2).abs() < 1e-6
                    } else {
                        false
                    }
                });

                if !is_duplicate {
                    roots.push(root);
                }
            }
        }

        if roots.is_empty() {
            for _ in 0..5 {
                let initial = rand_float() * 200.0 - 100.0;
                if let Some(root) = Self::newton_raphson(equation, var, initial, &engine)? {
                    let is_duplicate = roots.iter().any(|r| {
                        if let (Expr::Number(r1), Expr::Number(r2)) = (r, &root) {
                            (r1 - r2).abs() < 1e-6
                        } else {
                            false
                        }
                    });

                    if !is_duplicate {
                        roots.push(root);
                    }
                }
            }
        }

        Ok(roots)
    }

    fn newton_raphson(
        equation: &Expr,
        var: &str,
        initial: f64,
        engine: &Engine,
    ) -> Result<Option<Expr>, MathError> {
        let derivative = Calculus::differentiate(equation, var)?;
        let mut x = initial;
        let max_iterations = 100;
        let tolerance = 1e-10;

        for _ in 0..max_iterations {
            let mut vars = HashMap::new();
            vars.insert(var.to_string(), x);

            let f_val = match engine.evaluate_with_vars(equation, &vars)? {
                Expr::Number(n) => n,
                _ => return Ok(None),
            };

            if f_val.abs() < tolerance {
                return Ok(Some(Expr::Number(x)));
            }

            let df_val = match engine.evaluate_with_vars(&derivative, &vars)? {
                Expr::Number(n) => n,
                _ => return Ok(None),
            };

            if df_val.abs() < f64::EPSILON {
                return Ok(None);
            }

            let x_new = x - f_val / df_val;

            if (x_new - x).abs() < tolerance {
                return Ok(Some(Expr::Number(x_new)));
            }

            x = x_new;

            if !x.is_finite() {
                return Ok(None);
            }
        }

        Ok(None)
    }

    fn extract_linear_coefficients(equation: &Expr, var: &str) -> Result<(f64, f64), MathError> {
        let engine = Engine::new();

        let subst_zero = engine.substitute(equation, var, &Expr::zero())?;
        let b = match engine.evaluate(&subst_zero)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::SolverError(
                    "Cannot extract constant term".to_string(),
                ))
            }
        };

        let subst_one = engine.substitute(equation, var, &Expr::one())?;
        let val_at_one = match engine.evaluate(&subst_one)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::SolverError(
                    "Cannot extract linear coefficient".to_string(),
                ))
            }
        };

        let a = val_at_one - b;

        Ok((a, b))
    }

    fn extract_quadratic_coefficients(
        equation: &Expr,
        var: &str,
    ) -> Result<(f64, f64, f64), MathError> {
        let engine = Engine::new();

        let subst_zero = engine.substitute(equation, var, &Expr::zero())?;
        let c = match engine.evaluate(&subst_zero)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::SolverError(
                    "Cannot extract constant term".to_string(),
                ))
            }
        };

        let subst_one = engine.substitute(equation, var, &Expr::one())?;
        let val_at_one = match engine.evaluate(&subst_one)? {
            Expr::Number(n) => n,
            _ => return Err(MathError::SolverError("Cannot evaluate at x=1".to_string())),
        };

        let subst_neg_one = engine.substitute(equation, var, &Expr::Number(-1.0))?;
        let val_at_neg_one = match engine.evaluate(&subst_neg_one)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::SolverError(
                    "Cannot evaluate at x=-1".to_string(),
                ))
            }
        };

        let a = (val_at_one + val_at_neg_one - 2.0 * c) / 2.0;
        let b = (val_at_one - val_at_neg_one) / 2.0;

        Ok((a, b, c))
    }

    fn create_linear(a: f64, b: f64, var: &str) -> Expr {
        Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Multiply,
                left: Box::new(Expr::Number(a)),
                right: Box::new(Expr::Symbol(var.to_string())),
            }),
            right: Box::new(Expr::Number(b)),
        }
    }

    pub fn factor(expr: &Expr) -> Result<Expr, MathError> {
        match expr {
            Expr::Binary {
                op: BinaryOp::Add, ..
            }
            | Expr::Binary {
                op: BinaryOp::Subtract,
                ..
            } => Self::factor_polynomial(expr),
            _ => Ok(expr.clone()),
        }
    }

    fn factor_polynomial(expr: &Expr) -> Result<Expr, MathError> {
        let vars = Self::collect_variables(expr);

        if vars.len() != 1 {
            return Ok(expr.clone());
        }

        let var = &vars[0];
        let degree = expr.degree(var);

        if degree == 2 {
            let roots = Self::solve(expr, var)?;
            if roots.len() == 2 {
                if let (Expr::Number(r1), Expr::Number(r2)) = (&roots[0], &roots[1]) {
                    let factor1 = Expr::Binary {
                        op: BinaryOp::Subtract,
                        left: Box::new(Expr::Symbol(var.clone())),
                        right: Box::new(Expr::Number(*r1)),
                    };
                    let factor2 = Expr::Binary {
                        op: BinaryOp::Subtract,
                        left: Box::new(Expr::Symbol(var.clone())),
                        right: Box::new(Expr::Number(*r2)),
                    };
                    return Ok(Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(factor1),
                        right: Box::new(factor2),
                    });
                }
            }
        }

        Ok(expr.clone())
    }

    fn collect_variables(expr: &Expr) -> Vec<String> {
        let mut vars = Vec::new();
        Self::collect_vars_internal(expr, &mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_vars_internal(expr: &Expr, vars: &mut Vec<String>) {
        match expr {
            Expr::Symbol(s) => {
                if !vars.contains(s) {
                    vars.push(s.clone());
                }
            }
            Expr::Binary { left, right, .. } => {
                Self::collect_vars_internal(left, vars);
                Self::collect_vars_internal(right, vars);
            }
            Expr::Unary { expr: inner, .. } => {
                Self::collect_vars_internal(inner, vars);
            }
            Expr::Function { args, .. } => {
                for arg in args {
                    Self::collect_vars_internal(arg, vars);
                }
            }
            _ => {}
        }
    }
}

fn rand_float() -> f64 {
    // Simple pseudo-random number generator
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f64;
    nanos / 1_000_000_000.0
}
