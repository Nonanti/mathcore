use crate::engine::Engine;
use crate::types::{BinaryOp, Expr, MathError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ODESolution {
    pub t: Vec<f64>,
    pub y: Vec<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub enum BoundaryCondition {
    InitialValue {
        t0: f64,
        y0: Vec<f64>,
    },
    BoundaryValue {
        ta: f64,
        tb: f64,
        ya: Vec<f64>,
        yb: Vec<f64>,
    },
}

/// Ordinary differential equation solver
pub struct DifferentialEquations;

impl DifferentialEquations {
    /// Solve first-order ODE using Runge-Kutta 4th order method
    /// dy/dt = f(t, y)
    pub fn solve_ode_first_order(
        f: &Expr,
        t_var: &str,
        y_var: &str,
        initial_condition: (f64, f64),
        t_final: f64,
        steps: usize,
    ) -> Result<ODESolution, MathError> {
        let (t0, y0) = initial_condition;
        let h = (t_final - t0) / steps as f64;

        let mut t_values = vec![t0];
        let mut y_values = vec![vec![y0]];

        let engine = Engine::new();
        let mut t = t0;
        let mut y = y0;

        for _ in 0..steps {
            let mut vars = HashMap::new();
            vars.insert(t_var.to_string(), t);
            vars.insert(y_var.to_string(), y);

            let k1 = match engine.evaluate_with_vars(f, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "ODE function must return number".to_string(),
                    ))
                }
            };

            vars.insert(t_var.to_string(), t + h / 2.0);
            vars.insert(y_var.to_string(), y + h * k1 / 2.0);

            let k2 = match engine.evaluate_with_vars(f, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "ODE function must return number".to_string(),
                    ))
                }
            };

            vars.insert(t_var.to_string(), t + h / 2.0);
            vars.insert(y_var.to_string(), y + h * k2 / 2.0);

            let k3 = match engine.evaluate_with_vars(f, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "ODE function must return number".to_string(),
                    ))
                }
            };

            vars.insert(t_var.to_string(), t + h);
            vars.insert(y_var.to_string(), y + h * k3);

            let k4 = match engine.evaluate_with_vars(f, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "ODE function must return number".to_string(),
                    ))
                }
            };

            // Runge-Kutta 4th order
            y += h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            t += h;

            t_values.push(t);
            y_values.push(vec![y]);
        }

        Ok(ODESolution {
            t: t_values,
            y: y_values,
        })
    }

    /// Solve system of first-order ODEs: dy/dt = F(t, y)
    pub fn solve_ode_system(
        functions: &[Expr],
        t_var: &str,
        y_vars: &[String],
        initial_conditions: (f64, Vec<f64>),
        t_final: f64,
        steps: usize,
    ) -> Result<ODESolution, MathError> {
        if functions.len() != y_vars.len() || functions.len() != initial_conditions.1.len() {
            return Err(MathError::InvalidOperation(
                "Dimension mismatch in ODE system".to_string(),
            ));
        }

        let (t0, y0) = initial_conditions;
        let h = (t_final - t0) / steps as f64;
        let n = y_vars.len();

        let mut t_values = vec![t0];
        let mut y_values = vec![y0.clone()];

        let engine = Engine::new();
        let mut t = t0;
        let mut y = y0;

        for _ in 0..steps {
            let mut k1 = vec![0.0; n];
            let mut k2 = vec![0.0; n];
            let mut k3 = vec![0.0; n];
            let mut k4 = vec![0.0; n];

            // k1
            let mut vars = HashMap::new();
            vars.insert(t_var.to_string(), t);
            for (i, var) in y_vars.iter().enumerate() {
                vars.insert(var.clone(), y[i]);
            }

            for (i, f) in functions.iter().enumerate() {
                k1[i] = match engine.evaluate_with_vars(f, &vars)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "ODE function must return number".to_string(),
                        ))
                    }
                };
            }

            // k2
            vars.insert(t_var.to_string(), t + h / 2.0);
            for (i, var) in y_vars.iter().enumerate() {
                vars.insert(var.clone(), y[i] + h * k1[i] / 2.0);
            }

            for (i, f) in functions.iter().enumerate() {
                k2[i] = match engine.evaluate_with_vars(f, &vars)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "ODE function must return number".to_string(),
                        ))
                    }
                };
            }

            // k3
            vars.insert(t_var.to_string(), t + h / 2.0);
            for (i, var) in y_vars.iter().enumerate() {
                vars.insert(var.clone(), y[i] + h * k2[i] / 2.0);
            }

            for (i, f) in functions.iter().enumerate() {
                k3[i] = match engine.evaluate_with_vars(f, &vars)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "ODE function must return number".to_string(),
                        ))
                    }
                };
            }

            // k4
            vars.insert(t_var.to_string(), t + h);
            for (i, var) in y_vars.iter().enumerate() {
                vars.insert(var.clone(), y[i] + h * k3[i]);
            }

            for (i, f) in functions.iter().enumerate() {
                k4[i] = match engine.evaluate_with_vars(f, &vars)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "ODE function must return number".to_string(),
                        ))
                    }
                };
            }

            // Update using RK4
            for i in 0..n {
                y[i] += h * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]) / 6.0;
            }
            t += h;

            t_values.push(t);
            y_values.push(y.clone());
        }

        Ok(ODESolution {
            t: t_values,
            y: y_values,
        })
    }

    /// Solve second-order ODE: y'' + p(x)y' + q(x)y = r(x)
    pub fn solve_ode_second_order(
        p: &Expr,
        q: &Expr,
        r: &Expr,
        x_var: &str,
        initial_conditions: (f64, f64, f64), // (x0, y0, y'0)
        x_final: f64,
        steps: usize,
    ) -> Result<ODESolution, MathError> {
        // Convert to system of first-order ODEs
        // Let u = y, v = y'
        // Then: u' = v, v' = r(x) - p(x)v - q(x)u

        let (x0, y0, dy0) = initial_conditions;
        let h = (x_final - x0) / steps as f64;

        let mut x_values = vec![x0];
        let mut y_values = vec![vec![y0, dy0]];

        let engine = Engine::new();
        let mut x = x0;
        let mut u = y0;
        let mut v = dy0;

        for _ in 0..steps {
            let mut vars = HashMap::new();
            vars.insert(x_var.to_string(), x);

            let p_val = match engine.evaluate_with_vars(p, &vars)? {
                Expr::Number(n) => n,
                _ => 0.0,
            };

            let q_val = match engine.evaluate_with_vars(q, &vars)? {
                Expr::Number(n) => n,
                _ => 0.0,
            };

            let r_val = match engine.evaluate_with_vars(r, &vars)? {
                Expr::Number(n) => n,
                _ => 0.0,
            };

            // RK4 for the system
            let k1_u = v;
            let k1_v = r_val - p_val * v - q_val * u;

            let k2_u = v + h * k1_v / 2.0;
            let k2_v = r_val - p_val * (v + h * k1_v / 2.0) - q_val * (u + h * k1_u / 2.0);

            let k3_u = v + h * k2_v / 2.0;
            let k3_v = r_val - p_val * (v + h * k2_v / 2.0) - q_val * (u + h * k2_u / 2.0);

            let k4_u = v + h * k3_v;
            let k4_v = r_val - p_val * (v + h * k3_v) - q_val * (u + h * k3_u);

            u += h * (k1_u + 2.0 * k2_u + 2.0 * k3_u + k4_u) / 6.0;
            v += h * (k1_v + 2.0 * k2_v + 2.0 * k3_v + k4_v) / 6.0;
            x += h;

            x_values.push(x);
            y_values.push(vec![u, v]);
        }

        Ok(ODESolution {
            t: x_values,
            y: y_values,
        })
    }

    /// Solve using Euler's method (simpler but less accurate)
    pub fn euler_method(
        f: &Expr,
        t_var: &str,
        y_var: &str,
        initial_condition: (f64, f64),
        t_final: f64,
        steps: usize,
    ) -> Result<ODESolution, MathError> {
        let (t0, y0) = initial_condition;
        let h = (t_final - t0) / steps as f64;

        let mut t_values = vec![t0];
        let mut y_values = vec![vec![y0]];

        let engine = Engine::new();
        let mut t = t0;
        let mut y = y0;

        for _ in 0..steps {
            let mut vars = HashMap::new();
            vars.insert(t_var.to_string(), t);
            vars.insert(y_var.to_string(), y);

            let dy_dt = match engine.evaluate_with_vars(f, &vars)? {
                Expr::Number(n) => n,
                _ => {
                    return Err(MathError::InvalidOperation(
                        "ODE function must return number".to_string(),
                    ))
                }
            };

            y += h * dy_dt;
            t += h;

            t_values.push(t);
            y_values.push(vec![y]);
        }

        Ok(ODESolution {
            t: t_values,
            y: y_values,
        })
    }

    /// Solve stiff ODEs using implicit method
    #[allow(clippy::too_many_arguments)]
    pub fn solve_stiff_ode(
        f: &Expr,
        jacobian: Option<&Expr>,
        t_var: &str,
        y_var: &str,
        initial_condition: (f64, f64),
        t_final: f64,
        steps: usize,
        tolerance: f64,
    ) -> Result<ODESolution, MathError> {
        // Backward Euler method with Newton iteration
        let (t0, y0) = initial_condition;
        let h = (t_final - t0) / steps as f64;

        let mut t_values = vec![t0];
        let mut y_values = vec![vec![y0]];

        let engine = Engine::new();
        let mut t = t0;
        let mut y = y0;

        for _ in 0..steps {
            t += h;
            let mut y_new = y;

            // Newton iteration for implicit equation
            for _ in 0..10 {
                let mut vars = HashMap::new();
                vars.insert(t_var.to_string(), t);
                vars.insert(y_var.to_string(), y_new);

                let f_val = match engine.evaluate_with_vars(f, &vars)? {
                    Expr::Number(n) => n,
                    _ => {
                        return Err(MathError::InvalidOperation(
                            "ODE function must return number".to_string(),
                        ))
                    }
                };

                let residual = y_new - y - h * f_val;

                if residual.abs() < tolerance {
                    break;
                }

                // Compute Jacobian
                let jac = if let Some(j) = jacobian {
                    match engine.evaluate_with_vars(j, &vars)? {
                        Expr::Number(n) => n,
                        _ => 1.0,
                    }
                } else {
                    // Numerical Jacobian
                    let eps = 1e-8;
                    vars.insert(y_var.to_string(), y_new + eps);
                    let f_plus = match engine.evaluate_with_vars(f, &vars)? {
                        Expr::Number(n) => n,
                        _ => f_val,
                    };
                    (f_plus - f_val) / eps
                };

                let newton_denominator = 1.0 - h * jac;
                if newton_denominator.abs() < 1e-10 {
                    return Err(MathError::InvalidOperation(
                        "Newton iteration failed".to_string(),
                    ));
                }

                y_new -= residual / newton_denominator;
            }

            y = y_new;
            t_values.push(t);
            y_values.push(vec![y]);
        }

        Ok(ODESolution {
            t: t_values,
            y: y_values,
        })
    }

    /// Analytical solution for linear ODEs with constant coefficients
    pub fn solve_linear_constant_coeff(
        coefficients: &[f64],       // [a_n, a_{n-1}, ..., a_1, a_0]
        initial_conditions: &[f64], // [y(0), y'(0), ..., y^{(n-1)}(0)]
    ) -> Result<Expr, MathError> {
        if coefficients.is_empty() || initial_conditions.is_empty() {
            return Err(MathError::InvalidOperation(
                "Empty coefficients or initial conditions".to_string(),
            ));
        }

        let n = coefficients.len() - 1;
        if initial_conditions.len() != n {
            return Err(MathError::InvalidOperation(
                "Initial conditions count mismatch".to_string(),
            ));
        }

        // Find characteristic polynomial roots
        use crate::solver::Solver;

        // Build characteristic polynomial
        let mut char_poly = Expr::zero();
        for (i, &coeff) in coefficients.iter().enumerate() {
            let power = n - i;
            let term = if power == 0 {
                Expr::Number(coeff)
            } else {
                Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(Expr::Number(coeff)),
                    right: Box::new(Expr::Binary {
                        op: BinaryOp::Power,
                        left: Box::new(Expr::Symbol("r".to_string())),
                        right: Box::new(Expr::Number(power as f64)),
                    }),
                }
            };

            char_poly = Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(char_poly),
                right: Box::new(term),
            };
        }

        let roots = Solver::solve(&char_poly, "r")?;

        // Build general solution
        let mut solution = Expr::zero();
        let t = Expr::Symbol("t".to_string());

        for (i, root) in roots.iter().enumerate() {
            let c = Expr::Symbol(format!("C{}", i + 1));

            let term = match root {
                Expr::Number(r) => {
                    // C_i * e^(r*t)
                    Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(c),
                        right: Box::new(Expr::Function {
                            name: "exp".to_string(),
                            args: vec![Expr::Binary {
                                op: BinaryOp::Multiply,
                                left: Box::new(Expr::Number(*r)),
                                right: Box::new(t.clone()),
                            }],
                        }),
                    }
                }
                Expr::Complex(c_num) => {
                    // Handle complex roots: e^(αt) * (C1*cos(βt) + C2*sin(βt))
                    let alpha = c_num.re;
                    let beta = c_num.im;

                    let exp_part = Expr::Function {
                        name: "exp".to_string(),
                        args: vec![Expr::Binary {
                            op: BinaryOp::Multiply,
                            left: Box::new(Expr::Number(alpha)),
                            right: Box::new(t.clone()),
                        }],
                    };

                    let cos_part = Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(c.clone()),
                        right: Box::new(Expr::Function {
                            name: "cos".to_string(),
                            args: vec![Expr::Binary {
                                op: BinaryOp::Multiply,
                                left: Box::new(Expr::Number(beta)),
                                right: Box::new(t.clone()),
                            }],
                        }),
                    };

                    let sin_part = Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(Expr::Symbol(format!("C{}", i + 2))),
                        right: Box::new(Expr::Function {
                            name: "sin".to_string(),
                            args: vec![Expr::Binary {
                                op: BinaryOp::Multiply,
                                left: Box::new(Expr::Number(beta)),
                                right: Box::new(t.clone()),
                            }],
                        }),
                    };

                    Expr::Binary {
                        op: BinaryOp::Multiply,
                        left: Box::new(exp_part),
                        right: Box::new(Expr::Binary {
                            op: BinaryOp::Add,
                            left: Box::new(cos_part),
                            right: Box::new(sin_part),
                        }),
                    }
                }
                _ => c,
            };

            solution = Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(solution),
                right: Box::new(term),
            };
        }

        Ok(solution)
    }
}

