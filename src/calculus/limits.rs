use crate::engine::Engine;
use crate::types::{Expr, MathError};
use std::collections::HashMap;

pub struct Limits;

impl Limits {
    pub fn limit(
        expr: &Expr,
        var: &str,
        point: f64,
        direction: LimitDirection,
    ) -> Result<Expr, MathError> {
        let epsilon = match direction {
            LimitDirection::Left => -1e-10,
            LimitDirection::Right => 1e-10,
            LimitDirection::Both => 0.0,
        };

        if direction == LimitDirection::Both {
            let left = Self::limit(expr, var, point, LimitDirection::Left)?;
            let right = Self::limit(expr, var, point, LimitDirection::Right)?;

            if Self::expressions_equal(&left, &right, 1e-9) {
                return Ok(left);
            } else {
                return Err(MathError::InvalidOperation(
                    "Limit does not exist (left != right)".to_string(),
                ));
            }
        }

        Self::numerical_limit(expr, var, point, epsilon)
    }

    pub fn limit_at_infinity(expr: &Expr, var: &str, positive: bool) -> Result<Expr, MathError> {
        let large_value = if positive { 1e10 } else { -1e10 };
        Self::numerical_limit(expr, var, large_value, 0.0)
    }

    fn numerical_limit(
        expr: &Expr,
        var: &str,
        point: f64,
        epsilon: f64,
    ) -> Result<Expr, MathError> {
        let engine = Engine::new();
        let mut vars = HashMap::new();

        let test_points = if epsilon == 0.0 {
            vec![point]
        } else {
            let mut points = Vec::new();
            for i in 1..=10 {
                let delta = epsilon / (i as f64);
                points.push(point + delta);
            }
            points
        };

        let mut results = Vec::new();
        for test_point in test_points {
            vars.insert(var.to_string(), test_point);
            match engine.evaluate_with_vars(expr, &vars) {
                Ok(Expr::Number(n)) if n.is_finite() => results.push(n),
                Ok(Expr::Number(n)) if n.is_infinite() => {
                    return Ok(Expr::Symbol(if n.is_sign_positive() {
                        "∞".to_string()
                    } else {
                        "-∞".to_string()
                    }));
                }
                _ => {}
            }
        }

        if results.is_empty() {
            return Err(MathError::InvalidOperation(
                "Cannot compute limit".to_string(),
            ));
        }

        let avg = results.iter().sum::<f64>() / results.len() as f64;
        let variance =
            results.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / results.len() as f64;

        if variance < 1e-12 {
            Ok(Expr::Number(avg))
        } else {
            let trend = results.windows(2).map(|w| w[1] - w[0]).sum::<f64>();
            if trend.abs() > 1e-6 {
                Ok(Expr::Symbol(if trend > 0.0 {
                    "∞".to_string()
                } else {
                    "-∞".to_string()
                }))
            } else {
                Ok(Expr::Number(avg))
            }
        }
    }

    pub fn is_continuous_at(expr: &Expr, var: &str, point: f64) -> Result<bool, MathError> {
        let engine = Engine::new();
        let mut vars = HashMap::new();
        vars.insert(var.to_string(), point);

        let value_at_point = match engine.evaluate_with_vars(expr, &vars)? {
            Expr::Number(n) if n.is_finite() => n,
            _ => return Ok(false),
        };

        match Self::limit(expr, var, point, LimitDirection::Both)? {
            Expr::Number(limit_val) => Ok((value_at_point - limit_val).abs() < 1e-9),
            _ => Ok(false),
        }
    }

    pub fn lhopital_rule(
        numerator: &Expr,
        denominator: &Expr,
        var: &str,
        point: f64,
    ) -> Result<Expr, MathError> {
        use crate::calculus::Calculus;

        let engine = Engine::new();
        let mut vars = HashMap::new();
        vars.insert(var.to_string(), point);

        let num_at_point = engine.evaluate_with_vars(numerator, &vars)?;
        let den_at_point = engine.evaluate_with_vars(denominator, &vars)?;

        match (&num_at_point, &den_at_point) {
            (Expr::Number(n), Expr::Number(d)) if n.abs() < 1e-10 && d.abs() < 1e-10 => {
                let num_deriv = Calculus::differentiate(numerator, var)?;
                let den_deriv = Calculus::differentiate(denominator, var)?;

                let ratio = Expr::Binary {
                    op: crate::types::BinaryOp::Divide,
                    left: Box::new(num_deriv),
                    right: Box::new(den_deriv),
                };

                Self::limit(&ratio, var, point, LimitDirection::Both)
            }
            _ => {
                let ratio = Expr::Binary {
                    op: crate::types::BinaryOp::Divide,
                    left: Box::new(numerator.clone()),
                    right: Box::new(denominator.clone()),
                };
                Self::limit(&ratio, var, point, LimitDirection::Both)
            }
        }
    }

    fn expressions_equal(expr1: &Expr, expr2: &Expr, tolerance: f64) -> bool {
        match (expr1, expr2) {
            (Expr::Number(n1), Expr::Number(n2)) => (n1 - n2).abs() < tolerance,
            (Expr::Symbol(s1), Expr::Symbol(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitDirection {
    Left,
    Right,
    Both,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_simple_limit() {
        let expr = Parser::parse("x^2").unwrap();
        let limit = Limits::limit(&expr, "x", 2.0, LimitDirection::Both).unwrap();

        if let Expr::Number(n) = limit {
            assert!((n - 4.0).abs() < 1e-9);
        } else {
            panic!("Expected number");
        }
    }

    #[test]
    fn test_limit_at_infinity() {
        let expr = Parser::parse("1/x").unwrap();
        let limit = Limits::limit_at_infinity(&expr, "x", true).unwrap();

        if let Expr::Number(n) = limit {
            assert!(n.abs() < 1e-9);
        } else {
            panic!("Expected number near zero");
        }
    }
}
