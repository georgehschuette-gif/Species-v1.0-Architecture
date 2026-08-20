// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct PositiveFeedback {
    pub gain: f64,
    pub saturation_point: f64,
    pub current_output: f64,
    pub is_saturated: bool,
}

impl PositiveFeedback {
    pub fn new(gain: f64, saturation_point: f64) -> Self {
        Self { gain, saturation_point, current_output: 0.0, is_saturated: false }
    }

    pub fn feed(&mut self, input: f64) -> Result<f64, crate::DynamicsError> {
        if self.is_saturated {
            return Ok(self.current_output);
        }
        let output = self.current_output + self.gain * input;
        if output >= self.saturation_point {
            self.is_saturated = true;
            self.current_output = self.saturation_point;
        } else {
            self.current_output = output;
        }
        Ok(self.current_output)
    }
}

impl fmt::Display for PositiveFeedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PositiveFeedback(gain={:.2}, output={:.2}, saturated={})", self.gain, self.current_output, self.is_saturated)
    }
}