/// Partial differential equations solver using finite difference methods
pub struct PDESolver;

impl PDESolver {
    /// Solve heat equation: ∂u/∂t = α * ∂²u/∂x²
    #[allow(clippy::too_many_arguments)]
    pub fn solve_heat_equation(
        alpha: f64,
        initial_condition: &dyn Fn(f64) -> f64,
        boundary_left: f64,
        boundary_right: f64,
        x_range: (f64, f64),
        t_final: f64,
        nx: usize,
        nt: usize,
    ) -> Result<Vec<Vec<f64>>, MathError> {
        let (x_min, x_max) = x_range;
        let dx = (x_max - x_min) / (nx - 1) as f64;
        let dt = t_final / (nt - 1) as f64;

        let stability = alpha * dt / (dx * dx);
        if stability > 0.5 {
            return Err(MathError::InvalidOperation(format!(
                "Unstable: α*dt/dx² = {} > 0.5",
                stability
            )));
        }

        let mut u = vec![vec![0.0; nx]; nt];

        // Initial condition
        for i in 0..nx {
            let x = x_min + i as f64 * dx;
            u[0][i] = initial_condition(x);
        }

        // Time evolution
        for n in 0..nt - 1 {
            // Boundary conditions
            u[n + 1][0] = boundary_left;
            u[n + 1][nx - 1] = boundary_right;

            // Interior points (explicit finite difference)
            for i in 1..nx - 1 {
                u[n + 1][i] = u[n][i] + stability * (u[n][i + 1] - 2.0 * u[n][i] + u[n][i - 1]);
            }
        }

        Ok(u)
    }

