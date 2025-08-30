//! MathCore - symbolic math library for Rust
//!
//! basically a CAS (computer algebra system) that can do symbolic math,
//! solve equations, differentiate, integrate, etc.
//!
//! ```rust
//! use mathcore::MathCore;
//!
//! let math = MathCore::new();
//!
//! // basic stuff
//! let result = math.calculate("2 + 3 * 4").unwrap();
//! assert_eq!(result, 14.0);
//!
//! // calculus
//! let derivative = MathCore::differentiate("x^2", "x").unwrap();
//!
//! // solve equations
//! let roots = MathCore::solve("x^2 - 4", "x").unwrap();
//! ```

pub mod calculus;
pub mod differential;
pub mod engine;
pub mod matrix;
pub mod ml;
pub mod parser;
pub mod precision;
pub mod solver;
pub mod transforms;
pub mod types;

use std::collections::HashMap;
pub use types::{Expr, MathError};

pub struct MathCore {
    engine: engine::Engine,
}

impl MathCore {
    pub fn new() -> Self {
        MathCore {
            engine: engine::Engine::new(),
        }
    }

    pub fn parse(expression: &str) -> Result<Expr, MathError> {
        parser::Parser::parse(expression)
    }

    pub fn calculate(&self, expression: &str) -> Result<f64, MathError> {
        let expr = Self::parse(expression)?;
        let result = self.engine.evaluate(&expr)?;

        match result {
            Expr::Number(n) => Ok(n),
            _ => Err(MathError::InvalidOperation(
                "Result is not a real number".to_string(),
            )),
        }
    }

    pub fn evaluate(&self, expression: &str) -> Result<Expr, MathError> {
        let expr = Self::parse(expression)?;
        self.engine.evaluate(&expr)
    }

    pub fn evaluate_with_vars(
        &self,
        expression: &str,
        vars: &HashMap<String, f64>,
    ) -> Result<f64, MathError> {
        let expr = Self::parse(expression)?;
        let result = self.engine.evaluate_with_vars(&expr, vars)?;

        match result {
            Expr::Number(n) => Ok(n),
            _ => Err(MathError::InvalidOperation(
                "Result is not a real number".to_string(),
            )),
        }
    }

    pub fn differentiate(expression: &str, var: &str) -> Result<Expr, MathError> {
        let expr = Self::parse(expression)?;
        calculus::Calculus::differentiate(&expr, var)
    }

    pub fn integrate(expression: &str, var: &str) -> Result<Expr, MathError> {
        let expr = Self::parse(expression)?;
        calculus::Calculus::integrate(&expr, var)
    }

    pub fn numerical_integrate(
        expression: &str,
        var: &str,
        lower: f64,
        upper: f64,
    ) -> Result<f64, MathError> {
        let expr = Self::parse(expression)?;
        calculus::Calculus::numerical_integrate(&expr, var, lower, upper, 1000)
    }

    pub fn solve(equation: &str, var: &str) -> Result<Vec<Expr>, MathError> {
        let expr = Self::parse(equation)?;
        solver::Solver::solve(&expr, var)
    }

    pub fn factor(expression: &str) -> Result<Expr, MathError> {
        let expr = Self::parse(expression)?;
        solver::Solver::factor(&expr)
    }

    pub fn simplify(expression: &str) -> Result<Expr, MathError> {
        let expr = Self::parse(expression)?;
        Ok(Self::simplify_expr(&expr))
    }

    // TODO: add more simplification rules

