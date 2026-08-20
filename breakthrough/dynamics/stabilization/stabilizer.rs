// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Stabilizer {
    pub setpoint: f64,
    pub tolerance: f64,
    pub measurement: f64,
}

impl Stabilizer {
    pub fn new(setpoint: f64, tolerance: f64) -> Self {
        Self { setpoint, tolerance, measurement: setpoint }
    }

    pub fn measure(&mut self, value: f64) -> Result<(), crate::DynamicsError> {
        self.measurement = value;
        if (value - self.setpoint).abs() > self.tolerance {
            return Err(crate::DynamicsError::StabilizationFailure(
                format!("Measurement {:.2} out of tolerance {:.2}", value, self.tolerance)
            ));
        }
        Ok(())
    }

    pub fn deviation(&self) -> f64 {
        (self.measurement - self.setpoint).abs()
    }
}

impl fmt::Display for Stabilizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stabilizer(setpoint={:.2}, measurement={:.2}, deviation={:.2})", self.setpoint, self.measurement, self.deviation())
    }
}

