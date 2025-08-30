use crate::types::{BinaryOp, Context, Expr, MathError, UnaryOp};
use num_complex::Complex64;
use std::collections::HashMap;

// eval engine for expressions
pub struct Engine {
    context: Context,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            context: Context::with_defaults(),
        }
    }

    pub fn with_context(context: Context) -> Self {
        Engine { context }
    }

    /// evaluate expr
    pub fn evaluate(&self, expr: &Expr) -> Result<Expr, MathError> {
        self.eval_expr(expr)
    }

    pub fn evaluate_with_vars(
        &self,
        expr: &Expr,
        vars: &HashMap<String, f64>,
    ) -> Result<Expr, MathError> {
        let mut temp_expr = expr.clone();
        for (name, value) in vars {
            temp_expr = self.substitute(&temp_expr, name, &Expr::Number(*value))?;
        }
        self.eval_expr(&temp_expr)
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Expr, MathError> {
        match expr {
            Expr::Number(_) | Expr::Complex(_) => Ok(expr.clone()),

            Expr::Symbol(name) => self
                .context
                .get_var(name)
                .cloned()
                .ok_or_else(|| MathError::UndefinedVariable(name.clone())),

            Expr::Binary { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.apply_binary_op(*op, &left_val, &right_val)
            }

            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                self.apply_unary_op(*op, &val)
            }

            Expr::Function { name, args } => {
                let evaluated_args: Result<Vec<_>, _> =
                    args.iter().map(|arg| self.eval_expr(arg)).collect();
                let evaluated_args = evaluated_args?;
                self.apply_function(name, &evaluated_args)
            }

            _ => Ok(expr.clone()),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn apply_binary_op(&self, op: BinaryOp, left: &Expr, right: &Expr) -> Result<Expr, MathError> {
        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                let result = match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Subtract => l - r,
                    BinaryOp::Multiply => l * r,
                    BinaryOp::Divide => {
                        if r.abs() < f64::EPSILON {
                            return Err(MathError::DivisionByZero);
                        }
                        l / r
                    }
                    BinaryOp::Power => l.powf(*r),
                    BinaryOp::Modulo => l % r,
                };

                if result.is_finite() {
                    Ok(Expr::Number(result))
                } else {
                    Err(MathError::Overflow)
                }
            }

            (Expr::Complex(l), Expr::Complex(r)) => {
                let result = match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Subtract => l - r,
                    BinaryOp::Multiply => l * r,
                    BinaryOp::Divide => {
                        if r.norm() < f64::EPSILON {
                            return Err(MathError::DivisionByZero);
                        }
                        l / r
                    }
                    BinaryOp::Power => l.powc(*r),
                    BinaryOp::Modulo => {
                        return Err(MathError::InvalidOperation(
                            "Modulo not defined for complex numbers".to_string(),
                        ));
                    }
                };
                Ok(Expr::Complex(result))
            }

            (Expr::Number(n), Expr::Complex(c)) => {
                let l = Complex64::new(*n, 0.0);
                self.apply_binary_op(op, &Expr::Complex(l), &Expr::Complex(*c))
            }

            (Expr::Complex(c), Expr::Number(n)) => {
                let r = Complex64::new(*n, 0.0);
                self.apply_binary_op(op, &Expr::Complex(*c), &Expr::Complex(r))
            }

            _ => Ok(Expr::Binary {
                op,
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
            }),
        }
    }

    fn apply_unary_op(&self, op: UnaryOp, expr: &Expr) -> Result<Expr, MathError> {
        match (op, expr) {
            (UnaryOp::Negate, Expr::Number(n)) => Ok(Expr::Number(-n)),
            (UnaryOp::Negate, Expr::Complex(c)) => Ok(Expr::Complex(-c)),

            (UnaryOp::Abs, Expr::Number(n)) => Ok(Expr::Number(n.abs())),
            (UnaryOp::Abs, Expr::Complex(c)) => Ok(Expr::Number(c.norm())),

            (UnaryOp::Factorial, Expr::Number(n)) => {
                if *n < 0.0 || n.fract() != 0.0 {
                    return Err(MathError::InvalidOperation(
                        "Factorial requires non-negative integer".to_string(),
                    ));
                }
                let mut result = 1.0;
                let mut i = 2.0;
                while i <= *n {
                    result *= i;
                    i += 1.0;
                }
                Ok(Expr::Number(result))
            }

            _ => Ok(Expr::Unary {
                op,
                expr: Box::new(expr.clone()),
            }),
        }
    }

    fn apply_function(&self, name: &str, args: &[Expr]) -> Result<Expr, MathError> {
        match name {
            "sin" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) => Ok(Expr::Number(n.sin())),
                    Expr::Complex(c) => Ok(Expr::Complex(c.sin())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "cos" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) => Ok(Expr::Number(n.cos())),
                    Expr::Complex(c) => Ok(Expr::Complex(c.cos())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "tan" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) => Ok(Expr::Number(n.tan())),
                    Expr::Complex(c) => Ok(Expr::Complex(c.tan())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "ln" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) if *n > 0.0 => Ok(Expr::Number(n.ln())),
                    Expr::Complex(c) => Ok(Expr::Complex(c.ln())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "log" => {
                if args.len() != 2 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        2,
                        args.len(),
                    ));
                }
                match (&args[0], &args[1]) {
                    (Expr::Number(n), Expr::Number(base))
                        if *n > 0.0 && *base > 0.0 && *base != 1.0 =>
                    {
                        Ok(Expr::Number(n.log(*base)))
                    }
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "exp" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) => Ok(Expr::Number(n.exp())),
                    Expr::Complex(c) => Ok(Expr::Complex(c.exp())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "sqrt" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                match &args[0] {
                    Expr::Number(n) if *n >= 0.0 => Ok(Expr::Number(n.sqrt())),
                    Expr::Number(n) => Ok(Expr::Complex(Complex64::new(0.0, (-n).sqrt()))),
                    Expr::Complex(c) => Ok(Expr::Complex(c.sqrt())),
                    _ => Ok(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                }
            }

            "abs" => {
                if args.len() != 1 {
                    return Err(MathError::InvalidArgumentCount(
                        name.to_string(),
                        1,
                        args.len(),
                    ));
                }
                self.apply_unary_op(UnaryOp::Abs, &args[0])
            }

            "min" => {
                if args.is_empty() {
                    return Err(MathError::InvalidArgumentCount(name.to_string(), 1, 0));
                }
                let nums: Result<Vec<f64>, _> = args
                    .iter()
                    .map(|arg| match arg {
                        Expr::Number(n) => Ok(*n),
                        _ => Err(MathError::InvalidOperation(
                            "min requires numeric arguments".to_string(),
                        )),
                    })
                    .collect();
                let nums = nums?;
                Ok(Expr::Number(nums.into_iter().fold(f64::INFINITY, f64::min)))
            }

            "max" => {
                if args.is_empty() {
                    return Err(MathError::InvalidArgumentCount(name.to_string(), 1, 0));
                }
                let nums: Result<Vec<f64>, _> = args
                    .iter()
                    .map(|arg| match arg {
                        Expr::Number(n) => Ok(*n),
                        _ => Err(MathError::InvalidOperation(
                            "max requires numeric arguments".to_string(),
                        )),
                    })
                    .collect();
                let nums = nums?;
                Ok(Expr::Number(
                    nums.into_iter().fold(f64::NEG_INFINITY, f64::max),
                ))
            }

            _ => Err(MathError::UndefinedFunction(name.to_string())),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn substitute(&self, expr: &Expr, var: &str, value: &Expr) -> Result<Expr, MathError> {
        match expr {
            Expr::Symbol(s) if s == var => Ok(value.clone()),
            Expr::Symbol(_) => Ok(expr.clone()),
            Expr::Number(_) | Expr::Complex(_) => Ok(expr.clone()),

            Expr::Binary { op, left, right } => {
                let new_left = self.substitute(left, var, value)?;
                let new_right = self.substitute(right, var, value)?;
                Ok(Expr::Binary {
                    op: *op,
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                })
            }

            Expr::Unary { op, expr: inner } => {
                let new_inner = self.substitute(inner, var, value)?;
                Ok(Expr::Unary {
                    op: *op,
                    expr: Box::new(new_inner),
                })
            }

            Expr::Function { name, args } => {
                let new_args: Result<Vec<_>, _> = args
                    .iter()
                    .map(|arg| self.substitute(arg, var, value))
                    .collect();
                Ok(Expr::Function {
                    name: name.clone(),
                    args: new_args?,
                })
            }

            _ => Ok(expr.clone()),
        }
    }
}
