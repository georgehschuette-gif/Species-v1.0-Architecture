// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaoticDynamics {
    pub state: [f64; 3],
    pub lyapunov_exponent: f64,
    pub iteration: usize,
}

impl ChaoticDynamics {
    pub fn new(initial_state: [f64; 3], lyapunov_exponent: f64) -> Self {
        Self { state: initial_state, lyapunov_exponent, iteration: 0 }
    }

    pub fn step(&mut self) {
        self.iteration += 1;
        let [x, y, z] = self.state;
        let sigma = 10.0;
        let rho = 28.0;
        let beta = 8.0 / 3.0;
        self.state = [
            sigma * (y - x),
            x * (rho - z) - y,
            x * y - beta * z,
        ];
    }

    pub fn is_chaotic(&self) -> bool {
        self.lyapunov_exponent > 0.0
    }
}

impl fmt::Display for ChaoticDynamics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChaoticDynamics(state=[{:.2}, {:.2}, {:.2}], λ={:.2})", self.state[0], self.state[1], self.state[2], self.lyapunov_exponent)
    }
}

