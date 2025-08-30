use crate::calculus::Calculus;
use crate::engine::Engine;
use crate::types::{BinaryOp, Expr, MathError};
use nalgebra::DMatrix;
use std::collections::HashMap;

pub struct Optimization;

impl Optimization {
    pub fn gradient(expr: &Expr, vars: &[String]) -> Result<Vec<Expr>, MathError> {
        let mut gradient = Vec::new();

        for var in vars {
            let partial = Calculus::differentiate(expr, var)?;
            gradient.push(partial);
        }

        Ok(gradient)
    }

    pub fn hessian(expr: &Expr, vars: &[String]) -> Result<Vec<Vec<Expr>>, MathError> {
        let mut hessian = vec![vec![Expr::zero(); vars.len()]; vars.len()];

        for (i, var_i) in vars.iter().enumerate() {
            for (j, var_j) in vars.iter().enumerate() {
                let first_deriv = Calculus::differentiate(expr, var_i)?;
                let second_deriv = Calculus::differentiate(&first_deriv, var_j)?;
                hessian[i][j] = second_deriv;
            }
        }

        Ok(hessian)
    }

    pub fn gradient_descent(
        loss_fn: &Expr,
        initial_params: HashMap<String, f64>,
        learning_rate: f64,
        iterations: usize,
    ) -> Result<HashMap<String, f64>, MathError> {
        let engine = Engine::new();
        let mut params = initial_params.clone();
        let var_names: Vec<String> = params.keys().cloned().collect();

        let gradients = Self::gradient(loss_fn, &var_names)?;

        for _ in 0..iterations {
            let mut updates = HashMap::new();

            for (var_name, grad_expr) in var_names.iter().zip(gradients.iter()) {
                let grad_value = match engine.evaluate_with_vars(grad_expr, &params)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "Gradient must be numeric".to_string(),
                        ))
                    }
                };

                let current_value = params[var_name];
                updates.insert(var_name.clone(), current_value - learning_rate * grad_value);
            }

            params = updates;
        }

        Ok(params)
    }

    pub fn automatic_differentiation(
        expr: &Expr,
        var: &str,
        value: f64,
    ) -> Result<(f64, f64), MathError> {
        let engine = Engine::new();
        let mut vars = HashMap::new();
        vars.insert(var.to_string(), value);

        let function_value = match engine.evaluate_with_vars(expr, &vars)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::InvalidOperation(
                    "Expression must evaluate to number".to_string(),
                ))
            }
        };

        let derivative = Calculus::differentiate(expr, var)?;
        let derivative_value = match engine.evaluate_with_vars(&derivative, &vars)? {
            Expr::Number(n) => n,
            _ => {
                return Err(MathError::InvalidOperation(
                    "Derivative must evaluate to number".to_string(),
                ))
            }
        };

        Ok((function_value, derivative_value))
    }

    pub fn jacobian(
        functions: &[Expr],
        vars: &[String],
        point: &HashMap<String, f64>,
    ) -> Result<DMatrix<f64>, MathError> {
        let engine = Engine::new();
        let mut jacobian = DMatrix::zeros(functions.len(), vars.len());

        for (i, func) in functions.iter().enumerate() {
            for (j, var) in vars.iter().enumerate() {
                let partial = Calculus::differentiate(func, var)?;
                let value = match engine.evaluate_with_vars(&partial, point)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "Jacobian entry must be numeric".to_string(),
                        ))
                    }
                };
                jacobian[(i, j)] = value;
            }
        }

        Ok(jacobian)
    }

    pub fn taylor_series(
        expr: &Expr,
        var: &str,
        center: f64,
        order: usize,
    ) -> Result<Expr, MathError> {
        let engine = Engine::new();
        let mut series = Expr::zero();
        let mut current_deriv = expr.clone();
        let mut factorial = 1.0;

        for n in 0..=order {
            if n > 0 {
                current_deriv = Calculus::differentiate(&current_deriv, var)?;
                factorial *= n as f64;
            }

            let mut eval_point = HashMap::new();
            eval_point.insert(var.to_string(), center);

            let coeff_value = match engine.evaluate_with_vars(&current_deriv, &eval_point)? {
                Expr::Number(v) => v / factorial,
                _ => continue,
            };

            let x_minus_center = Expr::Binary {
                op: BinaryOp::Subtract,
                left: Box::new(Expr::Symbol(var.to_string())),
                right: Box::new(Expr::Number(center)),
            };

            let power_term = if n == 0 {
                Expr::Number(coeff_value)
            } else if n == 1 {
                Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(Expr::Number(coeff_value)),
                    right: Box::new(x_minus_center),
                }
            } else {
                let power = Expr::Binary {
                    op: BinaryOp::Power,
                    left: Box::new(x_minus_center),
                    right: Box::new(Expr::Number(n as f64)),
                };
                Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(Expr::Number(coeff_value)),
                    right: Box::new(power),
                }
            };

            series = Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(series),
                right: Box::new(power_term),
            };
        }

        Ok(series)
    }

    pub fn optimize_newton(
        func: &Expr,
        var: &str,
        initial: f64,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<f64, MathError> {
        let engine = Engine::new();
        let first_deriv = Calculus::differentiate(func, var)?;
        let second_deriv = Calculus::differentiate(&first_deriv, var)?;

        let mut x = initial;

        for _ in 0..max_iterations {
            let mut vars = HashMap::new();
            vars.insert(var.to_string(), x);

            let f_prime = match engine.evaluate_with_vars(&first_deriv, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "Derivative must be numeric".to_string(),
                    ))
                }
            };

            if f_prime.abs() < tolerance {
                return Ok(x);
            }

            let f_double_prime = match engine.evaluate_with_vars(&second_deriv, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "Second derivative must be numeric".to_string(),
                    ))
                }
            };

            if f_double_prime.abs() < 1e-10 {
                return Err(MathError::InvalidOperation(
                    "Second derivative too small".to_string(),
                ));
            }

            x -= f_prime / f_double_prime;
        }

        Ok(x)
    }

    pub fn lagrange_multipliers(
        objective: &Expr,
        constraints: &[Expr],
        vars: &[String],
    ) -> Result<Vec<Expr>, MathError> {
        let mut lagrangian = objective.clone();
        let mut lambda_vars = Vec::new();

        for (i, constraint) in constraints.iter().enumerate() {
            let lambda_var = format!("λ{}", i);
            lambda_vars.push(lambda_var.clone());

            let lambda_term = Expr::Binary {
                op: BinaryOp::Multiply,
                left: Box::new(Expr::Symbol(lambda_var)),
                right: Box::new(constraint.clone()),
            };

            lagrangian = Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(lagrangian),
                right: Box::new(lambda_term),
            };
        }

        let mut all_vars = vars.to_vec();
        all_vars.extend(lambda_vars);

        Self::gradient(&lagrangian, &all_vars)
    }
}

