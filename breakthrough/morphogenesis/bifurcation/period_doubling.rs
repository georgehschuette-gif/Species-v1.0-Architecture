// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct PeriodDoubling {
    pub parameter: f64,
    pub period: usize,
    pub bifurcation_points: Vec<f64>,
}

impl PeriodDoubling {
    pub fn new(parameter: f64) -> Self {
        Self { parameter, period: 1, bifurcation_points: vec![parameter] }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn period(&self) -> usize { self.period }
    pub fn double(&mut self, new_parameter: f64) -> Result<(), MorphogenesisError> {
        if self.period >= 16 { return Err(MorphogenesisError::CapacityExceeded { max: 16, attempted: self.period + 1 }); }
        self.period *= 2;
        self.parameter = new_parameter;
        self.bifurcation_points.push(new_parameter);
        Ok(())
    }
    pub fn bifurcation_count(&self) -> usize { self.bifurcation_points.len() }
    pub fn is_chaotic(&self) -> bool { self.period >= 8 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.period == 0 { return Err(MorphogenesisError::OutOfRange { field: "period".into(), value: 0.0, min: 1.0, max: 16.0 }); }
        Ok(())
    }
}

impl fmt::Debug for PeriodDoubling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeriodDoubling").field("period", &self.period).field("bifurcations", &self.bifurcation_points.len()).finish()
    }
}
