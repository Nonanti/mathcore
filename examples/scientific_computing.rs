use mathcore::differential::{DifferentialEquations, PDESolver};
use mathcore::parser::Parser;

fn main() {
    println!("Scientific Computing Examples\n");

    population_dynamics();
    heat_diffusion();
    wave_propagation();
    pendulum_motion();
}

fn population_dynamics() {
    println!("=== Population Dynamics (Logistic Growth) ===");

    // dP/dt = r*P*(1 - P/K)
    // where r = growth rate, K = carrying capacity
    let r = 0.5;
    let k = 1000.0;

    // Parse the logistic growth equation
    let expr = Parser::parse(&format!("{}*P*(1 - P/{})", r, k)).unwrap();

    let solution = DifferentialEquations::solve_ode_first_order(
        &expr,
        "t",
        "P",
        (0.0, 10.0), // Initial population of 10
        20.0,        // Time span of 20 units
        200,         // 200 time steps
    )
    .unwrap();

    // Print selected points
    println!("Time\tPopulation");
    for i in (0..solution.t.len()).step_by(20) {
        println!("{:.1}\t{:.1}", solution.t[i], solution.y[i][0]);
    }

    println!("\nPopulation stabilizes at carrying capacity K = {}\n", k);
}

fn heat_diffusion() {
    println!("=== Heat Diffusion in a Rod ===");

    // Initial temperature distribution: hot spot in the middle
    let initial_temp = |x: f64| {
        if (x - 0.5).abs() < 0.1 {
            100.0 // Hot spot
        } else {
            20.0 // Room temperature
        }
    };

    let solution = PDESolver::solve_heat_equation(
        0.01, // Thermal diffusivity
        &initial_temp,
        20.0,       // Left boundary temperature
        20.0,       // Right boundary temperature
        (0.0, 1.0), // Rod from x=0 to x=1
        0.5,        // Time = 0.5
        50,         // 50 spatial points
        100,        // 100 time steps
    )
    .unwrap();

    println!("Temperature distribution after heat diffusion:");
    println!("Position\tInitial\t\tFinal");

    for i in (0..50).step_by(5) {
        let x = i as f64 / 49.0;
        println!(
            "{:.2}\t\t{:.1}\t\t{:.1}",
            x,
            solution[0][i],  // Initial
            solution[99][i]  // Final
        );
    }
    println!();
}

fn wave_propagation() {
    println!("=== Wave Propagation ===");

    // Initial displacement: Gaussian pulse
    let initial_position = |x: f64| {
        let center = 0.5;
        let width = 0.05;
        (-(((x - center) / width).powi(2) as f64)).exp()
    };

    // Initially at rest
    let initial_velocity = |_: f64| 0.0;

    let solution = PDESolver::solve_wave_equation(
        0.5, // Wave speed
        &initial_position,
        &initial_velocity,
        (0.0, 1.0), // Domain
        1.0,        // Time
        100,        // Spatial points
        200,        // Time steps
    )
    .unwrap();

    println!("Wave amplitude at different times:");
    println!("Position\tt=0\t\tt=0.5\t\tt=1.0");

    for i in (0..100).step_by(10) {
        let x = i as f64 / 99.0;
        println!(
            "{:.2}\t\t{:.4}\t\t{:.4}\t\t{:.4}",
            x,
            solution[0][i],   // t=0
            solution[100][i], // t=0.5
            solution[199][i]  // t=1.0
        );
    }
    println!();
}

fn pendulum_motion() {
    println!("=== Nonlinear Pendulum ===");

    // Second-order ODE: θ'' + (g/L)*sin(θ) = 0
    // Convert to system: θ' = ω, ω' = -(g/L)*sin(θ)

    let g_over_l = 9.81; // g/L ratio

    // System of equations
    let theta_dot = Parser::parse("omega").unwrap();
    let omega_dot = Parser::parse(&format!("-{}*sin(theta)", g_over_l)).unwrap();

    let solution = DifferentialEquations::solve_ode_system(
        &[theta_dot, omega_dot],
        "t",
        &["theta".to_string(), "omega".to_string()],
        (0.0, vec![1.0, 0.0]), // Initial angle = 1 rad, initial velocity = 0
        10.0,                  // 10 seconds
        1000,                  // 1000 time steps
    )
    .unwrap();

    println!("Pendulum motion (nonlinear):");
    println!("Time\tAngle (rad)\tAngular Velocity");

    for i in (0..solution.t.len()).step_by(100) {
        println!(
            "{:.1}\t{:.4}\t\t{:.4}",
            solution.t[i],
            solution.y[i][0], // theta
            solution.y[i][1]  // omega
        );
    }

    // Calculate period
    let mut crossings = Vec::new();
    for i in 1..solution.y.len() {
        if solution.y[i - 1][0] > 0.0 && solution.y[i][0] <= 0.0 {
            crossings.push(solution.t[i]);
        }
    }

    if crossings.len() >= 2 {
        let period = 2.0 * (crossings[1] - crossings[0]);
        println!("\nEstimated period: {:.3} seconds", period);

        let linear_period = 2.0 * std::f64::consts::PI / (g_over_l as f64).sqrt();
        println!("Linear approximation: {:.3} seconds", linear_period);
        println!(
            "Difference: {:.1}%",
            100.0 * (period - linear_period).abs() / linear_period
        );
    }
}
