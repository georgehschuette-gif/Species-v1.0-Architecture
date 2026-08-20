// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerLaw {
    exponent: f64,
    x_min: f64,
    x_max: f64,
}

impl PowerLaw {
    pub fn new(exponent: f64, x_min: f64, x_max: f64) -> crate::Result<Self> {
        if exponent <= 1.0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                "power law exponent must be greater than 1.0".to_string(),
            ));
        }
        if x_min <= 0.0 || x_max <= x_min {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                "invalid range: x_min must be positive and less than x_max".to_string(),
            ));
        }
        Ok(PowerLaw {
            exponent,
            x_min,
            x_max,
        })
    }

    pub fn exponent(&self) -> f64 {
        self.exponent
    }

    pub fn x_min(&self) -> f64 {
        self.x_min
    }

    pub fn x_max(&self) -> f64 {
        self.x_max
    }

    pub fn pdf(&self, x: f64) -> f64 {
        if x < self.x_min || x > self.x_max {
            return 0.0;
        }
        let numerator = (self.exponent - 1.0) * self.x_min.powf(self.exponent - 1.0);
        let denominator = x.powf(self.exponent);
        numerator / denominator
    }

    pub fn cdf(&self, x: f64) -> f64 {
        if x < self.x_min {
            return 0.0;
        }
        if x >= self.x_max {
            return 1.0;
        }
        1.0 - (self.x_min / x).powf(self.exponent - 1.0)
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.exponent <= 1.0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                format!("invalid exponent: {}", self.exponent),
            ));
        }
        if self.x_min <= 0.0 || self.x_max <= self.x_min {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                format!("invalid range: [{}, {}]", self.x_min, self.x_max),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for PowerLaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PowerLaw(exponent={:.2}, range=[{:.2}, {:.2}])",
            self.exponent, self.x_min, self.x_max
        )
    }
}