    /// Solve wave equation: ∂²u/∂t² = c² * ∂²u/∂x²
    pub fn solve_wave_equation(
        c: f64,
        initial_position: &dyn Fn(f64) -> f64,
        initial_velocity: &dyn Fn(f64) -> f64,
        x_range: (f64, f64),
        t_final: f64,
        nx: usize,
        nt: usize,
    ) -> Result<Vec<Vec<f64>>, MathError> {
        let (x_min, x_max) = x_range;
        let dx = (x_max - x_min) / (nx - 1) as f64;
        let dt = t_final / (nt - 1) as f64;

        let courant = c * dt / dx;
        if courant > 1.0 {
            return Err(MathError::InvalidOperation(format!(
                "CFL condition violated: c*dt/dx = {} > 1",
                courant
            )));
        }

        let mut u = vec![vec![0.0; nx]; nt];
        let courant_sq = courant * courant;

        // Initial conditions
        for i in 0..nx {
            let x = x_min + i as f64 * dx;
            u[0][i] = initial_position(x);
        }

        // First time step using initial velocity
        for i in 1..nx - 1 {
            let x = x_min + i as f64 * dx;
            u[1][i] = u[0][i]
                + dt * initial_velocity(x)
                + 0.5 * courant_sq * (u[0][i + 1] - 2.0 * u[0][i] + u[0][i - 1]);
        }

        // Time evolution
        for n in 1..nt - 1 {
            for i in 1..nx - 1 {
                u[n + 1][i] = 2.0 * u[n][i] - u[n - 1][i]
                    + courant_sq * (u[n][i + 1] - 2.0 * u[n][i] + u[n][i - 1]);
            }
        }

        Ok(u)
    }

