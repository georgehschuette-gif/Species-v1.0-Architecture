// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldsError {
    InvalidPotential(String),
    InvalidGradient(String),
    InvalidCapacity(String),
}

impl fmt::Display for FieldsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldsError::InvalidPotential(msg) => write!(f, "Invalid potential: {}", msg),
            FieldsError::InvalidGradient(msg) => write!(f, "Invalid gradient: {}", msg),
            FieldsError::InvalidCapacity(msg) => write!(f, "Invalid capacity: {}", msg),
        }
    }
}

impl std::error::Error for FieldsError {}

pub struct OpportunityFields {
    pub field_id: u64,
    pub potential: f64,
    pub gradient: f64,
    pub capacity: u64,
    pub exploited: u64,
}

impl OpportunityFields {
    pub fn new(
        field_id: u64,
        potential: f64,
        gradient: f64,
        capacity: u64,
    ) -> Result<Self, FieldsError> {
        if !(0.0..=1.0).contains(&potential) {
            return Err(FieldsError::InvalidPotential(format!(
                "Potential {} out of range",
                potential
            )));
        }
        if gradient < 0.0 {
            return Err(FieldsError::InvalidGradient(format!(
                "Gradient {} must be non-negative",
                gradient
            )));
        }
        if capacity == 0 {
            return Err(FieldsError::InvalidCapacity(
                "Capacity must be positive".to_string(),
            ));
        }
        Ok(Self {
            field_id,
            potential,
            gradient,
            capacity,
            exploited: 0,
        })
    }

    pub fn exploit(&mut self, amount: u64) -> Result<f64, FieldsError> {
        if self.exploited >= self.capacity {
            return Err(FieldsError::InvalidCapacity(
                "Field capacity exhausted".to_string(),
            ));
        }
        let available = self.capacity - self.exploited;
        let take = amount.min(available);
        self.exploited += take;
        let yield_ =
            take as f64 * self.potential * (1.0 - self.exploited as f64 / self.capacity as f64);
        Ok(yield_)
    }

    pub fn potential(&self) -> f64 {
        self.potential
    }

    pub fn remaining_capacity(&self) -> u64 {
        self.capacity - self.exploited
    }
}

impl fmt::Display for OpportunityFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OpportunityFields(id={}, potential={:.2}, gradient={:.2}, used={}/{})",
            self.field_id, self.potential, self.gradient, self.exploited, self.capacity
        )
    }
}
