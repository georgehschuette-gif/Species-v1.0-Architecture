// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Regulator {
    pub gain: f64,
    pub integral: f64,
    pub derivative: f64,
    pub last_error: f64,
}

impl Regulator {
    pub fn new(gain: f64, integral: f64, derivative: f64) -> Self {
        Self { gain, integral, derivative, last_error: 0.0 }
    }

    pub fn compute(&mut self, error: f64) -> f64 {
        let output = self.gain * error + self.integral * self.last_error + self.derivative * (error - self.last_error);
        self.last_error = error;
        output
    }
}

impl fmt::Display for Regulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Regulator(gain={:.2}, integral={:.2}, derivative={:.2})", self.gain, self.integral, self.derivative)
    }
}

