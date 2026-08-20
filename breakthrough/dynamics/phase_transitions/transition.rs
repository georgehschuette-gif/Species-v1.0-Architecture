// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
    FirstOrder,
    SecondOrder,
    Continuous,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub transition_type: TransitionType,
    pub critical_temperature: f64,
    pub order_parameter_value: f64,
}

impl Transition {
    pub fn new(transition_type: TransitionType, critical_temperature: f64) -> Self {
        Self { transition_type, critical_temperature, order_parameter_value: 0.0 }
    }

    pub fn is_critical(&self, temperature: f64) -> bool {
        (temperature - self.critical_temperature).abs() < 1e-6
    }

    pub fn set_order_parameter(&mut self, value: f64) -> Result<(), crate::DynamicsError> {
        if value < 0.0 {
            return Err(crate::DynamicsError::TransitionError("Order parameter must be non-negative".to_string()));
        }
        self.order_parameter_value = value;
        Ok(())
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transition(type={:?}, Tc={:.2}, order={:.2})", self.transition_type, self.critical_temperature, self.order_parameter_value)
    }
}

