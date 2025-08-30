pub mod limits;

use crate::engine::Engine;
use crate::types::{BinaryOp, Expr, MathError, UnaryOp};
use std::collections::HashMap;

// calculus stuff
pub struct Calculus;

impl Calculus {
    /// differentiate expr w.r.t. var
    pub fn differentiate(expr: &Expr, var: &str) -> Result<Expr, MathError> {
        let result = Self::diff_internal(expr, var)?;
        Ok(Self::simplify_basic(&result))
    }

    fn diff_internal(expr: &Expr, var: &str) -> Result<Expr, MathError> {
        match expr {
            Expr::Number(_) | Expr::Complex(_) => Ok(Expr::zero()),

            Expr::Symbol(s) => {
                if s == var {
                    Ok(Expr::one())
                } else {
                    Ok(Expr::zero())
                }
            }

            Expr::Binary { op, left, right } => {
                match op {
                    BinaryOp::Add | BinaryOp::Subtract => {
                        let dl = Self::diff_internal(left, var)?;
                        let dr = Self::diff_internal(right, var)?;
                        Ok(Expr::Binary {
                            op: *op,
                            left: Box::new(dl),
                            right: Box::new(dr),
                        })
                    }

                    BinaryOp::Multiply => {
                        // product rule: (f*g)' = f'*g + f*g'
                        let dl = Self::diff_internal(left, var)?;
                        let dr = Self::diff_internal(right, var)?;

                        let term1 = Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: Box::new(dl),
                            right: right.clone(),
                        };

                        let term2 = Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: left.clone(),
                            right: Box::new(dr),
                        };

                        Ok(Expr::Binary {
                            op: BinaryOp::Add,
                            left: Box::new(term1),
                            right: Box::new(term2),
                        })
                    }

                    BinaryOp::Divide => {
                        let dl = Self::diff_internal(left, var)?;
                        let dr = Self::diff_internal(right, var)?;

                        let numerator_term1 = Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: Box::new(dl),
                            right: right.clone(),
                        };

                        let numerator_term2 = Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: left.clone(),
                            right: Box::new(dr),
                        };

                        let numerator = Expr::Binary {
                            op: BinaryOp::Subtract,
                            left: Box::new(numerator_term1),
                            right: Box::new(numerator_term2),
                        };

                        let denominator = Expr::Binary {
                            op: BinaryOp::Power,
                            left: right.clone(),
                            right: Box::new(Expr::Number(2.0)),
                        };

                        Ok(Expr::Binary {
                            op: BinaryOp::Divide,
                            left: Box::new(numerator),
                            right: Box::new(denominator),
                        })
                    }

                    BinaryOp::Power => {
                        if !left.contains_var(var) && !right.contains_var(var) {
                            return Ok(Expr::zero());
                        }

                        if !right.contains_var(var) {
                            if let Expr::Number(n) = **right {
                                let new_exp = Expr::Number(n - 1.0);
                                let coeff = right.clone();
                                let base_deriv = Self::diff_internal(left, var)?;

                                let power_part = Expr::Binary {
                                    op: BinaryOp::Power,
                                    left: left.clone(),
                                    right: Box::new(new_exp),
                                };

                                let result = Expr::Binary {
                                    op: BinaryOp::Multiply,
                                    left: coeff,
                                    right: Box::new(power_part),
                                };

                                Ok(Expr::Binary {
                                    op: BinaryOp::Multiply,
                                    left: Box::new(result),
                                    right: Box::new(base_deriv),
                                })
                            } else {
                                let ln_base = Expr::Function {
                                    name: "ln".to_string(),
                                    args: vec![*left.clone()],
                                };

                                let product = Expr::Binary {
                                    op: BinaryOp::Multiply,
                                    left: right.clone(),
                                    right: Box::new(ln_base),
                                };

                                let exp_part = Expr::Function {
                                    name: "exp".to_string(),
                                    args: vec![product],
                                };

                                Self::diff_internal(&exp_part, var)
                            }
                        } else {
                            let ln_base = Expr::Function {
                                name: "ln".to_string(),
                                args: vec![*left.clone()],
                            };

                            let product = Expr::Binary {
                                op: BinaryOp::Multiply,
                                left: right.clone(),
                                right: Box::new(ln_base),
                            };

                            let exp_part = Expr::Function {
                                name: "exp".to_string(),
                                args: vec![product],
                            };

                            Self::diff_internal(&exp_part, var)
                        }
                    }