    /// Solve Laplace equation: ∇²u = 0
    pub fn solve_laplace_equation(
        boundary_conditions: &dyn Fn(f64, f64) -> Option<f64>,
        x_range: (f64, f64),
        y_range: (f64, f64),
        nx: usize,
        ny: usize,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<Vec<Vec<f64>>, MathError> {
        let (x_min, x_max) = x_range;
        let (y_min, y_max) = y_range;
        let dx = (x_max - x_min) / (nx - 1) as f64;
        let dy = (y_max - y_min) / (ny - 1) as f64;

        let mut u = vec![vec![0.0; nx]; ny];
        let mut is_boundary = vec![vec![false; nx]; ny];

        // Set boundary conditions
        for j in 0..ny {
            for i in 0..nx {
                let x = x_min + i as f64 * dx;
                let y = y_min + j as f64 * dy;

                if let Some(value) = boundary_conditions(x, y) {
                    u[j][i] = value;
                    is_boundary[j][i] = true;
                }
            }
        }

        // Gauss-Seidel iteration
        for _ in 0..max_iterations {
            let mut max_change = 0.0;

            for j in 1..ny - 1 {
                for i in 1..nx - 1 {
                    if !is_boundary[j][i] {
                        let old_value = u[j][i];
                        u[j][i] = 0.25 * (u[j][i + 1] + u[j][i - 1] + u[j + 1][i] + u[j - 1][i]);
                        max_change = f64::max(max_change, (u[j][i] - old_value).abs());
                    }
                }
            }

            if max_change < tolerance {
                break;
            }
        }

        Ok(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_first_order_ode() {
        // Solve dy/dt = -y, y(0) = 1
        // Analytical solution: y = e^(-t)
        let f = Parser::parse("-y").unwrap();
        let solution =
            DifferentialEquations::solve_ode_first_order(&f, "t", "y", (0.0, 1.0), 1.0, 100)
                .unwrap();

        // Check that y(1) ≈ e^(-1) ≈ 0.3679
        let final_y = solution.y.last().unwrap()[0];
        assert!((final_y - 0.3679).abs() < 0.01);
    }

    #[test]
    fn test_ode_system() {
        // Solve the system:
        // dx/dt = -y
        // dy/dt = x
        // Initial conditions: x(0) = 1, y(0) = 0
        // This represents circular motion

        let f1 = Parser::parse("-y").unwrap();
        let f2 = Parser::parse("x").unwrap();

        let solution = DifferentialEquations::solve_ode_system(
            &[f1, f2],
            "t",
            &["x".to_string(), "y".to_string()],
            (0.0, vec![1.0, 0.0]),
            6.28, // Approximately 2π
            1000,
        )
        .unwrap();

        // After one period, should return close to initial values
        let final_x = solution.y.last().unwrap()[0];
        let final_y = solution.y.last().unwrap()[1];

        assert!((final_x - 1.0).abs() < 0.1);
        assert!(final_y.abs() < 0.1);
    }

    #[test]
    fn test_heat_equation() {
        let initial = |x: f64| if (x - 0.5).abs() < 0.1 { 1.0 } else { 0.0 };

        let solution =
            PDESolver::solve_heat_equation(0.1, &initial, 0.0, 0.0, (0.0, 1.0), 0.1, 50, 100)
                .unwrap();

        assert_eq!(solution.len(), 100);
        assert_eq!(solution[0].len(), 50);
    }
}