    fn simplify_expr(expr: &Expr) -> Expr {
        use types::{BinaryOp, UnaryOp};

        // recursive simplification
        match expr {
            Expr::Binary { op, left, right } => {
                let left = Self::simplify_expr(left);
                let right = Self::simplify_expr(right);

                match (&left, op, &right) {
                    (Expr::Number(l), BinaryOp::Add, Expr::Number(r)) => Expr::Number(l + r),
                    (Expr::Number(l), BinaryOp::Subtract, Expr::Number(r)) => Expr::Number(l - r),
                    (Expr::Number(l), BinaryOp::Multiply, Expr::Number(r)) => Expr::Number(l * r),
                    (Expr::Number(l), BinaryOp::Divide, Expr::Number(r))
                        if r.abs() > f64::EPSILON =>
                    {
                        Expr::Number(l / r)
                    }
                    (Expr::Number(l), BinaryOp::Power, Expr::Number(r)) => Expr::Number(l.powf(*r)),

                    (e, BinaryOp::Add, other) | (other, BinaryOp::Add, e) if e.is_zero() => {
                        other.clone()
                    }
                    (e, BinaryOp::Subtract, other) if other.is_zero() => e.clone(),
                    (e, BinaryOp::Multiply, _) | (_, BinaryOp::Multiply, e) if e.is_zero() => {
                        Expr::zero()
                    }
                    (e, BinaryOp::Multiply, other) | (other, BinaryOp::Multiply, e)
                        if e.is_one() =>
                    {
                        other.clone()
                    }
                    (e, BinaryOp::Divide, other) if other.is_one() => e.clone(),
                    (_e, BinaryOp::Power, other) if other.is_zero() => Expr::one(),
                    (e, BinaryOp::Power, other) if other.is_one() => e.clone(),

                    (Expr::Symbol(s1), BinaryOp::Subtract, Expr::Symbol(s2)) if s1 == s2 => {
                        Expr::zero()
                    }

                    _ => Expr::Binary {
                        op: *op,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }

            Expr::Unary { op, expr: inner } => {
                let inner = Self::simplify_expr(inner);

                match (op, &inner) {
                    (UnaryOp::Negate, Expr::Number(n)) => Expr::Number(-n),
                    (
                        UnaryOp::Negate,
                        Expr::Unary {
                            op: UnaryOp::Negate,
                            expr: e,
                        },
                    ) => *e.clone(),
                    (UnaryOp::Abs, Expr::Number(n)) => Expr::Number(n.abs()),

                    _ => Expr::Unary {
                        op: *op,
                        expr: Box::new(inner),
                    },
                }
            }

            _ => expr.clone(),
        }
    }

    pub fn plot_ascii(
        expression: &str,
        var: &str,
        x_min: f64,
        x_max: f64,
        width: usize,
        height: usize,
    ) -> Result<String, MathError> {
        let expr = Self::parse(expression)?;
        let engine = engine::Engine::new();

        let mut points = Vec::new();
        let step = (x_max - x_min) / width as f64;

        for i in 0..=width {
            let x = x_min + i as f64 * step;
            let mut vars = HashMap::new();
            vars.insert(var.to_string(), x);

            if let Ok(Expr::Number(y)) = engine.evaluate_with_vars(&expr, &vars) {
                if y.is_finite() {
                    points.push((x, y));
                }
            }
        }

        if points.is_empty() {
            return Err(MathError::InvalidOperation(
                "No valid points to plot".to_string(),
            ));
        }

        let y_min = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let y_max = points
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);

        let mut plot = vec![vec![' '; width + 1]; height + 1];

        #[allow(clippy::needless_range_loop)]
        for y in 0..=height {
            plot[y][0] = '|';
        }
        for x in 0..=width {
            plot[height][x] = '-';
        }
        plot[height][0] = '+';

        for (x, y) in points {
            let plot_x = ((x - x_min) / (x_max - x_min) * width as f64) as usize;
            let plot_y = height - ((y - y_min) / (y_max - y_min) * height as f64) as usize;

            if plot_x <= width && plot_y <= height {
                plot[plot_y][plot_x] = '*';
            }
        }

        let mut result = String::new();
        result.push_str(&format!(
            "Plot of {} from x={} to x={}\n",
            expression, x_min, x_max
        ));
        result.push_str(&format!("y range: [{:.2}, {:.2}]\n\n", y_min, y_max));

        for row in plot {
            result.push_str(&row.iter().collect::<String>());
            result.push('\n');
        }

        Ok(result)
    }
}

impl Default for MathCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let math = MathCore::new();

        assert_eq!(math.calculate("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(math.calculate("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(math.calculate("2^3").unwrap(), 8.0);
        assert_eq!(math.calculate("10 / 2").unwrap(), 5.0);
    }

    #[test]
    fn test_functions() {
        let math = MathCore::new();

        let sin_val = math.calculate("sin(0)").unwrap();
        assert!(sin_val.abs() < 0.0001);

        let cos_val = math.calculate("cos(0)").unwrap();
        assert!((cos_val - 1.0).abs() < 0.0001);

        let exp_val = math.calculate("exp(0)").unwrap();
        assert!((exp_val - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_differentiation() {
        let deriv = MathCore::differentiate("x^2", "x").unwrap();
        println!("d/dx(x^2) = {}", deriv);

        let deriv = MathCore::differentiate("sin(x)", "x").unwrap();
        println!("d/dx(sin(x)) = {}", deriv);

        let deriv = MathCore::differentiate("x^3 + 2*x^2 + x + 1", "x").unwrap();
        println!("d/dx(x^3 + 2*x^2 + x + 1) = {}", deriv);
    }

    #[test]
    fn test_integration() {
        let integral = MathCore::integrate("x", "x").unwrap();
        println!("∫x dx = {}", integral);

        let integral = MathCore::integrate("x^2", "x").unwrap();
        println!("∫x^2 dx = {}", integral);

        let integral = MathCore::integrate("sin(x)", "x").unwrap();
        println!("∫sin(x) dx = {}", integral);
    }

    #[test]
    fn test_solving() {
        let roots = MathCore::solve("x^2 - 4", "x").unwrap();
        assert_eq!(roots.len(), 2);
        println!("Roots of x^2 - 4 = 0: {:?}", roots);

        let roots = MathCore::solve("x^2 + x - 6", "x").unwrap();
        assert_eq!(roots.len(), 2);
        println!("Roots of x^2 + x - 6 = 0: {:?}", roots);
    }

    #[test]
    fn test_variables() {
        let math = MathCore::new();
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), 3.0);
        vars.insert("b".to_string(), 4.0);

        let result = math.evaluate_with_vars("a^2 + b^2", &vars).unwrap();
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_simplification() {
        let simplified = MathCore::simplify("x - x").unwrap();
        assert!(simplified.is_zero());

        let simplified = MathCore::simplify("0 * x").unwrap();
        assert!(simplified.is_zero());

        let simplified = MathCore::simplify("1 * x").unwrap();
        println!("1 * x = {}", simplified);
    }
}
