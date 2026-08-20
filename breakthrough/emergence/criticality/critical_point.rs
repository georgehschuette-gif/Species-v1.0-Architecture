// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriticalPoint {
    system_size: usize,
    order_parameter: f64,
    susceptibility: f64,
}

impl CriticalPoint {
    pub fn new(system_size: usize, order_parameter: f64, susceptibility: f64) -> crate::Result<Self> {
        if system_size == 0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                "system size must be positive".to_string(),
            ));
        }
        if susceptibility < 0.0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                "susceptibility must be non-negative".to_string(),
            ));
        }
        Ok(CriticalPoint {
            system_size,
            order_parameter,
            susceptibility,
        })
    }

    pub fn system_size(&self) -> usize {
        self.system_size
    }

    pub fn order_parameter(&self) -> f64 {
        self.order_parameter
    }

    pub fn susceptibility(&self) -> f64 {
        self.susceptibility
    }

    pub fn is_critical(&self, threshold: f64) -> bool {
        self.susceptibility > threshold
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.system_size == 0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                "system size is zero".to_string(),
            ));
        }
        if self.susceptibility < 0.0 {
            return Err(crate::EmergenceError::CriticalPointUnstable(
                format!("invalid susceptibility: {}", self.susceptibility),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for CriticalPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CriticalPoint(size={}, order={:.3}, susceptibility={:.3})",
            self.system_size, self.order_parameter, self.susceptibility
        )
    }
}
