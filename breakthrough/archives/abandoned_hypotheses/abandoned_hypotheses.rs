// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AbandonedHypothesisError {
    InvalidYear(i32),
    EmptyDescription,
    NegativeFeasibility,
}

impl fmt::Display for AbandonedHypothesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidYear(year) => write!(f, "invalid abandonment year: {}", year),
            Self::EmptyDescription => write!(f, "hypothesis description must not be empty"),
            Self::NegativeFeasibility => write!(f, "revival feasibility must not be negative"),
        }
    }
}

impl std::error::Error for AbandonedHypothesisError {}

pub struct AbandonedHypothesis {
    pub id: u64,
    pub description: String,
    pub abandonment_year: i32,
    pub reason: String,
    pub revival_feasibility: f64,
}

impl AbandonedHypothesis {
    pub fn new(
        id: u64,
        description: String,
        abandonment_year: i32,
        reason: String,
        revival_feasibility: f64,
    ) -> Result<Self, AbandonedHypothesisError> {
        if abandonment_year <= 0 {
            return Err(AbandonedHypothesisError::InvalidYear(abandonment_year));
        }
        if description.trim().is_empty() {
            return Err(AbandonedHypothesisError::EmptyDescription);
        }
        if revival_feasibility < 0.0 {
            return Err(AbandonedHypothesisError::NegativeFeasibility);
        }
        Ok(Self {
            id,
            description,
            abandonment_year,
            reason,
            revival_feasibility,
        })
    }

    pub fn is_revivable(&self) -> bool {
        self.revival_feasibility > 0.5
    }

    pub fn age(&self, current_year: i32) -> i32 {
        current_year - self.abandonment_year
    }

    pub fn feasibility_score(&self) -> f64 {
        self.revival_feasibility * (1.0 / (1.0 + (self.age(2026) as f64 * 0.01)))
    }
}

impl fmt::Display for AbandonedHypothesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AbandonedHypothesis(id={}, year={}, revivable={}, feasibility={:.2})",
            self.id,
            self.abandonment_year,
            self.is_revivable(),
            self.feasibility_score()
        )
    }
}
