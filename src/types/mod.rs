use num_complex::Complex64;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Complex(Complex64),
    Symbol(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Expr>,
    },
    Derivative {
        expr: Box<Expr>,
        var: String,
        order: u32,
    },
    Integral {
        expr: Box<Expr>,
        var: String,
        lower: Option<Box<Expr>>,
        upper: Option<Box<Expr>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,
    Factorial,
    Abs,
}

// Type alias for custom functions
type CustomFunction = Rc<dyn Fn(&[Expr]) -> Result<Expr, MathError>>;

#[derive(Clone)]
pub struct Context {
    variables: HashMap<String, Expr>,
    functions: HashMap<String, CustomFunction>,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("variables", &self.variables)
            .field(
                "functions",
                &format!("{} custom functions", self.functions.len()),
            )
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MathError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Undefined function: {0}")]
    UndefinedFunction(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid argument count for function {0}: expected {1}, got {2}")]
    InvalidArgumentCount(String, usize, usize),

    #[error("Cannot solve equation: {0}")]
    SolverError(String),

    #[error("Numeric overflow")]
    Overflow,
}

impl Expr {
    pub fn zero() -> Self {
        Expr::Number(0.0)
    }

    pub fn one() -> Self {
        Expr::Number(1.0)
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Expr::Number(n) if n.abs() < f64::EPSILON)
    }

    pub fn is_one(&self) -> bool {
        matches!(self, Expr::Number(n) if (n - 1.0).abs() < f64::EPSILON)
    }

    pub fn is_constant(&self) -> bool {
        match self {
            Expr::Number(_) | Expr::Complex(_) => true,
            Expr::Binary { left, right, .. } => left.is_constant() && right.is_constant(),
            Expr::Unary { expr, .. } => expr.is_constant(),
            Expr::Function { args, .. } => args.iter().all(|arg| arg.is_constant()),
            _ => false,
        }
    }

    pub fn contains_var(&self, var: &str) -> bool {
        match self {
            Expr::Symbol(s) => s == var,
            Expr::Binary { left, right, .. } => left.contains_var(var) || right.contains_var(var),
            Expr::Unary { expr, .. } => expr.contains_var(var),
            Expr::Function { args, .. } => args.iter().any(|arg| arg.contains_var(var)),
            Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => expr.contains_var(var),
            _ => false,
        }
    }

    pub fn degree(&self, var: &str) -> u32 {
        match self {
            Expr::Symbol(s) if s == var => 1,
            Expr::Binary {
                op: BinaryOp::Power,
                left,
                right,
            } => {
                if left.contains_var(var) {
                    if let Expr::Number(n) = **right {
                        n as u32
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            Expr::Binary {
                op: BinaryOp::Multiply,
                left,
                right,
            } => left.degree(var) + right.degree(var),
            Expr::Binary {
                op: BinaryOp::Add | BinaryOp::Subtract,
                left,
                right,
            } => left.degree(var).max(right.degree(var)),
            _ => 0,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Complex(c) => {
                if c.im >= 0.0 {
                    write!(f, "{}+{}i", c.re, c.im)
                } else {
                    write!(f, "{}{}i", c.re, c.im)
                }
            }
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Binary { op, left, right } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                    BinaryOp::Power => "^",
                    BinaryOp::Modulo => "%",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }
            Expr::Unary { op, expr } => match op {
                UnaryOp::Negate => write!(f, "-({})", expr),
                UnaryOp::Factorial => write!(f, "{}!", expr),
                UnaryOp::Abs => write!(f, "|{}|", expr),
            },
            Expr::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Derivative { expr, var, order } => {
                write!(f, "d^{}/d{}^{}({})", order, var, order, expr)
            }
            Expr::Integral {
                expr,
                var,
                lower,
                upper,
            } => match (lower, upper) {
                (Some(l), Some(u)) => write!(f, "∫[{},{}] {} d{}", l, u, expr, var),
                _ => write!(f, "∫ {} d{}", expr, var),
            },
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut ctx = Self::new();
        ctx.set_var("pi", Expr::Number(std::f64::consts::PI));
        ctx.set_var("e", Expr::Number(std::f64::consts::E));
        ctx.set_var("tau", Expr::Number(std::f64::consts::TAU));
        ctx
    }

    pub fn set_var(&mut self, name: &str, value: Expr) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_var(&self, name: &str) -> Option<&Expr> {
        self.variables.get(name)
    }
}
