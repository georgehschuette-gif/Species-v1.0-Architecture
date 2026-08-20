// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderParameter {
    pub value: f64,
    pub susceptibility: f64,
}

impl OrderParameter {
    pub fn new(value: f64, susceptibility: f64) -> Result<Self, crate::DynamicsError> {
        if value < 0.0 {
            return Err(crate::DynamicsError::TransitionError("Order parameter must be non-negative".to_string()));
        }
        if susceptibility < 0.0 {
            return Err(crate::DynamicsError::TransitionError("Susceptibility must be non-negative".to_string()));
        }
        Ok(Self { value, susceptibility })
    }

    pub fn critical_exponent(&self) -> f64 {
        if self.value > 1e-6 {
            (self.value).ln()
        } else {
            0.0
        }
    }
}

impl fmt::Display for OrderParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderParameter(value={:.2}, susceptibility={:.2})", self.value, self.susceptibility)
    }
}