                    _ => Err(MathError::InvalidOperation(format!(
                        "Cannot differentiate {:?} operation",
                        op
                    ))),
                }
            }

            Expr::Unary { op, expr: inner } => match op {
                UnaryOp::Negate => {
                    let d_inner = Self::diff_internal(inner, var)?;
                    Ok(Expr::Unary {
                        op: UnaryOp::Negate,
                        expr: Box::new(d_inner),
                    })
                }

                UnaryOp::Abs => {
                    let sign = Expr::Binary {
                        op: BinaryOp::Divide,
                        left: inner.clone(),
                        right: Box::new(Expr::Unary {
                            op: UnaryOp::Abs,
                            expr: inner.clone(),
                        }),
                    };

                    let d_inner = Self::diff_internal(inner, var)?;

                    Ok(Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(sign),
                        right: Box::new(d_inner),
                    })
                }

                _ => Err(MathError::InvalidOperation(format!(
                    "Cannot differentiate {:?} operation",
                    op
                ))),
            },

            Expr::Function { name, args } => Self::diff_function(name, args, var),

            Expr::Derivative {
                expr: inner,
                var: d_var,
                order,
            } => {
                if d_var == var {
                    Ok(Expr::Derivative {
                        expr: inner.clone(),
                        var: var.to_string(),
                        order: order + 1,
                    })
                } else {
                    let d_inner = Self::diff_internal(inner, var)?;
                    Ok(Expr::Derivative {
                        expr: Box::new(d_inner),
                        var: d_var.clone(),
                        order: *order,
                    })
                }
            }

            _ => Err(MathError::InvalidOperation(
                "Cannot differentiate this expression".to_string(),
            )),
        }
    }

    fn diff_function(name: &str, args: &[Expr], var: &str) -> Result<Expr, MathError> {
        if args.is_empty() {
            return Err(MathError::InvalidArgumentCount(name.to_string(), 1, 0));
        }

        let arg = &args[0];
        let d_arg = Self::diff_internal(arg, var)?;

        let derivative = match name {
            "sin" => Expr::Function {
                name: "cos".to_string(),
                args: vec![arg.clone()],
            },

            "cos" => Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(Expr::Function {
                    name: "sin".to_string(),
                    args: vec![arg.clone()],
                }),
            },

            "tan" => Expr::Binary {
                op: BinaryOp::Power,
                left: Box::new(Expr::Function {
                    name: "sec".to_string(),
                    args: vec![arg.clone()],
                }),
                right: Box::new(Expr::Number(2.0)),
            },

            "ln" => Expr::Binary {
                op: BinaryOp::Divide,
                left: Box::new(Expr::one()),
                right: Box::new(arg.clone()),
            },

            "exp" => Expr::Function {
                name: "exp".to_string(),
                args: vec![arg.clone()],
            },

            "sqrt" => {
                let denominator = Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(Expr::Number(2.0)),
                    right: Box::new(Expr::Function {
                        name: "sqrt".to_string(),
                        args: vec![arg.clone()],
                    }),
                };

                Expr::Binary {
                    op: BinaryOp::Divide,
                    left: Box::new(Expr::one()),
                    right: Box::new(denominator),
                }
            }

            "sec" => {
                let tan_part = Expr::Function {
                    name: "tan".to_string(),
                    args: vec![arg.clone()],
                };
                let sec_part = Expr::Function {
                    name: "sec".to_string(),
                    args: vec![arg.clone()],
                };

                Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(sec_part),
                    right: Box::new(tan_part),
                }
            }

            _ => {
                return Ok(Expr::Derivative {
                    expr: Box::new(Expr::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                    var: var.to_string(),
                    order: 1,
                });
            }
        };

        Ok(Expr::Binary {
            op: BinaryOp::Multiply,
            left: Box::new(derivative),
            right: Box::new(d_arg),
        })
    }

    /// Compute the symbolic integral of an expression
    pub fn integrate(expr: &Expr, var: &str) -> Result<Expr, MathError> {
        let result = Self::integrate_internal(expr, var)?;
        Ok(Self::simplify_basic(&result))
    }

    fn integrate_internal(expr: &Expr, var: &str) -> Result<Expr, MathError> {
        match expr {
            Expr::Number(n) => Ok(Expr::Binary {
                op: BinaryOp::Multiply,
                left: Box::new(Expr::Number(*n)),
                right: Box::new(Expr::Symbol(var.to_string())),
            }),

            Expr::Symbol(s) if s == var => Ok(Expr::Binary {
                op: BinaryOp::Divide,
                left: Box::new(Expr::Binary {
                    op: BinaryOp::Power,
                    left: Box::new(Expr::Symbol(var.to_string())),
                    right: Box::new(Expr::Number(2.0)),
                }),
                right: Box::new(Expr::Number(2.0)),
            }),

            Expr::Symbol(_) | Expr::Complex(_) => Ok(Expr::Binary {
                op: BinaryOp::Multiply,
                left: Box::new(expr.clone()),
                right: Box::new(Expr::Symbol(var.to_string())),
            }),

            Expr::Binary { op, left, right }
                if matches!(op, BinaryOp::Add | BinaryOp::Subtract) =>
            {
                let int_left = Self::integrate_internal(left, var)?;
                let int_right = Self::integrate_internal(right, var)?;
                Ok(Expr::Binary {
                    op: *op,
                    left: Box::new(int_left),
                    right: Box::new(int_right),
                })
            }

            Expr::Binary {
                op: BinaryOp::Multiply,
                left,
                right,
            } => {
                if !left.contains_var(var) {
                    let int_right = Self::integrate_internal(right, var)?;
                    Ok(Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: left.clone(),
                        right: Box::new(int_right),
                    })
                } else if !right.contains_var(var) {
                    let int_left = Self::integrate_internal(left, var)?;
                    Ok(Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(int_left),
                        right: right.clone(),
                    })
                } else {
                    Ok(Expr::Integral {
                        expr: Box::new(expr.clone()),
                        var: var.to_string(),
                        lower: None,
                        upper: None,
                    })
                }
            }

            Expr::Binary {
                op: BinaryOp::Power,
                left,
                right,
            } => {
                if let (Expr::Symbol(s), Expr::Number(n)) = (&**left, &**right) {
                    if s == var && (*n - (-1.0)).abs() > f64::EPSILON {
                        let new_exp = n + 1.0;
                        Ok(Expr::Binary {
                            op: BinaryOp::Divide,
                            left: Box::new(Expr::Binary {
                                op: BinaryOp::Power,
                                left: left.clone(),
                                right: Box::new(Expr::Number(new_exp)),
                            }),
                            right: Box::new(Expr::Number(new_exp)),
                        })
                    } else if s == var {
                        Ok(Expr::Function {
                            name: "ln".to_string(),
                            args: vec![Expr::Symbol(var.to_string())],
                        })
                    } else {
                        Ok(Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: Box::new(expr.clone()),
                            right: Box::new(Expr::Symbol(var.to_string())),
                        })
                    }
                } else {
                    Ok(Expr::Integral {
                        expr: Box::new(expr.clone()),
                        var: var.to_string(),
                        lower: None,
                        upper: None,
                    })
                }
            }

            Expr::Function { name, args } if args.len() == 1 => {
                Self::integrate_function(name, &args[0], var)
            }

            _ => Ok(Expr::Integral {
                expr: Box::new(expr.clone()),
                var: var.to_string(),
                lower: None,
                upper: None,
            }),
        }
    }

    fn integrate_function(name: &str, arg: &Expr, var: &str) -> Result<Expr, MathError> {
        if let Expr::Symbol(s) = arg {
            if s == var {
                let integral = match name {
                    "sin" => Expr::Unary {
                        op: UnaryOp::Negate,
                        expr: Box::new(Expr::Function {
                            name: "cos".to_string(),
                            args: vec![arg.clone()],
                        }),
                    },

                    "cos" => Expr::Function {
                        name: "sin".to_string(),
                        args: vec![arg.clone()],
                    },

                    "exp" => Expr::Function {
                        name: "exp".to_string(),
                        args: vec![arg.clone()],
                    },

                    _ => {
                        return Ok(Expr::Integral {
                            expr: Box::new(Expr::Function {
                                name: name.to_string(),
                                args: vec![arg.clone()],
                            }),
                            var: var.to_string(),
                            lower: None,
                            upper: None,
                        });
                    }
                };
                return Ok(integral);
            }
        }

        Ok(Expr::Integral {
            expr: Box::new(Expr::Function {
                name: name.to_string(),
                args: vec![arg.clone()],
            }),
            var: var.to_string(),
            lower: None,
            upper: None,
        })
    }

    pub fn numerical_integrate(
        expr: &Expr,
        var: &str,
        lower: f64,
        upper: f64,
        steps: usize,
    ) -> Result<f64, MathError> {
        let engine = Engine::new();
        let h = (upper - lower) / steps as f64;
        let mut sum = 0.0;

        for i in 0..steps {
            let x1 = lower + i as f64 * h;
            let x2 = x1 + h;
            let mid = (x1 + x2) / 2.0;

            let mut vars = HashMap::new();
            vars.insert(var.to_string(), x1);
            let y1 = match engine.evaluate_with_vars(expr, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "Expected numeric result".to_string(),
                    ))
                }
            };

            vars.insert(var.to_string(), x2);
            let y2 = match engine.evaluate_with_vars(expr, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "Expected numeric result".to_string(),
                    ))
                }
            };

            vars.insert(var.to_string(), mid);
            let y_mid = match engine.evaluate_with_vars(expr, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "Expected numeric result".to_string(),
                    ))
                }
            };

            sum += h * (y1 + 4.0 * y_mid + y2) / 6.0;
        }

        Ok(sum)
    }

    fn simplify_basic(expr: &Expr) -> Expr {
        match expr {
            Expr::Binary { op, left, right } => {
                let left = Self::simplify_basic(left);
                let right = Self::simplify_basic(right);

                match (op, &left, &right) {
                    (BinaryOp::Add, e, other) | (BinaryOp::Add, other, e) if e.is_zero() => {
                        other.clone()
                    }
                    (BinaryOp::Subtract, e, other) if other.is_zero() => e.clone(),
                    (BinaryOp::Subtract, e, other) if e.is_zero() => Expr::Unary {
                        op: UnaryOp::Negate,
                        expr: Box::new(other.clone()),
                    },
                    (BinaryOp::Multiply, e, _) | (BinaryOp::Multiply, _, e) if e.is_zero() => {
                        Expr::zero()
                    }
                    (BinaryOp::Multiply, e, other) | (BinaryOp::Multiply, other, e)
                        if e.is_one() =>
                    {
                        other.clone()
                    }
                    (BinaryOp::Divide, e, other) if other.is_one() => e.clone(),
                    (BinaryOp::Power, _e, other) if other.is_zero() => Expr::one(),
                    (BinaryOp::Power, e, other) if other.is_one() => e.clone(),

                    _ => Expr::Binary {
                        op: match op {
                            BinaryOp::Add => BinaryOp::Add,
                            BinaryOp::Subtract => BinaryOp::Subtract,
                            BinaryOp::Multiply => BinaryOp::Multiply,
                            BinaryOp::Divide => BinaryOp::Divide,
                            BinaryOp::Power => BinaryOp::Power,
                            BinaryOp::Modulo => BinaryOp::Modulo,
                        },
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }

            Expr::Unary { op, expr: inner } => {
                let inner = Self::simplify_basic(inner);

                match (op, &inner) {
                    (UnaryOp::Negate, Expr::Number(n)) => Expr::Number(-n),
                    (
                        UnaryOp::Negate,
                        Expr::Unary {
                            op: UnaryOp::Negate,
                            expr: e,
                        },
                    ) => *e.clone(),
                    _ => Expr::Unary {
                        op: *op,
                        expr: Box::new(inner),
                    },
                }
            }

            _ => expr.clone(),
        }
    }
}