pub struct SymbolicIntegration;

impl SymbolicIntegration {
    pub fn integrate_by_parts(u: &Expr, dv: &Expr, var: &str) -> Result<Expr, MathError> {
        let du = Calculus::differentiate(u, var)?;
        let v = Calculus::integrate(dv, var)?;

        let uv = Expr::Binary {
            op: BinaryOp::Multiply,
            left: Box::new(u.clone()),
            right: Box::new(v.clone()),
        };

        let vdu = Expr::Binary {
            op: BinaryOp::Multiply,
            left: Box::new(v),
            right: Box::new(du),
        };

        let integral_vdu = Calculus::integrate(&vdu, var)?;

        Ok(Expr::Binary {
            op: BinaryOp::Subtract,
            left: Box::new(uv),
            right: Box::new(integral_vdu),
        })
    }

    pub fn substitution_rule(
        expr: &Expr,
        old_var: &str,
        substitution: &Expr,
        new_var: &str,
    ) -> Result<Expr, MathError> {
        let engine = Engine::new();
        let substituted = engine.substitute(expr, old_var, substitution)?;

        let du = Calculus::differentiate(substitution, new_var)?;

        let integrand = Expr::Binary {
            op: BinaryOp::Multiply,
            left: Box::new(substituted),
            right: Box::new(du),
        };

        Calculus::integrate(&integrand, new_var)
    }

    pub fn partial_fractions(
        numerator: &Expr,
        denominator: &Expr,
        var: &str,
    ) -> Result<Vec<Expr>, MathError> {
        use crate::solver::Solver;

        let roots = Solver::solve(denominator, var)?;
        let mut fractions = Vec::new();

        for root in roots {
            if let Expr::Number(r) = root {
                let factor = Expr::Binary {
                    op: BinaryOp::Subtract,
                    left: Box::new(Expr::Symbol(var.to_string())),
                    right: Box::new(Expr::Number(r)),
                };

                let residue = Self::compute_residue(numerator, denominator, var, r)?;

                let fraction = Expr::Binary {
                    op: BinaryOp::Divide,
                    left: Box::new(residue),
                    right: Box::new(factor),
                };

                fractions.push(fraction);
            }
        }

        Ok(fractions)
    }

    fn compute_residue(
        numerator: &Expr,
        denominator: &Expr,
        var: &str,
        pole: f64,
    ) -> Result<Expr, MathError> {
        let engine = Engine::new();
        let factor = Expr::Binary {
            op: BinaryOp::Subtract,
            left: Box::new(Expr::Symbol(var.to_string())),
            right: Box::new(Expr::Number(pole)),
        };

        let reduced_den = Expr::Binary {
            op: BinaryOp::Divide,
            left: Box::new(denominator.clone()),
            right: Box::new(factor),
        };

        let ratio = Expr::Binary {
            op: BinaryOp::Divide,
            left: Box::new(numerator.clone()),
            right: Box::new(reduced_den),
        };

        let mut vars = HashMap::new();
        vars.insert(var.to_string(), pole);

        engine.evaluate_with_vars(&ratio, &vars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_gradient() {
        let expr = Parser::parse("x^2 + y^2").unwrap();
        let vars = vec!["x".to_string(), "y".to_string()];
        let grad = Optimization::gradient(&expr, &vars).unwrap();

        assert_eq!(grad.len(), 2);
        println!("Gradient of x^2 + y^2: [{}, {}]", grad[0], grad[1]);
    }

    #[test]
    fn test_taylor_series() {
        let expr = Parser::parse("sin(x)").unwrap();
        let taylor = Optimization::taylor_series(&expr, "x", 0.0, 5).unwrap();
        println!("Taylor series of sin(x): {}", taylor);
    }
}
